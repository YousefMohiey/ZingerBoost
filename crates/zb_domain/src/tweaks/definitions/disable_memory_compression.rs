use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Disable Memory Compression to reduce stutter in RAM-heavy games
pub struct DisableMemoryCompressionTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for DisableMemoryCompressionTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl DisableMemoryCompressionTweak {
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
impl Tweak for DisableMemoryCompressionTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "perf_disable_mem_compression".into(),
            name: "Disable Memory Compression".into(),
            description:
                "Disables Windows memory compression to reduce stutter in RAM-heavy games.".into(),
            category: TweakCategory::Performance,
            risk: RiskLevel::Moderate,
            requires_reboot: true,
            requires_admin: true,
            affected_keys: vec![RegPath::hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            )],
            source_url: Some(
                "https://docs.microsoft.com/windows/win32/memory/memory-compression".into(),
            ),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            );
            match provider.read(&path, "DisableCompression").await {
                Ok(RegValue::Dword(v)) => Ok(v == 1),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            );
            let val = provider
                .read(&path, "DisableCompression")
                .await
                .unwrap_or(RegValue::Dword(0)); // 0 = compression enabled (default)
            Ok(SnapshotData::Registry {
                path,
                name: "DisableCompression".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(
                    r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                ),
                name: "DisableCompression".into(),
                previous: RegValue::Dword(0),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            );
            provider
                .write(&path, "DisableCompression", &RegValue::Dword(1))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: true,
            message: "Memory compression disabled. May reduce stutter in RAM-heavy games. Reboot required.".into(),
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
            message: "Memory compression restored to previous state. Reboot required.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Disables Windows memory compression feature that compresses inactive memory pages to save RAM.".into(),
            why_it_helps: "Reduces CPU overhead from compression/decompression operations, which can cause stutter in RAM-heavy games. Improves memory access latency.".into(),
            potential_risks: Some("Increases RAM usage. May cause issues on systems with less than 16GB RAM. Revert if you experience out-of-memory errors.".into()),
            how_to_revert: "Restores the original DisableCompression value from the snapshot.".into(),
        }
    }
}
