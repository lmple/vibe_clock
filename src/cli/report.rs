use std::path::PathBuf;

use crate::db::Database;
use crate::error::AppError;
use crate::formatting::{self, format_duration};
use crate::services::{pdf, report};

/// Split a description into lines of at most `width` characters.
/// Splits at word boundaries where possible; hard-splits at `width` if no space is found.
fn wrap_description(desc: &str, width: usize) -> Vec<String> {
    if desc.len() <= width {
        return vec![desc.to_string()];
    }
    let mut chunks: Vec<String> = Vec::new();
    let mut remaining = desc;
    while remaining.len() > width {
        let split_at = match remaining[..width].rfind(' ') {
            Some(pos) => pos,
            None => width,
        };
        chunks.push(remaining[..split_at].to_string());
        remaining = remaining[split_at..].trim_start();
    }
    if !remaining.is_empty() {
        chunks.push(remaining.to_string());
    }
    chunks
}

pub fn handle_report(
    db: &Database,
    from: &str,
    to: Option<&str>,
    pdf_flag: bool,
    output: Option<&str>,
) -> Result<(), AppError> {
    let from_date = formatting::parse_date(from).map_err(|e| AppError::UserError(e.to_string()))?;
    let to_str = to.unwrap_or(from);
    let to_date = formatting::parse_date(to_str).map_err(|e| AppError::UserError(e.to_string()))?;

    if from_date > to_date {
        return Err(AppError::UserError(
            "--from date must be before or equal to --to date.".to_string(),
        ));
    }

    let pdf_path = pdf::resolve_pdf_path(output, pdf_flag, from_date, to_date)?;

    let report = report::generate_report(db, from_date, to_date)?;

    if report.project_summaries.is_empty() {
        println!(
            "No tasks found between {} and {}.",
            from_date.format("%Y-%m-%d"),
            to_date.format("%Y-%m-%d")
        );
        return Ok(());
    }

    // Report header
    println!(
        "Report: {} to {}",
        from_date.format("%Y-%m-%d"),
        to_date.format("%Y-%m-%d")
    );
    println!();

    // Part 1: Project Summary Table
    println!("Project Summary");
    let summary_sep = "-".repeat(36);
    println!("{:<25} {:<10}", "Project", "Total");
    println!("{summary_sep}");
    for summary in &report.project_summaries {
        println!(
            "{:<25} {:<10}",
            summary.name,
            format_duration(summary.total)
        );
    }
    println!("{summary_sep}");
    println!(
        "{:<25} {:<10}",
        "TOTAL",
        format_duration(report.grand_total)
    );
    println!();

    // Part 2: Per-Day Breakdown
    // Column widths: ID(6) Project(18) Description(40) Start(7) End(7) Duration(8)
    let day_sep = "-".repeat(6 + 1 + 18 + 1 + 40 + 1 + 7 + 1 + 7 + 1 + 8);

    for section in &report.daily_sections {
        println!("{}", section.date.format("%Y-%m-%d"));
        println!(
            "{:<6} {:<18} {:<40} {:<7} {:<7} {:<8}",
            "ID", "Project", "Description", "Start", "End", "Duration"
        );
        println!("{day_sep}");

        for entry in &section.entries {
            let start = entry
                .task
                .start_time
                .map(|t| t.format("%H:%M").to_string())
                .unwrap_or_else(|| "-".to_string());
            let end = entry
                .task
                .end_time
                .map(|t| t.format("%H:%M").to_string())
                .unwrap_or_else(|| "-".to_string());

            let chunks = wrap_description(&entry.task.description, 40);

            println!(
                "{:<6} {:<18} {:<40} {:<7} {:<7} {:<8}",
                entry.task.id,
                entry.project_name,
                chunks[0],
                start,
                end,
                format_duration(entry.task.duration_min)
            );

            for chunk in chunks.iter().skip(1) {
                println!("{:<6} {:<18} {:<40}", "", "", chunk);
            }
        }
        println!();
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_description_short_fits_in_one_chunk() {
        let chunks = wrap_description("Hello world", 40);
        assert_eq!(chunks, vec!["Hello world"]);
    }

    #[test]
    fn wrap_description_splits_at_word_boundary() {
        let desc = "This is a fairly long description that exceeds forty characters easily";
        let chunks = wrap_description(desc, 40);
        assert!(chunks.len() > 1);
        for chunk in &chunks {
            assert!(chunk.len() <= 40, "chunk too long: '{chunk}'");
        }
        // Reassembled text should equal original (modulo spaces at boundaries)
        let rejoined = chunks.join(" ");
        assert_eq!(rejoined, desc);
    }

    #[test]
    fn wrap_description_hard_splits_word_without_space() {
        let long_word = "a".repeat(50);
        let chunks = wrap_description(&long_word, 40);
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].len(), 40);
        assert_eq!(chunks[1].len(), 10);
    }
}
