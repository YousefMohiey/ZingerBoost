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
    let rp = WinRegistryProvider::new();
    vec![
        // ---- Visual Effects (consolidated) ----
        Arc::new(
            zb_domain::tweaks::definitions::VisualBestPerformanceTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::ShowFileExtensionsTweak::with_provider(rp.clone()),
        ),
        Arc::new(zb_domain::tweaks::definitions::EndTaskOnTaskbarTweak::with_provider(rp.clone())),
        Arc::new(zb_domain::tweaks::definitions::VerboseLogonTweak::with_provider(rp.clone())),
        // ---- Privacy ----
        Arc::new(zb_domain::tweaks::definitions::DisableTelemetryTweak::with_provider(rp.clone())),
        Arc::new(
            zb_domain::tweaks::definitions::DisableBackgroundAppsTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableAdvertisingIdTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableLockScreenAdsTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableStartSuggestionsTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableExplorerAdsTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableActivityHistoryTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableTailoredExperiencesTweak::with_provider(
                rp.clone(),
            ),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableFeedbackFrequencyTweak::with_provider(
                rp.clone(),
            ),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableLocationServicesTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableCortanaRegistryTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableConsumerFeaturesTweak::with_provider(rp.clone()),
        ),
        Arc::new(zb_domain::tweaks::definitions::DisableWpbtTweak::with_provider(rp.clone())),
        Arc::new(zb_domain::tweaks::definitions::DisableMeetNowTweak::with_provider(rp.clone())),
        // ---- Performance ----
        Arc::new(zb_domain::tweaks::definitions::DisableHibernationTweak::new()),
        Arc::new(zb_domain::tweaks::definitions::SetHighPerformanceTweak::new()),
        Arc::new(
            zb_domain::tweaks::definitions::DisableMemoryCompressionTweak::with_provider(
                rp.clone(),
            ),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableStartupDelayTweak::with_provider(rp.clone()),
        ),
        // ---- Gaming ----
        Arc::new(zb_domain::tweaks::definitions::DisableGameDvrTweak::with_provider(rp.clone())),
        Arc::new(
            zb_domain::tweaks::definitions::DisableFullscreenOptimizationsTweak::with_provider(
                rp.clone(),
            ),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::EnableHwGpuSchedulingTweak::with_provider(rp.clone()),
        ),
        // ---- Network ----
        Arc::new(
            zb_domain::tweaks::definitions::DisableNaglesAlgorithmTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableNetworkThrottlingTweak::with_provider(
                rp.clone(),
            ),
        ),
        Arc::new(zb_domain::tweaks::definitions::SetTcpAutotuningNormalTweak::new()),
        Arc::new(zb_domain::tweaks::definitions::DisableWifiSenseTweak::with_provider(rp.clone())),
        // ---- Windows Update ----
        Arc::new(
            zb_domain::tweaks::definitions::DisableAutoDriverUpdatesTweak::with_provider(
                rp.clone(),
            ),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableWUAutoRebootTweak::with_provider(rp.clone()),
        ),
        Arc::new(
            zb_domain::tweaks::definitions::DisableDeliveryOptimizationTweak::with_provider(
                rp.clone(),
            ),
        ),
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
        use zb_shared::constants::CREATE_NO_WINDOW;

        // First, try to find and run the NSIS uninstaller
        let program_files =
            std::env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".into());
        let uninstaller_path = PathBuf::from(&program_files)
            .join("ZingerBoost")
            .join("Uninstall ZingerBoost.exe");

        if uninstaller_path.exists() {
            // Run the NSIS uninstaller silently
            // Note: Cannot uninstall while app is running - NSIS will fail with "another application"
            // We need to spawn it as a detached process so it can try to uninstall
            use std::process::Command;

            // Use cmd /c start to spawn detached - the uninstaller will try to run
            // and likely fail because app is in use, but we give user instructions
            let result = Command::new("cmd")
                .creation_flags(CREATE_NO_WINDOW)
                .args(["/C", "start", "", &uninstaller_path.to_string_lossy()])
                .spawn();

            match result {
                Ok(_) => {
                    // Clean up app data immediately (this works even while app runs)
                    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
                        let dir = PathBuf::from(local_app_data).join("ZingerBoost");
                        let _ = std::fs::remove_dir_all(&dir);
                    }

                    // Check if uninstaller was able to run
                    // Give it a moment then check if files still exist
                    std::thread::sleep(std::time::Duration::from_millis(500));

                    let program_files = std::env::var("ProgramFiles")
                        .unwrap_or_else(|_| "C:\\Program Files".into());
                    let install_dir = PathBuf::from(&program_files).join("ZingerBoost");

                    if install_dir.exists() {
                        // Uninstaller likely couldn't remove (app in use) - guide user
                        Ok("Please close ZingerBoost completely, then try again from Windows Settings > Apps > ZingerBoost > Uninstall, or use the Start Menu shortcut.".to_string())
                    } else {
                        Ok("Uninstalled successfully. You may need to restart your PC to complete removal.".to_string())
                    }
                }
                Err(e) => Err(format!("Failed to launch uninstaller: {}", e)),
            }
        } else {
            // Fallback: just clean up app data if no uninstaller found
            if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
                let dir = PathBuf::from(local_app_data).join("ZingerBoost");
                if dir.exists() {
                    match std::fs::remove_dir_all(&dir) {
                        Ok(_) => Ok(
                            "Removed app data. Please manually uninstall from Settings > Apps."
                                .to_string(),
                        ),
                        Err(e) => Err(format!("Failed to remove data: {}", e)),
                    }
                } else {
                    Ok("App data not found. Please uninstall from Settings > Apps.".to_string())
                }
            } else {
                Err("Could not find LOCALAPPDATA".to_string())
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
        use std::os::windows::process::CommandExt;
        use std::process::Command;

        let output = Command::new("net")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["session"])
            .output();

        match output {
            Ok(o) => Ok(o.status.success()),
            Err(_) => Ok(false),
        }
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
        match tweak.capture_state().await {
            Ok(snapshot_data) => match tweak.apply().await {
                Ok(_result) => {
                    applied += 1;
                    if let Err(e) = engine
                        .snapshot_service()
                        .save_applied(&id, snapshot_data)
                        .await
                    {
                        tracing::error!("Failed to save snapshot for {}: {:?}", id, e);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to apply {}: {:?}", id, e);
                    failed.push(format!("{}: {:?}", id, e));
                }
            },
            Err(e) => {
                tracing::error!("Failed to capture state for {}: {:?}", id, e);
                failed.push(format!("{}: could not capture state: {:?}", id, e));
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
}

#[tauri::command]
pub async fn check_bloatware_installed(winget_id: String) -> Result<bool, String> {
    if winget_id.is_empty() {
        return Ok(true);
    }
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
}

#[tauri::command]
pub async fn install_software(winget_id: String) -> Result<String, String> {
    WingetInstaller::new().install(&winget_id)
}

// ============================================================================
// Tauri Commands - Metrics
// ============================================================================

#[tauri::command]
pub async fn get_metrics(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let metrics = state.metrics.current().await;
    tracing::debug!(
        "[metrics] cpu={:.1}% ram={:.1}% disk={:.1}% net={:.2}/{:.2}Mbps",
        metrics.cpu_percent,
        metrics.ram_percent,
        metrics.disk_active_percent,
        metrics.network_down_mbps,
        metrics.network_up_mbps
    );
    let dto = MetricsDto {
        cpu_percent: metrics.cpu_percent,
        ram_percent: metrics.ram_percent,
        ram_used_mb: metrics.ram_used_mb,
        ram_total_mb: metrics.ram_total_mb,
        disk_active_percent: metrics.disk_active_percent,
        network_down_mbps: metrics.network_down_mbps,
        network_up_mbps: metrics.network_up_mbps,
    };
    to_json(&dto)
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

            // Reset Hardware GPU Scheduling to default
            let _ = Command::new("reg")
                .creation_flags(CREATE_NO_WINDOW)
                .args([
                    "delete",
                    "HKLM\\SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
                    "/v",
                    "HwSchMode",
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
