use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct TaskEntry {
    pub id: i64,
    pub project_id: i64,
    pub description: String,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub duration_min: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
