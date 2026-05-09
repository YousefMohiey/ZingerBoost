use std::sync::Arc;
use zb_application::audit_service::AuditService;
use zb_application::snapshot_service::SnapshotService;
use zb_application::tweak_engine::TweakEngine;
use zb_domain::tweaks::definitions::*;
use zb_infrastructure::logging::init_logging;
use zb_infrastructure::persistence::{init_database, SqliteAuditLogger, SqliteRepo};
use zb_infrastructure::registry::WinRegistryProvider;
use zb_infrastructure::services::ServiceController;
use zb_infrastructure::windows_api::debloat_engine::DebloatEngine;
use zb_infrastructure::windows_api::metrics_collector::MetricsCollector;
use zb_infrastructure::windows_api::system_cleaner::SystemCleaner;
use zb_infrastructure::windows_api::winget::WingetInstaller;

pub struct AppState {
    pub engine: Arc<TweakEngine>,
    pub metrics: Arc<MetricsCollector>,
    pub winget: WingetInstaller,
    pub cleaner: SystemCleaner,
    pub service_ctrl: ServiceController,
    pub db_conn: Arc<tokio::sync::Mutex<rusqlite::Connection>>,
}

impl AppState {
    pub async fn new() -> Self {
        init_logging();
        let db_conn = init_database().expect("DB init failed");
        let rp = WinRegistryProvider::new();
        let rs = rp.clone();

        let tweaks: Vec<Arc<dyn zb_domain::tweaks::Tweak>> = vec![
            Arc::new(DisableGameDvrTweak::with_provider(rs.clone())),
            Arc::new(DisableTransparencyTweak::with_provider(rs.clone())),
            Arc::new(DisableAnimationsTweak::with_provider(rs.clone())),
            Arc::new(ShowFileExtensionsTweak::with_provider(rs.clone())),
            Arc::new(DisableStickyKeysTweak::with_provider(rs.clone())),
            Arc::new(DisableStartupDelayTweak::with_provider(rs.clone())),
            Arc::new(DisableBackgroundAppsTweak::with_provider(rs.clone())),
            Arc::new(DisableTelemetryTweak::with_provider(rs.clone())),
            Arc::new(DisableMenuDelayTweak::with_provider(rs.clone())),
            Arc::new(DisableCursorShadowTweak::with_provider(rs.clone())),
            Arc::new(DisableFontSmoothingTweak::with_provider(rs.clone())),
            Arc::new(DisableTaskbarAnimationsTweak::with_provider(rs.clone())),
            Arc::new(DisableAeroShakeTweak::with_provider(rs.clone())),
            Arc::new(DisableAeroSnapTweak::with_provider(rs.clone())),
            Arc::new(DisablePeekTweak::with_provider(rs.clone())),
            Arc::new(DisableSmoothScrollTweak::with_provider(rs.clone())),
            Arc::new(DisableComboAnimationTweak::with_provider(rs.clone())),
            Arc::new(DisableTaskbarBadgesTweak::with_provider(rs.clone())),
            Arc::new(DisableLockScreenAdsTweak::with_provider(rs.clone())),
            Arc::new(DisableStartSuggestionsTweak::with_provider(rs.clone())),
            Arc::new(DisableExplorerAdsTweak::with_provider(rs.clone())),
            Arc::new(DisableAdvertisingIdTweak::with_provider(rs.clone())),
            Arc::new(DisableMeetNowTweak::with_provider(rs.clone())),
            Arc::new(DisableHibernationTweak::new()),
            Arc::new(SetHighPerformanceTweak::new()),
        ];

        let snapshot_service = SqliteRepo::from_connection(db_conn.clone());
        let audit_service = SqliteAuditLogger::from_connection(db_conn.clone());
        let engine = Arc::new(TweakEngine::new(tweaks, snapshot_service, audit_service));
        let metrics = MetricsCollector::new();

        Self {
            engine,
            metrics,
            winget: WingetInstaller::new(),
            cleaner: SystemCleaner::new(),
            service_ctrl: ServiceController::new(),
            db_conn,
        }
    }
}
