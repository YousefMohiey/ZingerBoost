use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable Start menu app suggestions and ads
pub struct DisableStartSuggestionsTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableStartSuggestionsTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableStartSuggestionsTweak {
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
impl Tweak for DisableStartSuggestionsTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "privacy_start_ads".into(),
            name: "Disable Start Menu Ads".into(),
            description:
                "Prevents suggested apps and sponsored content from appearing in the Start menu."
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
            match provider.read(&path, "SystemPaneSuggestionsEnabled").await {
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
                .read(&path, "SystemPaneSuggestionsEnabled")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "SystemPaneSuggestionsEnabled".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(
                    r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                ),
                name: "SystemPaneSuggestionsEnabled".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager");
            provider
                .write(&path, "SystemPaneSuggestionsEnabled", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Start menu ads and app suggestions disabled.".into(),
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
            message: "Start menu suggestions restored to previous setting.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables sponsored app suggestions and 'recommended' content in the Windows Start menu.".into(),
            why_it_helps: "Removes advertisements and promoted apps from your Start menu, giving you a cleaner, distraction-free experience.".into(),
            potential_risks: Some("You may miss occasional tips about new Windows features. All manually installed apps remain accessible.".into()),
            how_to_revert: "Restores the original SystemPaneSuggestionsEnabled registry value.".into(),
        }
    }
}
