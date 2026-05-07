use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable the Windows advertising ID used for cross-app tracking
pub struct DisableAdvertisingIdTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableAdvertisingIdTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableAdvertisingIdTweak {
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
impl Tweak for DisableAdvertisingIdTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "privacy_advertising_id".into(),
            name: "Disable Advertising ID".into(),
            description:
                "Turns off the unique advertising ID that Windows uses to track you across apps."
                    .into(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![RegPath::hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
            )],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo");
            match provider.read(&path, "Enabled").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo");
            let val = provider
                .read(&path, "Enabled")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "Enabled".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo"),
                name: "Enabled".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo");
            provider
                .write(&path, "Enabled", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message:
                "Windows advertising ID disabled. Apps can no longer use it for personalized ads."
                    .into(),
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
            message: "Advertising ID restored to previous setting.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables the unique advertising identifier that Windows assigns to your device for tracking across applications.".into(),
            why_it_helps: "Prevents apps from using your advertising ID to build a profile of your behavior. This reduces targeted advertising and cross-app tracking.".into(),
            potential_risks: Some("Apps that rely on personalized ads will show generic ads instead. No loss of app functionality.".into()),
            how_to_revert: "Restores the original Enabled registry value under AdvertisingInfo.".into(),
        }
    }
}
