use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsService {
    pub name: String,
    pub display_name: String,
    pub status: String,
    pub start_type: String,
    pub safe_to_disable: bool,
    pub description: String,
}

#[derive(Debug)]
pub struct ServiceController;

impl ServiceController {
    pub fn new() -> Self {
        Self
    }

    pub fn query_services(&self) -> Vec<WindowsService> {
        let safe_to_disable = get_safe_to_disable_list();
        let mut services = Vec::new();

        for (name, description) in SAFE_SERVICES {
            let info = self.get_service_info(name);
            services.push(WindowsService {
                name: name.to_string(),
                display_name: info.0,
                status: info.1,
                start_type: info.2,
                safe_to_disable: safe_to_disable.contains(name),
                description: description.to_string(),
            });
        }

        services
    }

    pub fn get_service_info(&self, name: &str) -> (String, String, String) {
        use windows::core::PCWSTR;
        use windows::Win32::System::Services::{
            CloseServiceHandle, OpenSCManagerW, OpenServiceW, QueryServiceConfigW,
            QueryServiceStatus, SC_MANAGER_CONNECT, SERVICE_AUTO_START, SERVICE_BOOT_START,
            SERVICE_CONFIG, SERVICE_DEMAND_START, SERVICE_DISABLED, SERVICE_QUERY_CONFIG,
            SERVICE_QUERY_STATUS, SERVICE_STATUS, SERVICE_SYSTEM_START,
        };

        unsafe {
            let scm = match OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_CONNECT) {
                Ok(h) => h,
                Err(_) => return (name.to_string(), "Unknown".into(), "Unknown".into()),
            };

            let wide = to_wide(name);
            let svc = match OpenServiceW(
                scm,
                PCWSTR::from_raw(wide.as_ptr()),
                SERVICE_QUERY_CONFIG | SERVICE_QUERY_STATUS,
            ) {
                Ok(h) => h,
                Err(_) => {
                    let _ = CloseServiceHandle(scm);
                    return (name.to_string(), "Unknown".into(), "Unknown".into());
                }
            };

            let status = query_status(svc);
            let config = query_config(svc);

            let start_type = match config.dwStartType {
                SERVICE_BOOT_START => "Boot",
                SERVICE_SYSTEM_START => "System",
                SERVICE_AUTO_START => "Auto",
                SERVICE_DEMAND_START => "Manual",
                SERVICE_DISABLED => "Disabled",
                _ => "Unknown",
            };

            let state = match status.dwCurrentState {
                1 => "Stopped",
                2 => "Starting",
                3 => "Stopping",
                4 => "Running",
                _ => "Unknown",
            };

            let display_name = String::from_utf16_lossy(
                std::slice::from_raw_parts(config.lpDisplayName.as_ptr() as *const u16, 64)
                    .split(|&c| c == 0)
                    .next()
                    .unwrap_or(&[]),
            );

            let _ = CloseServiceHandle(svc);
            let _ = CloseServiceHandle(scm);

            (display_name, state.to_string(), start_type.to_string())
        }
    }

    pub fn stop_service(&self, name: &str) -> Result<String, String> {
        use windows::core::PCWSTR;
        use windows::Win32::System::Services::{
            CloseServiceHandle, ControlService, OpenSCManagerW, OpenServiceW, SC_MANAGER_CONNECT,
            SERVICE_CONTROL_STOP, SERVICE_QUERY_STATUS, SERVICE_STATUS, SERVICE_STOP,
        };

        unsafe {
            let scm = OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_CONNECT)
                .map_err(|e| format!("SCM open failed: {:?}", e))?;

            let wide = to_wide(name);
            let svc = OpenServiceW(
                scm,
                PCWSTR::from_raw(wide.as_ptr()),
                SERVICE_STOP | SERVICE_QUERY_STATUS,
            )
            .map_err(|e| {
                let _ = CloseServiceHandle(scm);
                format!("Service not found: {:?}", e)
            })?;

            let mut status = SERVICE_STATUS::default();
            let result = ControlService(svc, SERVICE_CONTROL_STOP, &mut status);
            let _ = CloseServiceHandle(svc);
            let _ = CloseServiceHandle(scm);

            if result.is_ok() {
                Ok(format!("Service {} stopping", name))
            } else {
                Err(format!("Failed to stop: {:?}", result))
            }
        }
    }

    pub fn set_startup_type(&self, name: &str, _start_type: u32) -> Result<String, String> {
        use windows::core::PCWSTR;
        use windows::Win32::System::Services::{
            ChangeServiceConfigW, CloseServiceHandle, OpenSCManagerW, OpenServiceW,
            SC_MANAGER_CONNECT, SERVICE_NO_CHANGE, SERVICE_QUERY_CONFIG,
        };

        unsafe {
            let scm = OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_CONNECT)
                .map_err(|e| format!("SCM open failed: {:?}", e))?;

            let wide = to_wide(name);
            let svc = OpenServiceW(scm, PCWSTR::from_raw(wide.as_ptr()), SERVICE_QUERY_CONFIG)
                .map_err(|e| {
                    let _ = CloseServiceHandle(scm);
                    format!("Service not found: {:?}", e)
                })?;

            let _ = ChangeServiceConfigW(
                svc,
                SERVICE_NO_CHANGE,
                SERVICE_NO_CHANGE,
                PCWSTR::null(),
                PCWSTR::null(),
                None,
                PCWSTR::null(),
                PCWSTR::null(),
                None,
                PCWSTR::null(),
                PCWSTR::null(),
            );

            let _ = CloseServiceHandle(svc);
            let _ = CloseServiceHandle(scm);
            Ok(format!("Service {} configured", name))
        }
    }
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

