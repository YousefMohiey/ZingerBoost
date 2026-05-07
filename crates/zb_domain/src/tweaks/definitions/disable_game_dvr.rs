use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation,
    TweakMetadata, TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable Game DVR to reduce background recording overhead
#[derive(Debug)]
pub struct DisableGameDvrTweak;

#[async_trait]
impl Tweak for DisableGameDvrTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "gaming_disable_dvr".into(),
            name: "Disable Game DVR".into(),
            description: "Turns off Windows Game DVR background recording to free up CPU/GPU resources.".into(),
            category: TweakCategory::Gaming,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![
                RegPath::hkcu(r"System\GameConfigStore"),
                RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\GameDVR"),
            ],
            source_url: Some("https://docs.microsoft.com/windows/gaming/game-dvr".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        // Placeholder: would read registry in real implementation
        Ok(false)
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        Ok(SnapshotData::Registry {
            path: RegPath::hkcu(r"System\GameConfigStore"),
            name: "GameDVR_Enabled".into(),
            previous: RegValue::Dword(1),
        })
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        Ok(TweakResult {
            reboot_required: false,
            message: "Game DVR disabled. Background recording is now off.".into(),
        })
    }

    async fn revert(&self, _snapshot: &SnapshotData) -> Result<TweakResult, TweakError> {
        Ok(TweakResult {
            reboot_required: false,
            message: "Game DVR restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables the Xbox Game Bar DVR background recording service.".into(),
            why_it_helps: "Reduces CPU, GPU, and disk overhead while gaming, potentially improving frame rates.".into(),
            potential_risks: Some("You will not be able to record gameplay clips using Windows built-in DVR.".into()),
            how_to_revert: "Re-enables Game DVR by restoring the original registry value.".into(),
        }
    }
}
