use async_trait::async_trait;
use zb_domain::errors::SnapshotError;
use zb_domain::snapshots::SystemSnapshot;
use zb_shared::types::SnapshotData;

#[async_trait]
pub trait SnapshotService: Send + Sync {
    /// Save a full system snapshot
    async fn save_snapshot(&self, snapshot: SystemSnapshot) -> Result<(), SnapshotError>;

    /// Save a single applied tweak record (for single-tweak snapshots)
    async fn save_applied(&self, tweak_id: &str, data: SnapshotData) -> Result<(), SnapshotError>;

    /// Get the last snapshot data for a specific tweak
    async fn get_last_snapshot_data(&self, tweak_id: &str) -> Result<SnapshotData, SnapshotError>;

    /// List all snapshots
    async fn list_snapshots(&self) -> Result<Vec<SystemSnapshot>, SnapshotError>;

    /// Restore a full snapshot by ID
    async fn restore_snapshot(&self, id: &str) -> Result<(), SnapshotError>;

    /// Delete a single snapshot by ID
    async fn delete_snapshot(&self, id: &str) -> Result<(), SnapshotError>;

    /// Clear all snapshots
    async fn clear_snapshots(&self) -> Result<(), SnapshotError>;
}