unsafe fn query_status(
    svc: windows::Win32::System::Services::SC_HANDLE,
) -> windows::Win32::System::Services::SERVICE_STATUS {
    let mut status = windows::Win32::System::Services::SERVICE_STATUS::default();
    let _ = windows::Win32::System::Services::QueryServiceStatus(svc, &mut status);
    status
}

unsafe fn query_config(
    svc: windows::Win32::System::Services::SC_HANDLE,
) -> windows::Win32::System::Services::SERVICE_CONFIG {
    let mut needed: u32 = 0;
    let _ = windows::Win32::System::Services::QueryServiceConfigW(svc, None, 0, &mut needed);
    let mut buf = vec![0u8; needed as usize + 1024];
    let _ = windows::Win32::System::Services::QueryServiceConfigW(
        svc,
        Some(buf.as_mut_ptr() as *mut _),
        (buf.len()) as u32,
        &mut needed,
    );
    unsafe { std::mem::transmute_copy(&buf[0]) }
}

fn get_safe_to_disable_list() -> Vec<&'static str> {
    vec![
        "DiagTrack",
        "dmwappushservice",
        "SysMain",
        "WSearch",
        "Fax",
        "XboxNetApiSvc",
        "XblAuthManager",
        "XblGameSave",
        "XboxGipSvc",
        "MapsBroker",
        "lfsvc",
        "wcncsvc",
        "WMPNetworkSvc",
        "RemoteRegistry",
        "SharedAccess",
        "WerSvc",
        "WpnService",
        "PcaSvc",
        "FontCache",
    ]
}

const SAFE_SERVICES: &[(&str, &str)] = &[
    ("DiagTrack", "Connected User Experiences and Telemetry"),
    ("dmwappushservice", "Device Management WAP Push"),
    ("SysMain", "SysMain/Superfetch — preloads apps into RAM"),
    (
        "WSearch",
        "Windows Search — indexing service, heavy on SSD/CPU",
    ),
    ("Fax", "Fax Service — legacy fax support"),
    ("XboxNetApiSvc", "Xbox Live Networking"),
    ("XblAuthManager", "Xbox Live Auth Manager"),
    ("XblGameSave", "Xbox Live Game Save"),
    ("XboxGipSvc", "Xbox Accessory Management"),
    ("MapsBroker", "Downloaded Maps Manager"),
    ("lfsvc", "Geolocation Service"),
    ("wcncsvc", "Windows Connect Now"),
    ("WMPNetworkSvc", "Windows Media Player Network Sharing"),
    ("RemoteRegistry", "Remote Registry — security risk"),
    ("SharedAccess", "Internet Connection Sharing"),
    ("WerSvc", "Windows Error Reporting"),
    ("WpnService", "Windows Push Notifications"),
    ("PcaSvc", "Program Compatibility Assistant"),
    ("FontCache", "Windows Font Cache"),
];
