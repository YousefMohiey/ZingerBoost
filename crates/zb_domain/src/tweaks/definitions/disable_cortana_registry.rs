use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Disable Cortana via registry keys for deep disable
pub struct DisableCortanaRegistryTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for DisableCortanaRegistryTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableCortanaRegistryTweak {
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
impl Tweak for DisableCortanaRegistryTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "privacy_disable_cortana_reg".into(),
            name: "Disable Cortana (Registry)".into(),
            description: "Deep disables Cortana via registry keys beyond just removing the AppX package.".into(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![
                RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\Windows Search"),
                RegPath::hkcu(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search"),
            ],
            source_url: Some("https://docs.microsoft.com/windows/configuration/cortana-at-work/cortana-at-work-policy-settings".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\Windows Search");
            match provider.read(&path, "AllowCortana").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\Windows Search");
            let val = provider
                .read(&path, "AllowCortana")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "AllowCortana".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\Windows Search"),
                name: "AllowCortana".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            // HKLM policy key
            let hklm_path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\Windows Search");
            provider
                .write(&hklm_path, "AllowCortana", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;

            // HKCU search keys
            let hkcu_path = RegPath::hkcu(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search");
            provider
                .write(&hkcu_path, "BingSearchEnabled", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
            provider
                .write(&hkcu_path, "CortanaConsent", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Cortana disabled via registry. Search will no longer include web results."
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
            message: "Cortana restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables Cortana and web search integration via registry policy keys.".into(),
            why_it_helps: "Improves privacy by preventing Microsoft from collecting search queries. Reduces background processes and network activity.".into(),
            potential_risks: Some("Windows Search will no longer include web results. Cortana voice assistant will be unavailable.".into()),
            how_to_revert: "Restores the original AllowCortana value from the snapshot.".into(),
        }
    }
}
