use serde::{Deserialize, Serialize};
use std::fs;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use zb_shared::constants::CREATE_NO_WINDOW;
const MAX_DIR_SCAN_DURATION: Duration = Duration::from_secs(10);

fn system_drive() -> PathBuf {
    std::env::var("SystemDrive")
        .map(|d| PathBuf::from(format!("{}\\", d)))
        .unwrap_or_else(|_| PathBuf::from(r"C:\"))
}

fn windows_dir() -> PathBuf {
    std::env::var("windir")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(r"C:\Windows"))
}

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
    pub errors: Vec<String>,
    pub deleted_paths: Vec<String>,
}

pub struct SystemCleaner;

impl Default for SystemCleaner {
    fn default() -> Self {
        Self::new()
    }
}

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
                errors: vec!["Unknown category".to_string()],
                deleted_paths: vec![],
            },
        }
    }

    // --- Scanning ---

    fn scan_recycle_bin(&self) -> CleanCategory {
        let path = system_drive().join("$Recycle.Bin");
        let size = dir_size(&path);
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
        let path = windows_dir().join("Temp");
        CleanCategory {
            id: "windows_temp".into(),
            name: "Windows Temp".into(),
            description: "System-wide temporary files in Windows\\Temp".into(),
            risk: "safe".into(),
            size_bytes: dir_size(&path),
        }
    }

    fn scan_windows_logs(&self) -> CleanCategory {
        let path = windows_dir().join("Logs");
        CleanCategory {
            id: "windows_logs".into(),
            name: "Windows Logs".into(),
            description: "System and application log files".into(),
            risk: "moderate".into(),
            size_bytes: dir_size(&path),
        }
    }

    fn scan_windows_update_cache(&self) -> CleanCategory {
        let p1 = windows_dir().join("SoftwareDistribution\\Download");
        let p2 = windows_dir().join("DeliveryOptimization");
        CleanCategory {
            id: "windows_update".into(),
            name: "Windows Update Cache".into(),
            description: "Old Windows Update and Delivery Optimization files".into(),
            risk: "moderate".into(),
            size_bytes: dir_size(&p1) + dir_size(&p2),
        }
    }

    fn scan_prefetch(&self) -> CleanCategory {
        CleanCategory {
            id: "prefetch".into(),
            name: "Prefetch Data".into(),
            description: "Windows Prefetch files — speeds app launch but accumulates".into(),
            risk: "moderate".into(),
            size_bytes: dir_size(&windows_dir().join("Prefetch")),
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
        let rec_bin = system_drive().join("$Recycle.Bin");
        let before = dir_size(&rec_bin);
        
        // If recycle bin doesn't exist or is already empty, treat as success
        if !rec_bin.exists() || before == 0 {
            return CleanResult {
                category: "recycle_bin".into(),
                bytes_freed: before,
                items_removed: 0,
                success: true,
                errors: vec![],
                deleted_paths: vec![],
            };
        }
        
        let drive_letter = system_drive().to_string_lossy().chars().next().unwrap_or('C');
        let result = Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["-Command", &format!(
                "Clear-RecycleBin -DriveLetter {} -Force -ErrorAction SilentlyContinue; if ($?) {{ Write-Host 'SUCCESS' }}",
                drive_letter
            )])
            .output();
        
        let success = match &result {
            Ok(o) => {
                let stdout = String::from_utf8_lossy(&o.stdout);
                stdout.contains("SUCCESS")
            }
            Err(_) => false,
        };
        
        let errors = if !success {
            match result {
                Ok(o) => {
                    let stderr = String::from_utf8_lossy(&o.stderr).trim().to_string();
                    if stderr.is_empty() {
                        vec!["Recycle Bin could not be cleared (may be in use)".to_string()]
                    } else {
                        vec![stderr]
                    }
                }
                Err(e) => vec![format!("PowerShell failed: {}", e)],
            }
        } else {
            vec![]
        };
        
        let after = dir_size(&rec_bin);
        let bytes_freed = before.saturating_sub(after);
        
        CleanResult {
            category: "recycle_bin".into(),
            bytes_freed,
            items_removed: 0,
            success,
            errors,
            deleted_paths: vec![],
        }
    }

    fn clean_temp_files(&self) -> CleanResult {
        let temp = std::env::var("TEMP").unwrap_or_default();
        let before = dir_size(Path::new(&temp));
        let (items, deleted, errors) = remove_dir_contents_with_errors(&temp);
        let after = dir_size(Path::new(&temp));
        CleanResult {
            category: "temp_files".into(),
            bytes_freed: before.saturating_sub(after),
            items_removed: items,
            success: items > 0 || before == 0,
            errors,
            deleted_paths: deleted,
        }
    }

    fn clean_browser_cache(&self) -> CleanResult {
        let local = get_local_appdata();
        let mut before = 0u64;
        let mut after = 0u64;
        let mut items = 0u32;
        let mut deleted = Vec::new();
        let mut errors = Vec::new();

        // Chrome/Edge/Brave use standard "User Data\Default\Cache" path
        let chromium_browsers = [
            "Google\\Chrome",
            "Microsoft\\Edge",
            "BraveSoftware\\Brave-Browser",
        ];
        for browser in &chromium_browsers {
            let cache = local.join(browser).join("User Data\\Default\\Cache");
            before += dir_size(&cache);
            let (count, paths, errs) = remove_dir_contents_with_errors(&cache);
            after += dir_size(&cache);
            items += count;
            deleted.extend(paths);
            errors.extend(errs);
        }

        // Firefox uses Profiles\<profile>\cache2\ path (non-standard)
        let firefox_profiles = local.join("Mozilla\\Firefox\\Profiles");
        if firefox_profiles.exists() {
            if let Ok(entries) = std::fs::read_dir(&firefox_profiles) {
                for entry in entries.flatten() {
                    let profile_path = entry.path();
                    if profile_path.is_dir() {
                        let cache = profile_path.join("cache2");
                        before += dir_size(&cache);
                        let (count, paths, errs) = remove_dir_contents_with_errors(&cache);
                        after += dir_size(&cache);
                        items += count;
                        deleted.extend(paths);
                        errors.extend(errs);
                    }
                }
            }
        }

        CleanResult {
            category: "browser_cache".into(),
            bytes_freed: before.saturating_sub(after),
            items_removed: items,
            success: items > 0 || before == 0,
            errors,
            deleted_paths: deleted,
        }
    }

    fn clean_windows_temp(&self) -> CleanResult {
        let temp = windows_dir().join("Temp");
        let before = dir_size(&temp);
        let (items, deleted, errors) = remove_dir_contents_with_errors(&temp);
        let after = dir_size(&temp);
        CleanResult {
            category: "windows_temp".into(),
            bytes_freed: before.saturating_sub(after),
            items_removed: items,
            success: items > 0 || before == 0,
            errors,
            deleted_paths: deleted,
        }
    }

    fn clean_windows_logs(&self) -> CleanResult {
        let logs = windows_dir().join("Logs");
        let before = dir_size(&logs);
        let (items, deleted, errors) = remove_dir_contents_with_errors(&logs);
        let after = dir_size(&logs);
        CleanResult {
            category: "windows_logs".into(),
            bytes_freed: before.saturating_sub(after),
            items_removed: items,
            success: items > 0 || before == 0,
            errors,
            deleted_paths: deleted,
        }
    }

    fn clean_windows_update(&self) -> CleanResult {
        let p1 = windows_dir().join("SoftwareDistribution\\Download");
        let p2 = windows_dir().join("DeliveryOptimization");
        let before = dir_size(&p1) + dir_size(&p2);
        let (items1, deleted1, errs1) = remove_dir_contents_with_errors(&p1);
        let (items2, deleted2, errs2) = remove_dir_contents_with_errors(&p2);
        let after = dir_size(&p1) + dir_size(&p2);
        let mut deleted = deleted1;
        deleted.extend(deleted2);
        let mut errors = errs1;
        errors.extend(errs2);
        CleanResult {
            category: "windows_update".into(),
            bytes_freed: before.saturating_sub(after),
            items_removed: items1 + items2,
            success: (items1 + items2) > 0 || before == 0,
            errors,
            deleted_paths: deleted,
        }
    }

    fn clean_prefetch(&self) -> CleanResult {
        let prefetch = windows_dir().join("Prefetch");
        let before = dir_size(&prefetch);
        let (items, deleted, errors) = remove_dir_contents_with_errors(&prefetch);
        let after = dir_size(&prefetch);
        CleanResult {
            category: "prefetch".into(),
            bytes_freed: before.saturating_sub(after),
            items_removed: items,
            success: items > 0 || before == 0,
            errors,
            deleted_paths: deleted,
        }
    }

    fn clean_dns_cache(&self) -> CleanResult {
        let result = Command::new("ipconfig")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["/flushdns"])
            .output();
        
        let success = result.as_ref().map(|o| o.status.success()).unwrap_or(false);
        let errors = if !success {
            match result {
                Ok(o) => vec![String::from_utf8_lossy(&o.stderr).to_string()],
                Err(e) => vec![e.to_string()],
            }
        } else {
            vec![]
        };
        
        CleanResult {
            category: "dns_cache".into(),
            bytes_freed: 0,
            items_removed: 0,
            success,
            errors,
            deleted_paths: vec![],
        }
    }

    fn clean_thumbnails(&self) -> CleanResult {
        let explorer = get_local_appdata().join("Microsoft\\Windows\\Explorer");
        let before = dir_size(&explorer);
        let (items, deleted, errors) = remove_dir_contents_with_errors(&explorer);
        let after = dir_size(&explorer);
        CleanResult {
            category: "thumbnails".into(),
            bytes_freed: before.saturating_sub(after),
            items_removed: items,
            success: items > 0 || before == 0,
            errors,
            deleted_paths: deleted,
        }
    }
}

