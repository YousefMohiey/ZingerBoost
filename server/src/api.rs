use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use serde::{Deserialize, Serialize};
use zb_infrastructure::windows_api::debloat_engine::DebloatEngine;

use crate::app::AppState;

pub async fn get_metrics(state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let app = state.lock().await;
    let metrics = app.metrics.current().await;
    Ok(warp::reply::json(&serde_json::json!({
        "success": true,
        "data": {
            "cpu_percent": metrics.cpu_percent,
            "ram_percent": metrics.ram_percent,
            "ram_used_mb": metrics.ram_used_mb,
            "ram_total_mb": metrics.ram_total_mb,
            "disk_active_percent": metrics.disk_active_percent,
            "network_down_mbps": metrics.network_down_mbps,
            "network_up_mbps": metrics.network_up_mbps,
        }
    })))
}

pub async fn list_tweaks(state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let app = state.lock().await;
    let tweaks = app.engine.list_tweaks();
    let metadata: Vec<_> = tweaks.iter().map(|t| t.metadata()).collect();
    Ok(warp::reply::json(&serde_json::json!({ "success": true, "data": metadata })))
}

pub async fn apply_tweak(body: serde_json::Value, state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let id = body["id"].as_str().unwrap_or("");
    let app = state.lock().await;
    let rt = tokio::runtime::Handle::current();
    match rt.block_on(app.engine.apply_single(id)) {
        Ok(r) => Ok(warp::reply::json(&serde_json::json!({"success": true, "data": {"message": r.message, "reboot": r.reboot_required}}))),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"success": false, "error": e.to_string()}))),
    }
}

pub async fn revert_tweak(body: serde_json::Value, state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let id = body["id"].as_str().unwrap_or("");
    let app = state.lock().await;
    let rt = tokio::runtime::Handle::current();
    match rt.block_on(app.engine.revert(id)) {
        Ok(r) => Ok(warp::reply::json(&serde_json::json!({"success": true, "data": {"message": r.message}}))),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"success": false, "error": e.to_string()}))),
    }
}

pub async fn list_services(state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let app = state.lock().await;
    let services = app.service_ctrl.query_services();
    Ok(warp::reply::json(&serde_json::json!({ "success": true, "data": services })))
}

pub async fn stop_service(body: serde_json::Value, state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let name = body["name"].as_str().unwrap_or("");
    let app = state.lock().await;
    match app.service_ctrl.stop_service(name) {
        Ok(msg) => Ok(warp::reply::json(&serde_json::json!({"success": true, "message": msg}))),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"success": false, "error": e}))),
    }
}

pub async fn disable_service(body: serde_json::Value, state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let name = body["name"].as_str().unwrap_or("");
    let app = state.lock().await;
    match app.service_ctrl.disable_service(name) {
        Ok(msg) => Ok(warp::reply::json(&serde_json::json!({"success": true, "message": msg}))),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"success": false, "error": e}))),
    }
}

pub async fn scan_cleaner(state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let app = state.lock().await;
    let results = app.cleaner.scan_categories();
    Ok(warp::reply::json(&serde_json::json!({"success": true, "data": results})))
}

pub async fn run_cleaner(body: serde_json::Value, state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let id = body["id"].as_str().unwrap_or("");
    let app = state.lock().await;
    let result = app.cleaner.clean_category(id);
    Ok(warp::reply::json(&serde_json::json!({"success": result.success, "data": result})))
}

pub async fn list_bloatware(state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let bloat = zb_shared::software::get_bloatware_catalog();
    let protected = zb_shared::software::get_protected_apps();
    Ok(warp::reply::json(&serde_json::json!({"success": true, "data": {"bloatware": bloat, "protected": protected}})))
}

pub async fn remove_bloatware(body: serde_json::Value, state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let name = body["name"].as_str().unwrap_or("");
    match DebloatEngine::remove_appx_package(name) {
        Ok(msg) => Ok(warp::reply::json(&serde_json::json!({"success": true, "message": msg}))),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"success": false, "error": e.to_string()}))),
    }
}

pub async fn list_software(state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let catalog = zb_shared::software::get_software_catalog();
    Ok(warp::reply::json(&serde_json::json!({"success": true, "data": catalog})))
}

pub async fn list_snapshots(state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let app = state.lock().await;
    let rt = tokio::runtime::Handle::current();
    match rt.block_on(app.engine.snapshot_service().list_snapshots()) {
        Ok(s) => Ok(warp::reply::json(&serde_json::json!({"success": true, "data": s}))),
        Err(e) => Ok(warp::reply::json(&serde_json::json!({"success": false, "error": e.to_string()}))),
    }
}

pub async fn get_audit(state: Arc<Mutex<AppState>>) -> Result<impl warp::Reply, warp::Rejection> {
    let app = state.lock().await;
    let entries = app.engine.audit_service().get_recent(100).await;
    Ok(warp::reply::json(&serde_json::json!({"success": true, "data": entries})))
}
