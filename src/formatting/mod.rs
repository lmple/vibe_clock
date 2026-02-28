pub mod duration;

use anyhow::{Result, bail};
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};

pub use duration::{format_duration, parse_duration};

use crate::models::TaskEntry;

/// Parse a date string into a NaiveDate.
///
/// Supported formats:
/// - "today" → current date
/// - "yesterday" → yesterday's date
/// - ISO 8601 date (e.g., "2026-02-28")
pub fn parse_date(input: &str) -> Result<NaiveDate> {
    let input = input.trim().to_lowercase();
    match input.as_str() {
        "today" => Ok(Local::now().date_naive()),
        "yesterday" => Ok(Local::now()
            .date_naive()
            .pred_opt()
            .expect("date underflow")),
        _ => NaiveDate::parse_from_str(&input, "%Y-%m-%d").map_err(|_| {
            anyhow::anyhow!("Invalid date: '{input}'. Use YYYY-MM-DD, 'today', or 'yesterday'")
        }),
    }
}

/// Parse a time string into a NaiveDateTime.
///
/// Supported formats:
/// - ISO 8601 datetime (e.g., "2026-02-28T14:30")
/// - HH:MM (assumes today's date)
pub fn parse_time(input: &str) -> Result<NaiveDateTime> {
    let input = input.trim();

    // Try full ISO 8601 datetime first
    if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S") {
        return Ok(dt);
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M") {
        return Ok(dt);
    }

    // Try HH:MM (assumes today)
    if let Ok(time) = NaiveTime::parse_from_str(input, "%H:%M") {
        let today = Local::now().date_naive();
        return Ok(NaiveDateTime::new(today, time));
    }

    bail!("Invalid time: '{input}'. Use YYYY-MM-DDTHH:MM or HH:MM")
}

/// Format a table of task entries as plain text with aligned columns.
pub fn format_task_table(tasks: &[TaskEntry], project_names: &[(&str, i64)]) -> String {
    if tasks.is_empty() {
        return String::new();
    }

    let mut lines = Vec::new();
    lines.push(format!(
        "{:<6} {:<15} {:<30} {:<6} {:<6} {:<8}",
        "ID", "Project", "Description", "Start", "End", "Duration"
    ));
    lines.push("-".repeat(75));

    for task in tasks {
        let project_name = project_names
            .iter()
            .find(|(_, id)| *id == task.project_id)
            .map(|(name, _)| *name)
            .unwrap_or("?");

        let start = task
            .start_time
            .map(|t| t.format("%H:%M").to_string())
            .unwrap_or_else(|| "-".to_string());

        let end = task
            .end_time
            .map(|t| t.format("%H:%M").to_string())
            .unwrap_or_else(|| "-".to_string());

        let dur = format_duration(task.duration_min);

        let desc = if task.description.len() > 30 {
            format!("{}...", &task.description[..27])
        } else {
            task.description.clone()
        };

        lines.push(format!(
            "{:<6} {:<15} {:<30} {:<6} {:<6} {:<8}",
            task.id, project_name, desc, start, end, dur
        ));
    }

    lines.join("\n")
}

/// Format per-project totals and a grand total.
pub fn format_totals(per_project: &[(&str, i64)], grand_total: i64) -> String {
    let mut lines = Vec::new();
    lines.push(String::new());
    lines.push("Totals:".to_string());

    for (name, minutes) in per_project {
        lines.push(format!("  {:<20} {}", name, format_duration(*minutes)));
    }

    lines.push(format!(
        "  {:<20} {}",
        "TOTAL",
        format_duration(grand_total)
    ));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_today() {
        let result = parse_date("today").unwrap();
        assert_eq!(result, Local::now().date_naive());
    }

    #[test]
    fn parse_date_yesterday() {
        let result = parse_date("yesterday").unwrap();
        let expected = Local::now().date_naive().pred_opt().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_date_iso() {
        let result = parse_date("2026-01-15").unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2026, 1, 15).unwrap());
    }

    #[test]
    fn parse_date_invalid() {
        assert!(parse_date("not-a-date").is_err());
    }

    #[test]
    fn parse_time_hhmm() {
        let result = parse_time("14:30").unwrap();
        assert_eq!(result.time(), NaiveTime::from_hms_opt(14, 30, 0).unwrap());
    }

    #[test]
    fn parse_time_iso_datetime() {
        let result = parse_time("2026-01-15T09:00").unwrap();
        assert_eq!(
            result,
            NaiveDate::from_ymd_opt(2026, 1, 15)
                .unwrap()
                .and_hms_opt(9, 0, 0)
                .unwrap()
        );
    }

    #[test]
    fn parse_time_invalid() {
        assert!(parse_time("nope").is_err());
    }

    #[test]
    fn format_totals_output() {
        let per_project = vec![("Acme", 120i64), ("Beta", 45i64)];
        let result = format_totals(&per_project, 165);
        assert!(result.contains("Acme"));
        assert!(result.contains("2h"));
        assert!(result.contains("Beta"));
        assert!(result.contains("45m"));
        assert!(result.contains("2h 45m"));
    }
}
