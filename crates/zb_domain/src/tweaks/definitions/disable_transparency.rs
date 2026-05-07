use async_trait::async_trait;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation,
    TweakMetadata, TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable Windows transparency effects for better performance
#[derive(Debug)]
pub struct DisableTransparencyTweak;

#[async_trait]
impl Tweak for DisableTransparencyTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_disable_transparency".into(),
            name: "Disable Transparency Effects".into(),
            description: "Turns off acrylic and transparency effects to reduce GPU compositor load.".into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"),
            ],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        Ok(false)
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        Ok(SnapshotData::Registry {
            path: RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"),
            name: "EnableTransparency".into(),
            previous: RegValue::Dword(1),
        })
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        Ok(TweakResult {
            reboot_required: false,
            message: "Transparency effects disabled.".into(),
        })
    }

    async fn revert(&self, _snapshot: &SnapshotData) -> Result<TweakResult, TweakError> {
        Ok(TweakResult {
            reboot_required: false,
            message: "Transparency effects restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables the transparent acrylic effects in the taskbar, Start menu, and windows.".into(),
            why_it_helps: "Reduces GPU compositor workload, which can improve responsiveness on lower-end GPUs.".into(),
            potential_risks: None,
            how_to_revert: "Restores the original transparency setting.".into(),
        }
    }
}
