use chrono::NaiveDate;

use crate::db::Database;
use crate::error::AppError;
use crate::models::TaskEntry;

pub struct DailyJournal {
    pub date: NaiveDate,
    pub tasks: Vec<TaskEntry>,
    pub project_totals: Vec<(String, i64)>,
    pub grand_total: i64,
}

pub fn get_daily_journal(db: &Database, date: NaiveDate) -> Result<DailyJournal, AppError> {
    let date_str = date.format("%Y-%m-%d").to_string();
    let tasks = db.list_tasks_for_date(&date_str)?;

    let projects = db.list_projects()?;
    let mut project_totals: Vec<(String, i64)> = Vec::new();
    let mut grand_total: i64 = 0;

    for task in &tasks {
        grand_total += task.duration_min;
        let project_name = projects
            .iter()
            .find(|p| p.id == task.project_id)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "?".to_string());

        if let Some(entry) = project_totals
            .iter_mut()
            .find(|(name, _)| name == &project_name)
        {
            entry.1 += task.duration_min;
        } else {
            project_totals.push((project_name, task.duration_min));
        }
    }

    Ok(DailyJournal {
        date,
        tasks,
        project_totals,
        grand_total,
    })
}
