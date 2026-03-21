use std::collections::BTreeMap;

use chrono::{NaiveDate, NaiveDateTime};

use crate::db::Database;
use crate::error::AppError;
use crate::models::TaskEntry;

pub struct Report {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub project_summaries: Vec<ProjectSummary>,
    pub daily_sections: Vec<DailySection>,
    pub grand_total: i64,
}

pub struct ProjectSummary {
    pub name: String,
    pub total: i64,
}

pub struct DailySection {
    pub date: NaiveDate,
    pub entries: Vec<DailyEntry>,
}

pub struct DailyEntry {
    pub task: TaskEntry,
    pub project_name: String,
}

pub fn generate_report(db: &Database, from: NaiveDate, to: NaiveDate) -> Result<Report, AppError> {
    let from_str = from.format("%Y-%m-%d").to_string();
    let to_str = to.format("%Y-%m-%d").to_string();
    let tasks = db.list_tasks_for_date_range(&from_str, &to_str)?;

    let projects = db.list_projects()?;
    let mut project_summaries: Vec<ProjectSummary> = Vec::new();
    let mut daily_map: BTreeMap<NaiveDate, Vec<DailyEntry>> = BTreeMap::new();
    let mut grand_total: i64 = 0;

    for task in tasks {
        grand_total += task.duration_min;

        let project_name = projects
            .iter()
            .find(|p| p.id == task.project_id)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "?".to_string());

        if let Some(summary) = project_summaries
            .iter_mut()
            .find(|s| s.name == project_name)
        {
            summary.total += task.duration_min;
        } else {
            project_summaries.push(ProjectSummary {
                name: project_name.clone(),
                total: task.duration_min,
            });
        }

        let task_date = task
            .start_time
            .map(|t| t.date())
            .unwrap_or_else(|| task.created_at.date());

        daily_map
            .entry(task_date)
            .or_default()
            .push(DailyEntry { task, project_name });
    }

    let daily_sections: Vec<DailySection> = daily_map
        .into_iter()
        .map(|(date, mut entries)| {
            entries.sort_by_key(|e| (e.task.start_time.unwrap_or(NaiveDateTime::MAX), e.task.id));
            DailySection { date, entries }
        })
        .collect();

    Ok(Report {
        from,
        to,
        project_summaries,
        daily_sections,
        grand_total,
    })
}
