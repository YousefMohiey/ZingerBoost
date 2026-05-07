use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable taskbar animations for a snappier taskbar
pub struct DisableTaskbarAnimationsTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableTaskbarAnimationsTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableTaskbarAnimationsTweak {
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
impl Tweak for DisableTaskbarAnimationsTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_taskbar_animations".into(),
            name: "Disable Taskbar Animations".into(),
            description: "Turns off animation effects on the taskbar for quicker interaction."
                .into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![RegPath::hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            )],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced");
            match provider.read(&path, "TaskbarAnimations").await {
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
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced");
            let val = provider
                .read(&path, "TaskbarAnimations")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "TaskbarAnimations".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"),
                name: "TaskbarAnimations".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced");
            provider
                .write(&path, "TaskbarAnimations", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Taskbar animations disabled. Taskbar will respond instantly.".into(),
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
            message: "Taskbar animations restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables the transition and hover animations on the taskbar and thumbnails.".into(),
            why_it_helps: "Makes the taskbar feel faster by removing visual delays during hover and thumbnail preview.".into(),
            potential_risks: None,
            how_to_revert: "Restores the original TaskbarAnimations value from the snapshot.".into(),
        }
    }
}
