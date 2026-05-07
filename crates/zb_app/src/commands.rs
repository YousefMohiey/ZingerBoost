use tauri::State;
use zb_application::dto::{
    app_error_from_anyhow, app_error_from_snapshot, app_error_from_tweak, ApplyRequestDto,
    AuditLogDto, BatchApplyRequestDto, SystemMetricsDto, TweakExplanationDto, TweakListDto,
    TweakResultDto,
};
use zb_shared::types::AppErrorDto;

use crate::AppState;

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
