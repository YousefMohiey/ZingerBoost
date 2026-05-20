pub mod audit_service;
pub mod dto;
pub mod snapshot_service;
pub mod tweak_engine;

pub use audit_service::*;
pub use dto::*;
pub use snapshot_service::*;
pub use tweak_engine::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use zb_domain::tweaks::Tweak;
    use zb_shared::types::{
        AuditEntry, RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation,
        TweakMetadata, TweakResult,
    };

    struct MockTweak;

    #[async_trait::async_trait]
    impl Tweak for MockTweak {
        fn metadata(&self) -> TweakMetadata {
            TweakMetadata {
                id: "test_tweak".into(),
                name: "Test Tweak".into(),
                description: "A test tweak".into(),
                category: TweakCategory::Performance,
                risk: RiskLevel::Safe,
                requires_reboot: false,
                requires_admin: false,
                affected_keys: vec![],
                source_url: None,
            }
        }

        async fn is_applied(&self) -> Result<bool, zb_domain::errors::TweakError> {
            Ok(false)
        }

        async fn capture_state(&self) -> Result<SnapshotData, zb_domain::errors::TweakError> {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu("Software\\Test"),
                name: "test".into(),
                previous: RegValue::Dword(0),
            })
        }

        async fn apply(&self) -> Result<TweakResult, zb_domain::errors::TweakError> {
            Ok(TweakResult {
                reboot_required: false,
                message: "Applied".into(),
            })
        }

        async fn revert(
            &self,
            _data: &SnapshotData,
        ) -> Result<TweakResult, zb_domain::errors::TweakError> {
            Ok(TweakResult {
                reboot_required: false,
                message: "Reverted".into(),
            })
        }

        fn explain(&self) -> TweakExplanation {
            TweakExplanation {
                what_it_does: "Does something".into(),
                why_it_helps: "Improves performance".into(),
                potential_risks: None,
                how_to_revert: "Toggle off".into(),
            }
        }
    }

    struct MockSnapshotService;

    #[async_trait::async_trait]
    impl SnapshotService for MockSnapshotService {
        async fn save_snapshot(
            &self,
            _snapshot: zb_domain::snapshots::SystemSnapshot,
        ) -> Result<(), zb_domain::errors::SnapshotError> {
            Ok(())
        }

        async fn save_applied(
            &self,
            _tweak_id: &str,
            _data: SnapshotData,
        ) -> Result<(), zb_domain::errors::SnapshotError> {
            Ok(())
        }

        async fn get_last_snapshot_data(
            &self,
            _tweak_id: &str,
        ) -> Result<SnapshotData, zb_domain::errors::SnapshotError> {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu("Software\\Test"),
                name: "test".into(),
                previous: RegValue::Dword(0),
            })
        }

        async fn list_snapshots(
            &self,
        ) -> Result<Vec<zb_domain::snapshots::SystemSnapshot>, zb_domain::errors::SnapshotError>
        {
            Ok(vec![])
        }

        async fn restore_snapshot(
            &self,
            _id: &str,
        ) -> Result<(), zb_domain::errors::SnapshotError> {
            Ok(())
        }

        async fn delete_snapshot(&self, _id: &str) -> Result<(), zb_domain::errors::SnapshotError> {
            Ok(())
        }

        async fn clear_snapshots(&self) -> Result<(), zb_domain::errors::SnapshotError> {
            Ok(())
        }
    }

    struct MockAuditService;

    #[async_trait::async_trait]
    impl AuditService for MockAuditService {
        async fn log(&self, _entry: AuditEntry) {}

        async fn get_recent(&self, _limit: usize) -> Vec<AuditEntry> {
            vec![]
        }
    }

    #[tokio::test]
    async fn test_tweak_engine_creation() {
        let tweaks: Vec<Arc<dyn Tweak>> = vec![Arc::new(MockTweak)];
        let snapshot_service = Arc::new(MockSnapshotService);
        let audit_service = Arc::new(MockAuditService);
        let engine = TweakEngine::new(tweaks, snapshot_service, audit_service);

        let list = engine.list_tweaks();
        assert_eq!(list.len(), 1);
    }

    #[tokio::test]
    async fn test_tweak_engine_get_tweak() {
        let tweaks: Vec<Arc<dyn Tweak>> = vec![Arc::new(MockTweak)];
        let snapshot_service = Arc::new(MockSnapshotService);
        let audit_service = Arc::new(MockAuditService);
        let engine = TweakEngine::new(tweaks, snapshot_service, audit_service);

        let found = engine.get_tweak("test_tweak");
        assert!(found.is_some());

        let not_found = engine.get_tweak("nonexistent_tweak");
        assert!(not_found.is_none());
    }
}
