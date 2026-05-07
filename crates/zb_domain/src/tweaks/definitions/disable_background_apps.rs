use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation,
    TweakMetadata, TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable background apps to free up resources
pub struct DisableBackgroundAppsTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl DisableBackgroundAppsTweak {
    pub fn new() -> Self {
        Self { provider: None }
    }

    pub fn with_provider(provider: Arc<dyn crate::registry::RegistryProvider>) -> Self {
        Self { provider: Some(provider) }
    }
}

#[async_trait]
impl Tweak for DisableBackgroundAppsTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "privacy_disable_background_apps".into(),
            name: "Disable Background Apps".into(),
            description: "Prevents UWP apps from running in the background.".into(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications"),
            ],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications");
            match provider.read(&path, "GlobalUserDisabled").await {
                Ok(RegValue::Dword(v)) => Ok(v == 1),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications");
            let val = provider.read(&path, "GlobalUserDisabled").await.unwrap_or(RegValue::Dword(0));
            Ok(SnapshotData::Registry { path, name: "GlobalUserDisabled".into(), previous: val })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications"),
                name: "GlobalUserDisabled".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications");
            provider.write(&path, "GlobalUserDisabled", &RegValue::Dword(1)).await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Background apps disabled. UWP apps will no longer run in the background.".into(),
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
            message: "Background apps restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Prevents Windows Store apps from running in the background.".into(),
            why_it_helps: "Reduces CPU, RAM, and battery usage by stopping background UWP app processes.".into(),
            potential_risks: Some("Live tiles will not update, and some apps may not receive push notifications.".into()),
            how_to_revert: "Restores the original GlobalUserDisabled registry value.".into(),
        }
    }
}
