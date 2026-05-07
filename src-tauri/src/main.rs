#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use zb_app::AppState;
use zb_application::audit_service::AuditService;
use zb_application::snapshot_service::SnapshotService;
use zb_application::tweak_engine::TweakEngine;
use zb_domain::tweaks::definitions::{
    DisableAdvertisingIdTweak, DisableAeroShakeTweak, DisableAeroSnapTweak, DisableAnimationsTweak,
    DisableBackgroundAppsTweak, DisableComboAnimationTweak, DisableCursorShadowTweak,
    DisableExplorerAdsTweak, DisableFontSmoothingTweak, DisableGameDvrTweak,
    DisableHibernationTweak, DisableLockScreenAdsTweak, DisableMeetNowTweak, DisableMenuDelayTweak,
    DisablePeekTweak, DisableSmoothScrollTweak, DisableStartSuggestionsTweak,
    DisableStartupDelayTweak, DisableStickyKeysTweak, DisableTaskbarAnimationsTweak,
    DisableTaskbarBadgesTweak, DisableTelemetryTweak, DisableTransparencyTweak,
    SetHighPerformanceTweak, ShowFileExtensionsTweak,
};
use zb_infrastructure::logging::init_logging;
use zb_infrastructure::persistence::{init_database, SqliteAuditLogger, SqliteRepo};
use zb_infrastructure::registry::WinRegistryProvider;
use zb_infrastructure::windows_api::metrics_collector::MetricsCollector;
use zb_infrastructure::windows_api::winget::WingetInstaller;

fn main() {
    init_logging();

    let db_conn = init_database()
        .expect("Failed to initialize database at %LOCALAPPDATA%\\ZingerBoost\\data.db");

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
        Arc::new(DisableHibernationTweak::new()),
        Arc::new(SetHighPerformanceTweak::new()),
    ];

    let snapshot_service: Arc<dyn SnapshotService> = SqliteRepo::from_connection(db_conn.clone());
    let audit_service: Arc<dyn AuditService> = SqliteAuditLogger::from_connection(db_conn);

    let engine = Arc::new(TweakEngine::new(tweaks, snapshot_service, audit_service));
    let metrics_collector = MetricsCollector::new();
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
