use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable Windows Consumer Features (auto-install of Store apps)
pub struct DisableConsumerFeaturesTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableConsumerFeaturesTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableConsumerFeaturesTweak {
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
impl Tweak for DisableConsumerFeaturesTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "privacy_disable_consumer_features".into(),
            name: "Disable Consumer Features".into(),
            description: "Prevents Windows from automatically installing games, third-party apps, and Store links.".into(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![RegPath::hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
            )],
            source_url: Some("https://winutil.christitus.com/dev/tweaks/essential-tweaks/consumerfeatures".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\CloudContent");
            match provider.read(&path, "DisableWindowsConsumerFeatures").await {
                Ok(RegValue::Dword(v)) => Ok(v == 1),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\CloudContent");
            let val = provider
                .read(&path, "DisableWindowsConsumerFeatures")
                .await
                .unwrap_or(RegValue::Dword(0));
            Ok(SnapshotData::Registry {
                path,
                name: "DisableWindowsConsumerFeatures".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\CloudContent"),
                name: "DisableWindowsConsumerFeatures".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            provider
                .write(
                    &RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\CloudContent"),
                    "DisableWindowsConsumerFeatures",
                    &RegValue::Dword(1),
                )
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Consumer features disabled.".into(),
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
            message: "Consumer features restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Stops Windows from automatically installing promotional apps and games for the signed-in user.".into(),
            why_it_helps: "Reduces unwanted app installations and system clutter.".into(),
            potential_risks: Some("Some default Store apps may become inaccessible (e.g., Phone Link).".into()),
            how_to_revert: "Re-enables Windows Consumer Features.".into(),
        }
    }
}
