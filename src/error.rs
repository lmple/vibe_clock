use std::fmt;

/// Application error types mapping to specific exit codes.
///
/// - `UserError` (exit code 1): Invalid input, duplicate names, missing resources
/// - `SystemError` (exit code 2): Database failures, I/O errors, crypto errors
#[derive(Debug)]
pub enum AppError {
    UserError(String),
    SystemError(String),
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::UserError(_) => 1,
            AppError::SystemError(_) => 2,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::UserError(msg) | AppError::SystemError(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::SystemError(format!("Database error: {err}"))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::SystemError(format!("I/O error: {err}"))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::SystemError(err.to_string())
    }
}
