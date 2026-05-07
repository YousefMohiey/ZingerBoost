use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for tweaks, snapshots, etc.
pub type Id = String;

/// Registry path abstraction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegRoot {
    Hkcr,
    Hkcu,
    Hklm,
    Hku,
    Hkcc,
}

impl fmt::Display for RegRoot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegRoot::Hkcr => write!(f, "HKEY_CLASSES_ROOT"),
            RegRoot::Hkcu => write!(f, "HKEY_CURRENT_USER"),
            RegRoot::Hklm => write!(f, "HKEY_LOCAL_MACHINE"),
            RegRoot::Hku => write!(f, "HKEY_USERS"),
            RegRoot::Hkcc => write!(f, "HKEY_CURRENT_CONFIG"),
        }
    }
}

/// A validated registry path
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegPath {
    pub root: RegRoot,
    pub path: String,
}

impl RegPath {
    pub fn new(root: RegRoot, path: impl Into<String>) -> Self {
        Self {
            root,
            path: path.into(),
        }
    }

    pub fn hkcu(path: impl Into<String>) -> Self {
        Self::new(RegRoot::Hkcu, path)
    }

    pub fn hklm(path: impl Into<String>) -> Self {
        Self::new(RegRoot::Hklm, path)
    }
}

/// Typed registry value to prevent type mismatches
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegValue {
    Dword(u32),
    Qword(u64),
    Sz(String),
    ExpandSz(String),
    Binary(Vec<u8>),
    /// Represents a value that does not exist
    Absent,
}

/// Risk level for tweaks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Safe,
    Moderate,
    Advanced,
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiskLevel::Safe => write!(f, "safe"),
            RiskLevel::Moderate => write!(f, "moderate"),
            RiskLevel::Advanced => write!(f, "advanced"),
        }
    }
}

/// Tweak category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TweakCategory {
    Visual,
    Privacy,
    Performance,
    Gaming,
    Debloat,
    Network,
    Startup,
}

impl fmt::Display for TweakCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TweakCategory::Visual => write!(f, "visual"),
            TweakCategory::Privacy => write!(f, "privacy"),
            TweakCategory::Performance => write!(f, "performance"),
            TweakCategory::Gaming => write!(f, "gaming"),
            TweakCategory::Debloat => write!(f, "debloat"),
            TweakCategory::Network => write!(f, "network"),
            TweakCategory::Startup => write!(f, "startup"),
        }
    }
}

/// Metadata describing a tweak to the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweakMetadata {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub category: TweakCategory,
    pub risk: RiskLevel,
    pub requires_reboot: bool,
    pub requires_admin: bool,
    pub affected_keys: Vec<RegPath>,
    pub source_url: Option<String>,
}

/// Result of applying or reverting a tweak
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweakResult {
    pub reboot_required: bool,
    pub message: String,
}

/// Explanation shown to users before applying a tweak
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweakExplanation {
    pub what_it_does: String,
    pub why_it_helps: String,
    pub potential_risks: Option<String>,
    pub how_to_revert: String,
}

/// Snapshot data captured before applying a tweak
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnapshotData {
    Registry { path: RegPath, name: String, previous: RegValue },
    Service { name: String, previous_start_type: u32 },
    PowerPlan { previous_guid: String },
    UwpApp { package_family_name: String },
    Other(String),
}

/// System metrics for live dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_percent: f64,
    pub ram_percent: f64,
    pub ram_used_mb: u64,
    pub ram_total_mb: u64,
    pub disk_active_percent: f64,
    pub network_down_mbps: f64,
    pub network_up_mbps: f64,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: AuditLevel,
    pub category: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditLevel {
    Info,
    Warn,
    Error,
    Debug,
}

/// DTO for frontend-friendly errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppErrorDto {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}
