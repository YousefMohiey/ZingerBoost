use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Disable Fullscreen Optimizations to force true exclusive fullscreen mode
pub struct DisableFullscreenOptimizationsTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for DisableFullscreenOptimizationsTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableFullscreenOptimizationsTweak {
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
impl Tweak for DisableFullscreenOptimizationsTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "gaming_disable_fs_optimizations".into(),
            name: "Disable Fullscreen Optimizations".into(),
            description: "Forces true exclusive fullscreen mode in games, reducing input latency and improving performance.".into(),
            category: TweakCategory::Gaming,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![
                RegPath::hkcu(r"System\GameConfigStore"),
            ],
            source_url: Some("https://docs.microsoft.com/windows/gaming/game-bar".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"System\GameConfigStore");
            match provider.read(&path, "GameDVR_FSEBehaviorMode").await {
                Ok(RegValue::Dword(v)) => Ok(v == 2),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"System\GameConfigStore");
            let val = provider
                .read(&path, "GameDVR_FSEBehaviorMode")
                .await
                .unwrap_or(RegValue::Dword(0));
            Ok(SnapshotData::Registry {
                path,
                name: "GameDVR_FSEBehaviorMode".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"System\GameConfigStore"),
                name: "GameDVR_FSEBehaviorMode".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"System\GameConfigStore");
            // 2 = Force disable fullscreen optimizations
            provider
                .write(&path, "GameDVR_FSEBehaviorMode", &RegValue::Dword(2))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message:
                "Fullscreen optimizations disabled. Games will use true exclusive fullscreen mode."
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
            message: "Fullscreen optimizations restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables Windows fullscreen optimizations that run games in a borderless windowed mode instead of true exclusive fullscreen.".into(),
            why_it_helps: "Reduces input latency and can improve frame rates by allowing games to take exclusive control of the display. Eliminates the overhead of Windows composition.".into(),
            potential_risks: Some("Alt-Tab may be slower when switching out of games. Some overlay applications (Discord, GeForce Experience) may not work in exclusive fullscreen.".into()),
            how_to_revert: "Restores the original GameDVR_FSEBehaviorMode value from the snapshot.".into(),
        }
    }
}
