use clap::Parser;

use vibe_clock::cli::{self, Cli};
use vibe_clock::clock_trait::SystemClock;
use vibe_clock::error::AppError;
use vibe_clock::formatting::format_duration;
use vibe_clock::services::clock as clock_service;
use vibe_clock::{config, crypto, db};

fn main() {
    let cli = Cli::parse();

    if let Err(err) = run(cli) {
        eprintln!("Error: {err}");
        std::process::exit(err.exit_code());
    }
}

fn run(cli: Cli) -> Result<(), AppError> {
    let db_path = config::resolve_db_path().map_err(|e| AppError::SystemError(e.to_string()))?;
    let passphrase = crypto::get_passphrase().map_err(|e| AppError::SystemError(e.to_string()))?;
    let db = db::Database::open(&db_path, &passphrase)?;
    let clock = SystemClock;

    // Clock crash recovery (FR-016): warn if clock was left running
    if let Some(info) = clock_service::recover_clock(&db, &clock)? {
        eprintln!(
            "Warning: Clock still running for '{}' on project '{}' since {} ({} elapsed).",
            info.description,
            info.project_name,
            info.start_time,
            format_duration(info.elapsed_min)
        );
    }

    match cli.command {
        cli::Command::Project { action } => {
            cli::project::handle_project(&db, &clock, action)?;
        }
        cli::Command::Clock { action } => {
            cli::clock::handle_clock(&db, &clock, action)?;
        }
        cli::Command::Task { action } => {
            cli::task::handle_task(&db, &clock, action)?;
        }
        cli::Command::Journal { date } => {
            cli::journal::handle_journal(&db, date.as_deref())?;
        }
        cli::Command::Report { from, to } => {
            cli::report::handle_report(&db, &from, &to)?;
        }
    }

    Ok(())
}
