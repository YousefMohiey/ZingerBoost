use async_trait::async_trait;
use std::sync::Arc;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{
    RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW, HKEY,
    HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS,
    KEY_ALL_ACCESS, KEY_READ, REG_BINARY, REG_DWORD, REG_EXPAND_SZ, REG_MULTI_SZ, REG_QWORD,
    REG_SZ, REG_VALUE_TYPE,
};
use zb_domain::errors::RegistryError;
use zb_domain::registry::RegistryProvider;
use zb_shared::types::{RegPath, RegRoot, RegValue};

/// Helper: decode a Windows UTF-16 LE buffer, stopping at null terminator.
/// Uses chunks(2) instead of chunks_exact to handle odd-length buffers gracefully.
fn decode_utf16_buffer(buffer: &[u8]) -> Vec<u16> {
    buffer
        .chunks(2)
        .filter(|c| c.len() == 2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .take_while(|&c| c != 0)
        .collect()
}

/// Windows registry provider using windows-rs
#[derive(Debug)]
pub struct WinRegistryProvider;

impl WinRegistryProvider {
    pub fn new() -> Arc<dyn RegistryProvider> {
        Arc::new(Self)
    }

    fn root_to_hkey(&self, root: &RegRoot) -> HKEY {
        match root {
            RegRoot::Hkcr => HKEY_CLASSES_ROOT,
            RegRoot::Hkcu => HKEY_CURRENT_USER,
            RegRoot::Hklm => HKEY_LOCAL_MACHINE,
            RegRoot::Hku => HKEY_USERS,
            RegRoot::Hkcc => HKEY_CURRENT_CONFIG,
        }
    }

    fn open_key(
        &self,
        path: &RegPath,
        access: windows::Win32::System::Registry::REG_SAM_FLAGS,
    ) -> Result<HKEY, RegistryError> {
        let root = self.root_to_hkey(&path.root);
        let wide_path: Vec<u16> = path.path.encode_utf16().chain(std::iter::once(0)).collect();
        let mut hkey = HKEY::default();
        let result = unsafe {
            RegOpenKeyExW(
                root,
                windows::core::PCWSTR(wide_path.as_ptr()),
                0,
                access,
                &mut hkey,
            )
        };
        if result == ERROR_SUCCESS {
            Ok(hkey)
        } else if result == windows::Win32::Foundation::ERROR_ACCESS_DENIED {
            Err(RegistryError::AccessDenied)
        } else if result == windows::Win32::Foundation::ERROR_FILE_NOT_FOUND {
            Err(RegistryError::ReadFailed(format!(
                "Registry key not found: {}\\{}",
                path.path, path.root
            )))
        } else {
            Err(RegistryError::ReadFailed(format!(
                "RegOpenKeyExW failed: 0x{:08X}",
                result.0
            )))
        }
    }

    fn read_raw_value(
        &self,
        hkey: HKEY,
        name: &str,
    ) -> Result<(Vec<u8>, REG_VALUE_TYPE), RegistryError> {
        let wide_name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        let mut data_type = REG_VALUE_TYPE(0);
        let mut data_size: u32 = 0;

        let result = unsafe {
            RegQueryValueExW(
                hkey,
                windows::core::PCWSTR(wide_name.as_ptr()),
                None,
                Some(&mut data_type),
                None,
                Some(&mut data_size),
            )
        };

        if result != ERROR_SUCCESS {
            return Err(RegistryError::ValueNotFound);
        }

        let mut buffer = vec![0u8; data_size as usize];
        let result = unsafe {
            RegQueryValueExW(
                hkey,
                windows::core::PCWSTR(wide_name.as_ptr()),
                None,
                Some(&mut data_type),
                Some(buffer.as_mut_ptr()),
                Some(&mut data_size),
            )
        };

        if result != ERROR_SUCCESS {
            return Err(RegistryError::ReadFailed(format!(
                "Query failed: {:?}",
                result
            )));
        }

        buffer.truncate(data_size as usize);
        Ok((buffer, data_type))
    }
}

#[async_trait]
impl RegistryProvider for WinRegistryProvider {
    async fn read(&self, path: &RegPath, name: &str) -> Result<RegValue, RegistryError> {
        let hkey = self.open_key(path, KEY_READ)?;
        let raw_result = self.read_raw_value(hkey, name);
        let (buffer, data_type) = match raw_result {
            Ok(v) => v,
            Err(e) => {
                let _ = unsafe { RegCloseKey(hkey) };
                return Err(e);
            }
        };
        let close_result = unsafe { RegCloseKey(hkey) };
        if close_result != ERROR_SUCCESS {
            tracing::warn!("Failed to close registry key: {:?}", close_result);
        }

        match data_type {
            REG_DWORD => {
                if buffer.len() >= 4 {
                    let value = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
                    Ok(RegValue::Dword(value))
                } else {
                    Err(RegistryError::ReadFailed("Invalid DWORD size".into()))
                }
            }
            REG_QWORD => {
                if buffer.len() >= 8 {
                    let value = u64::from_le_bytes([
                        buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5],
                        buffer[6], buffer[7],
                    ]);
                    Ok(RegValue::Qword(value))
                } else {
                    Err(RegistryError::ReadFailed("Invalid QWORD size".into()))
                }
            }
            REG_SZ => {
                let wide = decode_utf16_buffer(&buffer);
                let string = String::from_utf16(&wide).map_err(|e| {
                    RegistryError::ReadFailed(format!("UTF-16 decode error: {}", e))
                })?;
                Ok(RegValue::Sz(string))
            }
            REG_EXPAND_SZ => {
                let wide = decode_utf16_buffer(&buffer);
                let string = String::from_utf16(&wide).map_err(|e| {
                    RegistryError::ReadFailed(format!("UTF-16 decode error: {}", e))
                })?;
                Ok(RegValue::ExpandSz(string))
            }
            REG_MULTI_SZ => {
                let raw = String::from_utf16(&decode_utf16_buffer(&buffer))
                    .map_err(|e| RegistryError::ReadFailed(format!("UTF-16 decode error: {}", e)))?;
                let strings: Vec<String> = raw
                    .split('\0')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();
                Ok(RegValue::MultiSz(strings))
            }
            REG_BINARY => Ok(RegValue::Binary(buffer)),
            _ => Err(RegistryError::ReadFailed(format!(
                "Unsupported registry type: {:?}",
                data_type
            ))),
        }
    }

    async fn write(&self, path: &RegPath, name: &str, val: &RegValue) -> Result<(), RegistryError> {
        let hkey = self.open_key(path, KEY_ALL_ACCESS)?;
        let wide_name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();

        let result = match val {
            RegValue::Dword(v) => unsafe {
                RegSetValueExW(
                    hkey,
                    windows::core::PCWSTR(wide_name.as_ptr()),
                    0,
                    REG_DWORD,
                    Some(&v.to_le_bytes()),
                )
            },
            RegValue::Qword(v) => unsafe {
                RegSetValueExW(
                    hkey,
                    windows::core::PCWSTR(wide_name.as_ptr()),
                    0,
                    REG_QWORD,
                    Some(&v.to_le_bytes()),
                )
            },
            RegValue::Sz(v) => {
                let wide: Vec<u16> = v.encode_utf16().chain(std::iter::once(0)).collect();
                let bytes: Vec<u8> = wide.iter().flat_map(|&c| c.to_le_bytes()).collect();
                unsafe {
                    RegSetValueExW(
                        hkey,
                        windows::core::PCWSTR(wide_name.as_ptr()),
                        0,
                        REG_SZ,
                        Some(&bytes),
                    )
                }
            }
            RegValue::ExpandSz(v) => {
                let wide: Vec<u16> = v.encode_utf16().chain(std::iter::once(0)).collect();
                let bytes: Vec<u8> = wide.iter().flat_map(|&c| c.to_le_bytes()).collect();
                unsafe {
                    RegSetValueExW(
                        hkey,
                        windows::core::PCWSTR(wide_name.as_ptr()),
                        0,
                        REG_EXPAND_SZ,
                        Some(&bytes),
                    )
                }
            }
            RegValue::Binary(v) => unsafe {
                RegSetValueExW(
                    hkey,
                    windows::core::PCWSTR(wide_name.as_ptr()),
                    0,
                    REG_BINARY,
                    Some(v),
                )
            },
            RegValue::Absent => {
                let _ = unsafe { RegCloseKey(hkey) };
                return self.delete(path, name).await;
            }
            RegValue::MultiSz(v) => {
                let flat: String = v.join("\0");
                let wide: Vec<u16> = flat.encode_utf16().chain(std::iter::once(0)).collect();
                let bytes: Vec<u8> = wide.iter().flat_map(|&c| c.to_le_bytes()).collect();
                unsafe {
                    RegSetValueExW(
                        hkey,
                        windows::core::PCWSTR(wide_name.as_ptr()),
                        0,
                        REG_MULTI_SZ,
                        Some(&bytes),
                    )
                }
            }
        };

        let close_result = unsafe { RegCloseKey(hkey) };
        if close_result != ERROR_SUCCESS {
            tracing::warn!("Failed to close registry key: {:?}", close_result);
        }

        if result == ERROR_SUCCESS {
            Ok(())
        } else {
            Err(RegistryError::WriteFailed(format!(
                "RegSetValueExW failed: {:?}",
                result
            )))
        }
    }

    async fn delete(&self, path: &RegPath, name: &str) -> Result<(), RegistryError> {
        let hkey = self.open_key(path, KEY_ALL_ACCESS)?;
        let wide_name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        let result = unsafe { RegDeleteValueW(hkey, windows::core::PCWSTR(wide_name.as_ptr())) };
        let close_result = unsafe { RegCloseKey(hkey) };
        if close_result != ERROR_SUCCESS {
            tracing::warn!("Failed to close registry key: {:?}", close_result);
        }

        if result == ERROR_SUCCESS {
            Ok(())
        } else {
            Err(RegistryError::DeleteFailed(format!(
                "RegDeleteValueW failed: {:?}",
                result
            )))
        }
    }
}
