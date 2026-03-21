# vibe_clock Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-03-21

## Active Technologies
- Rust 2024 edition, MSRV 1.85.0 + genpdfi (PDF generation), chrono (dates), clap v4 (CLI) (002-pdf-report-export)
- No changes — reads from existing SQLite/SQLCipher database (002-pdf-report-export)
- Rust 2024 edition, MSRV 1.85.0 + clap v4 (CLI), chrono (date/time), rusqlite (bundled-sqlcipher-vendored-openssl), anyhow (errors) (003-fix-date-input)
- SQLite with SQLCipher encryption — no schema changes required (003-fix-date-input)
- Rust 2024 edition, MSRV 1.85.0 + clap v4 (CLI), genpdfi (PDF), chrono (dates), rusqlite/SQLCipher (storage) (004-report-layout)

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
- 004-report-layout: Added Rust 2024 edition, MSRV 1.85.0 + clap v4 (CLI), genpdfi (PDF), chrono (dates), rusqlite/SQLCipher (storage)
- 004-report-layout: Added [if applicable, e.g., PostgreSQL, CoreData, files or N/A]
- 003-fix-date-input: Added Rust 2024 edition, MSRV 1.85.0 + clap v4 (CLI), chrono (date/time), rusqlite (bundled-sqlcipher-vendored-openssl), anyhow (errors)

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
