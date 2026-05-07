use async_trait::async_trait;
use zb_shared::types::AuditEntry;

#[async_trait]
pub trait AuditService: Send + Sync {
    async fn log(&self, entry: AuditEntry);
    async fn get_recent(&self, limit: usize) -> Vec<AuditEntry>;
}
