const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use zb_application::tweak_engine::TweakEngine;
use zb_domain::tweaks::Tweak;
use zb_infrastructure::registry::WinRegistryProvider;
use zb_infrastructure::windows_api::debloat_engine::DebloatEngine;
use zb_infrastructure::windows_api::winget::WingetInstaller;
use zb_shared::constants::CREATE_NO_WINDOW;
use zb_shared::software::{get_bloatware_catalog, get_software_catalog};

use crate::state::AppState;

// ============================================================================
// Tweak Factory
// ============================================================================

pub fn make_all_tweaks() -> Vec<Arc<dyn Tweak>> {
    vec![
        Arc::new(zb_domain::tweaks::definitions::VisualBestPerformanceTweak::new()),
        Arc::new(zb_domain::tweaks::definitions::EndTaskOnTaskbarTweak::new()),
        Arc::new(zb_domain::tweaks::definitions::VerboseLogonTweak::new()),
        Arc::new(zb_domain::tweaks::definitions::DisableTelemetryTweak::new()),
        Arc::new(zb_domain::tweaks::definitions::DisableConsumerFeaturesTweak::new()),
        Arc::new(zb_domain::tweaks::definitions::SetHighPerformanceTweak::new()),
        Arc::new(zb_domain::tweaks::definitions::DisableHibernationTweak::new()),
        Arc::new(zb_domain::tweaks::definitions::DisableSuperfetchTweak::new()),
        Arc::new(zb_domain::tweaks::definitions::DisableWpbtTweak::new()),
        Arc::new(zb_domain::tweaks::definitions::ShowFileExtensionsTweak::new()),
    ]
}

