use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable Aero Shake (minimize all windows by shaking a window)
pub struct DisableAeroShakeTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableAeroShakeTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableAeroShakeTweak {
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
impl Tweak for DisableAeroShakeTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_aero_shake".into(),
            name: "Disable Aero Shake".into(),
            description: "Prevents window shake from minimizing all other windows.".into(),
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
            match provider.read(&path, "DisallowShaking").await {
                Ok(RegValue::Dword(v)) => Ok(v == 1),
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
                .read(&path, "DisallowShaking")
                .await
                .unwrap_or(RegValue::Dword(0));
            Ok(SnapshotData::Registry {
                path,
                name: "DisallowShaking".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"),
                name: "DisallowShaking".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced");
            provider
                .write(&path, "DisallowShaking", &RegValue::Dword(1))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Aero Shake disabled. Shaking a window will no longer minimize others.".into(),
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
            message: "Aero Shake restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables the Aero Shake feature that minimizes all other windows when you shake a window title bar.".into(),
            why_it_helps: "Prevents accidental window minimization, which is especially useful for users who frequently reposition windows.".into(),
            potential_risks: None,
            how_to_revert: "Restores the original DisallowShaking value from the snapshot.".into(),
        }
    }
}
