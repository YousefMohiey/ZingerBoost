use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable ads and promotions in File Explorer
pub struct DisableExplorerAdsTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableExplorerAdsTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableExplorerAdsTweak {
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
impl Tweak for DisableExplorerAdsTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "privacy_explorer_ads".into(),
            name: "Disable Explorer Ads".into(),
            description: "Prevents ads and promotional content from appearing in File Explorer."
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
            match provider
                .read(&path, "SubscribedContent-338393Enabled")
                .await
            {
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
                .read(&path, "SubscribedContent-338393Enabled")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "SubscribedContent-338393Enabled".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(
                    r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                ),
                name: "SubscribedContent-338393Enabled".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager");
            provider
                .write(
                    &path,
                    "SubscribedContent-338393Enabled",
                    &RegValue::Dword(0),
                )
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "File Explorer ads and promotional content disabled.".into(),
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
            message: "File Explorer content restored to previous setting.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables promotional content and advertisements shown within Windows File Explorer.".into(),
            why_it_helps: "Removes Microsoft's in-Explorer ads for OneDrive, Microsoft 365, and other services, keeping your file manager clean and focused.".into(),
            potential_risks: Some("File Explorer will no longer show tips about Microsoft services. No loss of functionality.".into()),
            how_to_revert: "Restores the original SubscribedContent-338393Enabled registry value.".into(),
        }
    }
}
