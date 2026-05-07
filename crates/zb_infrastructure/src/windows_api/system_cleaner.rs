use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub risk: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanResult {
    pub category: String,
    pub bytes_freed: u64,
    pub items_removed: u32,
    pub success: bool,
}

pub struct SystemCleaner;

impl SystemCleaner {
    pub fn new() -> Self {
        Self
    }

    pub fn scan_categories(&self) -> Vec<CleanCategory> {
        vec![
            self.scan_recycle_bin(),
            self.scan_temp_files(),
            self.scan_browser_cache(),
            self.scan_windows_temp(),
            self.scan_windows_logs(),
            self.scan_windows_update_cache(),
            self.scan_prefetch(),
            self.scan_dns_cache(),
            self.scan_thumbnail_cache(),
        ]
    }

    pub fn clean_category(&self, id: &str) -> CleanResult {
        match id {
            "recycle_bin" => self.clean_recycle_bin(),
            "temp_files" => self.clean_temp_files(),
            "browser_cache" => self.clean_browser_cache(),
            "windows_temp" => self.clean_windows_temp(),
            "windows_logs" => self.clean_windows_logs(),
            "windows_update" => self.clean_windows_update(),
            "prefetch" => self.clean_prefetch(),
            "dns_cache" => self.clean_dns_cache(),
            "thumbnails" => self.clean_thumbnails(),
            _ => CleanResult {
                category: id.into(),
                bytes_freed: 0,
                items_removed: 0,
                success: false,
            },
        }
    }

    // --- Scanning ---

    fn scan_recycle_bin(&self) -> CleanCategory {
        let path = Path::new(r"C:\$Recycle.Bin");
        let size = dir_size(path);
        CleanCategory {
            id: "recycle_bin".into(),
            name: "Recycle Bin".into(),
            description: "Files waiting in the Recycle Bin".into(),
            risk: "safe".into(),
            size_bytes: size,
        }
    }

    fn scan_temp_files(&self) -> CleanCategory {
        let mut size = 0;
        if let Ok(temp) = std::env::var("TEMP") {
            size += dir_size(Path::new(&temp));
        }
        CleanCategory {
            id: "temp_files".into(),
            name: "Temporary Files".into(),
            description: "User temp files in %TEMP%".into(),
            risk: "safe".into(),
            size_bytes: size,
        }
    }

    fn scan_browser_cache(&self) -> CleanCategory {
        let mut size = 0;
        let local = get_local_appdata();
        for browser in &[
            "Google\\Chrome",
            "Microsoft\\Edge",
            "Mozilla\\Firefox",
            "BraveSoftware\\Brave-Browser",
        ] {
            size += dir_size(&local.join(browser).join("User Data\\Default\\Cache"));
        }
        CleanCategory {
            id: "browser_cache".into(),
            name: "Browser Cache".into(),
            description: "Chrome, Edge, Firefox, Brave browser caches".into(),
            risk: "safe".into(),
            size_bytes: size,
        }
    }

    fn scan_windows_temp(&self) -> CleanCategory {
        CleanCategory {
            id: "windows_temp".into(),
            name: "Windows Temp".into(),
            description: "System-wide temporary files in C:\\Windows\\Temp".into(),
            risk: "safe".into(),
            size_bytes: dir_size(Path::new(r"C:\Windows\Temp")),
        }
    }

    fn scan_windows_logs(&self) -> CleanCategory {
        CleanCategory {
            id: "windows_logs".into(),
            name: "Windows Logs".into(),
            description: "System and application log files".into(),
            risk: "moderate".into(),
            size_bytes: dir_size(Path::new(r"C:\Windows\Logs")),
        }
    }

    fn scan_windows_update_cache(&self) -> CleanCategory {
        CleanCategory {
            id: "windows_update".into(),
            name: "Windows Update Cache".into(),
            description: "Old Windows Update and Delivery Optimization files".into(),
            risk: "moderate".into(),
            size_bytes: dir_size(&Path::new(r"C:\Windows\SoftwareDistribution\Download"))
                + dir_size(&Path::new(r"C:\Windows\DeliveryOptimization")),
        }
    }

    fn scan_prefetch(&self) -> CleanCategory {
        CleanCategory {
            id: "prefetch".into(),
            name: "Prefetch Data".into(),
            description: "Windows Prefetch files — speeds app launch but accumulates".into(),
            risk: "moderate".into(),
            size_bytes: dir_size(Path::new(r"C:\Windows\Prefetch")),
        }
    }

    fn scan_dns_cache(&self) -> CleanCategory {
        CleanCategory {
            id: "dns_cache".into(),
            name: "DNS Cache".into(),
            description: "Flush DNS resolver cache".into(),
            risk: "safe".into(),
            size_bytes: 0,
        }
    }

