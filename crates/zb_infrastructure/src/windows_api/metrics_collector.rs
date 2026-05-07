use std::sync::Arc;
use tokio::sync::Mutex;
use zb_shared::types::SystemMetrics;

/// Collects live system metrics for the dashboard
#[derive(Debug)]
pub struct MetricsCollector;

impl MetricsCollector {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    pub async fn current(&self) -> SystemMetrics {
        // Placeholder: real implementation uses PDH counters and GlobalMemoryStatusEx
        SystemMetrics {
            cpu_percent: 15.0,
            ram_percent: 42.0,
            ram_used_mb: 8192,
            ram_total_mb: 16384,
            disk_active_percent: 5.0,
            network_down_mbps: 0.5,
            network_up_mbps: 0.1,
        }
    }
}
