use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable Aero Peek (taskbar thumbnail preview on hover)
pub struct DisablePeekTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisablePeekTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisablePeekTweak {
    pub fn new() -> Self {
        Self { provider: None }
    }

    pub fn with_provider(provider: Arc<dyn crate::registry::RegistryProvider>) -> Self {
        Self {
            provider: Some(provider),
        }
    }
}

#[async_trait]
impl Tweak for DisablePeekTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_peek".into(),
            name: "Disable Peek".into(),
            description: "Turns off the preview thumbnails when hovering over taskbar icons."
                .into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![RegPath::hkcu(r"Software\Microsoft\Windows\DWM")],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Software\Microsoft\Windows\DWM");
            match provider.read(&path, "EnableAeroPeek").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Software\Microsoft\Windows\DWM");
            let val = provider
                .read(&path, "EnableAeroPeek")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "EnableAeroPeek".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Software\Microsoft\Windows\DWM"),
                name: "EnableAeroPeek".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Software\Microsoft\Windows\DWM");
            provider
                .write(&path, "EnableAeroPeek", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Peek previews disabled. Taskbar hover will no longer show thumbnails.".into(),
        })
    }

    async fn revert(&self, snapshot: &SnapshotData) -> Result<TweakResult, TweakError> {
        if let SnapshotData::Registry {
            path,
            name,
            previous,
        } = snapshot
        {
            if let Some(provider) = &self.provider {
                provider
                    .write(path, name, previous)
                    .await
                    .map_err(|e| TweakError::Registry(e.to_string()))?;
            }
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Peek previews restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables the thumbnail preview that appears when hovering over a taskbar button, and the desktop peek at the far right of the taskbar.".into(),
            why_it_helps: "Reduces visual clutter and eliminates a common distraction, especially for users who rely on alt-tab for window switching.".into(),
            potential_risks: None,
            how_to_revert: "Restores the original EnableAeroPeek value from the snapshot.".into(),
        }
    }
}
