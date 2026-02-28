use crate::clock_trait::Clock;
use crate::db::Database;
use crate::error::AppError;

pub fn start_clock(
    db: &Database,
    project_name: &str,
    description: &str,
    clock: &dyn Clock,
) -> Result<(String, String), AppError> {
    if db.get_clock_state()?.is_some() {
        return Err(AppError::UserError(
            "Clock already running. Use 'vibe-clock clock stop' first, or 'vibe-clock clock status' to check.".to_string(),
        ));
    }

    let project = super::resolve_project(db, project_name)?;

    let now = clock.now();
    db.insert_clock_state(project.id, description, now)?;

    Ok((project.name, now.format("%H:%M").to_string()))
}

pub fn stop_clock(db: &Database, clock: &dyn Clock) -> Result<StopResult, AppError> {
    let state = db
        .get_clock_state()?
        .ok_or_else(|| AppError::UserError("No clock is running.".to_string()))?;

    let now = clock.now();
    let duration_min = (now - state.start_time).num_minutes().max(1);

    let project = db.find_project_by_id(state.project_id)?;
    let project_name = project.map(|p| p.name).unwrap_or_else(|| "?".to_string());

    // Transaction: delete clock state + insert task entry
    let tx = db.conn.unchecked_transaction()?;
    tx.execute("DELETE FROM clock_state WHERE id = 1", [])?;
    tx.execute(
        "INSERT INTO task_entry (project_id, description, start_time, end_time, duration_min, created_at, updated_at) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            state.project_id,
            state.description,
            state.start_time.format("%Y-%m-%dT%H:%M:%S").to_string(),
            now.format("%Y-%m-%dT%H:%M:%S").to_string(),
            duration_min,
            now.format("%Y-%m-%dT%H:%M:%S").to_string(),
            now.format("%Y-%m-%dT%H:%M:%S").to_string(),
        ],
    )?;
    tx.commit()?;

    Ok(StopResult {
        project_name,
        description: state.description,
        duration_min,
    })
}

pub fn clock_status(db: &Database, clock: &dyn Clock) -> Result<Option<ClockStatusInfo>, AppError> {
    let state = db.get_clock_state()?;
    match state {
        Some(state) => {
            let now = clock.now();
            let elapsed_min = (now - state.start_time).num_minutes();
            let project = db.find_project_by_id(state.project_id)?;
            let project_name = project.map(|p| p.name).unwrap_or_else(|| "?".to_string());

            Ok(Some(ClockStatusInfo {
                project_name,
                description: state.description,
                start_time: state.start_time.format("%H:%M").to_string(),
                elapsed_min,
            }))
        }
        None => Ok(None),
    }
}

pub fn recover_clock(
    db: &Database,
    clock: &dyn Clock,
) -> Result<Option<ClockStatusInfo>, AppError> {
    clock_status(db, clock)
}

pub struct StopResult {
    pub project_name: String,
    pub description: String,
    pub duration_min: i64,
}

pub struct ClockStatusInfo {
    pub project_name: String,
    pub description: String,
    pub start_time: String,
    pub elapsed_min: i64,
}
