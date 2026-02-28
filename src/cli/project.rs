use std::io::{self, BufRead, Write};

use crate::clock_trait::Clock;
use crate::db::Database;
use crate::error::AppError;
use crate::services::project::{self, DeleteResult};

use super::ProjectAction;

pub fn handle_project(
    db: &Database,
    clock: &dyn Clock,
    action: ProjectAction,
) -> Result<(), AppError> {
    match action {
        ProjectAction::Add { name } => {
            let project = project::create_project(db, &name, clock.now())?;
            println!("Project '{}' created.", project.name);
        }
        ProjectAction::List => {
            let projects = project::list_projects(db)?;
            if projects.is_empty() {
                println!("No projects found. Create one with: vibe-clock project add <name>");
            } else {
                println!(
                    "{:<6} {:<20} {:<8} {:<20}",
                    "ID", "Name", "Tasks", "Created"
                );
                println!("{}", "-".repeat(56));
                for p in &projects {
                    let task_count = db.count_tasks_for_project(p.id)?;
                    println!(
                        "{:<6} {:<20} {:<8} {:<20}",
                        p.id,
                        p.name,
                        task_count,
                        p.created_at.format("%Y-%m-%d %H:%M")
                    );
                }
            }
        }
        ProjectAction::Edit { id, name } => {
            project::rename_project(db, id, &name, clock.now())?;
            println!("Project renamed to '{}'.", name.trim());
        }
        ProjectAction::Delete { id, yes } => match project::delete_project(db, id, yes)? {
            DeleteResult::Deleted { name } => {
                println!("Project '{name}' deleted.");
            }
            DeleteResult::NeedsConfirmation { name, task_count } => {
                eprint!(
                    "Project '{name}' has {task_count} tasks. Delete project and all tasks? [y/N] "
                );
                io::stderr().flush().ok();

                let stdin = io::stdin();
                let answer = stdin.lock().lines().next().transpose().ok().flatten();

                if answer.as_deref() == Some("y") || answer.as_deref() == Some("Y") {
                    project::delete_project(db, id, true)?;
                    println!("Project '{name}' deleted.");
                } else {
                    println!("Cancelled.");
                }
            }
        },
    }
    Ok(())
}
