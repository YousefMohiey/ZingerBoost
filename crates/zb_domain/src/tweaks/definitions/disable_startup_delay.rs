use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable startup delay to open apps faster after boot
pub struct DisableStartupDelayTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl DisableStartupDelayTweak {
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
impl Tweak for DisableStartupDelayTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "performance_disable_startup_delay".into(),
            name: "Disable Startup Delay".into(),
            description: "Removes the built-in 10-second delay before startup apps launch.".into(),
            category: TweakCategory::Performance,
            risk: RiskLevel::Safe,
            requires_reboot: true,
            requires_admin: false,
            affected_keys: vec![RegPath::hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
            )],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize");
            match provider.read(&path, "StartupDelayInMSec").await {
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
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize");
            let val = provider
                .read(&path, "StartupDelayInMSec")
                .await
                .unwrap_or(RegValue::Dword(10000));
            Ok(SnapshotData::Registry {
                path,
                name: "StartupDelayInMSec".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(
                    r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
                ),
                name: "StartupDelayInMSec".into(),
                previous: RegValue::Dword(10000),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize");
            provider
                .write(&path, "StartupDelayInMSec", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: true,
            message: "Startup delay removed. Startup apps will launch immediately after boot."
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
            reboot_required: true,
            message: "Startup delay restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Sets StartupDelayInMSec to 0, removing the 10-second built-in delay.".into(),
            why_it_helps: "Startup apps (like Discord, Spotify, etc.) launch immediately instead of waiting.".into(),
            potential_risks: Some("May slightly increase initial boot CPU/disk load as all startup apps launch at once.".into()),
            how_to_revert: "Restores the original StartupDelayInMSec value.".into(),
        }
    }
}
