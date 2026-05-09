use std::sync::Arc;
use tokio::sync::Mutex;
use crate::app::AppState;

// All api handlers return JSON
type ApiResult = Result<warp::reply::Json, warp::Rejection>;

fn j(v: serde_json::Value) -> ApiResult {
    Ok(warp::reply::json(&v))
}

pub async fn get_metrics(state: Arc<Mutex<AppState>>) -> ApiResult {
    let app = state.lock().await;
    let m = app.metrics.current().await;
    j(serde_json::json!({"success":true,"data":{
        "cpu_percent":m.cpu_percent,"ram_percent":m.ram_percent,
        "ram_used_mb":m.ram_used_mb,"ram_total_mb":m.ram_total_mb,
        "disk_active_percent":m.disk_active_percent,
        "network_down_mbps":m.network_down_mbps,"network_up_mbps":m.network_up_mbps
    }}))
}

pub async fn list_tweaks(state: Arc<Mutex<AppState>>) -> ApiResult {
    let app = state.lock().await;
    let tweaks = app.engine.list_tweaks();
    let metadata: Vec<_> = tweaks.iter().map(|t| t.metadata()).collect();
    j(serde_json::json!({"success":true,"data":metadata}))
}

pub async fn apply_tweak(body: serde_json::Value, state: Arc<Mutex<AppState>>) -> ApiResult {
    let id = body["id"].as_str().unwrap_or("");
    let app = state.lock().await;
    let rt = tokio::runtime::Handle::current();
    match rt.block_on(app.engine.apply_single(id)) {
        Ok(r) => j(serde_json::json!({"success":true,"data":{"message":r.message,"reboot":r.reboot_required}})),
        Err(e) => j(serde_json::json!({"success":false,"error":e.to_string()})),
    }
}

pub async fn revert_tweak(body: serde_json::Value, state: Arc<Mutex<AppState>>) -> ApiResult {
    let id = body["id"].as_str().unwrap_or("");
    let app = state.lock().await;
    let rt = tokio::runtime::Handle::current();
    match rt.block_on(app.engine.revert(id)) {
        Ok(r) => j(serde_json::json!({"success":true,"data":{"message":r.message}})),
        Err(e) => j(serde_json::json!({"success":false,"error":e.to_string()})),
    }
}

pub async fn list_services(state: Arc<Mutex<AppState>>) -> ApiResult {
    let app = state.lock().await;
    let services = app.service_ctrl.query_services();
    j(serde_json::json!({"success":true,"data":services}))
}

pub async fn stop_service(body: serde_json::Value, state: Arc<Mutex<AppState>>) -> ApiResult {
    let name = body["name"].as_str().unwrap_or("");
    let app = state.lock().await;
    match app.service_ctrl.stop_service(name) {
        Ok(msg) => j(serde_json::json!({"success":true,"message":msg})),
        Err(e) => j(serde_json::json!({"success":false,"error":e})),
    }
}

pub async fn disable_service(body: serde_json::Value, state: Arc<Mutex<AppState>>) -> ApiResult {
    let name = body["name"].as_str().unwrap_or("");
    let app = state.lock().await;
    match app.service_ctrl.disable_service(name) {
        Ok(msg) => j(serde_json::json!({"success":true,"message":msg})),
        Err(e) => j(serde_json::json!({"success":false,"error":e})),
    }
}

pub async fn scan_cleaner(state: Arc<Mutex<AppState>>) -> ApiResult {
    let app = state.lock().await;
    let results = app.cleaner.scan_categories();
    j(serde_json::json!({"success":true,"data":results}))
}

pub async fn run_cleaner(body: serde_json::Value, state: Arc<Mutex<AppState>>) -> ApiResult {
    let id = body["id"].as_str().unwrap_or("");
    let app = state.lock().await;
    let result = app.cleaner.clean_category(id);
    j(serde_json::json!({"success":result.success,"data":result}))
}

pub async fn list_bloatware(state: Arc<Mutex<AppState>>) -> ApiResult {
    let bloat = zb_shared::software::get_bloatware_catalog();
    let protected = zb_shared::software::get_protected_apps();
    j(serde_json::json!({"success":true,"data":{"bloatware":bloat,"protected":protected}}))
}

pub async fn remove_bloatware(body: serde_json::Value, state: Arc<Mutex<AppState>>) -> ApiResult {
    let name = body["name"].as_str().unwrap_or("");
    match zb_infrastructure::windows_api::debloat_engine::DebloatEngine::remove_appx_package(name) {
        Ok(msg) => j(serde_json::json!({"success":true,"message":msg})),
        Err(e) => j(serde_json::json!({"success":false,"error":e.to_string()})),
    }
}

pub async fn list_software(state: Arc<Mutex<AppState>>) -> ApiResult {
    let catalog = zb_shared::software::get_software_catalog();
    j(serde_json::json!({"success":true,"data":catalog}))
}

pub async fn list_snapshots(state: Arc<Mutex<AppState>>) -> ApiResult {
    let app = state.lock().await;
    let rt = tokio::runtime::Handle::current();
    match rt.block_on(app.engine.snapshot_service().list_snapshots()) {
        Ok(s) => j(serde_json::json!({"success":true,"data":s})),
        Err(e) => j(serde_json::json!({"success":false,"error":e.to_string()})),
    }
}

pub async fn get_audit(state: Arc<Mutex<AppState>>) -> ApiResult {
    let app = state.lock().await;
    let entries = app.engine.audit_service().get_recent(100).await;
    j(serde_json::json!({"success":true,"data":entries}))
}
