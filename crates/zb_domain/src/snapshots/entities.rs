use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zb_shared::types::{Id, SnapshotData};

/// A system snapshot capturing the state before a batch of tweaks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSnapshot {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub tweak_records: Vec<AppliedTweakRecord>,
}

impl SystemSnapshot {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            description: description.into(),
            tweak_records: Vec::new(),
        }
    }

    pub fn add_record(&mut self, tweak_id: Id, data: SnapshotData) {
        self.tweak_records.push(AppliedTweakRecord {
            tweak_id,
            snapshot_data: data,
        });
    }
}

/// A record of a single tweak's state within a snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedTweakRecord {
    pub tweak_id: Id,
    pub snapshot_data: SnapshotData,
}
