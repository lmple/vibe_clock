use std::path::PathBuf;

use anyhow::{Context, Result};

/// Resolve the database file path.
///
/// Priority:
/// 1. `VIBE_CLOCK_DB` environment variable
/// 2. Platform data directory: `<data_dir>/vibe-clock/vibe-clock.db`
///
/// Creates parent directories if they don't exist.
pub fn resolve_db_path() -> Result<PathBuf> {
    let path = if let Ok(env_path) = std::env::var("VIBE_CLOCK_DB") {
        PathBuf::from(env_path)
    } else {
        let data_dir = dirs::data_dir().context("Could not determine platform data directory")?;
        data_dir.join("vibe-clock").join("vibe-clock.db")
    };

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    Ok(path)
}
