use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use zb_application::snapshot_service::SnapshotService;
use zb_domain::errors::SnapshotError;
use zb_domain::snapshots::SystemSnapshot;
use zb_shared::types::SnapshotData;

/// In-memory snapshot repository (replace with SQLite + filesystem in production)
#[derive(Debug)]
pub struct InMemorySnapshotRepo {
    snapshots: Mutex<HashMap<String, SystemSnapshot>>,
    applied_records: Mutex<HashMap<String, Vec<(String, SnapshotData)>>>, // tweak_id -> [(snapshot_id, data)]
}

impl InMemorySnapshotRepo {
    pub fn new() -> Arc<dyn SnapshotService> {
        Arc::new(Self {
            snapshots: Mutex::new(HashMap::new()),
            applied_records: Mutex::new(HashMap::new()),
        })
    }
}

#[async_trait]
impl SnapshotService for InMemorySnapshotRepo {
    async fn save_snapshot(&self, snapshot: SystemSnapshot) -> Result<(), SnapshotError> {
        let mut snapshots = self.snapshots.lock().await;
        snapshots.insert(snapshot.id.to_string(), snapshot);
        Ok(())
    }

    async fn save_applied(&self, tweak_id: &str, data: SnapshotData) -> Result<(), SnapshotError> {
        let mut records = self.applied_records.lock().await;
        records
            .entry(tweak_id.to_string())
            .or_default()
            .push((Uuid::new_v4().to_string(), data));
        Ok(())
    }

    async fn get_last_snapshot_data(&self, tweak_id: &str) -> Result<SnapshotData, SnapshotError> {
        let records = self.applied_records.lock().await;
        records
            .get(tweak_id)
            .and_then(|v| v.last().map(|(_, data)| data.clone()))
            .ok_or_else(|| SnapshotError::NotFound(format!("No snapshot for tweak: {}", tweak_id)))
    }

    async fn list_snapshots(&self) -> Result<Vec<SystemSnapshot>, SnapshotError> {
        let snapshots = self.snapshots.lock().await;
        Ok(snapshots.values().cloned().collect())
    }

    async fn restore_snapshot(&self, id: &str) -> Result<(), SnapshotError> {
        let snapshots = self.snapshots.lock().await;
        snapshots
            .get(id)
            .ok_or_else(|| SnapshotError::NotFound(id.to_string()))?;
        // Real implementation would iterate tweak_records and call revert
        Ok(())
    }
}