fn dir_size(path: &Path) -> u64 {
    dir_size_with_timeout(path, MAX_DIR_SCAN_DURATION)
}

fn dir_size_with_timeout(path: &Path, timeout: Duration) -> u64 {
    if !path.exists() {
        return 0;
    }
    let start = Instant::now();
    let mut total = 0u64;
    let mut stack = vec![path.to_path_buf()];

    while let Some(current) = stack.pop() {
        if start.elapsed() > timeout {
            tracing::warn!("Directory scan timed out for: {:?}", path);
            break;
        }
        if let Ok(entries) = fs::read_dir(&current) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    stack.push(p);
                } else if let Ok(m) = p.metadata() {
                    total += m.len();
                }
            }
        }
    }
    total
}

fn remove_dir_contents_with_errors<P: AsRef<Path>>(path: P) -> (u32, Vec<String>, Vec<String>) {
    let p = path.as_ref();
    if !p.exists() {
        return (0, vec![], vec![]);
    }
    let mut count = 0u32;
    let mut errors = Vec::new();
    let mut deleted = Vec::new();
    
    match fs::read_dir(p) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let ep = entry.path();
                let path_str = ep.to_string_lossy().to_string();
                if ep.is_dir() {
                    match fs::remove_dir_all(&ep) {
                        Ok(_) => {
                            count += 1;
                            deleted.push(path_str);
                        }
                        Err(e) => errors.push(format!("Failed to remove {:?}: {}", ep, e)),
                    }
                } else {
                    match fs::remove_file(&ep) {
                        Ok(_) => {
                            count += 1;
                            deleted.push(path_str);
                        }
                        Err(e) => errors.push(format!("Failed to remove {:?}: {}", ep, e)),
                    }
                }
            }
        }
        Err(e) => errors.push(format!("Failed to read directory {:?}: {}", p, e)),
    }
    
    (count, deleted, errors)
}

fn get_local_appdata() -> PathBuf {
    std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}
