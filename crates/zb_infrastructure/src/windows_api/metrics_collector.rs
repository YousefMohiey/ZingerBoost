use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tracing::{error, info, warn};
use zb_shared::types::SystemMetrics;

struct MetricsState {
    cpu_percent: AtomicU64,
    ram_percent: AtomicU64,
    ram_used_mb: AtomicU64,
    ram_total_mb: AtomicU64,
    disk_active_percent: AtomicU64,
    network_down_mbps: AtomicU64,
    network_up_mbps: AtomicU64,
    // For fallback network tracking
    last_net_in: AtomicU64,
    last_net_out: AtomicU64,
    last_net_sample: AtomicU64,
}

impl MetricsState {
    fn new() -> Self {
        Self {
            cpu_percent: AtomicU64::new(0),
            ram_percent: AtomicU64::new(0),
            ram_used_mb: AtomicU64::new(0),
            ram_total_mb: AtomicU64::new(0),
            disk_active_percent: AtomicU64::new(0),
            network_down_mbps: AtomicU64::new(0),
            network_up_mbps: AtomicU64::new(0),
            last_net_in: AtomicU64::new(0),
            last_net_out: AtomicU64::new(0),
            last_net_sample: AtomicU64::new(0),
        }
    }

    fn set(&self, m: &SystemMetrics) {
        self.cpu_percent
            .store(f64_to_u64(m.cpu_percent), Ordering::Relaxed);
        self.ram_percent
            .store(f64_to_u64(m.ram_percent), Ordering::Relaxed);
        self.ram_used_mb.store(m.ram_used_mb, Ordering::Relaxed);
        self.ram_total_mb.store(m.ram_total_mb, Ordering::Relaxed);
        self.disk_active_percent
            .store(f64_to_u64(m.disk_active_percent), Ordering::Relaxed);
        self.network_down_mbps
            .store(f64_to_u64(m.network_down_mbps), Ordering::Relaxed);
        self.network_up_mbps
            .store(f64_to_u64(m.network_up_mbps), Ordering::Relaxed);
    }

    fn get(&self) -> SystemMetrics {
        SystemMetrics {
            cpu_percent: u64_to_f64(self.cpu_percent.load(Ordering::Relaxed)),
            ram_percent: u64_to_f64(self.ram_percent.load(Ordering::Relaxed)),
            ram_used_mb: self.ram_used_mb.load(Ordering::Relaxed),
            ram_total_mb: self.ram_total_mb.load(Ordering::Relaxed),
            disk_active_percent: u64_to_f64(self.disk_active_percent.load(Ordering::Relaxed)),
            network_down_mbps: u64_to_f64(self.network_down_mbps.load(Ordering::Relaxed)),
            network_up_mbps: u64_to_f64(self.network_up_mbps.load(Ordering::Relaxed)),
        }
    }
}

fn f64_to_u64(v: f64) -> u64 {
    if v.is_finite() && v >= 0.0 {
        (v * 100.0) as u64
    } else {
        0 // NaN, Infinity, or negative → 0
    }
}

fn u64_to_f64(v: u64) -> f64 {
    v as f64 / 100.0
}

pub struct MetricsCollector {
    state: Arc<MetricsState>,
}

impl MetricsCollector {
    pub fn new() -> Arc<Self> {
        let state = Arc::new(MetricsState::new());
        let state_clone = state.clone();

        thread::spawn(move || {
            #[cfg(target_os = "windows")]
            {
                // Catch panics so we can log them
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    run_sampler(&state_clone);
                }));
                if let Err(e) = result {
                    let msg = if let Some(s) = e.downcast_ref::<&str>() {
                        s.to_string()
                    } else if let Some(s) = e.downcast_ref::<String>() {
                        s.clone()
                    } else {
                        "unknown panic".to_string()
                    };
                    error!("[metrics] Sampler thread panicked: {}", msg);
                }
            }
            #[cfg(not(target_os = "windows"))]
            {
                loop {
                    thread::sleep(Duration::from_millis(500));
                }
            }
        });

        Arc::new(Self { state })
    }

    pub async fn current(&self) -> SystemMetrics {
        self.state.get()
    }

    /// Get current metrics as JSON string for event emission
    pub fn current_json(&self) -> String {
        serde_json::to_string(&self.state.get()).unwrap_or_default()
    }
}

