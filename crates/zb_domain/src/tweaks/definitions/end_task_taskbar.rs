use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Enable right-click "End Task" on taskbar (Win11)
pub struct EndTaskOnTaskbarTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for EndTaskOnTaskbarTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl EndTaskOnTaskbarTweak {
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
impl Tweak for EndTaskOnTaskbarTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_end_task_taskbar".into(),
            name: "Enable End Task on Taskbar".into(),
            description: "Adds an 'End Task' option when right-clicking a program in the taskbar."
                .into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![RegPath::hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings",
            )],
            source_url: Some(
                "https://winutil.christitus.com/dev/tweaks/essential-tweaks/endtaskontaskbar"
                    .into(),
            ),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings",
            );
            match provider.read(&path, "TaskbarEndTask").await {
                Ok(RegValue::Dword(v)) => Ok(v == 1),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings",
            );
            let val = provider
                .read(&path, "TaskbarEndTask")
                .await
                .unwrap_or(RegValue::Dword(0));
            Ok(SnapshotData::Registry {
                path,
                name: "TaskbarEndTask".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(
                    r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings",
                ),
                name: "TaskbarEndTask".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            provider
                .write(&RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings"), "TaskbarEndTask", &RegValue::Dword(1))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "End Task enabled on taskbar.".into(),
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
            message: "End Task restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does:
                "Enables the 'End Task' option in the right-click context menu of taskbar items."
                    .into(),
            why_it_helps:
                "Quickly kill frozen or unresponsive programs without opening Task Manager.".into(),
            potential_risks: None,
            how_to_revert: "Removes the End Task option from the taskbar context menu.".into(),
        }
    }
}
