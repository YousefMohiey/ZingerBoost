use async_trait::async_trait;
use zb_shared::types::{
    SnapshotData, TweakExplanation, TweakMetadata, TweakResult,
};

use crate::errors::TweakError;

/// Core trait for all tweaks in ZingerBoost.
#[async_trait]
pub trait Tweak: Send + Sync {
    /// Returns metadata describing this tweak
    fn metadata(&self) -> TweakMetadata;

    /// Checks whether this tweak is currently applied
    async fn is_applied(&self) -> Result<bool, TweakError>;

    /// Captures the current system state before applying
    async fn capture_state(&self) -> Result<SnapshotData, TweakError>;

    /// Applies the tweak
    async fn apply(&self) -> Result<TweakResult, TweakError>;

    /// Reverts the tweak using previously captured state
    async fn revert(&self, snapshot: &SnapshotData) -> Result<TweakResult, TweakError>;

    /// Returns a user-friendly explanation
    fn explain(&self) -> TweakExplanation;
}
