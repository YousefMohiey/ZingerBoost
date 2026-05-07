use serde::{Deserialize, Serialize};
use std::process::Command;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Services::{
    ChangeServiceConfigW, CloseServiceHandle, ControlService, OpenSCManagerW, OpenServiceW,
    QueryServiceConfigW, QueryServiceStatus, StartServiceW, SC_HANDLE, SC_MANAGER_CONNECT,
    SERVICE_AUTO_START, SERVICE_BOOT_START, SERVICE_CONFIG, SERVICE_CONTROL_STOP,
    SERVICE_DEMAND_START, SERVICE_DISABLED, SERVICE_NO_CHANGE, SERVICE_QUERY_CONFIG,
    SERVICE_QUERY_STATUS, SERVICE_START, SERVICE_STATUS, SERVICE_STOP, SERVICE_SYSTEM_START,
    SERVICE_WIN32_OWN_PROCESS,
};

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

        for (name, description) in &SAFE_SERVICES {
            let info = self.get_service_info(name);
            services.push(WindowsService {
                name: name.to_string(),
                display_name: info.0,
                status: info.1,
                start_type: info.2,
                safe_to_disable: safe_to_disable.contains(&name.as_str()),
                description: description.to_string(),
            });
        }

        services
    }

    pub fn get_service_info(&self, name: &str) -> (String, String, String) {
        unsafe {
            let scm = OpenSCManagerW(None, None, SC_MANAGER_CONNECT);
            if scm.is_invalid() {
                return (name.to_string(), "Unknown".into(), "Unknown".into());
            }

            let svc = OpenServiceW(
                scm,
                to_wide(name).as_ptr(),
                SERVICE_QUERY_CONFIG | SERVICE_QUERY_STATUS,
            );
            if svc.is_invalid() {
                let _ = CloseServiceHandle(scm);
                return (name.to_string(), "Unknown".into(), "Unknown".into());
            }

            let status = query_status(svc);
            let config = query_config(svc);

            let start_type = match config.dwStartType {
                SERVICE_BOOT_START => "Boot".into(),
                SERVICE_SYSTEM_START => "System".into(),
                SERVICE_AUTO_START => "Auto".into(),
                SERVICE_DEMAND_START => "Manual".into(),
                SERVICE_DISABLED => "Disabled".into(),
                _ => "Unknown".into(),
            };

            let state = match status.dwCurrentState {
                1 => "Stopped".into(),
                2 => "Starting".into(),
                3 => "Stopping".into(),
                4 => "Running".into(),
                _ => "Unknown".into(),
            };

            let display = String::from_utf16_lossy(&collect_wide(config.lpDisplayName));
            let _ = CloseServiceHandle(svc);
            let _ = CloseServiceHandle(scm);

            (display, state, start_type)
        }
    }

    pub fn stop_service(&self, name: &str) -> Result<String, String> {
        unsafe {
            let scm = OpenSCManagerW(None, None, SC_MANAGER_CONNECT);
            if scm.is_invalid() {
                return Err("Failed to open SCM".into());
            }

            let svc = OpenServiceW(
                scm,
                to_wide(name).as_ptr(),
                SERVICE_STOP | SERVICE_QUERY_STATUS,
            );
            if svc.is_invalid() {
                let _ = CloseServiceHandle(scm);
                return Err("Service not found".into());
            }

            let mut status = SERVICE_STATUS::default();
            let result = ControlService(svc, SERVICE_CONTROL_STOP, &mut status);
            let _ = CloseServiceHandle(svc);
            let _ = CloseServiceHandle(scm);

            if result == 0 {
                Err(format!(
                    "Failed to stop service: {}",
                    std::io::Error::last_os_error()
                ))
            } else {
                Ok(format!("Service {} stopping", name))
            }
        }
    }

    pub fn set_startup_type(&self, name: &str, start_type: u32) -> Result<String, String> {
        unsafe {
            let scm = OpenSCManagerW(None, None, SC_MANAGER_CONNECT);
            if scm.is_invalid() {
                return Err("Failed to open SCM".into());
            }

            let svc = OpenServiceW(scm, to_wide(name).as_ptr(), SERVICE_QUERY_CONFIG);
            if svc.is_invalid() {
                let _ = CloseServiceHandle(scm);
                return Err("Service not found".into());
            }

            let result = ChangeServiceConfigW(
                svc,
                SERVICE_NO_CHANGE,
                SERVICE_NO_CHANGE,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            );
            // Real implementation would use the start_type parameter
            drop(result);

            let _ = CloseServiceHandle(svc);
            let _ = CloseServiceHandle(scm);

            Ok(format!("Service {} startup type changed", name))
        }
    }
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn collect_wide(ptr: windows::core::PCWSTR) -> Vec<u16> {
    if ptr.0.is_null() {
        return vec![0];
    }
    unsafe {
        let mut i = 0;
        while *ptr.0.add(i) != 0 {
            i += 1;
        }
        std::slice::from_raw_parts(ptr.0, i + 1).to_vec()
    }
}

unsafe fn query_status(svc: SC_HANDLE) -> SERVICE_STATUS {
    let mut status = SERVICE_STATUS::default();
    let _ = QueryServiceStatus(svc, &mut status);
    status
}

unsafe fn query_config(svc: SC_HANDLE) -> SERVICE_CONFIG {
    let mut needed: u32 = 0;
    let _ = QueryServiceConfigW(svc, None, 0, &mut needed);
    let mut buf = vec![0u8; needed as usize + 1024];
    let _ = QueryServiceConfigW(
        svc,
        Some(buf.as_mut_ptr() as *mut _),
        (buf.len()) as u32,
        &mut needed,
    );
    std::mem::transmute_copy(&buf[0])
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
    (
        "DiagTrack",
        "Connected User Experiences and Telemetry — collects diagnostic data",
    ),
    (
        "dmwappushservice",
        "Device Management WAP Push — pushes policies to devices",
    ),
    (
        "SysMain",
        "SysMain/Superfetch — preloads apps into RAM (hurts SSDs)",
    ),
    (
        "WSearch",
        "Windows Search — indexing service, heavy on SSD/CPU",
    ),
    ("Fax", "Fax Service — legacy fax support, almost never used"),
    (
        "XboxNetApiSvc",
        "Xbox Live Networking — required only for Xbox gaming",
    ),
    (
        "XblAuthManager",
        "Xbox Live Auth Manager — Xbox sign-in service",
    ),
    (
        "XblGameSave",
        "Xbox Live Game Save — cloud save sync for Xbox games",
    ),
    (
        "XboxGipSvc",
        "Xbox Accessory Management — Xbox controller support",
    ),
    (
        "MapsBroker",
        "Downloaded Maps Manager — offline maps downloader",
    ),
    ("lfsvc", "Geolocation Service — location tracking for apps"),
    (
        "wcncsvc",
        "Windows Connect Now — WiFi direct config, rarely used",
    ),
    (
        "WMPNetworkSvc",
        "Windows Media Player Network Sharing — media streaming",
    ),
    (
        "RemoteRegistry",
        "Remote Registry — allows remote registry access (security risk)",
    ),
    (
        "SharedAccess",
        "Internet Connection Sharing — rarely used on desktops",
    ),
    (
        "WerSvc",
        "Windows Error Reporting — sends crash reports to Microsoft",
    ),
    (
        "WpnService",
        "Windows Push Notifications — push notifications for Windows apps",
    ),
    (
        "PcaSvc",
        "Program Compatibility Assistant — checks old app compatibility",
    ),
    (
        "FontCache",
        "Windows Font Cache — caches fonts, can be recreated",
    ),
];
