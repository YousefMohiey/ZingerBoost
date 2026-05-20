use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Disable Automatic Driver Updates via Windows Update
pub struct DisableAutoDriverUpdatesTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for DisableAutoDriverUpdatesTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableAutoDriverUpdatesTweak {
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
impl Tweak for DisableAutoDriverUpdatesTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "wu_disable_auto_drivers".into(),
            name: "Disable Automatic Driver Updates".into(),
            description: "Prevents Windows Update from automatically installing drivers. You control driver versions.".into(),
            category: TweakCategory::WindowsUpdate,
            risk: RiskLevel::Moderate,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![
                RegPath::hklm(r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching"),
            ],
            source_url: Some("https://docs.microsoft.com/windows-hardware/drivers/install/prevent-automatic-driver-updates".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching");
            match provider
                .read(&path, "ExcludeWUDriversInQualityUpdate")
                .await
            {
                Ok(RegValue::Dword(v)) => Ok(v == 1),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching");
            let val = provider
                .read(&path, "ExcludeWUDriversInQualityUpdate")
                .await
                .unwrap_or(RegValue::Dword(0));
            Ok(SnapshotData::Registry {
                path,
                name: "ExcludeWUDriversInQualityUpdate".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching"),
                name: "ExcludeWUDriversInQualityUpdate".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching");
            provider
                .write(
                    &path,
                    "ExcludeWUDriversInQualityUpdate",
                    &RegValue::Dword(1),
                )
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message:
                "Automatic driver updates disabled. You now control which drivers are installed."
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
            message: "Automatic driver updates restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Prevents Windows Update from automatically downloading and installing driver updates.".into(),
            why_it_helps: "Gives you full control over driver versions. Prevents Windows from overwriting optimized GPU/chipset drivers with generic ones that may reduce performance.".into(),
            potential_risks: Some("You must manually update drivers for security patches. New hardware may not work without manual driver installation.".into()),
            how_to_revert: "Restores the original ExcludeWUDriversInQualityUpdate value from the snapshot.".into(),
        }
    }
}
