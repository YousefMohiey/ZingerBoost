use async_trait::async_trait;
use std::os::windows::process::CommandExt;
use std::process::Command;
use zb_shared::types::{
    RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata, TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;
use zb_shared::constants::CREATE_NO_WINDOW;

/// Disable hibernation to free up disk space and reduce SSD wear
pub struct DisableHibernationTweak;

impl DisableHibernationTweak {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DisableHibernationTweak {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tweak for DisableHibernationTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "performance_disable_hibernation".into(),
            name: "Disable Hibernation".into(),
            description: "Turns off hibernation to free up disk space equal to your RAM size."
                .into(),
            category: TweakCategory::Performance,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![],
            source_url: None,
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        let output = Command::new("powercfg")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["/hibernate"])
            .output()
            .map_err(|e| TweakError::Unknown(format!("powercfg query failed: {}", e)))?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        // powercfg /hibernate returns current state in stdout
        Ok(!stdout.contains("on") || stdout.contains("off"))
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        let sys_drive = std::env::var("SystemDrive").unwrap_or_else(|_| "C:".into());
        let exists = std::path::Path::new(&format!("{}\\hiberfil.sys", sys_drive)).exists();
        Ok(SnapshotData::Other(format!("hibernation:{}", exists)))
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        let output = Command::new("powercfg")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["/hibernate", "off"])
            .output()
            .map_err(|e| TweakError::Unknown(format!("powercfg failed: {}", e)))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TweakError::Unknown(format!("powercfg error: {}", stderr)));
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Hibernation disabled. hiberfil.sys removed, freeing disk space.".into(),
        })
    }

    async fn revert(&self, _snapshot: &SnapshotData) -> Result<TweakResult, TweakError> {
        let output = Command::new("powercfg")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["/hibernate", "on"])
            .output()
            .map_err(|e| TweakError::Unknown(format!("powercfg failed: {}", e)))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TweakError::Unknown(format!("powercfg error: {}", stderr)));
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Hibernation restored. hiberfil.sys recreated.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables hibernation and deletes the hiberfil.sys file.".into(),
            why_it_helps: "Frees up disk space equal to your RAM size (e.g., 16 GB) and reduces SSD write wear. Sleep mode still works.".into(),
            potential_risks: Some("You will not be able to hibernate. Only Sleep and Shut Down remain.".into()),
            how_to_revert: "Runs powercfg /hibernate on to re-enable hibernation.".into(),
        }
    }
}
