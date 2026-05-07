use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable font smoothing for sharper text rendering
pub struct DisableFontSmoothingTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableFontSmoothingTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableFontSmoothingTweak {
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
impl Tweak for DisableFontSmoothingTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_font_smoothing".into(),
            name: "Disable Font Smoothing".into(),
            description: "Turns off ClearType font smoothing for sharper, more pixel-precise text."
                .into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![RegPath::hkcu(r"Control Panel\Desktop")],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Desktop");
            match provider.read(&path, "FontSmoothing").await {
                Ok(RegValue::Sz(v)) => Ok(v == "0"),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Desktop");
            let val = provider
                .read(&path, "FontSmoothing")
                .await
                .unwrap_or(RegValue::Sz("2".into()));
            Ok(SnapshotData::Registry {
                path,
                name: "FontSmoothing".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Control Panel\Desktop"),
                name: "FontSmoothing".into(),
                previous: RegValue::Sz("2".into()),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Desktop");
            provider
                .write(&path, "FontSmoothing", &RegValue::Sz("0".into()))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Font smoothing disabled. Text will render in its sharpest form.".into(),
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
            message: "Font smoothing restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables ClearType anti-aliasing so fonts render without sub-pixel smoothing.".into(),
            why_it_helps: "Some users prefer the crisper, more pixel-precise look of unsmoothed fonts, especially on lower-DPI displays.".into(),
            potential_risks: Some("Fonts may appear more jagged, especially at small sizes. Re-enable if text becomes hard to read.".into()),
            how_to_revert: "Restores the original FontSmoothing value from the snapshot.".into(),
        }
    }
}
