use serde::{Deserialize, Serialize};
use zb_shared::types::{
    AppErrorDto, AuditEntry, SystemMetrics, TweakExplanation, TweakMetadata, TweakResult,
};

/// DTO for tweak list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweakListDto {
    pub tweaks: Vec<TweakMetadata>,
}

/// DTO for apply request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyRequestDto {
    pub id: String,
}

/// DTO for batch apply request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchApplyRequestDto {
    pub ids: Vec<String>,
}

/// DTO for tweak result response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweakResultDto {
    pub reboot_required: bool,
    pub message: String,
}

impl From<TweakResult> for TweakResultDto {
    fn from(value: TweakResult) -> Self {
        Self {
            reboot_required: value.reboot_required,
            message: value.message,
        }
    }
}

/// DTO for system metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetricsDto {
    pub cpu_percent: f64,
    pub ram_percent: f64,
    pub ram_used_mb: u64,
    pub ram_total_mb: u64,
    pub disk_active_percent: f64,
    pub network_down_mbps: f64,
    pub network_up_mbps: f64,
}

impl From<SystemMetrics> for SystemMetricsDto {
    fn from(value: SystemMetrics) -> Self {
        Self {
            cpu_percent: value.cpu_percent,
            ram_percent: value.ram_percent,
            ram_used_mb: value.ram_used_mb,
            ram_total_mb: value.ram_total_mb,
            disk_active_percent: value.disk_active_percent,
            network_down_mbps: value.network_down_mbps,
            network_up_mbps: value.network_up_mbps,
        }
    }
}

/// DTO for audit log response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogDto {
    pub entries: Vec<AuditEntry>,
}

/// DTO for tweak explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweakExplanationDto {
    pub what_it_does: String,
    pub why_it_helps: String,
    pub potential_risks: Option<String>,
    pub how_to_revert: String,
}

impl From<TweakExplanation> for TweakExplanationDto {
    fn from(value: TweakExplanation) -> Self {
        Self {
            what_it_does: value.what_it_does,
            why_it_helps: value.why_it_helps,
            potential_risks: value.potential_risks,
            how_to_revert: value.how_to_revert,
        }
    }
}

/// Error mapping
impl From<zb_domain::errors::TweakError> for AppErrorDto {
    fn from(e: zb_domain::errors::TweakError) -> Self {
        AppErrorDto {
            code: "TWEAK_ERROR".into(),
            message: e.to_string(),
            details: None,
        }
    }
}

impl From<zb_domain::errors::SnapshotError> for AppErrorDto {
    fn from(e: zb_domain::errors::SnapshotError) -> Self {
        AppErrorDto {
            code: "SNAPSHOT_ERROR".into(),
            message: e.to_string(),
            details: None,
        }
    }
}

impl From<anyhow::Error> for AppErrorDto {
    fn from(e: anyhow::Error) -> Self {
        AppErrorDto {
            code: "INTERNAL_ERROR".into(),
            message: e.to_string(),
            details: None,
        }
    }
}
