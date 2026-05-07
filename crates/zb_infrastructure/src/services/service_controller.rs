use serde::{Deserialize, Serialize};
use windows::core::PCWSTR;

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
        let safe = get_safe_to_disable_list();
        SAFE_SERVICES
            .iter()
            .map(|(name, desc)| {
                let info = self.get_service_info(name);
                WindowsService {
                    name: name.to_string(),
                    display_name: info.0,
                    status: info.1,
                    start_type: info.2,
                    safe_to_disable: safe.contains(name),
                    description: desc.to_string(),
                }
            })
            .collect()
    }

    pub fn get_service_info(&self, name: &str) -> (String, String, String) {
        use windows::Win32::System::Services::{
            CloseServiceHandle, OpenSCManagerW, OpenServiceW, QueryServiceConfigW,
            QueryServiceStatus, SC_MANAGER_CONNECT, SERVICE_CONFIG, SERVICE_QUERY_CONFIG,
            SERVICE_QUERY_STATUS,
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

            let status = get_status(svc);
            let config = get_config(svc);

            let start_type = match config.0.dwStartType {
                s if s == 0 => "Boot",
                s if s == 1 => "System",
                s if s == 2 => "Auto",
                s if s == 3 => "Manual",
                s if s == 4 => "Disabled",
                _ => "Unknown",
            };

            let state = match status.dwCurrentState.0 {
                1 => "Stopped",
                2 => "Starting",
                3 => "Stopping",
                4 => "Running",
                _ => "Unknown",
            };

            let display = extract_display(&config);

            let _ = CloseServiceHandle(svc);
            let _ = CloseServiceHandle(scm);
            (display, state.to_string(), start_type.to_string())
        }
    }

    pub fn stop_service(&self, name: &str) -> Result<String, String> {
        use windows::Win32::System::Services::{
            CloseServiceHandle, ControlService, OpenSCManagerW, OpenServiceW, SC_MANAGER_CONNECT,
            SERVICE_CONTROL_STOP, SERVICE_QUERY_STATUS, SERVICE_STATUS, SERVICE_STOP,
        };

        unsafe {
            let scm = OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_CONNECT)
                .map_err(|e| format!("SCM: {:?}", e))?;
            let wide = to_wide(name);
            let svc = OpenServiceW(
                scm,
                PCWSTR::from_raw(wide.as_ptr()),
                SERVICE_STOP | SERVICE_QUERY_STATUS,
            )
            .map_err(|e| {
                let _ = CloseServiceHandle(scm);
                format!("OpenService: {:?}", e)
            })?;
            let mut s = SERVICE_STATUS::default();
            let _ = ControlService(svc, SERVICE_CONTROL_STOP, &mut s);
            let _ = CloseServiceHandle(svc);
            let _ = CloseServiceHandle(scm);
            Ok(format!("{} stopping", name))
        }
    }

    pub fn set_startup_type(&self, name: &str, _start_type: u32) -> Result<String, String> {
        use windows::Win32::System::Services::{
            ChangeServiceConfigW, CloseServiceHandle, OpenSCManagerW, OpenServiceW,
            SC_MANAGER_CONNECT, SERVICE_QUERY_CONFIG,
        };

        unsafe {
            let scm = OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_CONNECT)
                .map_err(|e| format!("SCM: {:?}", e))?;
            let wide = to_wide(name);
            let svc = OpenServiceW(scm, PCWSTR::from_raw(wide.as_ptr()), SERVICE_QUERY_CONFIG)
                .map_err(|e| {
                    let _ = CloseServiceHandle(scm);
                    format!("OpenService: {:?}", e)
                })?;

            let _ = ChangeServiceConfigW(
                svc,
                windows::Win32::System::Services::SERVICE_WIN32_OWN_PROCESS,
                windows::Win32::System::Services::SERVICE_DISABLED,
                PCWSTR::null(),
                PCWSTR::null(),
                None,
                None,
                PCWSTR::null(),
                PCWSTR::null(),
                None,
                PCWSTR::null(),
            );

            let _ = CloseServiceHandle(svc);
            let _ = CloseServiceHandle(scm);
            Ok(format!("{} configured", name))
        }
    }
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

unsafe fn get_status(
    svc: windows::Win32::System::Services::SC_HANDLE,
) -> windows::Win32::System::Services::SERVICE_STATUS {
    let mut s = windows::Win32::System::Services::SERVICE_STATUS::default();
    let _ = windows::Win32::System::Services::QueryServiceStatus(svc, &mut s);
    s
}

unsafe fn get_config(
    svc: windows::Win32::System::Services::SC_HANDLE,
) -> windows::Win32::System::Services::SERVICE_CONFIG {
    use windows::Win32::System::Services::QueryServiceConfigW;
    let mut needed: u32 = 0;
    let _ = QueryServiceConfigW(svc, None, 0, &mut needed);
    let mut buf = vec![0u8; needed as usize + 1024];
    let _ = QueryServiceConfigW(
        svc,
        Some(buf.as_mut_ptr() as *mut _),
        buf.len() as u32,
        &mut needed,
    );
    std::mem::transmute_copy(&buf[0])
}

unsafe fn extract_display(_config: &windows::Win32::System::Services::SERVICE_CONFIG) -> String {
    "Service".to_string()
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
    ("WSearch", "Windows Search — indexing service"),
    ("Fax", "Fax Service"),
    ("XboxNetApiSvc", "Xbox Live Networking"),
    ("XblAuthManager", "Xbox Live Auth Manager"),
    ("XblGameSave", "Xbox Live Game Save"),
    ("XboxGipSvc", "Xbox Accessory Management"),
    ("MapsBroker", "Downloaded Maps Manager"),
    ("lfsvc", "Geolocation Service"),
    ("wcncsvc", "Windows Connect Now"),
    ("WMPNetworkSvc", "Windows Media Player Network Sharing"),
    ("RemoteRegistry", "Remote Registry"),
    ("SharedAccess", "Internet Connection Sharing"),
    ("WerSvc", "Windows Error Reporting"),
    ("WpnService", "Windows Push Notifications"),
    ("PcaSvc", "Program Compatibility Assistant"),
    ("FontCache", "Windows Font Cache"),
];
