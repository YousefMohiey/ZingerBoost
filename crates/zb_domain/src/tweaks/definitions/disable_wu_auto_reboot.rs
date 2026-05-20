use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Disable Windows Update Auto-Reboot to prevent unexpected restarts
pub struct DisableWUAutoRebootTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for DisableWUAutoRebootTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableWUAutoRebootTweak {
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
impl Tweak for DisableWUAutoRebootTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "wu_disable_auto_reboot".into(),
            name: "Disable WU Auto-Reboot".into(),
            description: "Prevents Windows from automatically rebooting after installing updates."
                .into(),
            category: TweakCategory::WindowsUpdate,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![RegPath::hklm(
                r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU",
            )],
            source_url: Some(
                "https://docs.microsoft.com/windows/deployment/update/manage-windows-updates"
                    .into(),
            ),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU");
            match provider.read(&path, "NoAutoRebootWithLoggedOnUsers").await {
                Ok(RegValue::Dword(v)) => Ok(v == 1),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU");
            let val = provider
                .read(&path, "NoAutoRebootWithLoggedOnUsers")
                .await
                .unwrap_or(RegValue::Dword(0));
            Ok(SnapshotData::Registry {
                path,
                name: "NoAutoRebootWithLoggedOnUsers".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU"),
                name: "NoAutoRebootWithLoggedOnUsers".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU");
            provider
                .write(&path, "NoAutoRebootWithLoggedOnUsers", &RegValue::Dword(1))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Auto-reboot after updates disabled. You control when to restart.".into(),
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
            message: "Auto-reboot after updates restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Prevents Windows from automatically restarting your computer after installing updates when a user is logged in.".into(),
            why_it_helps: "Prevents unexpected reboots during gaming, work, or streaming. Gives you control over when to restart.".into(),
            potential_risks: Some("Updates may require a reboot to take effect. You must remember to restart manually for security patches.".into()),
            how_to_revert: "Restores the original NoAutoRebootWithLoggedOnUsers value from the snapshot.".into(),
        }
    }
}