#[cfg(target_os = "windows")]
fn run_sampler(state: &MetricsState) {
    use windows::core::PCWSTR;
    use windows::Win32::System::Performance::{
        PdhAddEnglishCounterW, PdhCollectQueryData, PdhGetFormattedCounterValue, PdhOpenQueryW,
        PDH_FMT_COUNTERVALUE, PDH_FMT_DOUBLE,
    };
    use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

    info!("[metrics] Sampler thread started");

    unsafe {
        // --- Open PDH query ---
        let mut query = std::mem::zeroed();
        if PdhOpenQueryW(PCWSTR::null(), 0, &mut query) != 0 {
            error!("[metrics] Failed to open PDH query");
            return;
        }
        info!("[metrics] PDH query opened");

        // --- Add CPU counter ---
        // Use % Processor Time first (matches Task Manager), fallback to others
        let cpu_paths = [
            "\\Processor Information(_Total)\\% Processor Time",
            "\\Processor(_Total)\\% Processor Time",
            "\\Processor Information(_Total)\\% Processor Utility",
        ];
        let mut cpu_counter = std::mem::zeroed();
        let mut cpu_ok = false;
        for path in &cpu_paths {
            let path_w: Vec<u16> = format!("{path}\0").encode_utf16().collect();
            if PdhAddEnglishCounterW(
                query,
                PCWSTR::from_raw(path_w.as_ptr()),
                0,
                &mut cpu_counter,
            ) == 0
            {
                cpu_ok = true;
                info!("[metrics] CPU counter added: {}", path);
                break;
            }
        }
        if !cpu_ok {
            warn!("[metrics] Failed to add CPU counter");
        }

        // --- Add Disk counter ---
        let disk_path: Vec<u16> = "\\PhysicalDisk(_Total)\\% Disk Time\0"
            .encode_utf16()
            .collect();
        let mut disk_counter = std::mem::zeroed();
        let disk_ok = PdhAddEnglishCounterW(
            query,
            PCWSTR::from_raw(disk_path.as_ptr()),
            0,
            &mut disk_counter,
        ) == 0;
        if !disk_ok {
            warn!("[metrics] Failed to add disk counter");
        }

        // --- Add Network counters ---
        // Use GetAdaptersAddresses for reliable adapter detection on all systems including VMs
        use windows::core::PWSTR;
        use windows::Win32::NetworkManagement::IpHelper::{
            GetAdaptersAddresses, GAA_FLAG_INCLUDE_ALL_INTERFACES,
        };
        use windows::Win32::NetworkManagement::Ndis::IF_OPER_STATUS;
        use windows::Win32::System::Performance::{PdhEnumObjectItemsW, PERF_DETAIL};

        let mut net_down_counters: Vec<isize> = Vec::new();
        let mut net_up_counters: Vec<isize> = Vec::new();

        // PRIMARY: Use wildcard - typeperf confirmed this works!
        // This is the SIMPLEST and MOST RELIABLE approach
        // If wildcard succeeds, we skip individual adapters to avoid double-counting
        let mut wildcard_added = false;
        let down_path = "\\Network Interface(*)\\Bytes Received/sec\0";
        let down_path_w: Vec<u16> = down_path.encode_utf16().collect();
        let mut down_counter: isize = 0;
        if PdhAddEnglishCounterW(
            query,
            PCWSTR::from_raw(down_path_w.as_ptr()),
            0,
            &mut down_counter,
        ) == 0
        {
            net_down_counters.push(down_counter);
            wildcard_added = true;
            info!("[metrics] Added wildcard down counter: *");
        }

        let up_path = "\\Network Interface(*)\\Bytes Sent/sec\0";
        let up_path_w: Vec<u16> = up_path.encode_utf16().collect();
        let mut up_counter: isize = 0;
        if PdhAddEnglishCounterW(
            query,
            PCWSTR::from_raw(up_path_w.as_ptr()),
            0,
            &mut up_counter,
        ) == 0
        {
            net_up_counters.push(up_counter);
            wildcard_added = true;
            info!("[metrics] Added wildcard up counter: *");
        }

        // Method 1 (backup): GetAdaptersAddresses for additional interfaces
        // ONLY used when wildcard fails — to avoid double-counting
        if !wildcard_added {
            // First call to get required buffer size
            let mut buf_size: u32 = 0;
            let initial_result = GetAdaptersAddresses(
                0, // AF_UNSPEC - get both IPv4 and IPv6
                GAA_FLAG_INCLUDE_ALL_INTERFACES,
                None,
                None,
                &mut buf_size,
            );

            info!(
                "[metrics] GetAdaptersAddresses initial result={}, required size={}",
                initial_result, buf_size
            );

            // Allocate proper buffer and call again
            if buf_size > 0 {
                let mut buf: Vec<u8> = vec![0; buf_size as usize];
                let result = GetAdaptersAddresses(
                    0,
                    GAA_FLAG_INCLUDE_ALL_INTERFACES,
                    None,
                    Some(buf.as_mut_ptr() as *mut _),
                    &mut buf_size,
                );

                info!("[metrics] GetAdaptersAddresses second result={}", result);

                if result == 0 {
                    use windows::Win32::NetworkManagement::IpHelper::IP_ADAPTER_ADDRESSES_LH;

                    let mut adapter = buf.as_ptr() as *const IP_ADAPTER_ADDRESSES_LH;
                    let mut found_count = 0;

                    while !adapter.is_null() {
                        let info = &*adapter;

                        // Skip non-operational adapters
                        if info.OperStatus != IF_OPER_STATUS(1) {
                            // IfOperStatusUp
                            adapter = info.Next;
                            continue;
                        }

                        // Get adapter name (FriendlyName)
                        let friendly_name = info.FriendlyName;
                        if !friendly_name.is_null() {
                            let name_len = (0..)
                                .take_while(|&i| *friendly_name.0.offset(i as isize) != 0)
                                .count();
                            let name_slice = std::slice::from_raw_parts(friendly_name.0, name_len);
                            if let Ok(name) = String::from_utf16(name_slice) {
                                if !name.is_empty() {
                                    // Skip loopback and known pseudo-adapters
                                    let lower = name.to_lowercase();
                                    if !lower.contains("loopback")
                                        && !lower.contains("isatap")
                                        && !lower.contains("teredo")
                                    {
                                        // Escape PDH-special chars
                                        let escaped = name
                                            .replace('\\', "\\\\")
                                            .replace('(', "\\(")
                                            .replace(')', "\\)")
                                            .replace('#', "\\#");

                                        info!(
                                            "[metrics] Found active adapter: {} -> {}",
                                            name, escaped
                                        );

                                        // Add Bytes Received/sec
                                        let down_path = format!(
                                            "\\Network Interface({escaped})\\Bytes Received/sec\0"
                                        );
                                        let down_path_w: Vec<u16> =
                                            down_path.encode_utf16().collect();
                                        let mut down_counter: isize = 0;
                                        if PdhAddEnglishCounterW(
                                            query,
                                            PCWSTR::from_raw(down_path_w.as_ptr()),
                                            0,
                                            &mut down_counter,
                                        ) == 0
                                        {
                                            net_down_counters.push(down_counter);
                                            info!("[metrics] Added network down: {}", name);
                                        } else {
                                            info!(
                                                "[metrics] Failed to add down counter for {}",
                                                name
                                            );
                                        }

                                        // Add Bytes Sent/sec
                                        let up_path = format!(
                                            "\\Network Interface({escaped})\\Bytes Sent/sec\0"
                                        );
                                        let up_path_w: Vec<u16> = up_path.encode_utf16().collect();
                                        let mut up_counter: isize = 0;
                                        if PdhAddEnglishCounterW(
                                            query,
                                            PCWSTR::from_raw(up_path_w.as_ptr()),
                                            0,
                                            &mut up_counter,
                                        ) == 0
                                        {
                                            net_up_counters.push(up_counter);
                                            info!("[metrics] Added network up: {}", name);
                                        } else {
                                            info!(
                                                "[metrics] Failed to add up counter for {}",
                                                name
                                            );
                                        }

                                        found_count += 1;
                                    }
                                }
                            }
                        }

                        adapter = info.Next;
                    }

                    info!(
                        "[metrics] GetAdaptersAddresses found {} active adapters",
                        found_count
                    );
                }

                // Fallback: Use wildcard since PDH supports * and typeperf confirmed it works
                if net_down_counters.is_empty() {
                    info!("[metrics] Trying wildcard network counter");
                    let down_path = "\\Network Interface(*)\\Bytes Received/sec\0";
                    let down_path_w: Vec<u16> = down_path.encode_utf16().collect();
                    let mut down_counter: isize = 0;
                    if PdhAddEnglishCounterW(
                        query,
                        PCWSTR::from_raw(down_path_w.as_ptr()),
                        0,
                        &mut down_counter,
                    ) == 0
                    {
                        net_down_counters.push(down_counter);
                        info!("[metrics] Added wildcard down counter");
                    }

                    let up_path = "\\Network Interface(*)\\Bytes Sent/sec\0";
                    let up_path_w: Vec<u16> = up_path.encode_utf16().collect();
                    let mut up_counter: isize = 0;
                    if PdhAddEnglishCounterW(
                        query,
                        PCWSTR::from_raw(up_path_w.as_ptr()),
                        0,
                        &mut up_counter,
                    ) == 0
                    {
                        net_up_counters.push(up_counter);
                        info!("[metrics] Added wildcard up counter");
                    }
                }

                info!(
                    "[metrics] Network counters: down={} up={}",
                    net_down_counters.len(),
                    net_up_counters.len()
                );
            }
        } // end if !wildcard_added

        // Method 2: Fallback to PDH enumeration - this gives us the REAL instance names that PDH understands
        if net_down_counters.is_empty() {
            info!(
                "[metrics] GetAdaptersAddresses found no usable counters, trying PDH enumeration"
            );

            let obj_name: Vec<u16> = "Network Interface\0".encode_utf16().collect();
            let mut counter_len: u32 = 0;
            let mut instance_len: u32 = 0;

            let enum_result = PdhEnumObjectItemsW(
                PCWSTR::null(),
                PCWSTR::null(),
                PCWSTR::from_raw(obj_name.as_ptr()),
                PWSTR(std::ptr::null_mut()),
                &mut counter_len,
                PWSTR(std::ptr::null_mut()),
                &mut instance_len,
                PERF_DETAIL(0),
                0,
            );

            if (enum_result == 0x800007D2 || enum_result == 0) && instance_len > 0 {
                let mut counter_buf: Vec<u16> = vec![0; counter_len as usize + 1];
                let mut instance_buf: Vec<u16> = vec![0; instance_len as usize + 1];

                let enum_result2 = PdhEnumObjectItemsW(
                    PCWSTR::null(),
                    PCWSTR::null(),
                    PCWSTR::from_raw(obj_name.as_ptr()),
                    PWSTR(counter_buf.as_mut_ptr()),
                    &mut counter_len,
                    PWSTR(instance_buf.as_mut_ptr()),
                    &mut instance_len,
                    PERF_DETAIL(0),
                    0,
                );

                if enum_result2 == 0 && instance_len > 0 {
                    let raw = &instance_buf[..instance_len as usize];
                    let mut instances: Vec<String> = Vec::new();
                    let mut current = Vec::new();
                    for &ch in raw {
                        if ch == 0 {
                            if !current.is_empty() {
                                if let Ok(s) = String::from_utf16(&current) {
                                    instances.push(s);
                                }
                                current.clear();
                            }
                        } else {
                            current.push(ch);
                        }
                    }

                    info!(
                        "[metrics] PDH enumerated {} instances: {:?}",
                        instances.len(),
                        instances
                    );

                    // Use the FIRST valid non-loopback interface
                    for instance in &instances {
                        let lower = instance.to_lowercase();
                        if lower.contains("loopback")
                            || lower.contains("isatap")
                            || lower.contains("teredo")
                        {
                            continue;
                        }

                        info!("[metrics] Trying PDH instance: {}", instance);

                        let escaped = instance
                            .replace('\\', "\\\\")
                            .replace('(', "\\(")
                            .replace(')', "\\)")
                            .replace('#', "\\#");

                        let down_path =
                            format!("\\Network Interface({escaped})\\Bytes Received/sec\0");
                        let down_path_w: Vec<u16> = down_path.encode_utf16().collect();
                        let mut down_counter: isize = 0;
                        if PdhAddEnglishCounterW(
                            query,
                            PCWSTR::from_raw(down_path_w.as_ptr()),
                            0,
                            &mut down_counter,
                        ) == 0
                        {
                            net_down_counters.push(down_counter);
                            info!("[metrics] PDH down OK: {}", instance);
                        }

                        let up_path = format!("\\Network Interface({escaped})\\Bytes Sent/sec\0");
                        let up_path_w: Vec<u16> = up_path.encode_utf16().collect();
                        let mut up_counter: isize = 0;
                        if PdhAddEnglishCounterW(
                            query,
                            PCWSTR::from_raw(up_path_w.as_ptr()),
                            0,
                            &mut up_counter,
                        ) == 0
                        {
                            net_up_counters.push(up_counter);
                            info!("[metrics] PDH up OK: {}", instance);
                        }

                        if !net_down_counters.is_empty() && !net_up_counters.is_empty() {
                            break;
                        }
                    }
                }
            }
        }

        // Method 3: Last resort - try more common adapter names (expanded list)
        if net_down_counters.is_empty() {
            info!("[metrics] Still no counters, trying expanded hardcoded names");

            // Try WITHOUT escaping - some systems use raw names
            let raw_adapters = [
                "*", // Try wildcard - may work on some systems
                "eth0",
                "eth1",
                "en0",
                "en1",
                "Ethernet",
                "Wi-Fi",
                "Wireless",
                "Local Area Connection",
            ];

            for adapter in &raw_adapters {
                if *adapter == "*" {
                    // Try wildcard approach
                    let down_path = "\\Network Interface(*)\\Bytes Received/sec\0";
                    let down_path_w: Vec<u16> = down_path.encode_utf16().collect();
                    let mut down_counter: isize = 0;
                    if PdhAddEnglishCounterW(
                        query,
                        PCWSTR::from_raw(down_path_w.as_ptr()),
                        0,
                        &mut down_counter,
                    ) == 0
                    {
                        net_down_counters.push(down_counter);
                        info!("[metrics] Wildcard down OK");
                    }

                    let up_path = "\\Network Interface(*)\\Bytes Sent/sec\0";
                    let up_path_w: Vec<u16> = up_path.encode_utf16().collect();
                    let mut up_counter: isize = 0;
                    if PdhAddEnglishCounterW(
                        query,
                        PCWSTR::from_raw(up_path_w.as_ptr()),
                        0,
                        &mut up_counter,
                    ) == 0
                    {
                        net_up_counters.push(up_counter);
                        info!("[metrics] Wildcard up OK");
                    }
                } else {
                    let escaped = adapter
                        .replace('\\', "\\\\")
                        .replace('(', "\\(")
                        .replace(')', "\\)")
                        .replace('#', "\\#");

                    let down_path = format!("\\Network Interface({escaped})\\Bytes Received/sec\0");
                    let down_path_w: Vec<u16> = down_path.encode_utf16().collect();
                    let mut down_counter: isize = 0;
                    if PdhAddEnglishCounterW(
                        query,
                        PCWSTR::from_raw(down_path_w.as_ptr()),
                        0,
                        &mut down_counter,
                    ) == 0
                    {
                        net_down_counters.push(down_counter);
                        info!("[metrics] Hardcoded down OK: {}", adapter);
                    }

                    let up_path = format!("\\Network Interface({escaped})\\Bytes Sent/sec\0");
                    let up_path_w: Vec<u16> = up_path.encode_utf16().collect();
                    let mut up_counter: isize = 0;
                    if PdhAddEnglishCounterW(
                        query,
                        PCWSTR::from_raw(up_path_w.as_ptr()),
                        0,
                        &mut up_counter,
                    ) == 0
                    {
                        net_up_counters.push(up_counter);
                        info!("[metrics] Hardcoded up OK: {}", adapter);
                    }
                }

                if !net_down_counters.is_empty() && !net_up_counters.is_empty() {
                    break;
                }
            }
        }

        // Method 4: Direct _Total interface (aggregate of all)
        if net_down_counters.is_empty() {
            info!("[metrics] Trying _Total interface");

            let down_path = "\\Network Interface(_Total)\\Bytes Received/sec\0";
            let down_path_w: Vec<u16> = down_path.encode_utf16().collect();
            let mut down_counter: isize = 0;
            if PdhAddEnglishCounterW(
                query,
                PCWSTR::from_raw(down_path_w.as_ptr()),
                0,
                &mut down_counter,
            ) == 0
            {
                net_down_counters.push(down_counter);
                info!("[metrics] _Total down OK");
            }

            let up_path = "\\Network Interface(_Total)\\Bytes Sent/sec\0";
            let up_path_w: Vec<u16> = up_path.encode_utf16().collect();
            let mut up_counter: isize = 0;
            if PdhAddEnglishCounterW(
                query,
                PCWSTR::from_raw(up_path_w.as_ptr()),
                0,
                &mut up_counter,
            ) == 0
            {
                net_up_counters.push(up_counter);
                info!("[metrics] _Total up OK");
            }
        }

        // Method 5: Try non-English counter paths (some systems use localized names)
        if net_down_counters.is_empty() {
            info!("[metrics] Trying non-English counter paths");

            // These are common non-English network interface names
            let locale_paths = [
                (
                    "\\Network Interface(ethernet)\\Bytes Received/sec",
                    "\\Network Interface(ethernet)\\Bytes Sent/sec",
                ),
                (
                    "\\Network Interface(以太网)\\Bytes Received/sec",
                    "\\Network Interface(以太网)\\Bytes Sent/sec",
                ),
                (
                    "\\Network Interface(LAN)\\Bytes Received/sec",
                    "\\Network Interface(LAN)\\Bytes Sent/sec",
                ),
            ];

            for (down, up) in &locale_paths {
                let down_w: Vec<u16> = down.encode_utf16().collect();
                let mut down_counter: isize = 0;
                if PdhAddEnglishCounterW(
                    query,
                    PCWSTR::from_raw(down_w.as_ptr()),
                    0,
                    &mut down_counter,
                ) == 0
                {
                    net_down_counters.push(down_counter);
                    info!("[metrics] Locale down OK");
                }

                let up_w: Vec<u16> = up.encode_utf16().collect();
                let mut up_counter: isize = 0;
                if PdhAddEnglishCounterW(query, PCWSTR::from_raw(up_w.as_ptr()), 0, &mut up_counter)
                    == 0
                {
                    net_up_counters.push(up_counter);
                    info!("[metrics] Locale up OK");
                }
            }
        }

        info!(
            "[metrics] Network counters: down={} up={}",
            net_down_counters.len(),
            net_up_counters.len()
        );

        // Prime counters - need multiple collections for PDH to return valid data
        for _ in 0..5 {
            let _ = PdhCollectQueryData(query);
            thread::sleep(Duration::from_millis(100));
        }
        info!("[metrics] Counters primed with 5 collections");

        info!("[metrics] Sampler ready, starting loop");

        let mut sample_count: u64 = 0;

        // Main loop
        info!("[metrics] Entering main sampling loop");

        loop {
            thread::sleep(Duration::from_millis(1000));
            sample_count += 1;

            let _ = PdhCollectQueryData(query);

            // CPU
            let mut cpu = 0.0;
            if cpu_ok {
                let mut val: PDH_FMT_COUNTERVALUE = std::mem::zeroed();
                if PdhGetFormattedCounterValue(cpu_counter, PDH_FMT_DOUBLE, None, &mut val) == 0 {
                    let raw = val.Anonymous.doubleValue;
                    if raw.is_finite() {
                        cpu = raw.clamp(0.0, 100.0);
                    }
                }
            }

            // Disk
            let mut disk = 0.0;
            if disk_ok {
                let mut val: PDH_FMT_COUNTERVALUE = std::mem::zeroed();
                if PdhGetFormattedCounterValue(disk_counter, PDH_FMT_DOUBLE, None, &mut val) == 0 {
                    let raw = val.Anonymous.doubleValue;
                    if raw.is_finite() {
                        disk = raw.clamp(0.0, 100.0);
                    }
                }
            }

            // Network
            // Network: sum all active interface counters (matches Task Manager)
            let mut net_down = 0.0;
            for counter in net_down_counters.iter() {
                let mut val: PDH_FMT_COUNTERVALUE = std::mem::zeroed();
                let result = PdhGetFormattedCounterValue(*counter, PDH_FMT_DOUBLE, None, &mut val);
                if result == 0 {
                    let raw = val.Anonymous.doubleValue;
                    if raw.is_finite() && raw >= 0.0 {
                        net_down += raw * 8.0 / 1_000_000.0;
                    }
                }
            }

            let mut net_up = 0.0;
            for counter in net_up_counters.iter() {
                let mut val: PDH_FMT_COUNTERVALUE = std::mem::zeroed();
                let result = PdhGetFormattedCounterValue(*counter, PDH_FMT_DOUBLE, None, &mut val);
                if result == 0 {
                    let raw = val.Anonymous.doubleValue;
                    if raw.is_finite() && raw >= 0.0 {
                        net_up += raw * 8.0 / 1_000_000.0;
                    }
                }
            }

            // Fallback: if PDH gives zero, use netstat
            if (net_down == 0.0 && net_up == 0.0 && sample_count > 10)
                || net_down_counters.is_empty()
            {
                if let Some((curr_in, curr_out)) = get_network_fallback() {
                    let last_in = state.last_net_in.load(Ordering::Relaxed);
                    let last_out = state.last_net_out.load(Ordering::Relaxed);
                    if state.last_net_sample.load(Ordering::Relaxed) > 0 {
                        let di = curr_in.saturating_sub(last_in);
                        let do_ = curr_out.saturating_sub(last_out);
                        if di < 10_000_000_000 && do_ < 10_000_000_000 {
                            net_down = di as f64 * 8.0 / 1_000_000.0;
                            net_up = do_ as f64 * 8.0 / 1_000_000.0;
                        }
                    }
                    state.last_net_in.store(curr_in, Ordering::Relaxed);
                    state.last_net_out.store(curr_out, Ordering::Relaxed);
                    state.last_net_sample.store(sample_count, Ordering::Relaxed);
                }
            }

            // RAM
            let mut mem: MEMORYSTATUSEX = std::mem::zeroed();
            mem.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
            let _ = GlobalMemoryStatusEx(&mut mem);
            let total_mb = mem.ullTotalPhys / (1024 * 1024);
            let avail_mb = mem.ullAvailPhys / (1024 * 1024);
            let used_mb = total_mb.saturating_sub(avail_mb);
            let ram_pct = if total_mb > 0 {
                (used_mb as f64 / total_mb as f64) * 100.0
            } else {
                0.0
            };

            // Disk activity via PDH (matches Task Manager's disk utilization %)
            // Note: PDH % Disk Time can exceed 100% on multi-disk systems, cap at 100
            let disk_usage_pct = disk.min(100.0);

            let metrics = SystemMetrics {
                cpu_percent: cpu,
                ram_percent: ram_pct,
                ram_used_mb: used_mb,
                ram_total_mb: total_mb,
                disk_active_percent: disk_usage_pct,
                network_down_mbps: net_down,
                network_up_mbps: net_up,
            };

            state.set(&metrics);

            // Write to file directly to bypass tracing buffering
            if sample_count.is_multiple_of(5) {
                let msg = format!(
                    "[metrics] sample#{} cpu={:.1}% ram={:.1}%({}MB/{}) disk={:.1}% net={:.2}/{:.2}Mbps\n",
                    sample_count, cpu, ram_pct, used_mb, total_mb, disk_usage_pct, net_down, net_up
                );
                info!("{}", msg.trim());
                // Use LOCALAPPDATA env var instead of hardcoded path
                if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
                    let log_path = format!("{}\\ZingerBoost\\metrics_debug.log", local_appdata);
                    if let Ok(mut f) = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&log_path)
                    {
                        let _ = std::io::Write::write_all(&mut f, msg.as_bytes());
                        let _ = f.flush();
                    }
                }
            }
        }
    }
}

