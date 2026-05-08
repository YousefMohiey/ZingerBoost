use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Master switch to disable ALL Windows visual effects for maximum performance
pub struct DisableAllVisualEffectsTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableAllVisualEffectsTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableAllVisualEffectsTweak {
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
impl Tweak for DisableAllVisualEffectsTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_disable_all_effects".into(),
            name: "Disable All Visual Effects".into(),
            description: "Sets the Visual Effects master switch to 'Adjust for best performance', disabling all animations, shadows, and transparency.".into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer"),
                RegPath::hkcu(r"Control Panel\Desktop\WindowMetrics"),
            ],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer");
            match provider.read(&path, "VisualEffects").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer");
            let val = provider
                .read(&path, "VisualEffects")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "VisualEffects".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer"),
                name: "VisualEffects".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer");
            provider
                .write(&path, "VisualEffects", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
            let wm_path = RegPath::hkcu(r"Control Panel\Desktop\WindowMetrics");
            provider
                .write(&wm_path, "MinAnimate", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message:
                "All visual effects disabled. Windows will use the 'best performance' appearance."
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
            message: "Visual Effects master switch restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Sets the Windows Visual Effects master switch to 'Adjust for best performance', disabling all animations, shadows, transparency, smooth scrolling, and other visual effects system-wide.".into(),
            why_it_helps: "Dramatically reduces GPU and CPU overhead from visual effects, making the system feel faster — especially beneficial on older hardware or virtual machines.".into(),
            potential_risks: Some("The UI will look less polished. Individual visual effects can be re-enabled manually via System Properties > Performance Options.".into()),
            how_to_revert: "Restores the original VisualEffects registry value from the snapshot.".into(),
        }
    }
}
