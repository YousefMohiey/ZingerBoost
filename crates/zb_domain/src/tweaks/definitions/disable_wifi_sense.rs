use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Disable Wi-Fi Sense to prevent auto-connecting to suggested open hotspots
pub struct DisableWifiSenseTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for DisableWifiSenseTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableWifiSenseTweak {
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
impl Tweak for DisableWifiSenseTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "network_disable_wifi_sense".into(),
            name: "Disable Wi-Fi Sense".into(),
            description:
                "Prevents Windows from automatically connecting to suggested open Wi-Fi hotspots."
                    .into(),
            category: TweakCategory::Network,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![RegPath::hklm(
                r"SOFTWARE\Microsoft\WcmSvc\wifinetworkmanager\config",
            )],
            source_url: Some("https://docs.microsoft.com/windows/configuration/wi-fi-sense".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Microsoft\WcmSvc\wifinetworkmanager\config");
            match provider.read(&path, "AutoConnectAllowedOEM").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Microsoft\WcmSvc\wifinetworkmanager\config");
            let val = provider
                .read(&path, "AutoConnectAllowedOEM")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "AutoConnectAllowedOEM".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(r"SOFTWARE\Microsoft\WcmSvc\wifinetworkmanager\config"),
                name: "AutoConnectAllowedOEM".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Microsoft\WcmSvc\wifinetworkmanager\config");
            provider
                .write(&path, "AutoConnectAllowedOEM", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message:
                "Wi-Fi Sense disabled. Windows will no longer auto-connect to suggested hotspots."
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
            message: "Wi-Fi Sense restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables Wi-Fi Sense feature that automatically connects to crowdsourced open Wi-Fi hotspots.".into(),
            why_it_helps: "Improves security by preventing automatic connections to potentially unsafe open networks. Reduces background network activity.".into(),
            potential_risks: Some("You will need to manually connect to open Wi-Fi networks. No impact on your saved Wi-Fi networks.".into()),
            how_to_revert: "Re-enables Wi-Fi Sense by restoring the original AutoConnectAllowedOEM value.".into(),
        }
    }
}
