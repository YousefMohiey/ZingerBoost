use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation,
    TweakMetadata, TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable Game DVR to reduce background recording overhead
pub struct DisableGameDvrTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl DisableGameDvrTweak {
    pub fn new() -> Self {
        Self { provider: None }
    }

    pub fn with_provider(provider: Arc<dyn crate::registry::RegistryProvider>) -> Self {
        Self { provider: Some(provider) }
    }
}

#[async_trait]
impl Tweak for DisableGameDvrTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "gaming_disable_dvr".into(),
            name: "Disable Game DVR".into(),
            description: "Turns off Windows Game DVR background recording to free up CPU/GPU resources.".into(),
            category: TweakCategory::Gaming,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![
                RegPath::hkcu(r"System\GameConfigStore"),
                RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\GameDVR"),
            ],
            source_url: Some("https://docs.microsoft.com/windows/gaming/game-dvr".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"System\GameConfigStore");
            match provider.read(&path, "GameDVR_Enabled").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"System\GameConfigStore");
            let val = provider.read(&path, "GameDVR_Enabled").await.unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry { path, name: "GameDVR_Enabled".into(), previous: val })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"System\GameConfigStore"),
                name: "GameDVR_Enabled".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"System\GameConfigStore");
            provider.write(&path, "GameDVR_Enabled", &RegValue::Dword(0)).await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
            let hklm_path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\GameDVR");
            provider.write(&hklm_path, "AllowGameDVR", &RegValue::Dword(0)).await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Game DVR disabled. Background recording is now off.".into(),
        })
    }

    async fn revert(&self, snapshot: &SnapshotData) -> Result<TweakResult, TweakError> {
        if let SnapshotData::Registry { path, name, previous } = snapshot {
            if let Some(provider) = &self.provider {
                provider.write(path, name, previous).await
                    .map_err(|e| TweakError::Registry(e.to_string()))?;
            }
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Game DVR restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables the Xbox Game Bar DVR background recording service.".into(),
            why_it_helps: "Reduces CPU, GPU, and disk overhead while gaming, potentially improving frame rates.".into(),
            potential_risks: Some("You will not be able to record gameplay clips using Windows built-in DVR.".into()),
            how_to_revert: "Re-enables Game DVR by restoring the original registry value.".into(),
        }
    }
}
