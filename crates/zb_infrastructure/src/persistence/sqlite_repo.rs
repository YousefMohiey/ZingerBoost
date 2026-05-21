use async_trait::async_trait;
use rusqlite::{Connection, OptionalExtension};
use rusqlite_migration::{Migrations, M};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use zb_application::snapshot_service::SnapshotService;
use zb_domain::errors::SnapshotError;
use zb_domain::snapshots::SystemSnapshot;
use zb_shared::types::SnapshotData;

fn migrations() -> Migrations<'static> {
    Migrations::new(vec![
        M::up(concat!(
            "CREATE TABLE IF NOT EXISTS snapshots (",
            "id TEXT PRIMARY KEY,",
            "created_at TEXT NOT NULL,",
            "description TEXT NOT NULL",
            ");",
            "CREATE TABLE IF NOT EXISTS snapshot_tweaks (",
            "id INTEGER PRIMARY KEY AUTOINCREMENT,",
            "snapshot_id TEXT NOT NULL REFERENCES snapshots(id) ON DELETE CASCADE,",
            "tweak_id TEXT NOT NULL,",
            "snapshot_data TEXT NOT NULL",
            ");",
            "CREATE TABLE IF NOT EXISTS tweak_states (",
            "tweak_id TEXT PRIMARY KEY,",
            "last_snapshot_id TEXT,",
            "snapshot_data TEXT,",
            "updated_at TEXT NOT NULL",
            ");",
            "CREATE INDEX idx_snapshot_tweaks_snapshot_id ON snapshot_tweaks(snapshot_id);",
            "CREATE INDEX idx_snapshot_tweaks_tweak_id ON snapshot_tweaks(tweak_id);",
        )),
        M::up(concat!(
            "CREATE TABLE IF NOT EXISTS audit_log (",
            "id INTEGER PRIMARY KEY AUTOINCREMENT,",
            "timestamp TEXT NOT NULL,",
            "level TEXT NOT NULL,",
            "category TEXT NOT NULL,",
            "message TEXT NOT NULL,",
            "details TEXT",
            ");",
            "CREATE INDEX idx_audit_timestamp ON audit_log(timestamp);",
            "CREATE INDEX idx_audit_category ON audit_log(category);",
        )),
    ])
}

/// Initialize the ZingerBoost database at LOCALAPPDATA
pub fn init_database() -> Result<Arc<Mutex<Connection>>, anyhow::Error> {
    let local_app_data = std::env::var("LOCALAPPDATA")
        .unwrap_or_else(|_| std::env::var("APPDATA").unwrap_or_else(|_| ".".into()));
    let dir = PathBuf::from(&local_app_data).join("ZingerBoost");
    std::fs::create_dir_all(&dir)?;
    let db_path = dir.join("data.db");

    let mut conn = Connection::open(&db_path)?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
    migrations().to_latest(&mut conn)?;

    Ok(Arc::new(Mutex::new(conn)))
}

/// SQLite-backed snapshot and audit repository
pub struct SqliteRepo {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteRepo {
    pub fn from_connection(conn: Arc<Mutex<Connection>>) -> Arc<dyn SnapshotService> {
        Arc::new(Self { conn })
    }

    pub fn new_in_memory() -> Result<Arc<dyn SnapshotService>, anyhow::Error> {
        let mut conn = Connection::open_in_memory()?;
        migrations().to_latest(&mut conn)?;
        Ok(Arc::new(Self {
            conn: Arc::new(Mutex::new(conn)),
        }))
    }

    pub fn conn(&self) -> Arc<Mutex<Connection>> {
        self.conn.clone()
    }
}

#[async_trait]
impl SnapshotService for SqliteRepo {
    async fn save_snapshot(&self, snapshot: SystemSnapshot) -> Result<(), SnapshotError> {
        let conn = self.conn.lock().await;
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        tx.execute(
            "INSERT INTO snapshots (id, created_at, description) VALUES (?1, ?2, ?3)",
            [
                snapshot.id.to_string(),
                snapshot.created_at.to_rfc3339(),
                snapshot.description.clone(),
            ],
        )
        .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        for record in &snapshot.tweak_records {
            let data = serde_json::to_string(&record.snapshot_data)
                .map_err(|e| SnapshotError::Storage(e.to_string()))?;
            tx.execute(
                "INSERT INTO snapshot_tweaks (snapshot_id, tweak_id, snapshot_data) VALUES (?1, ?2, ?3)",
                [snapshot.id.to_string(), record.tweak_id.clone(), data],
            ).map_err(|e| SnapshotError::Storage(e.to_string()))?;
        }

        tx.commit()
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        let _ = conn.execute(
            "DELETE FROM snapshots WHERE id NOT IN (
                SELECT id FROM snapshots ORDER BY created_at DESC LIMIT 50
            )",
            [],
        );

        Ok(())
    }

    async fn save_applied(&self, tweak_id: &str, data: SnapshotData) -> Result<(), SnapshotError> {
        let conn = self.conn.lock().await;
        let json =
            serde_json::to_string(&data).map_err(|e| SnapshotError::Storage(e.to_string()))?;
        conn.execute(
            "INSERT INTO tweak_states (tweak_id, snapshot_data, updated_at) VALUES (?1, ?2, ?3)
             ON CONFLICT(tweak_id) DO UPDATE SET snapshot_data = excluded.snapshot_data, updated_at = excluded.updated_at",
            [tweak_id, &json, &chrono::Utc::now().to_rfc3339()],
        ).map_err(|e| SnapshotError::Storage(e.to_string()))?;
        Ok(())
    }

