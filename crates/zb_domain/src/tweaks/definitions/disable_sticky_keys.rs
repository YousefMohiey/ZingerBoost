use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable Sticky Keys popup that appears on rapid Shift presses
pub struct DisableStickyKeysTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl DisableStickyKeysTweak {
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
impl Tweak for DisableStickyKeysTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_disable_sticky_keys".into(),
            name: "Disable Sticky Keys Popup".into(),
            description:
                "Prevents the Sticky Keys dialog from appearing when pressing Shift repeatedly."
                    .into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![RegPath::hkcu(r"Control Panel\Accessibility\StickyKeys")],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Accessibility\StickyKeys");
            match provider.read(&path, "Flags").await {
                Ok(RegValue::Sz(v)) => Ok(v == "506" || v == "0x1FA"),
                Ok(RegValue::Binary(v)) => Ok(v == vec![0xFA, 0x01, 0x00, 0x00]),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Accessibility\StickyKeys");
            let val = provider
                .read(&path, "Flags")
                .await
                .unwrap_or(RegValue::Sz("510".into()));
            Ok(SnapshotData::Registry {
                path,
                name: "Flags".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Control Panel\Accessibility\StickyKeys"),
                name: "Flags".into(),
                previous: RegValue::Sz("510".into()),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Accessibility\StickyKeys");
            provider
                .write(&path, "Flags", &RegValue::Sz("506".into()))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Sticky Keys popup disabled. No more interruptions during gaming.".into(),
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
            message: "Sticky Keys popup restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does:
                "Disables the Sticky Keys shortcut that appears when you press Shift 5 times."
                    .into(),
            why_it_helps: "Eliminates an annoying popup that interrupts gaming and typing.".into(),
            potential_risks: None,
            how_to_revert: "Restores the original Sticky Keys Flags value.".into(),
        }
    }
}
