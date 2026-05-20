use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Disable Location Services to prevent apps from accessing GPS/location data
pub struct DisableLocationServicesTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for DisableLocationServicesTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableLocationServicesTweak {
    pub fn new() -> Self {
        Self { provider: None }
    }

    pub fn with_provider(provider: Arc<dyn RegistryProvider>) -> Self {
        Self {
            provider: Some(provider),
        }
    }
}

#[async_trait]
impl Tweak for DisableLocationServicesTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "privacy_disable_location".into(),
            name: "Disable Location Services".into(),
            description: "Prevents Windows and apps from accessing GPS and location data.".into(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![RegPath::hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
            )],
            source_url: Some(
                "https://docs.microsoft.com/windows/privacy/manage-windows-endpoints".into(),
            ),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors");
            match provider.read(&path, "DisableLocation").await {
                Ok(RegValue::Dword(v)) => Ok(v == 1),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors");
            let val = provider
                .read(&path, "DisableLocation")
                .await
                .unwrap_or(RegValue::Dword(0));
            Ok(SnapshotData::Registry {
                path,
                name: "DisableLocation".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors"),
                name: "DisableLocation".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors");
            provider
                .write(&path, "DisableLocation", &RegValue::Dword(1))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Location services disabled. Apps can no longer access your location.".into(),
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
            message: "Location services restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables Windows location services via group policy, preventing all apps from accessing GPS and location data.".into(),
            why_it_helps: "Improves privacy by preventing location tracking. Reduces background location polling and associated network activity.".into(),
            potential_risks: Some("Maps, weather apps, and location-based features will not work. Find My Device will be unavailable.".into()),
            how_to_revert: "Restores the original DisableLocation value from the snapshot.".into(),
        }
    }
}
