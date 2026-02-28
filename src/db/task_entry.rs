use anyhow::Result;
use chrono::NaiveDateTime;
use rusqlite::params;

use crate::models::TaskEntry;

use super::{Database, parse_datetime, parse_optional_datetime};

impl Database {
    pub fn insert_task_entry(
        &self,
        project_id: i64,
        description: &str,
        start_time: Option<NaiveDateTime>,
        end_time: Option<NaiveDateTime>,
        duration_min: i64,
        now: NaiveDateTime,
    ) -> Result<TaskEntry> {
        let start_str = start_time.map(|t| t.format("%Y-%m-%dT%H:%M:%S").to_string());
        let end_str = end_time.map(|t| t.format("%Y-%m-%dT%H:%M:%S").to_string());
        let now_str = now.format("%Y-%m-%dT%H:%M:%S").to_string();

        self.conn.execute(
            "INSERT INTO task_entry (project_id, description, start_time, end_time, duration_min, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![project_id, description, start_str, end_str, duration_min, now_str, now_str],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(TaskEntry {
            id,
            project_id,
            description: description.to_string(),
            start_time,
            end_time,
            duration_min,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn find_task_entry_by_id(&self, id: i64) -> Result<Option<TaskEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, description, start_time, end_time, duration_min, created_at, updated_at \
             FROM task_entry WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
            ))
        })?;
        match rows.next() {
            Some(r) => {
                let (
                    id,
                    project_id,
                    description,
                    start_time,
                    end_time,
                    duration_min,
                    created_at,
                    updated_at,
                ) = r?;
                Ok(Some(TaskEntry {
                    id,
                    project_id,
                    description,
                    start_time: parse_optional_datetime(start_time.as_deref())?,
                    end_time: parse_optional_datetime(end_time.as_deref())?,
                    duration_min,
                    created_at: parse_datetime(&created_at)?,
                    updated_at: parse_datetime(&updated_at)?,
                }))
            }
            None => Ok(None),
        }
    }

    pub fn list_tasks_for_date(&self, date: &str) -> Result<Vec<TaskEntry>> {
        // Match tasks where start_time date matches, or created_at date matches (for duration-only entries)
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, description, start_time, end_time, duration_min, created_at, updated_at \
             FROM task_entry \
             WHERE substr(COALESCE(start_time, created_at), 1, 10) = ?1 \
             ORDER BY COALESCE(start_time, created_at)",
        )?;
        let tasks = stmt
            .query_map(params![date], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, String>(7)?,
                ))
            })?
            .map(|r| {
                let (
                    id,
                    project_id,
                    description,
                    start_time,
                    end_time,
                    duration_min,
                    created_at,
                    updated_at,
                ) = r?;
                Ok(TaskEntry {
                    id,
                    project_id,
                    description,
                    start_time: parse_optional_datetime(start_time.as_deref())?,
                    end_time: parse_optional_datetime(end_time.as_deref())?,
                    duration_min,
                    created_at: parse_datetime(&created_at)?,
                    updated_at: parse_datetime(&updated_at)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(tasks)
    }

    pub fn list_tasks_for_date_range(
        &self,
        from_date: &str,
        to_date: &str,
    ) -> Result<Vec<TaskEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, description, start_time, end_time, duration_min, created_at, updated_at \
             FROM task_entry \
             WHERE substr(COALESCE(start_time, created_at), 1, 10) BETWEEN ?1 AND ?2 \
             ORDER BY COALESCE(start_time, created_at)",
        )?;
        let tasks = stmt
            .query_map(params![from_date, to_date], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, String>(7)?,
                ))
            })?
            .map(|r| {
                let (
                    id,
                    project_id,
                    description,
                    start_time,
                    end_time,
                    duration_min,
                    created_at,
                    updated_at,
                ) = r?;
                Ok(TaskEntry {
                    id,
                    project_id,
                    description,
                    start_time: parse_optional_datetime(start_time.as_deref())?,
                    end_time: parse_optional_datetime(end_time.as_deref())?,
                    duration_min,
                    created_at: parse_datetime(&created_at)?,
                    updated_at: parse_datetime(&updated_at)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(tasks)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_task_entry(
        &self,
        id: i64,
        description: Option<&str>,
        project_id: Option<i64>,
        start_time: Option<Option<NaiveDateTime>>,
        end_time: Option<Option<NaiveDateTime>>,
        duration_min: Option<i64>,
        now: NaiveDateTime,
    ) -> Result<bool> {
        let now_str = now.format("%Y-%m-%dT%H:%M:%S").to_string();

        // Read current values, apply changes, write back
        let current = self.find_task_entry_by_id(id)?;
        let Some(current) = current else {
            return Ok(false);
        };

        let desc = description.unwrap_or(&current.description);
        let proj = project_id.unwrap_or(current.project_id);
        let start = start_time.unwrap_or(current.start_time);
        let end = end_time.unwrap_or(current.end_time);
        let dur = duration_min.unwrap_or(current.duration_min);

        let start_str = start.map(|t| t.format("%Y-%m-%dT%H:%M:%S").to_string());
        let end_str = end.map(|t| t.format("%Y-%m-%dT%H:%M:%S").to_string());

        let rows = self.conn.execute(
            "UPDATE task_entry SET description = ?1, project_id = ?2, start_time = ?3, end_time = ?4, duration_min = ?5, updated_at = ?6 WHERE id = ?7",
            params![desc, proj, start_str, end_str, dur, now_str, id],
        )?;
        Ok(rows > 0)
    }

    pub fn delete_task_entry(&self, id: i64) -> Result<bool> {
        let rows = self
            .conn
            .execute("DELETE FROM task_entry WHERE id = ?1", params![id])?;
        Ok(rows > 0)
    }
}