    fn scan_thumbnail_cache(&self) -> CleanCategory {
        let size = dir_size(&get_local_appdata().join("Microsoft\\Windows\\Explorer"));
        CleanCategory {
            id: "thumbnails".into(),
            name: "Thumbnail Cache".into(),
            description: "Explorer thumbnail cache (thumbcache_*.db)".into(),
            risk: "safe".into(),
            size_bytes: size,
        }
    }

    // --- Cleaning ---

    fn clean_recycle_bin(&self) -> CleanResult {
        let before = dir_size(Path::new(r"C:\$Recycle.Bin"));
        let _ = Command::new("cmd")
            .args(["/c", "rd", "/s", "/q", r"C:\$Recycle.Bin"])
            .output();
        let _ = Command::new("powershell")
            .args([
                "-Command",
                "Clear-RecycleBin -Force -ErrorAction SilentlyContinue",
            ])
            .output();
        CleanResult {
            category: "recycle_bin".into(),
            bytes_freed: before,
            items_removed: 0,
            success: true,
        }
    }

    fn clean_temp_files(&self) -> CleanResult {
        let before = if let Ok(temp) = std::env::var("TEMP") {
            dir_size(Path::new(&temp))
        } else {
            0
        };
        if let Ok(temp) = std::env::var("TEMP") {
            let _ = remove_dir_contents(&temp);
        }
        CleanResult {
            category: "temp_files".into(),
            bytes_freed: before,
            items_removed: 0,
            success: true,
        }
    }

    fn clean_browser_cache(&self) -> CleanResult {
        let local = get_local_appdata();
        let mut before = 0u64;
        for browser in &[
            "Google\\Chrome",
            "Microsoft\\Edge",
            "Mozilla\\Firefox",
            "BraveSoftware\\Brave-Browser",
        ] {
            let cache = local.join(browser).join("User Data\\Default\\Cache");
            before += dir_size(&cache);
            let _ = remove_dir_contents(&cache);
        }
        CleanResult {
            category: "browser_cache".into(),
            bytes_freed: before,
            items_removed: 0,
            success: true,
        }
    }

    fn clean_windows_temp(&self) -> CleanResult {
        let before = dir_size(Path::new(r"C:\Windows\Temp"));
        let _ = remove_dir_contents(r"C:\Windows\Temp");
        CleanResult {
            category: "windows_temp".into(),
            bytes_freed: before,
            items_removed: 0,
            success: true,
        }
    }

    fn clean_windows_logs(&self) -> CleanResult {
        let before = dir_size(Path::new(r"C:\Windows\Logs"));
        let _ = remove_dir_contents(r"C:\Windows\Logs");
        CleanResult {
            category: "windows_logs".into(),
            bytes_freed: before,
            items_removed: 0,
            success: true,
        }
    }

    fn clean_windows_update(&self) -> CleanResult {
        let p1 = Path::new(r"C:\Windows\SoftwareDistribution\Download");
        let p2 = Path::new(r"C:\Windows\DeliveryOptimization");
        let before = dir_size(p1) + dir_size(p2);
        let _ = remove_dir_contents(p1);
        let _ = remove_dir_contents(p2);
        CleanResult {
            category: "windows_update".into(),
            bytes_freed: before,
            items_removed: 0,
            success: true,
        }
    }

    fn clean_prefetch(&self) -> CleanResult {
        let before = dir_size(Path::new(r"C:\Windows\Prefetch"));
        let _ = remove_dir_contents(r"C:\Windows\Prefetch");
        CleanResult {
            category: "prefetch".into(),
            bytes_freed: before,
            items_removed: 0,
            success: true,
        }
    }

    fn clean_dns_cache(&self) -> CleanResult {
        let _ = Command::new("ipconfig").args(["/flushdns"]).output();
        CleanResult {
            category: "dns_cache".into(),
            bytes_freed: 0,
            items_removed: 0,
            success: true,
        }
    }

    fn clean_thumbnails(&self) -> CleanResult {
        let explorer = get_local_appdata().join("Microsoft\\Windows\\Explorer");
        let before = dir_size(&explorer);
        let _ = remove_dir_contents(&explorer);
        CleanResult {
            category: "thumbnails".into(),
            bytes_freed: before,
            items_removed: 0,
            success: true,
        }
    }
}

fn dir_size(path: &Path) -> u64 {
    if !path.exists() {
        return 0;
    }
    let mut total = 0u64;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                total += dir_size(&p);
            } else if let Ok(m) = p.metadata() {
                total += m.len();
            }
        }
    }
    total
}

fn remove_dir_contents<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let p = path.as_ref();
    if !p.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(p)? {
        let entry = entry?;
        let ep = entry.path();
        if ep.is_dir() {
            let _ = fs::remove_dir_all(&ep);
        } else {
            let _ = fs::remove_file(&ep);
        }
    }
    Ok(())
}

fn get_local_appdata() -> PathBuf {
    std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}
