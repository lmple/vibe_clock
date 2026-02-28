use chrono::Local;

use crate::db::Database;
use crate::error::AppError;
use crate::formatting::{self, format_duration};
use crate::services::journal;

pub fn handle_journal(db: &Database, date_arg: Option<&str>) -> Result<(), AppError> {
    let date = match date_arg {
        Some(d) => formatting::parse_date(d).map_err(|e| AppError::UserError(e.to_string()))?,
        None => Local::now().date_naive(),
    };

    let daily = journal::get_daily_journal(db, date)?;

    if daily.tasks.is_empty() {
        println!("No tasks logged for {}.", date.format("%Y-%m-%d"));
        return Ok(());
    }

    // Text-mode output (used for piped output and integration tests)
    println!("Journal for {}:", date.format("%Y-%m-%d"));
    println!();
    println!(
        "{:<6} {:<15} {:<30} {:<6} {:<6} {:<8}",
        "ID", "Project", "Description", "Start", "End", "Duration"
    );
    println!("{}", "-".repeat(75));

    let projects = db.list_projects()?;
    for task in &daily.tasks {
        let project_name = projects
            .iter()
            .find(|p| p.id == task.project_id)
            .map(|p| p.name.as_str())
            .unwrap_or("?");

        let start = task
            .start_time
            .map(|t| t.format("%H:%M").to_string())
            .unwrap_or_else(|| "-".to_string());

        let end = task
            .end_time
            .map(|t| t.format("%H:%M").to_string())
            .unwrap_or_else(|| "-".to_string());

        let desc = if task.description.len() > 30 {
            format!("{}...", &task.description[..27])
        } else {
            task.description.clone()
        };

        println!(
            "{:<6} {:<15} {:<30} {:<6} {:<6} {:<8}",
            task.id,
            project_name,
            desc,
            start,
            end,
            format_duration(task.duration_min)
        );
    }

    println!();
    println!("Totals:");
    for (name, minutes) in &daily.project_totals {
        println!("  {:<20} {}", name, format_duration(*minutes));
    }
    println!("  {:<20} {}", "TOTAL", format_duration(daily.grand_total));

    Ok(())
}
