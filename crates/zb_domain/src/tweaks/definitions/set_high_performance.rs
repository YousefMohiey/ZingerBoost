use async_trait::async_trait;
use std::os::windows::process::CommandExt;
use std::process::Command;
use zb_shared::types::{
    RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata, TweakResult,
};

use crate::errors::TweakError;
use crate::tweaks::traits::Tweak;
use zb_shared::constants::CREATE_NO_WINDOW;
const HIGH_PERF_GUID: &str = "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c";
const BALANCED_GUID: &str = "381b4222-f694-41f0-9685-ff5bb260df2e";

/// Set High Performance power plan
pub struct SetHighPerformanceTweak;

impl SetHighPerformanceTweak {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SetHighPerformanceTweak {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tweak for SetHighPerformanceTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "performance_high_power".into(),
            name: "Set High Performance Power Plan".into(),
            description:
                "Switches the active power plan to High Performance for maximum CPU responsiveness."
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
            .args(["/getactivescheme"])
            .output()
            .map_err(|e| TweakError::Unknown(format!("powercfg query failed: {}", e)))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.contains(HIGH_PERF_GUID))
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        let output = Command::new("powercfg")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["/getactivescheme"])
            .output()
            .map_err(|e| TweakError::Unknown(format!("powercfg query failed: {}", e)))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let prev_guid = if stdout.contains(BALANCED_GUID) {
            BALANCED_GUID
        } else {
            "381b4222-f694-41f0-9685-ff5bb260df2e"
        };
        Ok(SnapshotData::PowerPlan {
            previous_guid: prev_guid.into(),
        })
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        let output = Command::new("powercfg")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["/setactive", HIGH_PERF_GUID])
            .output()
            .map_err(|e| TweakError::Unknown(format!("powercfg failed: {}", e)))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TweakError::Unknown(format!("powercfg error: {}", stderr)));
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "High Performance power plan activated.".into(),
        })
    }

    async fn revert(&self, snapshot: &SnapshotData) -> Result<TweakResult, TweakError> {
        if let SnapshotData::PowerPlan { previous_guid } = snapshot {
            let output = Command::new("powercfg")
                .creation_flags(CREATE_NO_WINDOW)
                .args(["/setactive", previous_guid])
                .output()
                .map_err(|e| TweakError::Unknown(format!("powercfg failed: {}", e)))?;
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(TweakError::Unknown(format!("powercfg error: {}", stderr)));
            }
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "Previous power plan restored.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does:
                "Sets the Windows power plan to High Performance, preventing CPU downclocking."
                    .into(),
            why_it_helps:
                "Improves CPU responsiveness and reduces input latency, especially on desktops."
                    .into(),
            potential_risks: Some(
                "May increase power consumption and heat output on laptops.".into(),
            ),
            how_to_revert: "Restores the previously active power plan (usually Balanced).".into(),
        }
    }
}
