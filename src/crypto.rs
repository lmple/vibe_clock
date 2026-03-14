use anyhow::{Context, Result};

const KEYRING_SERVICE: &str = "vibe-clock";
const KEYRING_USER: &str = "db-passphrase";

/// Retrieve the database passphrase.
///
/// Priority:
/// 1. `VIBE_CLOCK_KEY` environment variable (checked first — instant, non-blocking)
/// 2. OS keyring (via keyring crate — may block on locked keyrings)
/// 3. Terminal prompt (stores to keyring on first use)
pub fn get_passphrase() -> Result<String> {
    // Try environment variable first (non-blocking)
    if let Ok(passphrase) = std::env::var("VIBE_CLOCK_KEY") {
        return Ok(passphrase);
    }

    // Try keyring
    if let Ok(passphrase) = get_from_keyring() {
        return Ok(passphrase);
    }

    // Fall back to terminal prompt
    let passphrase = prompt_passphrase()?;
    store_in_keyring(&passphrase).ok();
    Ok(passphrase)
}

fn get_from_keyring() -> Result<String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)?;
    let passphrase = entry.get_password()?;
    Ok(passphrase)
}

fn store_in_keyring(passphrase: &str) -> Result<()> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)?;
    entry.set_password(passphrase)?;
    Ok(())
}

fn prompt_passphrase() -> Result<String> {
    use std::io::{self, BufRead, Write};

    eprint!("Enter database passphrase: ");
    io::stderr().flush()?;

    let stdin = io::stdin();
    let passphrase = stdin
        .lock()
        .lines()
        .next()
        .context("No input received")?
        .context("Failed to read passphrase")?;

    if passphrase.is_empty() {
        anyhow::bail!("Passphrase cannot be empty");
    }

    Ok(passphrase)
}
