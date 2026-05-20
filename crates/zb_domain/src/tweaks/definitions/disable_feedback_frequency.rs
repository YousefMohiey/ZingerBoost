use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Disable Feedback Frequency to stop Windows from prompting for feedback
pub struct DisableFeedbackFrequencyTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for DisableFeedbackFrequencyTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableFeedbackFrequencyTweak {
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
impl Tweak for DisableFeedbackFrequencyTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "privacy_disable_feedback".into(),
            name: "Disable Feedback Frequency".into(),
            description: "Stops Windows from prompting you for feedback and reduces diagnostic data collection.".into(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![
                RegPath::hkcu(r"SOFTWARE\Microsoft\Siuf\Rules"),
            ],
            source_url: Some("https://docs.microsoft.com/windows/privacy/manage-windows-endpoints".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"SOFTWARE\Microsoft\Siuf\Rules");
            match provider.read(&path, "NumberOfSIUFInPeriod").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"SOFTWARE\Microsoft\Siuf\Rules");
            let val = provider
                .read(&path, "NumberOfSIUFInPeriod")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "NumberOfSIUFInPeriod".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"SOFTWARE\Microsoft\Siuf\Rules"),
                name: "NumberOfSIUFInPeriod".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"SOFTWARE\Microsoft\Siuf\Rules");
            provider
                .write(&path, "NumberOfSIUFInPeriod", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
            provider
                .write(&path, "PeriodInNanoSeconds", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Feedback prompts disabled. Windows will no longer ask for your feedback."
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
            message: "Feedback frequency restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables Windows feedback prompts by setting the feedback frequency to zero.".into(),
            why_it_helps: "Improves privacy by reducing diagnostic data collection. Eliminates annoying feedback popups.".into(),
            potential_risks: Some("You will no longer be prompted to provide feedback to Microsoft. No impact on system functionality.".into()),
            how_to_revert: "Restores the original NumberOfSIUFInPeriod value from the snapshot.".into(),
        }
    }
}
