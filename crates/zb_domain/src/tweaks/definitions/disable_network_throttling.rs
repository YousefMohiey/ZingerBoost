use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Disable Network Throttling Index to prevent Windows from throttling network traffic
pub struct DisableNetworkThrottlingTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for DisableNetworkThrottlingTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableNetworkThrottlingTweak {
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
impl Tweak for DisableNetworkThrottlingTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "network_disable_throttling".into(),
            name: "Disable Network Throttling Index".into(),
            description: "Prevents Windows from throttling network traffic when multimedia playback is detected.".into(),
            category: TweakCategory::Network,
            risk: RiskLevel::Moderate,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![
                RegPath::hklm(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile"),
            ],
            source_url: Some("https://docs.microsoft.com/windows/win32/procthread/multimedia-class-scheduler-service".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
            );
            match provider.read(&path, "NetworkThrottlingIndex").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0xFFFFFFFF),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
            );
            let val = provider
                .read(&path, "NetworkThrottlingIndex")
                .await
                .unwrap_or(RegValue::Dword(10)); // Default is 10
            Ok(SnapshotData::Registry {
                path,
                name: "NetworkThrottlingIndex".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(
                    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
                ),
                name: "NetworkThrottlingIndex".into(),
                previous: RegValue::Dword(10),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
            );
            // 0xFFFFFFFF = -1 = disable throttling
            provider
                .write(
                    &path,
                    "NetworkThrottlingIndex",
                    &RegValue::Dword(0xFFFFFFFF),
                )
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Network throttling disabled. Multimedia network traffic will no longer be throttled.".into(),
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
            message: "Network throttling restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables Windows network throttling that limits bandwidth for non-multimedia applications when multimedia is playing.".into(),
            why_it_helps: "Prevents Windows from reducing network bandwidth for games and other applications when you're watching videos or listening to music.".into(),
            potential_risks: Some("May cause multimedia playback to stutter if network is heavily congested. Revert if you experience video buffering issues.".into()),
            how_to_revert: "Restores the original NetworkThrottlingIndex value from the snapshot.".into(),
        }
    }
}
