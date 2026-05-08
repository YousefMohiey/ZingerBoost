use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;
use zb_application::audit_service::AuditService;
use zb_application::snapshot_service::SnapshotService;
use zb_application::tweak_engine::TweakEngine;
use zb_domain::tweaks::definitions::*;
use zb_infrastructure::logging::init_logging;
use zb_infrastructure::persistence::{init_database, SqliteAuditLogger, SqliteRepo};
use zb_infrastructure::registry::WinRegistryProvider;
use zb_infrastructure::windows_api::debloat_engine::DebloatEngine;
use zb_infrastructure::windows_api::metrics_collector::MetricsCollector;
use zb_infrastructure::windows_api::winget::WingetInstaller;

pub mod api;

pub struct AppState {
    pub engine: Arc<TweakEngine>,
    pub metrics_collector: Arc<MetricsCollector>,
    pub winget: WingetInstaller,
}

static APP: OnceLock<AppState> = OnceLock::new();

fn get_app() -> &'static AppState {
    APP.get()
        .expect("App not initialized. Call init_app() first.")
}

#[no_mangle]
pub extern "C" fn init_app() -> i32 {
    init_logging();
    let db_conn = init_database().expect("Failed to init database");
    let registry_provider = WinRegistryProvider::new();

    let rp = registry_provider.clone();
    let tweaks: Vec<Arc<dyn zb_domain::tweaks::Tweak>> = vec![
        Arc::new(DisableGameDvrTweak::with_provider(rp.clone())),
        Arc::new(DisableTransparencyTweak::with_provider(rp.clone())),
        Arc::new(DisableAnimationsTweak::with_provider(rp.clone())),
        Arc::new(ShowFileExtensionsTweak::with_provider(rp.clone())),
        Arc::new(DisableStickyKeysTweak::with_provider(rp.clone())),
        Arc::new(DisableStartupDelayTweak::with_provider(rp.clone())),
        Arc::new(DisableBackgroundAppsTweak::with_provider(rp.clone())),
        Arc::new(DisableTelemetryTweak::with_provider(rp.clone())),
        Arc::new(DisableMenuDelayTweak::with_provider(rp.clone())),
        Arc::new(DisableCursorShadowTweak::with_provider(rp.clone())),
        Arc::new(DisableFontSmoothingTweak::with_provider(rp.clone())),
        Arc::new(DisableTaskbarAnimationsTweak::with_provider(rp.clone())),
        Arc::new(DisableAeroShakeTweak::with_provider(rp.clone())),
        Arc::new(DisableAeroSnapTweak::with_provider(rp.clone())),
        Arc::new(DisablePeekTweak::with_provider(rp.clone())),
        Arc::new(DisableSmoothScrollTweak::with_provider(rp.clone())),
        Arc::new(DisableComboAnimationTweak::with_provider(rp.clone())),
        Arc::new(DisableTaskbarBadgesTweak::with_provider(rp.clone())),
        Arc::new(DisableLockScreenAdsTweak::with_provider(rp.clone())),
        Arc::new(DisableStartSuggestionsTweak::with_provider(rp.clone())),
        Arc::new(DisableExplorerAdsTweak::with_provider(rp.clone())),
        Arc::new(DisableAdvertisingIdTweak::with_provider(rp.clone())),
        Arc::new(DisableMeetNowTweak::with_provider(rp.clone())),
        Arc::new(DisableAllVisualEffectsTweak::with_provider(rp.clone())),
        Arc::new(DisableDropShadowsTweak::with_provider(rp.clone())),
        Arc::new(DisableThumbnailsTweak::with_provider(rp.clone())),
        Arc::new(DisableMinMaxAnimTweak::with_provider(rp.clone())),
        Arc::new(DisableHibernationTweak::new()),
        Arc::new(SetHighPerformanceTweak::new()),
    ];

    let snapshot_service: Arc<dyn SnapshotService> = SqliteRepo::from_connection(db_conn.clone());
    let audit_service: Arc<dyn AuditService> = SqliteAuditLogger::from_connection(db_conn);
    let engine = Arc::new(TweakEngine::new(tweaks, snapshot_service, audit_service));
    let metrics_collector = MetricsCollector::new();
    let winget = WingetInstaller::new();

    let state = AppState {
        engine,
        metrics_collector,
        winget,
    };
    APP.set(state).map(|_| 0).unwrap_or(0);
    0
}

fn c_string(value: String) -> *mut c_char {
    match CString::new(value) {
        Ok(s) => s.into_raw(),
        Err(_) => CString::new("{\"error\":\"invalid string\"}")
            .expect("static error string is valid")
            .into_raw(),
    }
}

unsafe fn string_from_ptr(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

#[no_mangle]
pub extern "C" fn zingerboost_free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn zingerboost_list_tweaks() -> *mut c_char {
    c_string(api::list_tweaks())
}

#[no_mangle]
pub extern "C" fn zingerboost_apply_tweak(id: *const c_char) -> *mut c_char {
    c_string(api::apply_tweak(unsafe { string_from_ptr(id) }))
}

#[no_mangle]
pub extern "C" fn zingerboost_revert_tweak(id: *const c_char) -> *mut c_char {
    c_string(api::revert_tweak(unsafe { string_from_ptr(id) }))
}

#[no_mangle]
pub extern "C" fn zingerboost_get_metrics() -> *mut c_char {
    c_string(api::get_metrics())
}

#[no_mangle]
pub extern "C" fn zingerboost_get_tweak_explanation(id: *const c_char) -> *mut c_char {
    c_string(api::get_tweak_explanation(unsafe { string_from_ptr(id) }))
}

#[no_mangle]
pub extern "C" fn zingerboost_list_snapshots() -> *mut c_char {
    c_string(api::list_snapshots())
}

#[no_mangle]
pub extern "C" fn zingerboost_restore_snapshot(id: *const c_char) -> *mut c_char {
    c_string(api::restore_snapshot(unsafe { string_from_ptr(id) }))
}

#[no_mangle]
pub extern "C" fn zingerboost_list_software() -> *mut c_char {
    c_string(api::list_software())
}

#[no_mangle]
pub extern "C" fn zingerboost_list_bloatware() -> *mut c_char {
    c_string(api::list_bloatware())
}

#[no_mangle]
pub extern "C" fn zingerboost_install_software(winget_id: *const c_char) -> *mut c_char {
    c_string(api::install_software(unsafe { string_from_ptr(winget_id) }))
}

#[no_mangle]
pub extern "C" fn zingerboost_remove_bloatware(package_ids_json: *const c_char) -> *mut c_char {
    c_string(api::remove_bloatware(unsafe {
        string_from_ptr(package_ids_json)
    }))
}

#[no_mangle]
pub extern "C" fn zingerboost_list_services() -> *mut c_char {
    c_string(api::list_services())
}

#[no_mangle]
pub extern "C" fn zingerboost_stop_service(name: *const c_char) -> *mut c_char {
    c_string(api::stop_service(unsafe { string_from_ptr(name) }))
}

#[no_mangle]
pub extern "C" fn zingerboost_disable_service(name: *const c_char) -> *mut c_char {
    c_string(api::disable_service(unsafe { string_from_ptr(name) }))
}

#[no_mangle]
pub extern "C" fn zingerboost_scan_cleaner() -> *mut c_char {
    c_string(api::scan_cleaner())
}

#[no_mangle]
pub extern "C" fn zingerboost_run_cleaner(category: *const c_char) -> *mut c_char {
    c_string(api::run_cleaner(unsafe { string_from_ptr(category) }))
}
