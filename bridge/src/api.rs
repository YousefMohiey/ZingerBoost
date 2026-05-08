use crate::{get_app, AppState};
use serde::{Deserialize, Serialize};
use zb_infrastructure::services::ServiceController;
use zb_infrastructure::windows_api::debloat_engine::DebloatEngine;
use zb_infrastructure::windows_api::system_cleaner::SystemCleaner;
use zb_shared::software::{get_bloatware_catalog, get_protected_apps, get_software_catalog};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FfiResult<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> FfiResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    pub fn err(msg: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

pub fn list_tweaks() -> String {
    let app = get_app();
    let tweaks = app.engine.list_tweaks();
    let metadata: Vec<_> = tweaks.into_iter().map(|t| t.metadata()).collect();
    serde_json::to_string(&metadata).unwrap_or_default()
}

pub fn apply_tweak(id: String) -> String {
    let app = get_app();
    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(app.engine.apply_single(&id)) {
        Ok(result) => serde_json::to_string(&result).unwrap_or_default(),
        Err(e) => format!("{{\"error\":\"{}\"}}", e),
    }
}

pub fn revert_tweak(id: String) -> String {
    let app = get_app();
    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(app.engine.revert(&id)) {
        Ok(result) => serde_json::to_string(&result).unwrap_or_default(),
        Err(e) => format!("{{\"error\":\"{}\"}}", e),
    }
}

pub fn get_metrics() -> String {
    let app = get_app();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let metrics = rt.block_on(app.metrics_collector.current());
    serde_json::to_string(&metrics).unwrap_or_default()
}

pub fn get_tweak_explanation(id: String) -> String {
    let app = get_app();
    if let Some(tweak) = app.engine.get_tweak(&id) {
        serde_json::to_string(&tweak.explain()).unwrap_or_default()
    } else {
        "{}".to_string()
    }
}

pub fn list_snapshots() -> String {
    let app = get_app();
    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(app.engine.snapshot_service().list_snapshots()) {
        Ok(snapshots) => serde_json::to_string(&snapshots).unwrap_or_default(),
        Err(_) => "[]".to_string(),
    }
}

pub fn restore_snapshot(id: String) -> String {
    let app = get_app();
    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(app.engine.snapshot_service().restore_snapshot(&id)) {
        Ok(()) => serde_json::json!({"success": true}).to_string(),
        Err(e) => serde_json::json!({"success": false, "message": e.to_string()}).to_string(),
    }
}

pub fn get_audit_log(limit: i32) -> String {
    let app = get_app();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let entries = rt.block_on(app.engine.audit_service().get_recent(limit as usize));
    serde_json::to_string(&entries).unwrap_or_default()
}

pub fn list_software() -> String {
    let catalog = get_software_catalog();
    serde_json::to_string(&catalog).unwrap_or_default()
}

pub fn list_bloatware() -> String {
    let bloat = get_bloatware_catalog();
    let protected = get_protected_apps();
    let result = serde_json::json!({ "bloatware": bloat, "protected": protected });
    serde_json::to_string(&result).unwrap_or_default()
}

pub fn install_software(winget_id: String) -> String {
    let app = get_app();
    match app.winget.install(&winget_id) {
        Ok(msg) => serde_json::json!({"success": true, "message": msg}).to_string(),
        Err(e) => serde_json::json!({"success": false, "message": e}).to_string(),
    }
}

pub fn remove_bloatware(package_ids_json: String) -> String {
    let app = get_app();
    let ids: Vec<String> = serde_json::from_str(&package_ids_json).unwrap_or_default();
    let mut results = Vec::new();
    let rt = tokio::runtime::Runtime::new().unwrap();
    for id in ids {
        match rt.block_on(async { DebloatEngine::remove_appx_package(&id) }) {
            Ok(msg) => results.push(serde_json::json!({"success": true, "message": msg})),
            Err(e) => results.push(serde_json::json!({"success": false, "message": e.to_string()})),
        }
    }
    serde_json::to_string(&results).unwrap_or_default()
}

// --- Services ---

pub fn list_services() -> String {
    let sc = ServiceController::new();
    let services = sc.query_services();
    serde_json::to_string(&services).unwrap_or_default()
}

pub fn stop_service(name: String) -> String {
    let sc = ServiceController::new();
    match sc.stop_service(&name) {
        Ok(msg) => serde_json::json!({"success": true, "message": msg}).to_string(),
        Err(e) => serde_json::json!({"success": false, "message": e}).to_string(),
    }
}

pub fn disable_service(name: String) -> String {
    let sc = ServiceController::new();
    match sc.disable_service(&name) {
        Ok(msg) => serde_json::json!({"success": true, "message": msg}).to_string(),
        Err(e) => serde_json::json!({"success": false, "message": e}).to_string(),
    }
}

// --- System Cleaner ---

pub fn scan_cleaner() -> String {
    let cleaner = SystemCleaner::new();
    let categories = cleaner.scan_categories();
    serde_json::to_string(&categories).unwrap_or_default()
}

pub fn run_cleaner(category: String) -> String {
    let cleaner = SystemCleaner::new();
    let result = cleaner.clean_category(&category);
    serde_json::to_string(&result).unwrap_or_default()
}
