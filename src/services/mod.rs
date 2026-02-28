pub mod clock;
pub mod journal;
pub mod project;
pub mod report;
pub mod task;

use crate::db::Database;
use crate::error::AppError;

/// Resolve a project by name or ID.
pub fn resolve_project(
    db: &Database,
    name_or_id: &str,
) -> Result<crate::models::Project, AppError> {
    // Try as ID first
    if let Ok(id) = name_or_id.parse::<i64>() {
        if let Some(project) = db.find_project_by_id(id)? {
            return Ok(project);
        }
    }

    // Try as name
    if let Some(project) = db.find_project_by_name(name_or_id)? {
        return Ok(project);
    }

    Err(AppError::UserError(format!(
        "Project '{name_or_id}' not found."
    )))
}
