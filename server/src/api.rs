use std::sync::Arc;
use tokio::sync::Mutex;
use actix_web::{web, HttpResponse};
use crate::app::AppState;

type DS = web::Data<Arc<Mutex<AppState>>>;

pub async fn get_metrics(s: DS) -> HttpResponse {
    let app = s.lock().await;
    let m = app.metrics.current().await;
    HttpResponse::Ok().json(serde_json::json!({"success":true,"data":{
        "cpu_percent":m.cpu_percent,"ram_percent":m.ram_percent,
        "ram_used_mb":m.ram_used_mb,"ram_total_mb":m.ram_total_mb,
        "disk_active_percent":m.disk_active_percent,
        "network_down_mbps":m.network_down_mbps,"network_up_mbps":m.network_up_mbps
    }}))
}

pub async fn list_tweaks(s: DS) -> HttpResponse {
    let app = s.lock().await;
    let t = app.engine.list_tweaks();
    let meta: Vec<_> = t.iter().map(|t| t.metadata()).collect();
    HttpResponse::Ok().json(serde_json::json!({"success":true,"data":meta}))
}

pub async fn apply_tweak(s: DS, body: web::Json<serde_json::Value>) -> HttpResponse {
    let id = body["id"].as_str().unwrap_or("");
    let app = s.lock().await;
    let rt = tokio::runtime::Handle::current();
    match rt.block_on(app.engine.apply_single(id)) {
        Ok(r) => HttpResponse::Ok().json(serde_json::json!({"success":true,"data":{"message":r.message}})),
        Err(e) => HttpResponse::Ok().json(serde_json::json!({"success":false,"error":e.to_string()})),
    }
}

pub async fn revert_tweak(s: DS, body: web::Json<serde_json::Value>) -> HttpResponse {
    let id = body["id"].as_str().unwrap_or("");
    let app = s.lock().await;
    let rt = tokio::runtime::Handle::current();
    match rt.block_on(app.engine.revert(id)) {
        Ok(r) => HttpResponse::Ok().json(serde_json::json!({"success":true,"data":{"message":r.message}})),
        Err(e) => HttpResponse::Ok().json(serde_json::json!({"success":false,"error":e.to_string()})),
    }
}

pub async fn list_services(s: DS) -> HttpResponse {
    let app = s.lock().await;
    let svcs = app.service_ctrl.query_services();
    HttpResponse::Ok().json(serde_json::json!({"success":true,"data":svcs}))
}

pub async fn stop_service(s: DS, body: web::Json<serde_json::Value>) -> HttpResponse {
    let name = body["name"].as_str().unwrap_or("");
    let app = s.lock().await;
    match app.service_ctrl.stop_service(name) {
        Ok(msg) => HttpResponse::Ok().json(serde_json::json!({"success":true,"message":msg})),
        Err(e) => HttpResponse::Ok().json(serde_json::json!({"success":false,"error":e})),
    }
}

pub async fn disable_service(s: DS, body: web::Json<serde_json::Value>) -> HttpResponse {
    let name = body["name"].as_str().unwrap_or("");
    let app = s.lock().await;
    match app.service_ctrl.disable_service(name) {
        Ok(msg) => HttpResponse::Ok().json(serde_json::json!({"success":true,"message":msg})),
        Err(e) => HttpResponse::Ok().json(serde_json::json!({"success":false,"error":e})),
    }
}

pub async fn scan_cleaner(s: DS) -> HttpResponse {
    let app = s.lock().await;
    let r = app.cleaner.scan_categories();
    HttpResponse::Ok().json(serde_json::json!({"success":true,"data":r}))
}

pub async fn run_cleaner(s: DS, body: web::Json<serde_json::Value>) -> HttpResponse {
    let id = body["id"].as_str().unwrap_or("");
    let app = s.lock().await;
    let r = app.cleaner.clean_category(id);
    HttpResponse::Ok().json(serde_json::json!({"success":r.success,"data":r}))
}

pub async fn list_bloatware(_s: DS) -> HttpResponse {
    let bloat = zb_shared::software::get_bloatware_catalog();
    let protected = zb_shared::software::get_protected_apps();
    HttpResponse::Ok().json(serde_json::json!({"success":true,"data":{"bloatware":bloat,"protected":protected}}))
}

pub async fn remove_bloatware(_s: DS, body: web::Json<serde_json::Value>) -> HttpResponse {
    let name = body["name"].as_str().unwrap_or("");
    match zb_infrastructure::windows_api::debloat_engine::DebloatEngine::remove_appx_package(name) {
        Ok(msg) => HttpResponse::Ok().json(serde_json::json!({"success":true,"message":msg})),
        Err(e) => HttpResponse::Ok().json(serde_json::json!({"success":false,"error":e.to_string()})),
    }
}

pub async fn list_software(_s: DS) -> HttpResponse {
    let catalog = zb_shared::software::get_software_catalog();
    HttpResponse::Ok().json(serde_json::json!({"success":true,"data":catalog}))
}

pub async fn list_snapshots(s: DS) -> HttpResponse {
    let app = s.lock().await;
    let rt = tokio::runtime::Handle::current();
    match rt.block_on(app.engine.snapshot_service().list_snapshots()) {
        Ok(sn) => HttpResponse::Ok().json(serde_json::json!({"success":true,"data":sn})),
        Err(e) => HttpResponse::Ok().json(serde_json::json!({"success":false,"error":e.to_string()})),
    }
}

pub async fn get_audit(s: DS) -> HttpResponse {
    let app = s.lock().await;
    let entries = app.engine.audit_service().get_recent(100).await;
    HttpResponse::Ok().json(serde_json::json!({"success":true,"data":entries}))
}
