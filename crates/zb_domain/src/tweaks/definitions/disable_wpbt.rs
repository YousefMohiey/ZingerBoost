use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable WPBT (Windows Platform Binary Table) to prevent vendor-forced software installs
pub struct DisableWpbtTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableWpbtTweak {
    fn default() -> Self { Self::new() }
}

impl DisableWpbtTweak {
    pub fn new() -> Self { Self { provider: None } }
    pub fn with_provider(provider: Arc<dyn crate::registry::RegistryProvider>) -> Self {
        Self { provider: Some(provider) }
    }
}

#[async_trait]
impl Tweak for DisableWpbtTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "security_disable_wpbt".into(),
            name: "Disable WPBT (Vendor Bloat)".into(),
            description: "Prevents your computer vendor from force-installing software at boot via WPBT.".into(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            requires_reboot: true,
            requires_admin: true,
            affected_keys: vec![RegPath::hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager",
            )],
            source_url: Some("https://winutil.christitus.com/dev/tweaks/essential-tweaks/wpbt".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SYSTEM\CurrentControlSet\Control\Session Manager");
            match provider.read(&path, "DisableWpbtExecution").await {
                Ok(RegValue::Dword(v)) => Ok(v == 1),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SYSTEM\CurrentControlSet\Control\Session Manager");
            let val = provider.read(&path, "DisableWpbtExecution").await.unwrap_or(RegValue::Dword(0));
            Ok(SnapshotData::Registry { path, name: "DisableWpbtExecution".into(), previous: val })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(r"SYSTEM\CurrentControlSet\Control\Session Manager"),
                name: "DisableWpbtExecution".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            provider
                .write(&RegPath::hklm(r"SYSTEM\CurrentControlSet\Control\Session Manager"), "DisableWpbtExecution", &RegValue::Dword(1))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult { reboot_required: true, message: "WPBT execution disabled. Reboot to apply.".into() })
    }

    async fn revert(&self, snapshot: &SnapshotData) -> Result<TweakResult, TweakError> {
        if let SnapshotData::Registry { path, name, previous } = snapshot {
            if let Some(provider) = &self.provider {
                provider.write(path, name, previous).await.map_err(|e| TweakError::Registry(e.to_string()))?;
            }
        }
        Ok(TweakResult { reboot_required: true, message: "WPBT restored. Reboot to apply.".into() })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables WPBT which allows PC vendors to execute programs at boot time and force-install software without user consent.".into(),
            why_it_helps: "Prevents unwanted software installation by your PC manufacturer.".into(),
            potential_risks: Some("Anti-theft software or essential vendor drivers may be affected on some systems.".into()),
            how_to_revert: "Re-enables WPBT execution.".into(),
        }
    }
}
