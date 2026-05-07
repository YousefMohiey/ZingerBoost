use async_trait::async_trait;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use zb_application::audit_service::AuditService;
use zb_shared::types::{AuditEntry, AuditLevel};

/// SQLite-backed audit logger
#[derive(Debug)]
pub struct SqliteAuditLogger {
    conn: Mutex<Connection>,
}

impl SqliteAuditLogger {
    pub fn new(db_path: PathBuf) -> Result<Arc<dyn AuditService>, rusqlite::Error> {
        let conn = Connection::open(db_path)?;
        Ok(Arc::new(Self {
            conn: Mutex::new(conn),
        }))
    }

    pub fn new_in_memory() -> Result<Arc<dyn AuditService>, rusqlite::Error> {
        let conn = Connection::open_in_memory()?;
        Ok(Arc::new(Self {
            conn: Mutex::new(conn),
        }))
    }
}

#[async_trait]
impl AuditService for SqliteAuditLogger {
    async fn log(&self, entry: AuditEntry) {
        let details = entry.details.as_ref().map(|d| d.to_string());
        let level = match entry.level {
            AuditLevel::Info => "info",
            AuditLevel::Warn => "warn",
            AuditLevel::Error => "error",
            AuditLevel::Debug => "debug",
        };
        let _ = self.conn.lock().await.execute(
            "INSERT INTO audit_log (timestamp, level, category, message, details) VALUES (?1, ?2, ?3, ?4, ?5)",
            [
                entry.timestamp.to_rfc3339(),
                level.to_string(),
                entry.category,
                entry.message,
                details.unwrap_or_default(),
            ],
        );
    }

    async fn get_recent(&self, limit: usize) -> Vec<AuditEntry> {
        let conn = self.conn.lock().await;
        let mut stmt = match conn.prepare(
            "SELECT timestamp, level, category, message, details FROM audit_log ORDER BY timestamp DESC LIMIT ?1"
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let rows = match stmt.query_map([limit], |row| {
            let ts: String = row.get(0)?;
            let level_str: String = row.get(1)?;
            let category: String = row.get(2)?;
            let message: String = row.get(3)?;
            let details: Option<String> = row.get(4)?;

            let level = match level_str.as_str() {
                "warn" => AuditLevel::Warn,
                "error" => AuditLevel::Error,
                "debug" => AuditLevel::Debug,
                _ => AuditLevel::Info,
            };

            let timestamp = chrono::DateTime::parse_from_rfc3339(&ts)
                .unwrap_or_else(|_| chrono::DateTime::parse_from_rfc3339("1970-01-01T00:00:00Z").unwrap())
                .with_timezone(&chrono::Utc);

            let details = details.and_then(|d| serde_json::from_str(&d).ok());

            Ok(AuditEntry { timestamp, level, category, message, details })
        }) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };

        rows.filter_map(|r| r.ok()).collect()
    }
}
