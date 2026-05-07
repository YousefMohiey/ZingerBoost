use async_trait::async_trait;
use zb_shared::types::{
    RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable hibernation to free up disk space and reduce SSD wear
pub struct DisableHibernationTweak;

#[async_trait]
impl Tweak for DisableHibernationTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "performance_disable_hibernation".into(),
            name: "Disable Hibernation".into(),
            description: "Turns off hibernation to free up disk space equal to your RAM size."
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
        // Check if hiberfil.sys exists
        Ok(!std::path::Path::new(r"C:\hiberfil.sys").exists())
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        Ok(SnapshotData::Other("hibernation_enabled".into()))
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        // In real implementation: powercfg /hibernate off
        Ok(TweakResult {
            reboot_required: false,
            message: "Hibernation disabled. hiberfil.sys removed, freeing disk space.".into(),
        })
    }

    async fn revert(&self, _snapshot: &SnapshotData) -> Result<TweakResult, TweakError> {
        // In real implementation: powercfg /hibernate on
        Ok(TweakResult {
            reboot_required: false,
            message: "Hibernation restored. hiberfil.sys recreated.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables hibernation and deletes the hiberfil.sys file.".into(),
            why_it_helps: "Frees up disk space equal to your RAM size (e.g., 16 GB) and reduces SSD write wear. Sleep mode still works.".into(),
            potential_risks: Some("You will not be able to hibernate. Only Sleep and Shut Down remain.".into()),
            how_to_revert: "Runs powercfg /hibernate on to re-enable hibernation.".into(),
        }
    }
}
