use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;

/// Disable the cursor shadow for a cleaner visual
pub struct DisableCursorShadowTweak {
    pub provider: Option<Arc<dyn crate::registry::RegistryProvider>>,
}

impl Default for DisableCursorShadowTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableCursorShadowTweak {
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
impl Tweak for DisableCursorShadowTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "visual_cursor_shadow".into(),
            name: "Disable Cursor Shadow".into(),
            description: "Turns off the mouse cursor shadow for a cleaner appearance.".into(),
            category: TweakCategory::Visual,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: false,
            affected_keys: vec![RegPath::hkcu(r"Control Panel\Cursors")],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Cursors");
            match provider.read(&path, "CursorShadow").await {
                Ok(RegValue::Dword(v)) => Ok(v == 0),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Cursors");
            let val = provider
                .read(&path, "CursorShadow")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "CursorShadow".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hkcu(r"Control Panel\Cursors"),
                name: "CursorShadow".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hkcu(r"Control Panel\Cursors");
            provider
                .write(&path, "CursorShadow", &RegValue::Dword(0))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Cursor shadow disabled.".into(),
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
            message: "Cursor shadow restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables the drop shadow rendered behind the mouse cursor.".into(),
            why_it_helps:
                "Gives the cursor a cleaner, sharper appearance without the shadow effect.".into(),
            potential_risks: None,
            how_to_revert: "Restores the original CursorShadow registry value.".into(),
        }
    }
}
