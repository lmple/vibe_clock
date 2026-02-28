use chrono::NaiveDateTime;

use crate::db::Database;
use crate::error::AppError;
use crate::models::Project;

pub fn create_project(db: &Database, name: &str, now: NaiveDateTime) -> Result<Project, AppError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(AppError::UserError(
            "Project name cannot be empty.".to_string(),
        ));
    }

    if db.find_project_by_name(name)?.is_some() {
        return Err(AppError::UserError(format!(
            "Project '{name}' already exists."
        )));
    }

    Ok(db.insert_project(name, now)?)
}

pub fn list_projects(db: &Database) -> Result<Vec<Project>, AppError> {
    Ok(db.list_projects()?)
}

pub fn rename_project(
    db: &Database,
    id: i64,
    new_name: &str,
    now: NaiveDateTime,
) -> Result<(), AppError> {
    let new_name = new_name.trim();
    if new_name.is_empty() {
        return Err(AppError::UserError(
            "Project name cannot be empty.".to_string(),
        ));
    }

    if db.find_project_by_id(id)?.is_none() {
        return Err(AppError::UserError(format!(
            "Project with ID {id} not found."
        )));
    }

    if let Some(existing) = db.find_project_by_name(new_name)? {
        if existing.id != id {
            return Err(AppError::UserError(format!(
                "Project '{new_name}' already exists."
            )));
        }
    }

    db.update_project_name(id, new_name, now)?;
    Ok(())
}

pub fn delete_project(db: &Database, id: i64, force: bool) -> Result<DeleteResult, AppError> {
    let project = db
        .find_project_by_id(id)?
        .ok_or_else(|| AppError::UserError(format!("Project with ID {id} not found.")))?;

    let task_count = db.count_tasks_for_project(id)?;

    if task_count > 0 && !force {
        return Ok(DeleteResult::NeedsConfirmation {
            name: project.name,
            task_count,
        });
    }

    // Stop running clock if it belongs to this project
    if let Some(clock_state) = db.get_clock_state()? {
        if clock_state.project_id == id {
            db.delete_clock_state()?;
        }
    }

    db.delete_project(id)?;
    Ok(DeleteResult::Deleted { name: project.name })
}

pub enum DeleteResult {
    Deleted { name: String },
    NeedsConfirmation { name: String, task_count: i64 },
}
