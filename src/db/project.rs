use anyhow::Result;
use chrono::NaiveDateTime;
use rusqlite::params;

use crate::models::Project;

use super::{Database, parse_datetime};

impl Database {
    pub fn insert_project(&self, name: &str, now: NaiveDateTime) -> Result<Project> {
        let now_str = now.format("%Y-%m-%dT%H:%M:%S").to_string();
        self.conn.execute(
            "INSERT INTO project (name, created_at, updated_at) VALUES (?1, ?2, ?3)",
            params![name, now_str, now_str],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(Project {
            id,
            name: name.to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn list_projects(&self) -> Result<Vec<Project>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, created_at, updated_at FROM project ORDER BY name")?;
        let projects = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })?
            .map(|r| {
                let (id, name, created_at, updated_at) = r?;
                Ok(Project {
                    id,
                    name,
                    created_at: parse_datetime(&created_at)?,
                    updated_at: parse_datetime(&updated_at)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(projects)
    }

    pub fn find_project_by_id(&self, id: i64) -> Result<Option<Project>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, created_at, updated_at FROM project WHERE id = ?1")?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?;
        match rows.next() {
            Some(r) => {
                let (id, name, created_at, updated_at) = r?;
                Ok(Some(Project {
                    id,
                    name,
                    created_at: parse_datetime(&created_at)?,
                    updated_at: parse_datetime(&updated_at)?,
                }))
            }
            None => Ok(None),
        }
    }

    pub fn find_project_by_name(&self, name: &str) -> Result<Option<Project>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, created_at, updated_at FROM project WHERE name = ?1")?;
        let mut rows = stmt.query_map(params![name], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })?;
        match rows.next() {
            Some(r) => {
                let (id, name, created_at, updated_at) = r?;
                Ok(Some(Project {
                    id,
                    name,
                    created_at: parse_datetime(&created_at)?,
                    updated_at: parse_datetime(&updated_at)?,
                }))
            }
            None => Ok(None),
        }
    }

    pub fn update_project_name(&self, id: i64, new_name: &str, now: NaiveDateTime) -> Result<bool> {
        let now_str = now.format("%Y-%m-%dT%H:%M:%S").to_string();
        let rows = self.conn.execute(
            "UPDATE project SET name = ?1, updated_at = ?2 WHERE id = ?3",
            params![new_name, now_str, id],
        )?;
        Ok(rows > 0)
    }

    pub fn delete_project(&self, id: i64) -> Result<bool> {
        let rows = self
            .conn
            .execute("DELETE FROM project WHERE id = ?1", params![id])?;
        Ok(rows > 0)
    }

    pub fn count_tasks_for_project(&self, project_id: i64) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM task_entry WHERE project_id = ?1",
            params![project_id],
            |row| row.get(0),
        )?;
        Ok(count)
    }
}
