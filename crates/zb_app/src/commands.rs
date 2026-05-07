use tauri::State;
use zb_application::dto::{
    AppErrorDto, ApplyRequestDto, BatchApplyRequestDto, SystemMetricsDto, TweakExplanationDto,
    TweakListDto, TweakResultDto,
};
use zb_shared::types::{AppErrorDto as AppError, TweakExplanation};

use crate::AppState;

#[tauri::command]
pub async fn list_tweaks(state: State<'_, AppState>) -> Result<TweakListDto, AppError> {
    let tweaks = state.engine.list_tweaks();
    let metadata = tweaks.into_iter().map(|t| t.metadata()).collect();
    Ok(TweakListDto { tweaks: metadata })
}

#[tauri::command]
pub async fn apply_tweak(
    state: State<'_, AppState>,
    request: ApplyRequestDto,
) -> Result<TweakResultDto, AppError> {
    let result = state.engine.apply_single(&request.id).await?;
    Ok(result.into())
}

#[tauri::command]
pub async fn batch_apply_tweaks(
    state: State<'_, AppState>,
    request: BatchApplyRequestDto,
) -> Result<Vec<(String, TweakResultDto)>, AppError> {
    let results = state.engine.apply_batch(&request.ids).await?;
    Ok(results
        .into_iter()
        .map(|(id, r)| (id, r.into()))
        .collect())
}

#[tauri::command]
pub async fn revert_tweak(
    state: State<'_, AppState>,
    request: ApplyRequestDto,
) -> Result<TweakResultDto, AppError> {
    let result = state.engine.revert(&request.id).await?;
    Ok(result.into())
}

#[tauri::command]
pub async fn get_metrics(state: State<'_, AppState>) -> Result<SystemMetricsDto, AppError> {
    let metrics = state.metrics_collector.current().await;
    Ok(metrics.into())
}

#[tauri::command]
pub async fn get_tweak_explanation(
    state: State<'_, AppState>,
    id: String,
) -> Result<TweakExplanationDto, AppError> {
    let tweak = state
        .engine
        .get_tweak(&id)
        .ok_or_else(|| AppErrorDto::from(zb_domain::errors::TweakError::Validation(format!("Unknown tweak: {}", id))))?;
    Ok(tweak.explain().into())
}