// ============================================================================
// Data Transfer Objects
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweakDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub risk: String,
    pub is_applied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDto {
    pub name: String,
    pub display_name: String,
    pub status: String,
    pub start_type: String,
    pub safe_to_disable: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanerItemDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub risk: String,
    pub size_mb: f64,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloatwareDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub winget_id: String,
    pub subcategory: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareDto {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub winget_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsDto {
    pub cpu_percent: f64,
    pub ram_percent: f64,
    pub ram_used_mb: u64,
    pub ram_total_mb: u64,
    pub disk_active_percent: f64,
    pub network_down_mbps: f64,
    pub network_up_mbps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupDto {
    pub id: String,
    pub description: String,
    pub created_at: String,
    pub tweak_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntryDto {
    pub id: i64,
    pub timestamp: String,
    pub level: String,
    pub category: String,
    pub message: String,
    pub details: Option<String>,
}

// ============================================================================
// Helper Functions
// ============================================================================

fn to_json<T: Serialize>(data: &T) -> Result<String, String> {
    serde_json::to_string(data).map_err(|e| e.to_string())
}

async fn get_engine(state: &tauri::State<'_, AppState>) -> Result<Arc<TweakEngine>, String> {
    let engine = state.engine.lock().await.clone();
    match &engine {
        Some(_) => tracing::debug!("Engine acquired successfully"),
        None => tracing::error!("Engine is None — database init likely failed"),
    }
    engine.ok_or("Engine not initialized".to_string())
}

// ============================================================================
// Tauri Commands - System
// ============================================================================

#[tauri::command]
pub async fn debug_log(message: String) -> Result<String, String> {
    tracing::info!("[JS] {}", message);
    Ok("logged".into())
}

#[tauri::command]
pub async fn uninstall_app() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        use std::path::PathBuf;
        use std::process::Command;
        use zb_shared::constants::CREATE_NO_WINDOW;

        let program_files =
            std::env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".into());
        let uninstaller_path = PathBuf::from(&program_files)
            .join("ZingerBoost")
            .join("Uninstall ZingerBoost.exe");

        if uninstaller_path.exists() {
            // Create a temporary batch file to handle the uninstall process
            // This is needed because the app cannot uninstall itself while running,
            // and we also want to clean up AppData which NSIS doesn't do by default.
            let temp_dir = std::env::var("TEMP").unwrap_or_else(|_| "C:\\Windows\\Temp".into());
            let bat_path = PathBuf::from(&temp_dir).join("zb_uninstall.bat");

            let bat_content = format!(
                "@echo off\r\n\
                echo Waiting for ZingerBoost to close...\r\n\
                timeout /t 3 /nobreak > nul\r\n\
                echo Removing AppData...\r\n\
                rmdir /s /q \"%LOCALAPPDATA%\\ZingerBoost\"\r\n\
                rmdir /s /q \"%APPDATA%\\ZingerBoost\"\r\n\
                echo Starting Uninstaller...\r\n\
                start \"\" \"{}\" /S\r\n\
                (goto) 2>nul & del \"%~f0\"\r\n",
                uninstaller_path.display()
            );

            if let Err(e) = std::fs::write(&bat_path, bat_content) {
                return Err(format!("Failed to create uninstall script: {}", e));
            }

            // Spawn the batch file completely detached
            let result = Command::new("cmd")
                .creation_flags(CREATE_NO_WINDOW)
                .args(["/C", "start", "", &bat_path.to_string_lossy()])
                .spawn();

            match result {
                Ok(_) => {
                    // Tell the frontend that we're exiting
                    // We wait a tiny bit to ensure the message gets sent, then exit
                    std::thread::spawn(|| {
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        std::process::exit(0);
                    });

                    Ok("Uninstall process started. ZingerBoost will now close and remove all data.".to_string())
                }
                Err(e) => Err(format!("Failed to launch uninstall script: {}", e)),
            }
        } else {
            // Fallback: Just open Windows Settings
            let result = Command::new("ms-settings:appsfeatures")
                .creation_flags(CREATE_NO_WINDOW)
                .spawn();

            match result {
                Ok(_) => Ok("Windows Settings opened. Find 'ZingerBoost' in the app list and click Uninstall to remove it.".to_string()),
                Err(e) => Err(format!("Failed to open Settings: {}. Please manually go to Settings > Apps > ZingerBoost > Uninstall", e)),
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok("Uninstall not supported on this platform".to_string())
    }
}

#[tauri::command]
pub async fn check_for_updates() -> Result<String, String> {
    let current = APP_VERSION;

    // Try to fetch latest release from GitHub
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get("https://api.github.com/repos/YousefMohiey/ZingerBoost/releases/latest")
        .header("User-Agent", "ZingerBoost-UpdateChecker")
        .send()
        .await
        .map_err(|e| format!("Failed to check for updates: {}", e))?;

    if !response.status().is_success() {
        return Ok(format!(
            "Current: v{} (could not check for updates)",
            current
        ));
    }

    let body: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    let latest = body["tag_name"].as_str().unwrap_or("unknown");
    let latest_clean = latest.strip_prefix('v').unwrap_or(latest);

    let has_update = latest_clean != current;
    let result = serde_json::json!({
        "current": current,
        "latest": latest_clean,
        "has_update": has_update,
        "download_url": body["html_url"].as_str().unwrap_or("")
    });

    to_json(&result)
}

#[tauri::command]
pub async fn get_app_info() -> Result<String, String> {
    let info = serde_json::json!({
        "version": APP_VERSION,
        "name": "ZingerBoost",
        "author": "YousefMohiey",
        "license": "MIT",
        "tweak_count": 32,
        "service_count": 19,
        "cleaner_count": 9,
        "debloat_count": 44
    });
    to_json(&info)
}

#[tauri::command]
pub async fn check_admin() -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::Shell::IsUserAnAdmin;
        Ok(unsafe { IsUserAnAdmin().as_bool() })
    }
    #[cfg(not(target_os = "windows"))]
    Ok(true)
}

// ============================================================================
// Tauri Commands - Tweaks
// ============================================================================

#[tauri::command]
pub async fn get_tweaks(state: tauri::State<'_, AppState>) -> Result<String, String> {
    tracing::debug!("get_tweaks called");
    let engine = get_engine(&state).await?;
    let all_tweaks = engine.list_tweaks();
    tracing::info!("Engine has {} tweaks", all_tweaks.len());

    let tweaks: Vec<TweakDto> = all_tweaks
        .iter()
        .map(|t| {
            let m = t.metadata();
            TweakDto {
                id: m.id.clone(),
                name: m.name.clone(),
                description: m.description.clone(),
                category: m.category.to_string(),
                risk: m.risk.to_string(),
                is_applied: false,
            }
        })
        .collect();
    tracing::info!("get_tweaks returning {} tweaks", tweaks.len());
    to_json(&tweaks)
}

#[tauri::command]
pub async fn get_tweak_states(state: tauri::State<'_, AppState>) -> Result<String, String> {
    tracing::debug!("get_tweak_states called");
    let engine = get_engine(&state).await?;
    let tweaks = engine.list_tweaks();
    let mut states: Vec<(String, bool)> = Vec::with_capacity(tweaks.len());

    for tweak in &tweaks {
        let id = tweak.metadata().id.clone();
        let applied = tweak.is_applied().await.unwrap_or(false);
        states.push((id, applied));
    }
    tracing::info!("get_tweak_states returning {} states", states.len());
    to_json(&states)
}

#[tauri::command]
pub async fn apply_tweak(id: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let engine = get_engine(&state).await?;
    match engine.apply_single(&id).await {
        Ok(r) => Ok(serde_json::json!({"success": true, "message": r.message}).to_string()),
        Err(e) => {
            tracing::error!("apply_tweak failed: id={}, error={:?}", id, e);
            Err(format!("{:?}", e))
        }
    }
}

#[tauri::command]
pub async fn revert_tweak(id: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let engine = get_engine(&state).await?;
    match engine.revert(&id).await {
        Ok(r) => Ok(serde_json::json!({"success": true, "message": r.message}).to_string()),
        Err(e) => {
            tracing::error!("revert_tweak failed: id={}, error={:?}", id, e);
            Err(format!("{:?}", e))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    pub applied: usize,
    pub skipped: usize,
    pub failed: Vec<String>,
    pub message: String,
}

#[tauri::command]
pub async fn apply_all_tweaks(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let engine = get_engine(&state).await?;
    let tweaks = engine.list_tweaks();
    let mut applied = 0;
    let mut skipped = 0;
    let mut failed = Vec::new();

    for tweak in &tweaks {
        let id = tweak.metadata().id.clone();
        let is_already = tweak.is_applied().await.unwrap_or(false);
        if is_already {
            skipped += 1;
            continue;
        }
        // Use engine.apply_single to ensure audit logging and snapshot saving
        match engine.apply_single(&id).await {
            Ok(_result) => {
                applied += 1;
            }
            Err(e) => {
                tracing::error!("Failed to apply {}: {:?}", id, e);
                failed.push(format!("{}: {:?}", id, e));
            }
        }
    }

    let result = BatchResult {
        applied,
        skipped,
        failed: failed.clone(),
        message: if failed.is_empty() {
            format!("Applied {} tweak(s), {} already active", applied, skipped)
        } else {
            format!(
                "Applied {}, skipped {}, failed {} ({})",
                applied,
                skipped,
                failed.len(),
                failed.join(", ")
            )
        },
    };

    to_json(&result)
}

#[tauri::command]
pub async fn revert_all_tweaks(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let engine = get_engine(&state).await?;
    let tweaks = engine.list_tweaks();
    let mut reverted = 0;
    let mut skipped = 0;
    let mut failed = Vec::new();

    for tweak in &tweaks {
        let id = tweak.metadata().id.clone();
        let is_active = tweak.is_applied().await.unwrap_or(false);
        if !is_active {
            skipped += 1;
            continue;
        }
        match engine.snapshot_service().get_last_snapshot_data(&id).await {
            Ok(snapshot_data) => match tweak.revert(&snapshot_data).await {
                Ok(_result) => {
                    reverted += 1;
                    // Clear the tweak_state in DB so is_applied() returns false
                    if let Err(e) = engine.snapshot_service().clear_tweak_state(&id).await {
                        tracing::error!("Failed to clear tweak state for {}: {:?}", id, e);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to revert {}: {:?}", id, e);
                    failed.push(format!("{}: {:?}", id, e));
                }
            },
            Err(e) => {
                tracing::error!("No snapshot for {}: {:?}", id, e);
                failed.push(format!("{}: no snapshot found", id));
            }
        }
    }

    let result = BatchResult {
        applied: reverted,
        skipped,
        failed: failed.clone(),
        message: if failed.is_empty() {
            format!(
                "Reverted {} tweak(s), {} already inactive",
                reverted, skipped
            )
        } else {
            format!(
                "Reverted {}, skipped {}, failed {} ({})",
                reverted,
                skipped,
                failed.len(),
                failed.join(", ")
            )
        },
    };

    to_json(&result)
}

// ============================================================================
// Tauri Commands - Services
// ============================================================================

#[tauri::command]
pub async fn get_services(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let services: Vec<ServiceDto> = state
        .services
        .query_services()
        .into_iter()
        .map(|s| ServiceDto {
            name: s.name,
            display_name: s.display_name,
            status: s.status,
            start_type: s.start_type,
            safe_to_disable: s.safe_to_disable,
            description: s.description,
        })
        .collect();
    to_json(&services)
}

#[tauri::command]
pub async fn start_service(
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    state.services.start_service(&name)
}

#[tauri::command]
pub async fn stop_service(
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    state.services.stop_service(&name)
}

#[tauri::command]
pub async fn disable_service(
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    state.services.disable_service(&name)
}

// ============================================================================
// Tauri Commands - Cleaner
// ============================================================================

#[tauri::command]
pub async fn get_cleaner_items(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let items: Vec<CleanerItemDto> = state
        .cleaner
        .scan_categories()
        .into_iter()
        .map(|c| CleanerItemDto {
            id: c.id,
            name: c.name,
            description: c.description,
            risk: c.risk,
            size_mb: c.size_bytes as f64 / 1048576.0,
            errors: vec![],
        })
        .collect();
    to_json(&items)
}

#[tauri::command]
pub async fn clean_category(
    name: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let result = state.cleaner.clean_category(&name);
    let mut msg = format!(
        "{} - freed {:.1} MB, {} items removed",
        result.category,
        result.bytes_freed as f64 / 1048576.0,
        result.items_removed
    );
    if !result.errors.is_empty() {
        msg.push_str(&format!(
            ", {} error(s): {}",
            result.errors.len(),
            result.errors.join("; ")
        ));
    }
    Ok(msg)
}

#[tauri::command]
pub async fn clean_all(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let categories = state.cleaner.scan_categories();
    let mut total_freed = 0u64;
    let mut total_items = 0u32;
    let mut cleaned = 0;
    let mut all_errors: Vec<String> = Vec::new();

    for cat in &categories {
        let result = state.cleaner.clean_category(&cat.id);
        total_freed += result.bytes_freed;
        total_items += result.items_removed;
        if result.success {
            cleaned += 1;
        }
        if !result.errors.is_empty() {
            all_errors.extend(result.errors);
        }
    }

    let mut msg = format!(
        "Cleaned {} categories: {:.1} MB freed, {} items removed",
        cleaned,
        total_freed as f64 / 1048576.0,
        total_items
    );
    if !all_errors.is_empty() {
        msg.push_str(&format!(
            ", {} error(s): {}",
            all_errors.len(),
            all_errors.join("; ")
        ));
    }
    Ok(msg)
}

// ============================================================================
// Tauri Commands - Debloat
// ============================================================================

#[tauri::command]
pub async fn get_bloatware() -> Result<String, String> {
    let items: Vec<BloatwareDto> = get_bloatware_catalog()
        .into_iter()
        .map(|b| BloatwareDto {
            id: b.id.clone(),
            name: b.name,
            description: b.description,
            winget_id: b.winget_id,
            subcategory: derive_bloatware_subcategory(&b.id),
        })
        .collect();
    to_json(&items)
}

/// Derive subcategory from bloatware item ID prefix
fn derive_bloatware_subcategory(id: &str) -> String {
    // Games: xbox, game_bar, solitaire, candy crush, etc.
    if id.contains("xbox")
        || id.contains("game_bar")
        || id.contains("solitaire")
        || id.contains("candy")
        || id.contains("gaming")
    {
        return "games".into();
    }
    // System: mixed reality, 3d viewer, paint 3d, onedrive, widgets, cortana, etc.
    if id.contains("mixedreality")
        || id.contains("3dviewer")
        || id.contains("paint3d")
        || id.contains("onedrive")
        || id.contains("widgets")
        || id.contains("cortana")
        || id.contains("ads")
        || id.contains("family")
    {
        return "system".into();
    }
    // Everything else is apps
    "apps".into()
}

#[tauri::command]
pub async fn remove_bloatware(name: String) -> Result<String, String> {
    if name.is_empty() {
        return zb_infrastructure::windows_api::debloat_engine::DebloatEngine::remove_windows_ads()
            .map(|msg| format!("Ads disabled: {}", msg))
            .map_err(|e| e.to_string());
    }
    if name == "MicrosoftWindows.Client.WebExperience" {
        return zb_infrastructure::windows_api::debloat_engine::DebloatEngine::remove_widgets()
            .map(|msg| format!("Widgets removed: {}", msg))
            .map_err(|e| e.to_string());
    }
    DebloatEngine::remove_appx_package(&name).map_err(|e| e.to_string())
}

// ============================================================================
// Tauri Commands - Software Installer
// ============================================================================

#[tauri::command]
pub async fn get_software() -> Result<String, String> {
    let items: Vec<SoftwareDto> = get_software_catalog()
        .into_iter()
        .map(|s| SoftwareDto {
            id: s.id,
            name: s.name,
            description: s.description,
            category: s.category.to_string(),
            winget_id: s.winget_id,
        })
        .collect();
    to_json(&items)
}

#[tauri::command]
pub async fn check_software_installed(winget_id: String) -> Result<bool, String> {
    if winget_id == "built-in" {
        return Ok(true);
    }
    // Wrap blocking process calls in spawn_blocking
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            use std::process::Command;

            let output = Command::new("winget")
                .creation_flags(CREATE_NO_WINDOW)
                .args(["list", "--id", &winget_id, "--exact"])
                .output();

            match output {
                Ok(o) => {
                    let stdout = String::from_utf8_lossy(&o.stdout);
                    Ok(o.status.success() && stdout.contains(&winget_id))
                }
                Err(_) => Ok(false),
            }
        }
        #[cfg(not(target_os = "windows"))]
        Ok(false)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
pub async fn check_bloatware_installed(winget_id: String) -> Result<bool, String> {
    if winget_id.is_empty() {
        return Ok(true);
    }
    // Wrap blocking process calls in spawn_blocking
    tokio::task::spawn_blocking(move || {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            use std::process::Command;

            // First try AppX package detection (most bloatware is AppX)
            let ps_script = format!(
                "$pkg = Get-AppxPackage -AllUsers | Where-Object {{ $_.Name -like '*{}*' -or $_.PackageFamilyName -like '*{}*' }}; if ($pkg) {{ Write-Host 'FOUND' }}",
                winget_id.replace('\'', "''"),
                winget_id.replace('\'', "''")
            );
            let output = Command::new("powershell")
                .creation_flags(CREATE_NO_WINDOW)
                .args(["-NoProfile", "-Command", &ps_script])
                .output();

            if let Ok(o) = output {
                let stdout = String::from_utf8_lossy(&o.stdout);
                if stdout.contains("FOUND") {
                    return Ok(true);
                }
            }

            // Fallback to winget for desktop apps
            let output = Command::new("winget")
                .creation_flags(CREATE_NO_WINDOW)
                .args([
                    "list",
                    "--id",
                    &winget_id,
                    "--exact",
                    "--accept-source-agreements",
                ])
                .output();

            match output {
                Ok(o) => {
                    let stdout = String::from_utf8_lossy(&o.stdout);
                    Ok(o.status.success() && stdout.contains(&winget_id))
                }
                Err(_) => Ok(false),
            }
        }
        #[cfg(not(target_os = "windows"))]
        Ok(true)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
pub async fn install_software(winget_id: String) -> Result<String, String> {
    // Wrap blocking winget install in spawn_blocking to avoid freezing the async runtime
    tokio::task::spawn_blocking(move || WingetInstaller::new().install(&winget_id))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}

// ============================================================================
// Tauri Commands - Metrics
// ============================================================================

/// Returns metrics as a native struct — Tauri serializes it as a JSON object directly.
/// This avoids double-encoding issues that caused NaN on the frontend.
#[tauri::command]
pub async fn get_metrics(state: tauri::State<'_, AppState>) -> Result<MetricsDto, String> {
    let metrics = state.metrics.current().await;
    tracing::debug!(
        "[metrics] cpu={:.1}% ram={:.1}% disk={:.1}% net={:.2}/{:.2}Mbps",
        metrics.cpu_percent,
        metrics.ram_percent,
        metrics.disk_active_percent,
        metrics.network_down_mbps,
        metrics.network_up_mbps
    );
    Ok(MetricsDto {
        cpu_percent: if metrics.cpu_percent.is_finite() {
            metrics.cpu_percent
        } else {
            0.0
        },
        ram_percent: if metrics.ram_percent.is_finite() {
            metrics.ram_percent
        } else {
            0.0
        },
        ram_used_mb: metrics.ram_used_mb,
        ram_total_mb: metrics.ram_total_mb,
        disk_active_percent: if metrics.disk_active_percent.is_finite() {
            metrics.disk_active_percent
        } else {
            0.0
        },
        network_down_mbps: if metrics.network_down_mbps.is_finite() {
            metrics.network_down_mbps
        } else {
            0.0
        },
        network_up_mbps: if metrics.network_up_mbps.is_finite() {
            metrics.network_up_mbps
        } else {
            0.0
        },
    })
}

// ============================================================================
// Tauri Commands - Audit Log
// ============================================================================

#[tauri::command]
pub async fn get_audit_log(
    state: tauri::State<'_, AppState>,
    limit: Option<usize>,
) -> Result<String, String> {
    let engine = get_engine(&state).await?;
    let limit = limit.unwrap_or(100);
    let entries = engine.audit_service().get_recent_raw(limit).await?;

    let dtos: Vec<AuditEntryDto> = entries
        .into_iter()
        .map(
            |(id, timestamp, level, category, message, details)| AuditEntryDto {
                id,
                timestamp,
                level,
                category,
                message,
                details,
            },
        )
        .collect();

    to_json(&dtos)
}

#[tauri::command]
pub async fn clear_audit_log(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let engine = get_engine(&state).await?;
    engine.audit_service().clear().await
}

// ============================================================================
// Tauri Commands - Backups
// ============================================================================

#[tauri::command]
pub async fn create_backup(
    description: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let engine = get_engine(&state).await?;
    let tweaks = engine.list_tweaks();
    let mut snapshot = zb_domain::snapshots::SystemSnapshot::new(description);

    for tweak in &tweaks {
        if tweak.is_applied().await.unwrap_or(false) {
            let data = tweak
                .capture_state()
                .await
                .map_err(|e| format!("{:?}", e))?;
            snapshot.add_record(tweak.metadata().id.clone(), data);
        }
    }

    if snapshot.tweak_records.is_empty() {
        return Err("No tweaks are currently applied to backup".to_string());
    }

    let count = snapshot.tweak_records.len();
    engine
        .snapshot_service()
        .save_snapshot(snapshot)
        .await
        .map(|_| format!("Backup created with {} tweak(s)", count))
        .map_err(|e| format!("Failed: {:?}", e))
}

#[tauri::command]
pub async fn get_backups(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let engine = get_engine(&state).await?;
    let snapshots = engine
        .snapshot_service()
        .list_snapshots()
        .await
        .map_err(|e| format!("{:?}", e))?;

    let backups: Vec<BackupDto> = snapshots
        .into_iter()
        .map(|s| BackupDto {
            id: s.id.to_string(),
            description: s.description,
            created_at: s.created_at.to_rfc3339(),
            tweak_count: s.tweak_records.len(),
        })
        .collect();
    to_json(&backups)
}

#[tauri::command]
pub async fn restore_backup(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let engine = get_engine(&state).await?;
    let snapshots = engine
        .snapshot_service()
        .list_snapshots()
        .await
        .map_err(|e| format!("Failed to list snapshots: {:?}", e))?;

    let snapshot = snapshots
        .into_iter()
        .find(|s| s.id.to_string() == id)
        .ok_or_else(|| format!("Backup {} not found", id))?;

    let mut reverted = 0;
    let mut failed = Vec::new();

    for record in &snapshot.tweak_records {
        if let Some(tweak) = engine.get_tweak(&record.tweak_id) {
            match tweak.revert(&record.snapshot_data).await {
                Ok(_) => reverted += 1,
                Err(e) => failed.push(format!("{}: {:?}", record.tweak_id, e)),
            }
        } else {
            failed.push(format!("{}: tweak not found", record.tweak_id));
        }
    }

    let msg = if failed.is_empty() {
        format!("Backup restored: {} tweak(s) reverted", reverted)
    } else {
        format!(
            "Backup partially restored: {} reverted, {} failed ({})",
            reverted,
            failed.len(),
            failed.join(", ")
        )
    };

    Ok(msg)
}

#[tauri::command]
pub async fn delete_backup(
    id: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let engine = get_engine(&state).await?;
    engine
        .snapshot_service()
        .delete_snapshot(&id)
        .await
        .map(|_| "Backup deleted successfully".to_string())
        .map_err(|e| format!("Failed to delete backup: {:?}", e))
}

#[tauri::command]
pub async fn clear_backups(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let engine = get_engine(&state).await?;
    engine
        .snapshot_service()
        .clear_snapshots()
        .await
        .map(|_| "All backups cleared".to_string())
        .map_err(|e| format!("Failed to clear backups: {:?}", e))
}

// ============================================================================
// Tauri Commands - Favorites
// ============================================================================

#[tauri::command]
pub async fn get_favorites(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let favs = state.favorites.lock().await.get_all();
    to_json(&favs)
}

#[tauri::command]
pub async fn toggle_favorite(
    key: String,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let is_now_fav = state.favorites.lock().await.toggle(key);
    Ok(serde_json::json!({ "is_favorite": is_now_fav }).to_string())
}

#[tauri::command]
pub async fn is_favorite(key: String, state: tauri::State<'_, AppState>) -> Result<bool, String> {
    Ok(state.favorites.lock().await.is_favorite(&key))
}

// ============================================================================
// Tauri Commands - Game Mode
// ============================================================================

#[tauri::command]
pub async fn toggle_game_mode(active: bool) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        use std::process::Command;

        if active {
            let mut results = Vec::new();

            // Set High Performance power plan
            let power_guid = "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c";
            let output = Command::new("powercfg")
                .creation_flags(CREATE_NO_WINDOW)
                .args(["/setactive", power_guid])
                .output();
            results.push(
                if output.as_ref().map(|o| o.status.success()).unwrap_or(false) {
                    "Power plan: High Performance"
                } else {
                    "Power plan: failed"
                },
            );

            // Enable Hardware GPU Scheduling
            let _ = Command::new("reg")
                .creation_flags(CREATE_NO_WINDOW)
                .args([
                    "add",
                    "HKLM\\SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
                    "/v",
                    "HwSchMode",
                    "/t",
                    "REG_DWORD",
                    "/d",
                    "2",
                    "/f",
                ])
                .output();

            // Disable Game Bar (reduces overhead)
            let _ = Command::new("reg")
                .creation_flags(CREATE_NO_WINDOW)
                .args([
                    "add",
                    "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
                    "/v",
                    "AppCaptureEnabled",
                    "/t",
                    "REG_DWORD",
                    "/d",
                    "0",
                    "/f",
                ])
                .output();

            // Disable fullscreen optimizations
            let _ = Command::new("reg")
                .creation_flags(CREATE_NO_WINDOW)
                .args([
                    "add",
                    "HKCU\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\AppCompatFlags\\Layers",
                    "/v",
                    "HIGHDPIAWARE",
                    "/t",
                    "REG_SZ",
                    "/d",
                    "~ DISABLEDXMAXIMIZEDWINDOWEDMODE",
                    "/f",
                ])
                .output();

            Ok(format!("Game Mode activated: {}", results.join(", ")))
        } else {
            let mut results = Vec::new();

            // Set Balanced power plan
            let power_guid = "381b4222-f694-41f0-9685-ff5bb260df2e";
            let output = Command::new("powercfg")
                .creation_flags(CREATE_NO_WINDOW)
                .args(["/setactive", power_guid])
                .output();
            results.push(
                if output.as_ref().map(|o| o.status.success()).unwrap_or(false) {
                    "Power plan: Balanced"
                } else {
                    "Power plan: failed"
                },
            );

            // Reset Hardware GPU Scheduling to default (1 = enabled, which is Windows default)
            let _ = Command::new("reg")
                .creation_flags(CREATE_NO_WINDOW)
                .args([
                    "add",
                    "HKLM\\SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
                    "/v",
                    "HwSchMode",
                    "/t",
                    "REG_DWORD",
                    "/d",
                    "1",
                    "/f",
                ])
                .output();

            // Re-enable Game Bar
            let _ = Command::new("reg")
                .creation_flags(CREATE_NO_WINDOW)
                .args([
                    "add",
                    "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
                    "/v",
                    "AppCaptureEnabled",
                    "/t",
                    "REG_DWORD",
                    "/d",
                    "1",
                    "/f",
                ])
                .output();

            Ok(format!("Game Mode deactivated: {}", results.join(", ")))
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok(if active {
        "Game Mode activated (simulation)".to_string()
    } else {
        "Game Mode deactivated (simulation)".to_string()
    })
}
