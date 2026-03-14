use std::path::PathBuf;

use crate::db::Database;
use crate::error::AppError;
use crate::formatting::{self, format_duration};
use crate::services::{pdf, report};

pub fn handle_report(
    db: &Database,
    from: &str,
    to: Option<&str>,
    pdf_flag: bool,
    output: Option<&str>,
) -> Result<(), AppError> {
    let from_date = formatting::parse_date(from).map_err(|e| AppError::UserError(e.to_string()))?;
    // Default to same date as from if to is not provided
    let to_str = to.unwrap_or(from);
    let to_date = formatting::parse_date(to_str).map_err(|e| AppError::UserError(e.to_string()))?;

    if from_date > to_date {
        return Err(AppError::UserError(
            "--from date must be before or equal to --to date.".to_string(),
        ));
    }

    let pdf_path = pdf::resolve_pdf_path(output, pdf_flag, from_date, to_date)?;

    let report = report::generate_report(db, from_date, to_date)?;

    if report.project_sections.is_empty() {
        println!(
            "No tasks found between {} and {}.",
            from_date.format("%Y-%m-%d"),
            to_date.format("%Y-%m-%d")
        );
        return Ok(());
    }

    // Print terminal report
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

    // Generate PDF if requested
    if let Some(path) = pdf_path {
        let abs_path = if path.is_absolute() {
            path
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(&path)
        };
        pdf::render_pdf(&report, &abs_path)?;
        println!("PDF report saved to {}", abs_path.display());
    }

    Ok(())
}
