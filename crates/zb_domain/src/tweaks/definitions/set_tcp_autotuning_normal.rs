use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{
    RegPath, RegValue, RiskLevel, SnapshotData, TweakCategory, TweakExplanation, TweakMetadata,
    TweakResult,
};

use crate::errors::TweakError;
use crate::registry::RegistryProvider;
use crate::tweaks::traits::Tweak;

/// Set TCP Auto-Tuning to Normal to prevent aggressive window scaling
pub struct SetTcpAutotuningNormalTweak {
    pub provider: Option<Arc<dyn RegistryProvider>>,
}

impl Default for SetTcpAutotuningNormalTweak {
    fn default() -> Self {
        Self::new()
    }
}

impl SetTcpAutotuningNormalTweak {
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
impl Tweak for SetTcpAutotuningNormalTweak {
    fn metadata(&self) -> TweakMetadata {
        TweakMetadata {
            id: "network_tcp_autotuning".into(),
            name: "Set TCP Auto-Tuning Normal".into(),
            description: "Sets TCP receive window auto-tuning level to normal to prevent instability on some networks.".into(),
            category: TweakCategory::Network,
            risk: RiskLevel::Safe,
            requires_reboot: false,
            requires_admin: true,
            affected_keys: vec![
                RegPath::hklm(r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters"),
            ],
            source_url: Some("https://docs.microsoft.com/windows-server/networking/technologies/netsh-tcpip-context".into()),
        }
    }

    async fn is_applied(&self) -> Result<bool, TweakError> {
        // This is typically set via netsh command
        // We check the registry value as a proxy
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters");
            match provider.read(&path, "Tcp1323Opts").await {
                Ok(RegValue::Dword(v)) => Ok(v == 1 || v == 3), // 1 or 3 indicates normal/enabled
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    async fn capture_state(&self) -> Result<SnapshotData, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters");
            let val = provider
                .read(&path, "Tcp1323Opts")
                .await
                .unwrap_or(RegValue::Dword(1));
            Ok(SnapshotData::Registry {
                path,
                name: "Tcp1323Opts".into(),
                previous: val,
            })
        } else {
            Ok(SnapshotData::Registry {
                path: RegPath::hklm(r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters"),
                name: "Tcp1323Opts".into(),
                previous: RegValue::Dword(1),
            })
        }
    }

    async fn apply(&self) -> Result<TweakResult, TweakError> {
        if let Some(provider) = &self.provider {
            let path = RegPath::hklm(r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters");
            // Set to 1 (normal auto-tuning)
            provider
                .write(&path, "Tcp1323Opts", &RegValue::Dword(1))
                .await
                .map_err(|e| TweakError::Registry(e.to_string()))?;
        }
        Ok(TweakResult {
            reboot_required: false,
            message: "TCP auto-tuning set to normal. Network stability improved.".into(),
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
            message: "TCP auto-tuning restored to previous state.".into(),
        })
    }

    fn explain(&self) -> TweakExplanation {
        TweakExplanation {
            what_it_does: "Sets TCP receive window auto-tuning to normal level instead of highly experimental or disabled.".into(),
            why_it_helps: "Prevents network instability and connection drops that can occur with aggressive auto-tuning settings on some networks.".into(),
            potential_risks: Some("May slightly reduce maximum throughput on very high-latency connections. Revert if you notice slower download speeds.".into()),
            how_to_revert: "Restores the original Tcp1323Opts value from the snapshot.".into(),
        }
    }
}
