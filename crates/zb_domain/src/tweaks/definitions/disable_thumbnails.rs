use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable taskbar thumbnail previews and thumbnail caching
pub struct DisableThumbnailsTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableThumbnailsTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableThumbnailsTweak {
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
impl Tweak for DisableThumbnailsTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_thumbnails".into(),
            name: "Disable Taskbar Thumbnails".into(),
            description: "Disables thumbnail previews when hovering over taskbar buttons and clears the thumbnail cache.".into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![RegPath::hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            )],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced");
            match provider.read(&path, "IconsOnly").await {
                Ok(RegValue::Dword(v)) => Ok(v == 1),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced");
            let val = provider
                .read(&path, "IconsOnly")
                .await
                .unwrap_or(RegValue::Dword(0));
            Ok(SnapshotData::Registry {
                path,
                name: "IconsOnly".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"),
                name: "IconsOnly".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced");
            provider
                .write(&path, "IconsOnly", &RegValue::Dword(1))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
            provider
                .write(&path, "DisableThumbnailCache", &RegValue::Dword(1))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Taskbar thumbnails disabled. Hovering over taskbar buttons will no longer show previews.".into(),
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
            message: "Taskbar thumbnails restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables taskbar thumbnail previews and prevents Windows from caching window preview images.".into(),
            why_it_helps: "Reduces DWM memory usage and eliminates the brief delay when hovering over taskbar buttons, making the taskbar more responsive.".into(),
            potential_risks: Some("You will only see window titles instead of live thumbnail previews when hovering over taskbar buttons.".into()),
            how_to_revert: "Restores the original IconsOnly registry value from the snapshot and re-enables the thumbnail cache.".into(),
        }
    }
}