    async fn get_last_snapshot_data(&self, tweak_id: &str) -> Result<SnapshotData, SnapshotError> {
        let conn = self.conn.lock().await;
        let json: Option<String> = conn
            .query_row(
                "SELECT snapshot_data FROM tweak_states WHERE tweak_id = ?1",
                [tweak_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        match json {
            Some(j) => serde_json::from_str(&j).map_err(|e| SnapshotError::Storage(e.to_string())),
            None => Err(SnapshotError::NotFound(format!(
                "No snapshot for tweak: {}",
                tweak_id
            ))),
        }
    }

    async fn list_snapshots(&self) -> Result<Vec<SystemSnapshot>, SnapshotError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn
            .prepare("SELECT id, created_at, description FROM snapshots ORDER BY created_at DESC")
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let created_at: String = row.get(1)?;
                let description: String = row.get(2)?;
                Ok((id, created_at, description))
            })
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        let mut snapshots = Vec::new();
        for row in rows {
            let (id, created_at, description) =
                row.map_err(|e| SnapshotError::Storage(e.to_string()))?;
            let uuid = Uuid::parse_str(&id).map_err(|e| SnapshotError::Storage(e.to_string()))?;
            let dt = chrono::DateTime::parse_from_rfc3339(&created_at)
                .map_err(|e| SnapshotError::Storage(e.to_string()))?
                .with_timezone(&chrono::Utc);

            let mut tweak_stmt = conn
                .prepare(
                    "SELECT tweak_id, snapshot_data FROM snapshot_tweaks WHERE snapshot_id = ?1",
                )
                .map_err(|e| SnapshotError::Storage(e.to_string()))?;
            let tweak_rows = tweak_stmt
                .query_map([&id], |row| {
                    let tweak_id: String = row.get(0)?;
                    let data: String = row.get(1)?;
                    Ok((tweak_id, data))
                })
                .map_err(|e| SnapshotError::Storage(e.to_string()))?;

            let mut records = Vec::new();
            for tr in tweak_rows {
                let (tid, data) = tr.map_err(|e| SnapshotError::Storage(e.to_string()))?;
                let snapshot_data: SnapshotData = serde_json::from_str(&data)
                    .map_err(|e| SnapshotError::Storage(e.to_string()))?;
                records.push(zb_domain::snapshots::AppliedTweakRecord {
                    tweak_id: tid,
                    snapshot_data,
                });
            }

            snapshots.push(SystemSnapshot {
                id: uuid,
                created_at: dt,
                description,
                tweak_records: records,
            });
        }

        Ok(snapshots)
    }

    async fn restore_snapshot(&self, id: &str) -> Result<(), SnapshotError> {
        let conn = self.conn.lock().await;

        // Check snapshot exists
        let exists: bool = conn
            .query_row(
                "SELECT COUNT(*) > 0 FROM snapshots WHERE id = ?1",
                [id],
                |row| row.get(0),
            )
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;
        if !exists {
            return Err(SnapshotError::NotFound(format!(
                "Snapshot {} not found",
                id
            )));
        }

        let mut stmt = conn
            .prepare("SELECT tweak_id, snapshot_data FROM snapshot_tweaks WHERE snapshot_id = ?1")
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;
        let rows: Vec<(String, String)> = stmt
            .query_map([id], |row| Ok((row.get(0)?, row.get(1)?)))
            .map_err(|e| SnapshotError::Storage(e.to_string()))?
            .filter_map(|r| r.ok())
            .collect();

        for (tweak_id, json) in rows {
            conn.execute(
                "INSERT INTO tweak_states (tweak_id, snapshot_data, updated_at)
                 VALUES (?1, ?2, ?3)
                 ON CONFLICT(tweak_id) DO UPDATE SET
                 snapshot_data = excluded.snapshot_data,
                 updated_at = excluded.updated_at",
                [&tweak_id, &json, &chrono::Utc::now().to_rfc3339()],
            )
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete_snapshot(&self, id: &str) -> Result<(), SnapshotError> {
        let conn = self.conn.lock().await;
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        tx.execute("DELETE FROM snapshot_tweaks WHERE snapshot_id = ?1", [id])
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        let deleted = tx
            .execute("DELETE FROM snapshots WHERE id = ?1", [id])
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        if deleted == 0 {
            return Err(SnapshotError::NotFound(format!(
                "Snapshot {} not found",
                id
            )));
        }

        tx.commit()
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        Ok(())
    }

    async fn clear_snapshots(&self) -> Result<(), SnapshotError> {
        let conn = self.conn.lock().await;
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        tx.execute("DELETE FROM snapshot_tweaks", [])
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        tx.execute("DELETE FROM snapshots", [])
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        tx.commit()
            .map_err(|e| SnapshotError::Storage(e.to_string()))?;

        Ok(())
    }

    async fn clear_tweak_state(&self, tweak_id: &str) -> Result<(), SnapshotError> {
        let conn = self.conn.lock().await;
        conn.execute(
            "DELETE FROM tweak_states WHERE tweak_id = ?1",
            [tweak_id],
        )
        .map_err(|e| SnapshotError::Storage(e.to_string()))?;
        Ok(())
    }
}
