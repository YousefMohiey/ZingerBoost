use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Disable Activity History to stop Windows from recording app usage
pub struct DisableActivityHistoryTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for DisableActivityHistoryTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableActivityHistoryTweak {
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
impl Tweak for DisableActivityHistoryTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "privacy_disable_activity_hist".into(),
            name: "Disable Activity History".into(),
            description: "Stops Windows from recording and syncing app usage history across devices.".into(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![
                RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\System"),
            ],
            source_url: Some("https://docs.microsoft.com/windows/privacy/manage-connections-from-windows-operating-system-components-to-microsoft-services".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\System");
            match provider.read(&path, "PublishUserActivities").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\System");
            let val = provider
                .read(&path, "PublishUserActivities")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "PublishUserActivities".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\System"),
                name: "PublishUserActivities".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\System");
            provider
                .write(&path, "PublishUserActivities", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
            provider
                .write(&path, "EnableActivityFeed", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Activity history disabled. Windows will no longer track your app usage."
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
            message: "Activity history restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables Windows activity history and timeline features that track which apps you use and when.".into(),
            why_it_helps: "Improves privacy by preventing Microsoft from collecting your app usage patterns. Reduces background data sync.".into(),
            potential_risks: Some("Timeline feature will be unavailable. You won't be able to resume activities across devices.".into()),
            how_to_revert: "Restores the original PublishUserActivities value from the snapshot.".into(),
        }
    }
}