/// Fallback: get network bytes via GetIfTable2 when PDH counters unavailable
#[cfg(target_os = "windows")]
fn get_network_fallback() -> Option<(u64, u64)> {
    use windows::Win32::NetworkManagement::IpHelper::{FreeMibTable, GetIfTable2, MIB_IF_TABLE2};
    use windows::Win32::NetworkManagement::Ndis::IF_OPER_STATUS;

    unsafe {
        let mut table: *mut MIB_IF_TABLE2 = std::ptr::null_mut();
        if GetIfTable2(&mut table).is_ok() && !table.is_null() {
            let mut total_in = 0u64;
            let mut total_out = 0u64;

            let table_ref = &*table;
            let entries =
                std::slice::from_raw_parts(table_ref.Table.as_ptr(), table_ref.NumEntries as usize);

            for row in entries {
                // Only count operational interfaces and exclude loopback
                // dwType 24 is loopback (IF_TYPE_SOFTWARE_LOOPBACK)
                if row.OperStatus == IF_OPER_STATUS(1) && row.Type != 24 {
                    total_in += row.InOctets;
                    total_out += row.OutOctets;
                }
            }

            FreeMibTable(table as *mut std::ffi::c_void);

            if total_in > 0 || total_out > 0 {
                return Some((total_in, total_out));
            }
        }
    }
    None
}
