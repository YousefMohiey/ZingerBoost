use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Hide the Meet Now icon from the taskbar
pub struct DisableMeetNowTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableMeetNowTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableMeetNowTweak {
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
impl Tweak for DisableMeetNowTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "privacy_meet_now".into(),
            name: "Hide Meet Now".into(),
            description: "Removes the Skype Meet Now icon from the Windows taskbar.".into(),
            category: TweakCategory::Privacy,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![RegPath::hkcu(
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
            )],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer");
            match provider.read(&path, "HideSCAMeetNow").await {
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
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer");
            let val = provider
                .read(&path, "HideSCAMeetNow")
                .await
                .unwrap_or(RegValue::Dword(0));
            Ok(SnapshotData::Registry {
                path,
                name: "HideSCAMeetNow".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer"),
                name: "HideSCAMeetNow".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path =
                RegPath::hkcu(r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer");
            provider
                .write(&path, "HideSCAMeetNow", &RegValue::Dword(1))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Meet Now icon hidden from the taskbar. Restart Explorer to see the change."
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
            message: "Meet Now icon restored to previous setting.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Hides the Skype Meet Now button from the taskbar notification area.".into(),
            why_it_helps: "Removes a persistent icon for a service many users never use, reducing taskbar clutter and Skype integration in Windows.".into(),
            potential_risks: Some("You can still use Skype and video calling features manually. The icon is simply hidden, not removed.".into()),
            how_to_revert: "Restores the original HideSCAMeetNow registry value.".into(),
        }
    }
}
