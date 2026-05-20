use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Disable Tailored Experiences to stop Microsoft from using diagnostic data for personalization
pub struct DisableTailoredExperiencesTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for DisableTailoredExperiencesTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableTailoredExperiencesTweak {
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
impl Tweak for DisableTailoredExperiencesTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "privacy_disable_tailored".into(),
            name: "Disable Tailored Experiences".into(),
            description: "Prevents Microsoft from using diagnostic data to deliver personalized tips, ads, and recommendations.".into(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![
                RegPath::hkcu(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy"),
            ],
            source_url: Some("https://docs.microsoft.com/windows/privacy/manage-windows-endpoints".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy");
            match provider
                .read(&path, "TailoredExperiencesWithDiagnosticDataEnabled")
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
            let path = RegPath::hkcu(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy");
            let val = provider
                .read(&path, "TailoredExperiencesWithDiagnosticDataEnabled")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "TailoredExperiencesWithDiagnosticDataEnabled".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy"),
                name: "TailoredExperiencesWithDiagnosticDataEnabled".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy");
            provider
                .write(
                    &path,
                    "TailoredExperiencesWithDiagnosticDataEnabled",
                    &RegValue::Dword(0),
                )
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Tailored experiences disabled. Microsoft will no longer personalize content based on your data.".into(),
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
            message: "Tailored experiences restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables Microsoft's tailored experiences feature that uses your diagnostic data to deliver personalized tips, ads, and recommendations.".into(),
            why_it_helps: "Improves privacy by preventing Microsoft from analyzing your usage patterns for personalization. Reduces targeted advertising.".into(),
            potential_risks: Some("You may see less relevant tips and recommendations in Windows. No impact on core functionality.".into()),
            how_to_revert: "Restores the original TailoredExperiencesWithDiagnosticDataEnabled value from the snapshot.".into(),
        }
    }
}
