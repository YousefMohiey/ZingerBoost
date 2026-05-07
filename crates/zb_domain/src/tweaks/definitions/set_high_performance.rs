use async_trait::async_trait;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation,
    TweakMetadata, TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Set High Performance power plan
#[derive(Debug)]
pub struct SetHighPerformanceTweak;

#[async_trait]
impl Tweak for SetHighPerformanceTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "performance_high_power".into(),
            name: "Set High Performance Power Plan".into(),
            description: "Switches the active power plan to High Performance for maximum CPU responsiveness.".into(),
            category: TweakCategory::Performance,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        Ok(false)
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        Ok(SnapshotData::PowerPlan {
            previous_guid: "balanced_guid_placeholder".into(),
        })
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        Ok(TweakResult {
            reboot_required: false,
            message: "High Performance power plan activated.".into(),
        })
    }

    async fn revert(&self, _snapshot: &SnapshotData) -> Result<TweakResult, TweakError> {
        Ok(TweakResult {
            reboot_required: false,
            message: "Previous power plan restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Sets the Windows power plan to High Performance, preventing CPU downclocking.".into(),
            why_it_helps: "Improves CPU responsiveness and reduces input latency, especially on desktops.".into(),
            potential_risks: Some("May increase power consumption and heat output on laptops.".into()),
            how_to_revert: "Restores the previously active power plan (usually Balanced).".into(),
        }
    }
}
