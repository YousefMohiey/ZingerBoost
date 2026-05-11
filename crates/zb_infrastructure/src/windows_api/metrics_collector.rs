use std::sync::Arc;
use zb_shared::types::SystemMetrics;

pub struct MetricsCollector;

impl MetricsCollector {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    pub async fn current(&self) -> SystemMetrics {
        let (ram_used, ram_total, ram_pct) = read_ram();
        let cpu = tokio::task::spawn_blocking(|| read_cpu_counter())
            .await
            .unwrap_or(15.0);
        let disk = tokio::task::spawn_blocking(|| read_disk_counter())
            .await
            .unwrap_or(5.0);

        SystemMetrics {
            cpu_percent: cpu,
            ram_percent: ram_pct,
            ram_used_mb: ram_used,
            ram_total_mb: ram_total,
            disk_active_percent: disk,
            network_down_mbps: 0.5,
            network_up_mbps: 0.1,
        }
    }
}

fn read_cpu_counter() -> f64 {
    #[cfg(target_os = "windows")]
    {
        use windows::core::PCWSTR;
        use windows::Win32::System::Performance::{
            PdhAddEnglishCounterW, PdhCloseQuery, PdhCollectQueryData,
            PdhGetFormattedCounterValue, PdhOpenQueryW, PdhRemoveCounter, PDH_FMT_DOUBLE,
        };

        unsafe {
            let mut query = std::mem::zeroed();
            if PdhOpenQueryW(PCWSTR::null(), 0, &mut query) != 0 {
                return 15.0;
            }

            let cpu_path: Vec<u16> = "\\Processor(_Total)\\% Processor Time\0"
                .encode_utf16()
                .collect();
            let mut counter = std::mem::zeroed();
            if PdhAddEnglishCounterW(query, PCWSTR::from_raw(cpu_path.as_ptr()), 0, &mut counter)
                != 0
            {
                let _ = PdhCloseQuery(query);
                return 15.0;
            }

            let _ = PdhCollectQueryData(query);
            std::thread::sleep(std::time::Duration::from_millis(100));
            let _ = PdhCollectQueryData(query);

            let mut value = std::mem::zeroed();
            let result =
                PdhGetFormattedCounterValue(counter, PDH_FMT_DOUBLE, None, &mut value);
            let val = if result == 0 {
                value.Anonymous.doubleValue
            } else {
                15.0
            };
            let _ = PdhRemoveCounter(counter);
            let _ = PdhCloseQuery(query);
            val
        }
    }
    #[cfg(not(target_os = "windows"))]
    15.0
}

fn read_disk_counter() -> f64 {
    #[cfg(target_os = "windows")]
    {
        use windows::core::PCWSTR;
        use windows::Win32::System::Performance::{
            PdhAddEnglishCounterW, PdhCloseQuery, PdhCollectQueryData,
            PdhGetFormattedCounterValue, PdhOpenQueryW, PdhRemoveCounter, PDH_FMT_DOUBLE,
        };

        unsafe {
            let mut query = std::mem::zeroed();
            if PdhOpenQueryW(PCWSTR::null(), 0, &mut query) != 0 {
                return 5.0;
            }

            let disk_path: Vec<u16> = "\\PhysicalDisk(_Total)\\% Disk Time\0"
                .encode_utf16()
                .collect();
            let mut counter = std::mem::zeroed();
            if PdhAddEnglishCounterW(query, PCWSTR::from_raw(disk_path.as_ptr()), 0, &mut counter)
                != 0
            {
                let _ = PdhCloseQuery(query);
                return 5.0;
            }

            let _ = PdhCollectQueryData(query);
            std::thread::sleep(std::time::Duration::from_millis(100));
            let _ = PdhCollectQueryData(query);

            let mut value = std::mem::zeroed();
            let result =
                PdhGetFormattedCounterValue(counter, PDH_FMT_DOUBLE, None, &mut value);
            let val = if result == 0 {
                value.Anonymous.doubleValue
            } else {
                5.0
            };
            let _ = PdhRemoveCounter(counter);
            let _ = PdhCloseQuery(query);
            val
        }
    }
    #[cfg(not(target_os = "windows"))]
    5.0
}

fn read_ram() -> (u64, u64, f64) {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
        let mut mem = MEMORYSTATUSEX {
            dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
            ..Default::default()
        };
        unsafe {
            let _ = GlobalMemoryStatusEx(&mut mem);
        }
        let total = mem.ullTotalPhys / (1024 * 1024);
        let avail = mem.ullAvailPhys / (1024 * 1024);
        let used = total - avail;
        let pct = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        return (used, total, pct);
    }
    #[cfg(not(target_os = "windows"))]
    (8192, 16384, 42.0)
}
