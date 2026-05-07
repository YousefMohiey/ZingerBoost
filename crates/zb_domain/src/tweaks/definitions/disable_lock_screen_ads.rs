use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable rotating lock screen ads and tips
pub struct DisableLockScreenAdsTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableLockScreenAdsTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableLockScreenAdsTweak {
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
impl Tweak for DisableLockScreenAdsTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "privacy_lock_screen_ads".into(),
            name: "Disable Lock Screen Ads".into(),
            description: "Prevents rotating lock screen content like ads and tips from appearing."
                .into(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![RegPath::hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
            )],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager");
            match provider.read(&path, "RotatingLockScreenEnabled").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager");
            let val = provider
                .read(&path, "RotatingLockScreenEnabled")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "RotatingLockScreenEnabled".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(
                    r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                ),
                name: "RotatingLockScreenEnabled".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager");
            provider
                .write(&path, "RotatingLockScreenEnabled", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Lock screen ads and rotating content disabled.".into(),
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
            message: "Lock screen content restored to previous setting.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables the rotating lock screen feature that shows ads, tips, and sponsored content on your lock screen.".into(),
            why_it_helps: "Stops Microsoft from showing promotional content and 'fun facts' on your lock screen, reducing unwanted distractions and data collection.".into(),
            potential_risks: Some("The lock screen will still function but will use a static background image instead of rotating content.".into()),
            how_to_revert: "Restores the original RotatingLockScreenEnabled registry value.".into(),
        }
    }
}
