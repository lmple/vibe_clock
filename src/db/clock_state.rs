use anyhow::Result;
use chrono::NaiveDateTime;
use rusqlite::params;

use crate::models::ClockState;

use super::{Database, parse_datetime};

impl Database {
    pub fn insert_clock_state(
        &self,
        project_id: i64,
        description: &str,
        start_time: NaiveDateTime,
    ) -> Result<()> {
        let start_str = start_time.format("%Y-%m-%dT%H:%M:%S").to_string();
        self.conn.execute(
            "INSERT INTO clock_state (id, project_id, description, start_time) VALUES (1, ?1, ?2, ?3)",
            params![project_id, description, start_str],
        )?;
        Ok(())
    }

    pub fn get_clock_state(&self) -> Result<Option<ClockState>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, description, start_time FROM clock_state WHERE id = 1",
        )?;
        let mut rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?;
        match rows.next() {
            Some(r) => {
                let (id, project_id, description, start_time) = r?;
                Ok(Some(ClockState {
                    id,
                    project_id,
                    description,
                    start_time: parse_datetime(&start_time)?,
                }))
            }
            None => Ok(None),
        }
    }

    pub fn delete_clock_state(&self) -> Result<()> {
        self.conn
            .execute("DELETE FROM clock_state WHERE id = 1", [])?;
        Ok(())
    }
}
