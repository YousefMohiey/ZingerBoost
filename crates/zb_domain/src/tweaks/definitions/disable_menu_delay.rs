use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable menu show delay for instant context menus
pub struct DisableMenuDelayTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableMenuDelayTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableMenuDelayTweak {
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
impl Tweak for DisableMenuDelayTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_menu_delay".into(),
            name: "Disable Menu Delay".into(),
            description: "Sets the menu show delay to 0 for instant context menu popups.".into(),
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
            match provider.read(&path, "MenuShowDelay").await {
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
                .read(&path, "MenuShowDelay")
                .await
                .unwrap_or(RegValue::Sz("400".into()));
            Ok(SnapshotData::Registry {
                path,
                name: "MenuShowDelay".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Control Panel\Desktop"),
                name: "MenuShowDelay".into(),
                previous: RegValue::Sz("400".into()),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Desktop");
            provider
                .write(&path, "MenuShowDelay", &RegValue::Sz("0".into()))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Menu show delay set to 0. Context menus will appear instantly.".into(),
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
            message: "Menu show delay restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Sets the delay before context menus and submenus appear to 0 milliseconds.".into(),
            why_it_helps: "Eliminates the brief pause when right-clicking or navigating menus, making the UI feel more responsive.".into(),
            potential_risks: None,
            how_to_revert: "Restores the original MenuShowDelay value from the snapshot.".into(),
        }
    }
}
