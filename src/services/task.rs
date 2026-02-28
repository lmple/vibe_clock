use crate::clock_trait::Clock;
use crate::db::Database;
use crate::error::AppError;
use crate::formatting;
use crate::models::TaskEntry;

#[allow(clippy::too_many_arguments)]
pub fn add_task(
    db: &Database,
    project_name: &str,
    description: &str,
    start: Option<&str>,
    end: Option<&str>,
    duration: Option<&str>,
    _date: Option<&str>,
    clock: &dyn Clock,
) -> Result<TaskEntry, AppError> {
    let project = super::resolve_project(db, project_name)?;
    let now = clock.now();

    let (start_time, end_time, duration_min) = if let (Some(start_str), Some(end_str)) =
        (start, end)
    {
        let s =
            formatting::parse_time(start_str).map_err(|e| AppError::UserError(e.to_string()))?;
        let e = formatting::parse_time(end_str).map_err(|e| AppError::UserError(e.to_string()))?;
        if e <= s {
            return Err(AppError::UserError(
                "End time must be after start time.".to_string(),
            ));
        }
        let dur = (e - s).num_minutes();
        (Some(s), Some(e), dur)
    } else if let Some(dur_str) = duration {
        let dur =
            formatting::parse_duration(dur_str).map_err(|e| AppError::UserError(e.to_string()))?;
        (None, None, dur)
    } else {
        return Err(AppError::UserError(
            "Provide either --start/--end or --duration.".to_string(),
        ));
    };

    let task = db.insert_task_entry(
        project.id,
        description,
        start_time,
        end_time,
        duration_min,
        now,
    )?;

    Ok(task)
}

#[allow(clippy::too_many_arguments)]
pub fn edit_task(
    db: &Database,
    id: i64,
    description: Option<&str>,
    project_name: Option<&str>,
    start: Option<&str>,
    end: Option<&str>,
    duration: Option<&str>,
    clock: &dyn Clock,
) -> Result<(), AppError> {
    let existing = db
        .find_task_entry_by_id(id)?
        .ok_or_else(|| AppError::UserError(format!("Task with ID {id} not found.")))?;

    let project_id = if let Some(name) = project_name {
        Some(super::resolve_project(db, name)?.id)
    } else {
        None
    };

    let start_time = if let Some(s) = start {
        Some(Some(
            formatting::parse_time(s).map_err(|e| AppError::UserError(e.to_string()))?,
        ))
    } else {
        None
    };

    let end_time = if let Some(e) = end {
        Some(Some(
            formatting::parse_time(e).map_err(|e| AppError::UserError(e.to_string()))?,
        ))
    } else {
        None
    };

    let duration_min = if let Some(d) = duration {
        Some(formatting::parse_duration(d).map_err(|e| AppError::UserError(e.to_string()))?)
    } else if start_time.is_some() || end_time.is_some() {
        // Recalculate duration if times changed
        let s = start_time
            .unwrap_or(existing.start_time)
            .or(existing.start_time);
        let e = end_time.unwrap_or(existing.end_time).or(existing.end_time);
        if let (Some(s), Some(e)) = (s, e) {
            if e <= s {
                return Err(AppError::UserError(
                    "End time must be after start time.".to_string(),
                ));
            }
            Some((e - s).num_minutes())
        } else {
            None
        }
    } else {
        None
    };

    let now = clock.now();
    db.update_task_entry(
        id,
        description,
        project_id,
        start_time,
        end_time,
        duration_min,
        now,
    )?;
    Ok(())
}

pub fn delete_task(db: &Database, id: i64) -> Result<(String, i64), AppError> {
    let task = db
        .find_task_entry_by_id(id)?
        .ok_or_else(|| AppError::UserError(format!("Task with ID {id} not found.")))?;

    db.delete_task_entry(id)?;
    Ok((task.description, task.duration_min))
}
