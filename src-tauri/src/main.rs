#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::Manager;
use zb_app::AppState;
use zb_application::audit_service::AuditService;
use zb_application::snapshot_service::SnapshotService;
use zb_application::tweak_engine::TweakEngine;
use zb_domain::tweaks::definitions::{
    DisableAnimationsTweak, DisableBackgroundAppsTweak, DisableGameDvrTweak,
    DisableHibernationTweak, DisableStartupDelayTweak, DisableStickyKeysTweak,
    DisableTelemetryTweak, DisableTransparencyTweak, SetHighPerformanceTweak,
    ShowFileExtensionsTweak,
};
use zb_infrastructure::logging::init_logging;
use zb_infrastructure::persistence::{SqliteAuditLogger, SqliteRepo};
use zb_infrastructure::registry::WinRegistryProvider;
use zb_infrastructure::windows_api::metrics_collector::MetricsCollector;
use zb_infrastructure::windows_api::winget::WingetInstaller;

fn main() {
    init_logging();

    let registry_provider = WinRegistryProvider::new();

    let tweaks: Vec<Arc<dyn zb_domain::tweaks::Tweak>> = vec![
        Arc::new(DisableGameDvrTweak::with_provider(
            registry_provider.clone(),
        )),
        Arc::new(DisableTransparencyTweak::with_provider(
            registry_provider.clone(),
        )),
        Arc::new(DisableAnimationsTweak::with_provider(
            registry_provider.clone(),
        )),
        Arc::new(ShowFileExtensionsTweak::with_provider(
            registry_provider.clone(),
        )),
        Arc::new(DisableStickyKeysTweak::with_provider(
            registry_provider.clone(),
        )),
        Arc::new(DisableStartupDelayTweak::with_provider(
            registry_provider.clone(),
        )),
        Arc::new(DisableBackgroundAppsTweak::with_provider(
            registry_provider.clone(),
        )),
        Arc::new(DisableTelemetryTweak::with_provider(
            registry_provider.clone(),
        )),
        Arc::new(DisableHibernationTweak::new()),
        Arc::new(SetHighPerformanceTweak::new()),
    ];

    let snapshot_service: Arc<dyn SnapshotService> =
        SqliteRepo::new_in_memory().expect("Failed to create SQLite snapshot repository");
    let audit_service: Arc<dyn AuditService> =
        SqliteAuditLogger::new_in_memory().expect("Failed to create SQLite audit logger");

    let engine = Arc::new(TweakEngine::new(tweaks, snapshot_service, audit_service));
    let metrics_collector = Arc::new(MetricsCollector::new());
    let winget = WingetInstaller::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            engine,
            metrics_collector,
            winget,
        })
        .invoke_handler(tauri::generate_handler![
            zb_app::commands::list_tweaks,
            zb_app::commands::apply_tweak,
            zb_app::commands::batch_apply_tweaks,
            zb_app::commands::revert_tweak,
            zb_app::commands::get_metrics,
            zb_app::commands::get_tweak_explanation,
            zb_app::commands::list_snapshots,
            zb_app::commands::get_audit_log,
            zb_app::commands::list_software,
            zb_app::commands::list_bloatware,
            zb_app::commands::install_software,
            zb_app::commands::remove_bloatware,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
