use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Disable Delivery Optimization to stop P2P update sharing
pub struct DisableDeliveryOptimizationTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for DisableDeliveryOptimizationTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableDeliveryOptimizationTweak {
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
impl Tweak for DisableDeliveryOptimizationTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "wu_disable_delivery_opt".into(),
            name: "Disable Delivery Optimization".into(),
            description: "Stops Windows from sharing updates with other PCs on your network or over the internet.".into(),
            category: TweakCategory::WindowsUpdate,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![
                RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization"),
            ],
            source_url: Some("https://docs.microsoft.com/windows/deployment/delivery-optimization".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization");
            match provider.read(&path, "DODownloadMode").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization");
            let val = provider
                .read(&path, "DODownloadMode")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "DODownloadMode".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization"),
                name: "DODownloadMode".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization");
            provider
                .write(&path, "DODownloadMode", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message:
                "Delivery Optimization disabled. Windows will no longer share updates via P2P."
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
            message: "Delivery Optimization restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables Windows Delivery Optimization, which uses your PC to upload updates to other computers via P2P.".into(),
            why_it_helps: "Reduces bandwidth usage and network congestion. Improves privacy by stopping Microsoft from using your PC as an update distribution node.".into(),
            potential_risks: Some("In enterprise environments, this may increase internet bandwidth for updates. No impact on home users.".into()),
            how_to_revert: "Restores the original DODownloadMode value from the snapshot.".into(),
        }
    }
}
