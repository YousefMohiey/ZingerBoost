use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Default UserPreferencesMask value (bit 4 set for combo box animations enabled)
const DEFAULT_USER_PREFERENCES_MASK: [u8; 8] = [0x9E, 0x12, 0x03, 0x80, 0x12, 0x00, 0x00, 0x00];

/// Disable combo box slide animations
pub struct DisableComboAnimationTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableComboAnimationTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableComboAnimationTweak {
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
impl Tweak for DisableComboAnimationTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_combo_animation".into(),
            name: "Disable Combo Box Animations".into(),
            description: "Disables the slide-open animation on combo boxes and menus.".into(),
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
            match provider.read(&path, "UserPreferencesMask").await {
                Ok(RegValue::Binary(v)) if !v.is_empty() => Ok((v[0] & 0x10) == 0),
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
                .read(&path, "UserPreferencesMask")
                .await
                .unwrap_or(RegValue::Binary(DEFAULT_USER_PREFERENCES_MASK.to_vec()));
            Ok(SnapshotData::Registry {
                path,
                name: "UserPreferencesMask".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Control Panel\Desktop"),
                name: "UserPreferencesMask".into(),
                previous: RegValue::Binary(DEFAULT_USER_PREFERENCES_MASK.to_vec()),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Desktop");
            let current = provider
                .read(&path, "UserPreferencesMask")
                .await
                .unwrap_or(RegValue::Binary(DEFAULT_USER_PREFERENCES_MASK.to_vec()));
            if let RegValue::Binary(mut v) = current {
                if !v.is_empty() {
                    v[0] &= !0x10;
                }
                provider
                    .write(&path, "UserPreferencesMask", &RegValue::Binary(v))
                    .await
                    .map_err(|e| TweakError::Registry(e.to_string()))?;
            }
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Combo box animations disabled. Dropdowns will appear instantly.".into(),
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
            message: "Combo box animations restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Clears bit 4 in UserPreferencesMask to disable the slide-open animation on dropdown combo boxes and menus.".into(),
            why_it_helps: "Makes drop-down lists, combo boxes, and context menus appear instantly without a slide transition.".into(),
            potential_risks: Some("Shares UserPreferencesMask with other visual settings. Reverting restores the full binary snapshot to avoid side effects.".into()),
            how_to_revert: "Restores the full UserPreferencesMask binary value from the snapshot.".into(),
        }
    }
}
