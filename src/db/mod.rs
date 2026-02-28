mod clock_state;
mod project;
mod task_entry;

use std::path::Path;

use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use rusqlite::Connection;

pub(crate) fn parse_datetime(s: &str) -> Result<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
        .with_context(|| format!("Invalid datetime in database: '{s}'"))
}

pub(crate) fn parse_optional_datetime(s: Option<&str>) -> Result<Option<NaiveDateTime>> {
    match s {
        Some(s) => Ok(Some(parse_datetime(s)?)),
        None => Ok(None),
    }
}

pub struct Database {
    pub conn: Connection,
}

impl Database {
    /// Open an encrypted database at the given path with the provided passphrase.
    pub fn open(path: &Path, passphrase: &str) -> Result<Self> {
        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open database: {}", path.display()))?;

        // Apply encryption key
        conn.pragma_update(None, "key", passphrase)?;

        Self::configure(&conn)?;
        let db = Self { conn };
        db.migrate()?;
        Ok(db)
    }

    /// Open an unencrypted database (for testing).
    pub fn open_unencrypted(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)
            .with_context(|| format!("Failed to open database: {}", path.display()))?;

        Self::configure(&conn)?;
        let db = Self { conn };
        db.migrate()?;
        Ok(db)
    }

    fn configure(conn: &Connection) -> Result<()> {
        conn.pragma_update(None, "journal_mode", "DELETE")?;
        conn.pragma_update(None, "locking_mode", "EXCLUSIVE")?;
        conn.pragma_update(None, "synchronous", "FULL")?;
        conn.pragma_update(None, "busy_timeout", 5000)?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        Ok(())
    }

    fn migrate(&self) -> Result<()> {
        self.conn
            .execute_batch(SCHEMA_V1)
            .context("Failed to initialize database schema")?;
        Ok(())
    }
}

const SCHEMA_V1: &str = "
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY
);

INSERT OR IGNORE INTO schema_version (version) VALUES (1);

CREATE TABLE IF NOT EXISTS project (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS task_entry (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    start_time TEXT,
    end_time TEXT,
    duration_min INTEGER NOT NULL CHECK(duration_min > 0),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS clock_state (
    id INTEGER PRIMARY KEY CHECK(id = 1),
    project_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    start_time TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES project(id)
);

CREATE INDEX IF NOT EXISTS idx_task_entry_project_id ON task_entry(project_id);
CREATE INDEX IF NOT EXISTS idx_task_entry_start_time ON task_entry(start_time);
";
