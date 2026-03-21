use crate::clock_trait::Clock;
use crate::db::Database;
use crate::error::AppError;
use crate::formatting;
use crate::models::TaskEntry;
use chrono::{NaiveDateTime, NaiveTime};

#[allow(clippy::too_many_arguments)]
pub fn add_task(
    db: &Database,
    project_name: &str,
    description: &str,
    start: Option<&str>,
    end: Option<&str>,
    duration: Option<&str>,
    date: Option<&str>,
    clock: &dyn Clock,
) -> Result<TaskEntry, AppError> {
    let project = super::resolve_project(db, project_name)?;
    let now = clock.now();

    // Resolve the task date: from --date flag or today
    let task_date = match date {
        Some(d) => formatting::parse_date(d).map_err(|e| AppError::UserError(e.to_string()))?,
        None => now.date(),
    };

    let (start_time, end_time, duration_min) =
        if let (Some(start_str), Some(end_str)) = (start, end) {
            let s = formatting::parse_time(start_str, task_date)
                .map_err(|e| AppError::UserError(e.to_string()))?;
            let e = formatting::parse_time(end_str, task_date)
                .map_err(|e| AppError::UserError(e.to_string()))?;
            if e <= s {
                return Err(AppError::UserError(
                    "End time must be after start time.".to_string(),
                ));
            }
            let dur = (e - s).num_minutes();
            (Some(s), Some(e), dur)
        } else if let Some(dur_str) = duration {
            let dur = formatting::parse_duration(dur_str)
                .map_err(|e| AppError::UserError(e.to_string()))?;
            // When --date is explicitly provided, anchor the task to midnight of that date
            // so journal queries (which use COALESCE(start_time, created_at)) find it on the right day.
            let anchor = date.map(|_| NaiveDateTime::new(task_date, NaiveTime::MIN));
            (anchor, None, dur)
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
    date: Option<&str>,
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

    // Resolve the date context for this edit
    let new_date = match date {
        Some(d) => Some(formatting::parse_date(d).map_err(|e| AppError::UserError(e.to_string()))?),
        None => None,
    };

    // The date to use when interpreting --start/--end times
    let time_date = new_date.unwrap_or_else(|| {
        existing
            .start_time
            .map(|t| t.date())
            .unwrap_or_else(|| clock.now().date())
    });

    let start_time: Option<Option<NaiveDateTime>> = if let Some(s) = start {
        Some(Some(
            formatting::parse_time(s, time_date).map_err(|e| AppError::UserError(e.to_string()))?,
        ))
    } else if let Some(d) = new_date {
        // --date only: move existing start time to new date preserving time-of-day,
        // or anchor duration-only tasks to midnight of the new date
        let anchor = existing
            .start_time
            .map(|t| NaiveDateTime::new(d, t.time()))
            .unwrap_or_else(|| NaiveDateTime::new(d, NaiveTime::MIN));
        Some(Some(anchor))
    } else {
        None
    };

    let end_time: Option<Option<NaiveDateTime>> = if let Some(e) = end {
        Some(Some(
            formatting::parse_time(e, time_date).map_err(|e| AppError::UserError(e.to_string()))?,
        ))
    } else if let Some(d) = new_date {
        // --date only: move existing end time to new date; leave unchanged if task had no end time
        existing
            .end_time
            .map(|t| Some(Some(NaiveDateTime::new(d, t.time()))))
            .unwrap_or(None)
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
