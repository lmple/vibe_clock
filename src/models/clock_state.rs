use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct ClockState {
    pub id: i64,
    pub project_id: i64,
    pub description: String,
    pub start_time: NaiveDateTime,
}
