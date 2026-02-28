# vibe_clock Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-02-28

## Active Technologies

- Rust 2024 edition (MSRV 1.85.0) + clap v4, ratatui, crossterm, rusqlite (bundled-sqlcipher-vendored-openssl), chrono, keyring, dirs, anyhow (001-task-time-journal)
- SQLite with SQLCipher encryption (AES-256), DELETE journal mode (001-task-time-journal)

## Project Structure

```text
src/
├── main.rs
├── cli/          # clap subcommand definitions and dispatch
├── db/           # rusqlite database access layer
├── models/       # Data structures
├── services/     # Business logic
├── tui/          # ratatui interactive views
├── formatting/   # Duration display formatting
├── crypto.rs     # Passphrase management (keyring + env var + prompt)
├── clock_trait.rs # Time abstraction for testability
├── config.rs     # Database path resolution
└── error.rs      # Error types and exit codes

tests/
├── cli_*.rs      # Integration tests (assert_cmd)
├── common/       # Test helpers
└── edge_cases.rs # Edge case tests from spec
```

## Commands

```bash
cargo test && cargo clippy && cargo fmt --check
```

## Code Style

Rust 2024 edition: Follow standard conventions. Use `cargo fmt` for formatting, `cargo clippy` for linting.

## Recent Changes

- 001-task-time-journal: Added Rust 2024 + clap v4 + ratatui + rusqlite (SQLCipher) + chrono + keyring

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
