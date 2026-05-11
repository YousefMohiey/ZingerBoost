use serde::{Deserialize, Serialize};
use std::os::windows::process::CommandExt;
use std::process::Command;
use tracing;
use windows::core::PCWSTR;

const CREATE_NO_WINDOW: u32 = 0x08000000;

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
            .map(|(name, display, desc)| {
                let info = self.get_service_info(name);
                WindowsService {
                    name: name.to_string(),
                    display_name: display.to_string(),
                    status: info.0,
                    start_type: info.1,
                    safe_to_disable: safe.contains(name),
                    description: desc.to_string(),
                }
            })
            .collect()
    }

    pub fn get_service_info(&self, name: &str) -> (String, String) {
        // Get running status via SCM API
        let status = self.get_running_status(name);
        // Get start type via sc.exe query
        let start_type = self.get_start_type_sc(name);
        (status, start_type)
    }

    fn get_running_status(&self, name: &str) -> String {
        use windows::Win32::System::Services::{
            CloseServiceHandle, OpenSCManagerW, OpenServiceW, SC_MANAGER_CONNECT,
            SERVICE_QUERY_CONFIG, SERVICE_QUERY_STATUS,
        };

        unsafe {
            let scm = match OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_CONNECT) {
                Ok(h) => h,
                Err(_) => return "Unknown".into(),
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
                    return "Unknown".into();
                }
            };

            let mut s = windows::Win32::System::Services::SERVICE_STATUS::default();
            let _ = windows::Win32::System::Services::QueryServiceStatus(svc, &mut s);
            let _ = CloseServiceHandle(svc);
            let _ = CloseServiceHandle(scm);

            match s.dwCurrentState.0 {
                1 => "Stopped".into(),
                2 => "Starting".into(),
                3 => "Stopping".into(),
                4 => "Running".into(),
                _ => "Unknown".into(),
            }
        }
    }

    fn get_start_type_sc(&self, name: &str) -> String {
        let output = Command::new("sc")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["qc", name])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
            .unwrap_or_default();

        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.to_uppercase().contains("START_TYPE") {
                let parts: Vec<&str> = trimmed.split(':').collect();
                if parts.len() >= 2 {
                    return parts[1].trim().to_string();
                }
            }
            // Also check for "AUTO_START", "DEMAND_START", "DISABLED" etc.
            if trimmed.contains("AUTO_START") {
                return "Auto".into();
            }
            if trimmed.contains("DEMAND_START") {
                return "Manual".into();
            }
            if trimmed.contains("DISABLED") {
                return "Disabled".into();
            }
            if trimmed.contains("BOOT_START") {
                return "Boot".into();
            }
            if trimmed.contains("SYSTEM_START") {
                return "System".into();
            }
        }
        "Unknown".to_string()
    }

    pub fn stop_service(&self, name: &str) -> Result<String, String> {
        let output = Command::new("sc")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["stop", name])
            .output()
            .map_err(|e| format!("Failed to run sc: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() || stdout.contains("STOP_PENDING") {
            Ok(format!("{} is stopping", name))
        } else if stderr.contains("not started") || stdout.contains("not started") {
            Ok(format!("{} is already stopped", name))
        } else if stderr.contains("Access is denied") {
            Err("Access denied — run as Administrator".into())
        } else {
            Err(format!("Failed: {} {}", stdout, stderr))
        }
    }

    pub fn set_startup_type(&self, name: &str, start_type: &str) -> Result<String, String> {
        let output = Command::new("sc")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["config", name, "start=", start_type])
            .output()
            .map_err(|e| format!("Failed to run sc: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() || stdout.contains("SUCCESS") {
            Ok(format!("{} set to {}", name, start_type))
        } else if stderr.contains("Access is denied") {
            Err("Access denied — run as Administrator".into())
        } else {
            Err(format!("Failed: {} {}", stdout, stderr))
        }
    }

    /// Stop AND disable a service in one call
    pub fn disable_service(&self, name: &str) -> Result<String, String> {
        // First stop it
        match self.stop_service(name) {
            Ok(msg) => tracing::info!("{}", msg),
            Err(e) => tracing::warn!("Stop {}: {}", name, e),
        }
        // Then disable it
        self.set_startup_type(name, "disabled")
    }
}

fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
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

const SAFE_SERVICES: &[(&str, &str, &str)] = &[
    (
        "DiagTrack",
        "Connected User Experiences and Telemetry",
        "Collects diagnostic and usage data",
    ),
    (
        "dmwappushservice",
        "Device Management WAP Push Service",
        "Pushes enterprise policies to devices",
    ),
    (
        "SysMain",
        "SysMain (Superfetch)",
        "Preloads frequently-used apps into RAM — noticeable on SSDs",
    ),
    (
        "WSearch",
        "Windows Search",
        "Indexes files for search — heavy on CPU and disk I/O",
    ),
    (
        "Fax",
        "Fax Service",
        "Legacy fax support — almost never used on modern systems",
    ),
    (
        "XboxNetApiSvc",
        "Xbox Live Networking Service",
        "Required only for Xbox console multiplayer networking",
    ),
    (
        "XblAuthManager",
        "Xbox Live Auth Manager",
        "Handles Xbox Live sign-in authentication",
    ),
    (
        "XblGameSave",
        "Xbox Live Game Save",
        "Syncs Xbox game saves to the cloud",
    ),
    (
        "XboxGipSvc",
        "Xbox Accessory Management Service",
        "Manages Xbox controllers and accessories",
    ),
    (
        "MapsBroker",
        "Downloaded Maps Manager",
        "Manages offline map downloads — rarely used",
    ),
    (
        "lfsvc",
        "Geolocation Service",
        "Provides location data to apps — privacy concern",
    ),
    (
        "wcncsvc",
        "Windows Connect Now",
        "Simplifies WiFi Direct and wireless device configuration",
    ),
    (
        "WMPNetworkSvc",
        "Windows Media Player Network Sharing",
        "Shares media libraries over the network",
    ),
    (
        "RemoteRegistry",
        "Remote Registry",
        "Allows remote registry modification — security risk",
    ),
    (
        "SharedAccess",
        "Internet Connection Sharing",
        "Shares this PC's internet — rarely needed on desktops",
    ),
    (
        "WerSvc",
        "Windows Error Reporting",
        "Sends crash dumps and error reports to Microsoft",
    ),
    (
        "WpnService",
        "Windows Push Notification Service",
        "Delivers push notifications to Windows apps",
    ),
    (
        "PcaSvc",
        "Program Compatibility Assistant",
        "Detects and reports compatibility issues with older apps",
    ),
    (
        "FontCache",
        "Windows Font Cache Service",
        "Caches font data for faster rendering — can be recreated",
    ),
];
