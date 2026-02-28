use std::io::{self, BufRead, Write};

use crate::clock_trait::Clock;
use crate::db::Database;
use crate::error::AppError;
use crate::formatting::format_duration;
use crate::services::task;

use super::TaskAction;

pub fn handle_task(db: &Database, clock: &dyn Clock, action: TaskAction) -> Result<(), AppError> {
    match action {
        TaskAction::Add {
            project,
            description,
            start,
            end,
            duration,
            date,
        } => {
            let entry = task::add_task(
                db,
                &project,
                &description,
                start.as_deref(),
                end.as_deref(),
                duration.as_deref(),
                date.as_deref(),
                clock,
            )?;
            println!(
                "Task logged: {} for '{}' on project '{}'.",
                format_duration(entry.duration_min),
                entry.description,
                project
            );
        }
        TaskAction::Edit {
            id,
            description,
            project,
            start,
            end,
            duration,
        } => {
            task::edit_task(
                db,
                id,
                description.as_deref(),
                project.as_deref(),
                start.as_deref(),
                end.as_deref(),
                duration.as_deref(),
                clock,
            )?;
            println!("Task {id} updated.");
        }
        TaskAction::Delete { id, yes } => {
            let entry = db
                .find_task_entry_by_id(id)?
                .ok_or_else(|| AppError::UserError(format!("Task with ID {id} not found.")))?;

            if !yes {
                eprint!(
                    "Delete task '{}' ({})? [y/N] ",
                    entry.description,
                    format_duration(entry.duration_min)
                );
                io::stderr().flush().ok();

                let stdin = io::stdin();
                let answer = stdin.lock().lines().next().transpose().ok().flatten();

                if answer.as_deref() != Some("y") && answer.as_deref() != Some("Y") {
                    println!("Cancelled.");
                    return Ok(());
                }
            }

            task::delete_task(db, id)?;
            println!("Task {id} deleted.");
        }
    }
    Ok(())
}
