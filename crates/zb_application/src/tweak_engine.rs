use chrono::Utc;
use std::sync::Arc;
use tracing::{error, warn};
use zb_domain::errors::TweakError;
use zb_domain::snapshots::SystemSnapshot;
use zb_domain::tweaks::Tweak;
use zb_shared::types::{AuditEntry, AuditLevel, TweakResult};

use crate::audit_service::AuditService;
use crate::snapshot_service::SnapshotService;

/// Orchestrates the application of tweaks with safety guarantees.
pub struct TweakEngine {
    tweaks: Vec<Arc<dyn Tweak>>,
    snapshot_service: Arc<dyn SnapshotService>,
    audit_service: Arc<dyn AuditService>,
}

impl TweakEngine {
    pub fn new(
        tweaks: Vec<Arc<dyn Tweak>>,
        snapshot_service: Arc<dyn SnapshotService>,
        audit_service: Arc<dyn AuditService>,
    ) -> Self {
        Self {
            tweaks,
            snapshot_service,
            audit_service,
        }
    }

    /// Get all registered tweaks
    pub fn list_tweaks(&self) -> Vec<&Arc<dyn Tweak>> {
        self.tweaks.iter().collect()
    }

    /// Find a tweak by ID
    pub fn get_tweak(&self, id: &str) -> Option<Arc<dyn Tweak>> {
        self.tweaks.iter().find(|t| t.metadata().id == id).cloned()
    }

    /// Access the snapshot service
    pub fn snapshot_service(&self) -> Arc<dyn SnapshotService> {
        self.snapshot_service.clone()
    }

    /// Access the audit service
    pub fn audit_service(&self) -> Arc<dyn AuditService> {
        self.audit_service.clone()
    }

    /// Apply a single tweak with automatic snapshot
    pub async fn apply_single(&self, id: &str) -> Result<TweakResult, TweakError> {
        let tweak = self
            .get_tweak(id)
            .ok_or_else(|| {
                tracing::error!("Unknown tweak requested: {}", id);
                TweakError::Validation(format!("Unknown tweak: {}", id))
            })?;

        if tweak.is_applied().await? {
            return Err(TweakError::AlreadyApplied);
        }

        let snapshot_data = tweak.capture_state().await?;
        let result = tweak.apply().await?;

        // Single audit entry - no duplicate tracing calls
        self.audit_service
            .log(AuditEntry {
                timestamp: Utc::now(),
                level: AuditLevel::Info,
                category: "tweak".into(),
                message: format!("Applied tweak: {}", id),
                details: None,
            })
            .await;

        self.snapshot_service
            .save_applied(id, snapshot_data.clone())
            .await
            .map_err(|e| TweakError::Unknown(e.to_string()))?;

        let mut snapshot = SystemSnapshot::new(format!("Applied tweak: {}", id));
        snapshot.add_record(id.to_string(), snapshot_data);
        self.snapshot_service
            .save_snapshot(snapshot)
            .await
            .map_err(|e| TweakError::Unknown(e.to_string()))?;

        Ok(result)
    }

    /// Apply a batch of tweaks sequentially with a single snapshot
    pub async fn apply_batch(
        &self,
        ids: &[String],
    ) -> Result<Vec<(String, TweakResult)>, TweakError> {
        let mut results = Vec::new();
        let mut snapshot = SystemSnapshot::new(format!("Batch apply: {:?}", ids));
        let mut applied = Vec::new();

        for id in ids {
            let tweak = self
                .get_tweak(id)
                .ok_or_else(|| TweakError::Validation(format!("Unknown tweak: {}", id)))?;

            if tweak.is_applied().await? {
                warn!("Tweak {} already applied, skipping", id);
                continue;
            }

            let snapshot_data = tweak.capture_state().await?;
            match tweak.apply().await {
                Ok(result) => {
                    snapshot.add_record(id.clone(), snapshot_data.clone());
                    applied.push(id.clone());
                    results.push((id.clone(), result));

                    self.snapshot_service
                        .save_applied(id, snapshot_data)
                        .await
                        .map_err(|e| TweakError::Unknown(e.to_string()))?;
                }
                Err(e) => {
                    error!("Failed to apply tweak {}: {}", id, e);
                    // Rollback already applied tweaks
                    for applied_id in applied.iter().rev() {
                        if let Some(t) = self.get_tweak(applied_id) {
                            if let Some(record) = snapshot
                                .tweak_records
                                .iter()
                                .find(|r| &r.tweak_id == applied_id)
                            {
                                if let Err(revert_err) = t.revert(&record.snapshot_data).await {
                                    error!("Rollback failed for {}: {}", applied_id, revert_err);
                                }
                            }
                        }
                    }
                    return Err(e);
                }
            }
        }

        self.snapshot_service
            .save_snapshot(snapshot)
            .await
            .map_err(|e| TweakError::Unknown(e.to_string()))?;

        self.audit_service
            .log(AuditEntry {
                timestamp: Utc::now(),
                level: AuditLevel::Info,
                category: "tweak".into(),
                message: format!("Batch applied: {:?}", ids),
                details: None,
            })
            .await;

        Ok(results)
    }

    /// Revert a tweak using the last known snapshot
    pub async fn revert(&self, id: &str) -> Result<TweakResult, TweakError> {
        let tweak = self
            .get_tweak(id)
            .ok_or_else(|| {
                tracing::error!("Unknown tweak requested for revert: {}", id);
                TweakError::Validation(format!("Unknown tweak: {}", id))
            })?;

        let snapshot_data = self
            .snapshot_service
            .get_last_snapshot_data(id)
            .await
            .map_err(|_e| {
                tracing::error!("No snapshot found for tweak {}", id);
                TweakError::SnapshotMissing
            })?;

        let result = tweak.revert(&snapshot_data).await?;

        // Single audit entry - no duplicate tracing calls
        self.audit_service
            .log(AuditEntry {
                timestamp: Utc::now(),
                level: AuditLevel::Warn,
                category: "tweak".into(),
                message: format!("Reverted tweak: {}", id),
                details: None,
            })
            .await;

        Ok(result)
    }
}
