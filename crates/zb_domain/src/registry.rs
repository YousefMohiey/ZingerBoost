use crate::errors::RegistryError;
use async_trait::async_trait;
use std::sync::Arc;
use zb_shared::types::{RegPath, RegValue};

/// Abstract registry provider for testability
#[async_trait]
pub trait RegistryProvider: Send + Sync {
    async fn read(&self, path: &RegPath, name: &str) -> Result<RegValue, RegistryError>;
    async fn write(&self, path: &RegPath, name: &str, val: &RegValue) -> Result<(), RegistryError>;
    async fn delete(&self, path: &RegPath, name: &str) -> Result<(), RegistryError>;
}
