//! Core constants used across ZingerBoost.

/// App name for directories, registry, etc.
pub const APP_NAME: &str = "ZingerBoost";

/// App version from workspace
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Data directory relative to %LOCALAPPDATA%
pub const DATA_DIR: &str = "ZingerBoost";

/// Snapshot subdirectory
pub const SNAPSHOTS_DIR: &str = "snapshots";

/// Log subdirectory
pub const LOGS_DIR: &str = "logs";

/// Database file name
pub const DB_FILE: &str = "data.db";

/// Max snapshot retention count
pub const MAX_SNAPSHOT_RETENTION: usize = 50;

/// Audit log retention days
pub const AUDIT_LOG_RETENTION_DAYS: i64 = 7;

/// Risk levels
pub mod risk {
    pub const SAFE: &str = "safe";
    pub const MODERATE: &str = "moderate";
    pub const ADVANCED: &str = "advanced";
}

/// Tweak categories
pub mod category {
    pub const VISUAL: &str = "visual";
    pub const PRIVACY: &str = "privacy";
    pub const PERFORMANCE: &str = "performance";
    pub const GAMING: &str = "gaming";
    pub const DEBLOAT: &str = "debloat";
    pub const NETWORK: &str = "network";
    pub const STARTUP: &str = "startup";
}
