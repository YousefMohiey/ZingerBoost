use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Enable verbose logon messages
pub struct VerboseLogonTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for VerboseLogonTweak {
    fn default() -> Self { Self::new() }
}

impl VerboseLogonTweak {
    pub fn new() -> Self { Self { provider: None } }
    pub fn with_provider(provider: Arc<dyn crate::registry::RegistryProvider>) -> Self {
        Self { provider: Some(provider) }
    }
}

#[async_trait]
impl Tweak for VerboseLogonTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_verbose_logon".into(),
            name: "Enable Verbose Logon Messages".into(),
            description: "Shows detailed status messages during login instead of the animated dots.".into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![RegPath::hklm(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
            )],
            source_url: Some("https://winutil.christitus.com/dev/tweaks/customize-preferences/verboselogon".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System");
            match provider.read(&path, "VerboseStatus").await {
                Ok(RegValue::Dword(v)) => Ok(v == 1),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System");
            let val = provider.read(&path, "VerboseStatus").await.unwrap_or(RegValue::Dword(0));
            Ok(SnapshotData::Registry { path, name: "VerboseStatus".into(), previous: val })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System"),
                name: "VerboseStatus".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            provider
                .write(&RegPath::hklm(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System"), "VerboseStatus", &RegValue::Dword(1))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult { reboot_required: false, message: "Verbose logon enabled.".into() })
    }

    async fn revert(&self, snapshot: &SnapshotData) -> Result<TweakResult, TweakError> {
        if let SnapshotData::Registry { path, name, previous } = snapshot {
            if let Some(provider) = &self.provider {
                provider.write(path, name, previous).await.map_err(|e| TweakError::Registry(e.to_string()))?;
            }
        }
        Ok(TweakResult { reboot_required: false, message: "Verbose logon disabled.".into() })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Replaces the animated dots during login with detailed status messages showing what Windows is doing.".into(),
            why_it_helps: "Useful for troubleshooting boot/login issues by showing exactly what's happening.".into(),
            potential_risks: None,
            how_to_revert: "Restores the default animated dots during login.".into(),
        }
    }
}
