use serde::{Deserialize, Serialize};
use tauri::State;
use zb_application::dto::{
    app_error_from_anyhow, app_error_from_snapshot, app_error_from_tweak, ApplyRequestDto,
    AuditLogDto, BatchApplyRequestDto, SystemMetricsDto, TweakExplanationDto, TweakListDto,
    TweakResultDto,
};
use zb_shared::software::{get_bloatware_catalog, get_protected_apps, get_software_catalog};
use zb_shared::types::AppErrorDto;

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallRequest {
    pub winget_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveBloatwareRequest {
    pub package_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallResult {
    pub success: bool,
    pub message: String,
}

#[tauri::command]
pub async fn list_tweaks(state: State<'_, AppState>) -> Result<TweakListDto, AppErrorDto> {
    let tweaks = state.engine.list_tweaks();
    let metadata = tweaks.into_iter().map(|t| t.metadata()).collect();
    Ok(TweakListDto { tweaks: metadata })
}

#[tauri::command]
pub async fn apply_tweak(
    state: State<'_, AppState>,
    request: ApplyRequestDto,
) -> Result<TweakResultDto, AppErrorDto> {
    let result = state
        .engine
        .apply_single(&request.id)
        .await
        .map_err(app_error_from_tweak)?;
    Ok(result.into())
}

#[tauri::command]
pub async fn batch_apply_tweaks(
    state: State<'_, AppState>,
    request: BatchApplyRequestDto,
) -> Result<Vec<(String, TweakResultDto)>, AppErrorDto> {
    let results = state
        .engine
        .apply_batch(&request.ids)
        .await
        .map_err(app_error_from_tweak)?;
    Ok(results.into_iter().map(|(id, r)| (id, r.into())).collect())
}

#[tauri::command]
pub async fn revert_tweak(
    state: State<'_, AppState>,
    request: ApplyRequestDto,
) -> Result<TweakResultDto, AppErrorDto> {
    let result = state
        .engine
        .revert(&request.id)
        .await
        .map_err(app_error_from_tweak)?;
    Ok(result.into())
}

#[tauri::command]
pub async fn get_metrics(state: State<'_, AppState>) -> Result<SystemMetricsDto, AppErrorDto> {
    let metrics = state.metrics_collector.current().await;
    Ok(metrics.into())
}

#[tauri::command]
pub async fn get_tweak_explanation(
    state: State<'_, AppState>,
    id: String,
) -> Result<TweakExplanationDto, AppErrorDto> {
    let tweak = state.engine.get_tweak(&id).ok_or_else(|| AppErrorDto {
        code: "TWEAK_ERROR".into(),
        message: format!("Unknown tweak: {}", id),
        details: None,
    })?;
    Ok(tweak.explain().into())
}

#[tauri::command]
pub async fn list_snapshots(
    state: State<'_, AppState>,
) -> Result<Vec<zb_domain::snapshots::SystemSnapshot>, AppErrorDto> {
    let snapshots = state
        .engine
        .snapshot_service()
        .list_snapshots()
        .await
        .map_err(app_error_from_snapshot)?;
    Ok(snapshots)
}

#[tauri::command]
pub async fn get_audit_log(state: State<'_, AppState>) -> Result<AuditLogDto, AppErrorDto> {
    let entries = state.engine.audit_service().get_recent(100).await;
    Ok(AuditLogDto { entries })
}

#[tauri::command]
pub async fn list_software() -> Result<serde_json::Value, AppErrorDto> {
    let catalog = get_software_catalog();
    Ok(serde_json::to_value(catalog).unwrap_or_default())
}

#[tauri::command]
pub async fn list_bloatware() -> Result<serde_json::Value, AppErrorDto> {
    let bloat = get_bloatware_catalog();
    let protected = get_protected_apps();
    let result = serde_json::json!({
        "bloatware": bloat,
        "protected": protected,
    });
    Ok(result)
}

#[tauri::command]
pub async fn install_software(
    state: State<'_, AppState>,
    request: InstallRequest,
) -> Result<InstallResult, AppErrorDto> {
    let result = state
        .winget
        .install(&request.winget_id)
        .map_err(|e| AppErrorDto {
            code: "INSTALL_ERROR".into(),
            message: e,
            details: None,
        });

    match result {
        Ok(msg) => Ok(InstallResult {
            success: true,
            message: msg,
        }),
        Err(e) => Ok(InstallResult {
            success: false,
            message: e.message,
        }),
    }
}

#[tauri::command]
pub async fn remove_bloatware(
    state: State<'_, AppState>,
    request: RemoveBloatwareRequest,
) -> Result<Vec<InstallResult>, AppErrorDto> {
    let mut results = Vec::new();

    for package_id in &request.package_ids {
        match state.winget.remove_appx(package_id) {
            Ok(_) => results.push(InstallResult {
                success: true,
                message: format!("Removed {}", package_id),
            }),
            Err(e) => results.push(InstallResult {
                success: false,
                message: format!("Failed to remove {}: {}", package_id, e),
            }),
        }
    }

    Ok(results)
}
