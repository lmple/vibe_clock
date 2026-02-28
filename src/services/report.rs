use chrono::NaiveDate;

use crate::db::Database;
use crate::error::AppError;
use crate::models::TaskEntry;

pub struct Report {
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub project_sections: Vec<ProjectSection>,
    pub grand_total: i64,
}

pub struct ProjectSection {
    pub name: String,
    pub entries: Vec<TaskEntry>,
    pub total: i64,
}

pub fn generate_report(db: &Database, from: NaiveDate, to: NaiveDate) -> Result<Report, AppError> {
    let from_str = from.format("%Y-%m-%d").to_string();
    let to_str = to.format("%Y-%m-%d").to_string();
    let tasks = db.list_tasks_for_date_range(&from_str, &to_str)?;

    let projects = db.list_projects()?;
    let mut sections: Vec<ProjectSection> = Vec::new();
    let mut grand_total: i64 = 0;

    for task in &tasks {
        grand_total += task.duration_min;
        let project_name = projects
            .iter()
            .find(|p| p.id == task.project_id)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "?".to_string());

        if let Some(section) = sections.iter_mut().find(|s| s.name == project_name) {
            section.total += task.duration_min;
            section.entries.push(task.clone());
        } else {
            sections.push(ProjectSection {
                name: project_name,
                entries: vec![task.clone()],
                total: task.duration_min,
            });
        }
    }

    Ok(Report {
        from,
        to,
        project_sections: sections,
        grand_total,
    })
}
