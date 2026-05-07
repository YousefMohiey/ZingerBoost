use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable Aero Snap (window edge snapping)
pub struct DisableAeroSnapTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableAeroSnapTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableAeroSnapTweak {
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
impl Tweak for DisableAeroSnapTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_aero_snap".into(),
            name: "Disable Aero Snap".into(),
            description:
                "Prevents windows from automatically snapping to screen edges when dragged.".into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![RegPath::hkcu(r"Control Panel\Desktop")],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Desktop");
            match provider.read(&path, "WindowArrangementActive").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Desktop");
            let val = provider
                .read(&path, "WindowArrangementActive")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "WindowArrangementActive".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Control Panel\Desktop"),
                name: "WindowArrangementActive".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Desktop");
            provider
                .write(&path, "WindowArrangementActive", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Aero Snap disabled. Windows will no longer snap to screen edges.".into(),
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
            message: "Aero Snap restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables the window snap feature that auto-arranges windows when dragged to screen edges or corners.".into(),
            why_it_helps: "Gives you full manual control over window placement without Windows trying to dock or resize them automatically.".into(),
            potential_risks: None,
            how_to_revert: "Restores the original WindowArrangementActive value from the snapshot.".into(),
        }
    }
}
