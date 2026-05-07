use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable Windows animations for better responsiveness
pub struct DisableAnimationsTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableAnimationsTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableAnimationsTweak {
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
impl Tweak for DisableAnimationsTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_disable_animations".into(),
            name: "Disable Animations".into(),
            description: "Turns off window minimize/maximize animations for snappier UI.".into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![RegPath::hkcu(r"Control Panel\Desktop\WindowMetrics")],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Desktop");
            match provider.read(&path, "UserPreferencesMask").await {
                Ok(RegValue::Binary(v)) if v.len() >= 4 => {
                    // Check bit 1 (animations) - if clear, animations are disabled
                    Ok((v[0] & 0x02) == 0)
                }
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Desktop");
            let val =
                provider
                    .read(&path, "UserPreferencesMask")
                    .await
                    .unwrap_or(RegValue::Binary(vec![
                        0x9E, 0x12, 0x03, 0x80, 0x12, 0x00, 0x00, 0x00,
                    ]));
            Ok(SnapshotData::Registry {
                path,
                name: "UserPreferencesMask".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Control Panel\Desktop"),
                name: "UserPreferencesMask".into(),
                previous: RegValue::Binary(vec![0x9E, 0x12, 0x03, 0x80, 0x12, 0x00, 0x00, 0x00]),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Desktop");
            let current =
                provider
                    .read(&path, "UserPreferencesMask")
                    .await
                    .unwrap_or(RegValue::Binary(vec![
                        0x9E, 0x12, 0x03, 0x80, 0x12, 0x00, 0x00, 0x00,
                    ]));
            if let RegValue::Binary(mut v) = current {
                if !v.is_empty() {
                    v[0] &= !0x02; // Clear animation bit
                }
                provider
                    .write(&path, "UserPreferencesMask", &RegValue::Binary(v))
                    .await
                    .map_err(|e| TweakError::Registry(e.to_string()))?;
            }
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Window animations disabled. UI will feel snappier.".into(),
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
            message: "Window animations restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables window minimize/maximize and menu animations.".into(),
            why_it_helps: "Reduces visual latency and makes the UI feel more responsive, especially on slower GPUs.".into(),
            potential_risks: None,
            how_to_revert: "Restores the original UserPreferencesMask value.".into(),
        }
    }
}
