use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use zb_application::audit_service::AuditService;
use zb_shared::types::{AuditEntry, AuditLevel};

/// Simple in-memory audit logger (replace with SQLite in production)
#[derive(Debug)]
pub struct InMemoryAuditLogger {
    entries: Mutex<Vec<AuditEntry>>,
}

impl InMemoryAuditLogger {
    pub fn new() -> Arc<dyn AuditService> {
        Arc::new(Self {
            entries: Mutex::new(Vec::new()),
        })
    }
}

#[async_trait]
impl AuditService for InMemoryAuditLogger {
    async fn log(&self, entry: AuditEntry) {
        let mut entries = self.entries.lock().await;
        entries.push(entry);
    }

    async fn get_recent(&self, limit: usize) -> Vec<AuditEntry> {
        let entries = self.entries.lock().await;
        entries.iter().rev().take(limit).cloned().collect()
    }
}
