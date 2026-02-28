use crate::db::Database;
use crate::error::AppError;
use crate::formatting::{self, format_duration};
use crate::services::report;

pub fn handle_report(db: &Database, from: &str, to: &str) -> Result<(), AppError> {
    let from_date = formatting::parse_date(from).map_err(|e| AppError::UserError(e.to_string()))?;
    let to_date = formatting::parse_date(to).map_err(|e| AppError::UserError(e.to_string()))?;

    if from_date > to_date {
        return Err(AppError::UserError(
            "--from date must be before or equal to --to date.".to_string(),
        ));
    }

    let report = report::generate_report(db, from_date, to_date)?;

    if report.project_sections.is_empty() {
        println!(
            "No tasks found between {} and {}.",
            from_date.format("%Y-%m-%d"),
            to_date.format("%Y-%m-%d")
        );
        return Ok(());
    }

    println!(
        "Report: {} to {}",
        from_date.format("%Y-%m-%d"),
        to_date.format("%Y-%m-%d")
    );
    println!();

    for section in &report.project_sections {
        println!("## {} ({})", section.name, format_duration(section.total));
        println!(
            "  {:<6} {:<30} {:<12} {:<6} {:<6} {:<8}",
            "ID", "Description", "Date", "Start", "End", "Duration"
        );
        println!("  {}", "-".repeat(72));

        for task in &section.entries {
            let date = task
                .start_time
                .map(|t| t.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| task.created_at.format("%Y-%m-%d").to_string());

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
                "  {:<6} {:<30} {:<12} {:<6} {:<6} {:<8}",
                task.id,
                desc,
                date,
                start,
                end,
                format_duration(task.duration_min)
            );
        }
        println!();
    }

    println!("Grand Total: {}", format_duration(report.grand_total));

    Ok(())
}
