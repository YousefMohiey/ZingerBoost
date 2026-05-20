use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Enable Hardware-Accelerated GPU Scheduling to reduce input latency
pub struct EnableHwGpuSchedulingTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for EnableHwGpuSchedulingTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl EnableHwGpuSchedulingTweak {
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
impl Tweak for EnableHwGpuSchedulingTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "gaming_hw_gpu_scheduling".into(),
            name: "Enable Hardware GPU Scheduling".into(),
            description: "Enables hardware-accelerated GPU scheduling to reduce input latency and improve gaming performance.".into(),
            category: TweakCategory::Gaming,
            risk: RiskLevel::Safe,
            requires_reboot: true,
            requires_admin: true,
            affected_keys: vec![
                RegPath::hklm(r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers"),
            ],
            source_url: Some("https://docs.microsoft.com/windows/win32/direct3d12/hardware-accelerated-gpu-scheduling".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers");
            match provider.read(&path, "HwSchMode").await {
                Ok(RegValue::Dword(v)) => Ok(v == 2),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers");
            let val = provider
                .read(&path, "HwSchMode")
                .await
                .unwrap_or(RegValue::Dword(0)); // 0 = default (let Windows decide)
            Ok(SnapshotData::Registry {
                path,
                name: "HwSchMode".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers"),
                name: "HwSchMode".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers");
            // 2 = Force enable hardware scheduling
            provider
                .write(&path, "HwSchMode", &RegValue::Dword(2))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: true,
            message: "Hardware GPU scheduling enabled. Input latency reduced. Reboot required."
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
            reboot_required: true,
            message: "Hardware GPU scheduling restored to previous state. Reboot required.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Enables hardware-accelerated GPU scheduling (HAGS) which offloads GPU scheduling from the OS to the GPU hardware.".into(),
            why_it_helps: "Reduces input latency and can improve frame rates in GPU-bound games. Particularly beneficial for high-refresh-rate gaming.".into(),
            potential_risks: Some("Requires Windows 10 2004+ and compatible GPU driver. May cause instability on older systems. Revert if you experience crashes.".into()),
            how_to_revert: "Restores the original HwSchMode value from the snapshot.".into(),
        }
    }
}
