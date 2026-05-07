use async_trait::async_trait;
use std::sync::Arc;
use zb_domain::errors::RegistryError;
use zb_shared::types::{RegPath, RegValue};

/// Abstract registry provider for testability
#[async_trait]
pub trait RegistryProvider: Send + Sync {
    async fn read(&self, path: &RegPath, name: &str) -> Result<RegValue, RegistryError>;
    async fn write(&self, path: &RegPath, name: &str, val: &RegValue) -> Result<(), RegistryError>;
    async fn delete(&self, path: &RegPath, name: &str) -> Result<(), RegistryError>;
}

/// Windows registry provider using windows-rs
#[derive(Debug)]
pub struct WinRegistryProvider;

impl WinRegistryProvider {
    pub fn new() -> Arc<dyn RegistryProvider> {
        Arc::new(Self)
    }
}

#[async_trait]
impl RegistryProvider for WinRegistryProvider {
    async fn read(&self, _path: &RegPath, _name: &str) -> Result<RegValue, RegistryError> {
        // Placeholder: real implementation uses windows-rs Registry APIs
        Ok(RegValue::Dword(0))
    }

    async fn write(&self, _path: &RegPath, _name: &str, _val: &RegValue) -> Result<(), RegistryError> {
        // Placeholder
        Ok(())
    }

    async fn delete(&self, _path: &RegPath, _name: &str) -> Result<(), RegistryError> {
        // Placeholder
        Ok(())
    }
}
