use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable taskbar app icon badges (notification counts)
pub struct DisableTaskbarBadgesTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableTaskbarBadgesTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableTaskbarBadgesTweak {
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
impl Tweak for DisableTaskbarBadgesTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_taskbar_badges".into(),
            name: "Disable Taskbar Badges".into(),
            description: "Hides the unread count badges on taskbar app icons.".into(),
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
            match provider.read(&path, "TaskbarBadges").await {
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
                .read(&path, "TaskbarBadges")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "TaskbarBadges".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"),
                name: "TaskbarBadges".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced");
            provider
                .write(&path, "TaskbarBadges", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Taskbar badges disabled. Unread count badges will be hidden.".into(),
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
            message: "Taskbar badges restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Hides the notification count badges that appear on top of taskbar app icons (e.g., unread email count).".into(),
            why_it_helps: "Reduces visual distraction and clutter on the taskbar, giving a cleaner desktop experience.".into(),
            potential_risks: None,
            how_to_revert: "Restores the original TaskbarBadges value from the snapshot.".into(),
        }
    }
}
