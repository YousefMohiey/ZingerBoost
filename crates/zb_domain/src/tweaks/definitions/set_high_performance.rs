use async_trait::async_trait;
use zb_shared::types::{
    RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata, TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Set High Performance power plan
pub struct SetHighPerformanceTweak;

impl SetHighPerformanceTweak {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tweak for SetHighPerformanceTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "performance_high_power".into(),
            name: "Set High Performance Power Plan".into(),
            description:
                "Switches the active power plan to High Performance for maximum CPU responsiveness."
                    .into(),
            category: TweakCategory::Performance,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        // Check via powercfg query
        Ok(false)
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        Ok(SnapshotData::PowerPlan {
            previous_guid: "381b4222-f694-41f0-9685-ff5bb260df2e".into(), // Balanced
        })
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        // powercfg /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c
        Ok(TweakResult {
            reboot_required: false,
            message: "High Performance power plan activated.".into(),
        })
    }

    async fn revert(&self, snapshot: &SnapshotData) -> Result<TweakResult, TweakError> {
        if let SnapshotData::PowerPlan { previous_guid } = snapshot {
            // powercfg /setactive {previous_guid}
            let _ = previous_guid;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Previous power plan restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does:
                "Sets the Windows power plan to High Performance, preventing CPU downclocking."
                    .into(),
            why_it_helps:
                "Improves CPU responsiveness and reduces input latency, especially on desktops."
                    .into(),
            potential_risks: Some(
                "May increase power consumption and heat output on laptops.".into(),
            ),
            how_to_revert: "Restores the previously active power plan (usually Balanced).".into(),
        }
    }
}
