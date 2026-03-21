pub mod clock;
pub mod journal;
pub mod project;
pub mod report;
pub mod task;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "vibe-clock",
    version,
    about = "A daily task journal with time tracking"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Manage projects
    Project {
        #[command(subcommand)]
        action: ProjectAction,
    },
    /// Clock in/out for time tracking
    Clock {
        #[command(subcommand)]
        action: ClockAction,
    },
    /// Manage task entries
    Task {
        #[command(subcommand)]
        action: TaskAction,
    },
    /// View the daily task journal
    Journal {
        /// Date to view (YYYY-MM-DD, "today", or "yesterday"; defaults to today)
        date: Option<String>,
    },
    /// Generate time reports
    Report {
        /// Start date (inclusive, YYYY-MM-DD or "today"/"yesterday")
        #[arg(long)]
        from: String,
        /// End date (inclusive, YYYY-MM-DD or "today"/"yesterday"; defaults to same as --from)
        #[arg(long)]
        to: Option<String>,
        /// Export report as PDF to the current directory
        #[arg(long)]
        pdf: bool,
        /// Output path for PDF file (e.g., report.pdf or /path/to/dir/)
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProjectAction {
    /// Create a new project
    Add {
        /// Project name
        name: String,
    },
    /// List all projects
    List,
    /// Rename a project
    Edit {
        /// Project ID
        id: i64,
        /// New project name
        #[arg(long)]
        name: String,
    },
    /// Delete a project
    Delete {
        /// Project ID
        id: i64,
        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ClockAction {
    /// Start a time clock
    Start {
        /// Project name or ID
        project: String,
        /// Task description
        description: String,
    },
    /// Stop the running clock
    Stop,
    /// Check clock status
    Status,
}

#[derive(Subcommand, Debug)]
pub enum TaskAction {
    /// Manually add a task entry
    Add {
        /// Project name or ID
        project: String,
        /// Task description
        description: String,
        /// Start time (HH:MM, 24-hour clock, e.g., 9:00 or 14:30)
        #[arg(long)]
        start: Option<String>,
        /// End time (HH:MM, 24-hour clock, e.g., 17:30)
        #[arg(long)]
        end: Option<String>,
        /// Duration (e.g., 1h30m, 45m, 2h, or 90 for minutes)
        #[arg(long)]
        duration: Option<String>,
        /// Date for the entry (YYYY-MM-DD, 'today', or 'yesterday'; defaults to today)
        #[arg(long)]
        date: Option<String>,
    },
    /// Edit an existing task entry
    Edit {
        /// Task entry ID
        id: i64,
        /// New description
        #[arg(long)]
        description: Option<String>,
        /// Move to a different project (name or ID)
        #[arg(long)]
        project: Option<String>,
        /// New start time (HH:MM, 24-hour clock, e.g., 9:00 or 14:30)
        #[arg(long)]
        start: Option<String>,
        /// New end time (HH:MM, 24-hour clock)
        #[arg(long)]
        end: Option<String>,
        /// New duration (e.g., 1h30m, 45m, 2h, or 90 for minutes)
        #[arg(long)]
        duration: Option<String>,
        /// Move task to a different date (YYYY-MM-DD, 'today', or 'yesterday')
        #[arg(long)]
        date: Option<String>,
    },
    /// Delete a task entry
    Delete {
        /// Task entry ID
        id: i64,
        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },
}
