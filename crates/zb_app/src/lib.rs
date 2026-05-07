pub mod commands;

use std::sync::Arc;
use tauri::Manager;
use zb_application::audit_service::AuditService;
use zb_application::snapshot_service::SnapshotService;
use zb_application::tweak_engine::TweakEngine;
use zb_domain::tweaks::definitions::{
    DisableAnimationsTweak, DisableBackgroundAppsTweak, DisableGameDvrTweak,
    DisableHibernationTweak, DisableStickyKeysTweak, DisableStartupDelayTweak,
    DisableTelemetryTweak, DisableTransparencyTweak, SetHighPerformanceTweak,
    ShowFileExtensionsTweak,
};
use zb_infrastructure::logging::init_logging;
use zb_infrastructure::persistence::{SqliteAuditLogger, SqliteRepo};
use zb_infrastructure::registry::WinRegistryProvider;
use zb_infrastructure::windows_api::metrics_collector::MetricsCollector;

/// Shared application state injected into Tauri commands
pub struct AppState {
    pub engine: Arc<TweakEngine>,
    pub metrics_collector: Arc<MetricsCollector>,
}

pub fn run() {
    init_logging();

    let registry_provider = WinRegistryProvider::new();

    let tweaks: Vec<Arc<dyn zb_domain::tweaks::Tweak>> = vec![
        Arc::new(DisableGameDvrTweak::with_provider(registry_provider.clone())),
        Arc::new(DisableTransparencyTweak::with_provider(registry_provider.clone())),
        Arc::new(DisableAnimationsTweak::with_provider(registry_provider.clone())),
        Arc::new(ShowFileExtensionsTweak::with_provider(registry_provider.clone())),
        Arc::new(DisableStickyKeysTweak::with_provider(registry_provider.clone())),
        Arc::new(DisableStartupDelayTweak::with_provider(registry_provider.clone())),
        Arc::new(DisableBackgroundAppsTweak::with_provider(registry_provider.clone())),
        Arc::new(DisableTelemetryTweak::with_provider(registry_provider.clone())),
        Arc::new(DisableHibernationTweak::new()),
        Arc::new(SetHighPerformanceTweak::new()),
    ];

    let snapshot_service: Arc<dyn SnapshotService> = SqliteRepo::new_in_memory()
        .expect("Failed to create SQLite snapshot repository");
    let audit_service: Arc<dyn AuditService> = SqliteAuditLogger::new_in_memory()
        .expect("Failed to create SQLite audit logger");

    let engine = Arc::new(TweakEngine::new(tweaks, snapshot_service, audit_service));
    let metrics_collector = Arc::new(MetricsCollector::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            engine,
            metrics_collector,
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_tweaks,
            commands::apply_tweak,
            commands::batch_apply_tweaks,
            commands::revert_tweak,
            commands::get_metrics,
            commands::get_tweak_explanation,
            commands::list_snapshots,
            commands::get_audit_log,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
