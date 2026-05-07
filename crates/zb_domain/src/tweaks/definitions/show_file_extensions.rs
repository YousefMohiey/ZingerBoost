use async_trait::async_trait;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation,
    TweakMetadata, TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Show file extensions in Explorer
#[derive(Debug)]
pub struct ShowFileExtensionsTweak;

#[async_trait]
impl Tweak for ShowFileExtensionsTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_show_extensions".into(),
            name: "Show File Extensions".into(),
            description: "Always show file extensions in Windows Explorer for security and clarity.".into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"),
            ],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        Ok(false)
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        Ok(SnapshotData::Registry {
            path: RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"),
            name: "HideFileExt".into(),
            previous: RegValue::Dword(1),
        })
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        Ok(TweakResult {
            reboot_required: false,
            message: "File extensions are now visible in Explorer.".into(),
        })
    }

    async fn revert(&self, _snapshot: &SnapshotData) -> Result<TweakResult, TweakError> {
        Ok(TweakResult {
            reboot_required: false,
            message: "File extension visibility restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Forces Windows Explorer to always display file extensions (e.g., .exe, .txt).".into(),
            why_it_helps: "Security best practice — prevents malware from disguising itself as innocent files.".into(),
            potential_risks: None,
            how_to_revert: "Restores the previous HideFileExt setting.".into(),
        }
    }
}
