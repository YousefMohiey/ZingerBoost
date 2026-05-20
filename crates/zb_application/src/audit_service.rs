use async_trait::async_trait;
use zb_shared::types::AuditEntry;

#[async_trait]
pub trait AuditService: Send + Sync {
    async fn log(&self, entry: AuditEntry);
    async fn get_recent(&self, limit: usize) -> Vec<AuditEntry>;
    async fn get_recent_raw(
        &self,
        _limit: usize,
    ) -> Result<Vec<(i64, String, String, String, String, Option<String>)>, String> {
        Err("Raw audit log access not supported".to_string())
    }
    async fn clear(&self) -> Result<String, String> {
        Err("Audit log clear not supported".to_string())
    }
}
