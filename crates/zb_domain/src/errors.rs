use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum TweakError {
    #[error("Registry error: {0}")]
    Registry(String),

    #[error("Access denied — try running as Administrator")]
    AccessDenied,

    #[error("Tweak is already applied")]
    AlreadyApplied,

    #[error("Tweak is not applied")]
    NotApplied,

    #[error("Snapshot data missing or corrupted")]
    SnapshotMissing,

    #[error("Windows API error: {0}")]
    WinApi(String),

    #[error("Service error: {0}")]
    Service(String),

    #[error("Power plan error: {0}")]
    PowerPlan(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unknown tweak error: {0}")]
    Unknown(String),
}

#[derive(Error, Debug, Clone)]
pub enum SnapshotError {
    #[error("Failed to create snapshot: {0}")]
    CreateFailed(String),

    #[error("Failed to restore snapshot: {0}")]
    RestoreFailed(String),

    #[error("Snapshot not found: {0}")]
    NotFound(String),

    #[error("Storage error: {0}")]
    Storage(String),
}

#[derive(Error, Debug, Clone)]
pub enum RegistryError {
    #[error("Failed to read registry: {0}")]
    ReadFailed(String),

    #[error("Failed to write registry: {0}")]
    WriteFailed(String),

    #[error("Failed to delete registry value: {0}")]
    DeleteFailed(String),

    #[error("Invalid registry path: {0}")]
    InvalidPath(String),

    #[error("Access denied to registry key")]
    AccessDenied,

    #[error("Key not found")]
    KeyNotFound,

    #[error("Value not found")]
    ValueNotFound,
}

#[derive(Error, Debug, Clone)]
pub enum ServiceError {
    #[error("Failed to open service manager")]
    OpenManagerFailed,

    #[error("Failed to open service: {0}")]
    OpenServiceFailed(String),

    #[error("Failed to query service config: {0}")]
    QueryConfigFailed(String),

    #[error("Failed to change service config: {0}")]
    ChangeConfigFailed(String),

    #[error("Access denied")]
    AccessDenied,
}

#[derive(Error, Debug, Clone)]
pub enum BenchmarkError {
    #[error("Benchmark failed: {0}")]
    RunFailed(String),

    #[error("Benchmark cancelled")]
    Cancelled,
}
