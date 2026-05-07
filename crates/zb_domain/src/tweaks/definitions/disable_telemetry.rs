use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation,
    TweakMetadata, TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable basic Windows telemetry
pub struct DisableTelemetryTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl DisableTelemetryTweak {
    pub fn new() -> Self {
        Self { provider: None }
    }

    pub fn with_provider(provider: Arc<dyn crate::registry::RegistryProvider>) -> Self {
        Self { provider: Some(provider) }
    }
}

#[async_trait]
impl Tweak for DisableTelemetryTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "privacy_disable_telemetry".into(),
            name: "Disable Telemetry (Basic)".into(),
            description: "Sets telemetry to the minimum level to reduce data collection.".into(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![
                RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\DataCollection"),
            ],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\DataCollection");
            match provider.read(&path, "AllowTelemetry").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\DataCollection");
            let val = provider.read(&path, "AllowTelemetry").await.unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry { path, name: "AllowTelemetry".into(), previous: val })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\DataCollection"),
                name: "AllowTelemetry".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SOFTWARE\Policies\Microsoft\Windows\DataCollection");
            provider.write(&path, "AllowTelemetry", &RegValue::Dword(0)).await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Telemetry set to minimum (Security level). Windows will collect the least amount of diagnostic data.".into(),
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
            message: "Telemetry restored to previous level.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Sets the Windows telemetry level to Security (0), the minimum possible.".into(),
            why_it_helps: "Reduces the amount of diagnostic and usage data sent to Microsoft, improving privacy.".into(),
            potential_risks: Some("Some Windows Insider features and advanced diagnostics may be limited. Windows Update still works fully.".into()),
            how_to_revert: "Restores the original AllowTelemetry value (typically Basic or Full).".into(),
        }
    }
}
