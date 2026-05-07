pub mod commands;

use std::sync::Arc;
use zb_application::tweak_engine::TweakEngine;
use zb_infrastructure::windows_api::metrics_collector::MetricsCollector;
use zb_infrastructure::windows_api::winget::WingetInstaller;

pub struct AppState {
    pub engine: Arc<TweakEngine>,
    pub metrics_collector: Arc<MetricsCollector>,
    pub winget: WingetInstaller,
}
