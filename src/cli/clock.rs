use crate::clock_trait::Clock;
use crate::db::Database;
use crate::error::AppError;
use crate::formatting::format_duration;
use crate::services::clock;

use super::ClockAction;

pub fn handle_clock(db: &Database, clk: &dyn Clock, action: ClockAction) -> Result<(), AppError> {
    match action {
        ClockAction::Start {
            project,
            description,
        } => {
            let (project_name, time) = clock::start_clock(db, &project, &description, clk)?;
            println!("Clock started for '{description}' on project '{project_name}' at {time}.");
        }
        ClockAction::Stop => {
            let result = clock::stop_clock(db, clk)?;
            println!(
                "Clock stopped. Logged {} for '{}' on project '{}'.",
                format_duration(result.duration_min),
                result.description,
                result.project_name
            );
        }
        ClockAction::Status => match clock::clock_status(db, clk)? {
            Some(info) => {
                println!(
                    "Clock running: '{}' on project '{}' since {} ({} elapsed).",
                    info.description,
                    info.project_name,
                    info.start_time,
                    format_duration(info.elapsed_min)
                );
            }
            None => {
                println!("No clock is running.");
            }
        },
    }
    Ok(())
}
