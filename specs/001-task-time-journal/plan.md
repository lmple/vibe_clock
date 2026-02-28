# Implementation Plan: Task Time Journal

**Branch**: `001-task-time-journal` | **Date**: 2026-02-28 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-task-time-journal/spec.md`

## Summary

Build a CLI time-tracking application in Rust that allows users to manage projects, clock tasks with start/stop timers or manual time entries, view daily journals, and generate date-range reports. Data is stored in an encrypted SQLite database (SQLCipher) that can optionally reside on a cloud drive folder. The CLI uses clap for subcommand parsing and ratatui + crossterm for a reactive terminal UI with live-updating clock displays, table navigation, and interactive editing.

## Technical Context

**Language/Version**: Rust 2024 edition, MSRV 1.85.0
**Primary Dependencies**: clap v4 (CLI parsing), ratatui + crossterm (reactive TUI), rusqlite with bundled-sqlcipher-vendored-openssl (encrypted SQLite), chrono (date/time), keyring (passphrase storage), dirs (platform data directories), anyhow (error handling)
**Storage**: SQLite with SQLCipher encryption (AES-256), single file
**Testing**: cargo test, assert_cmd + predicates (CLI integration tests), tempfile (temp databases)
**Target Platform**: Linux, macOS, Windows (standalone statically-linked binary)
**Project Type**: CLI application
**Performance Goals**: <200ms per command (1,000 entries), <1s yearly report (2,000 entries), <100ms cold start
**Constraints**: Single-user, offline-capable, standalone binary, cloud-drive compatible
**Scale/Scope**: Single user, ~2,000 entries/year, 50 projects max typical usage

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Code Quality | PASS | Rust's type system + clippy enforce style and type safety. `cargo fmt` for formatting. Dependencies are minimal and justified (see research.md). |
| II. Testing Standards | PASS | TDD workflow planned. Integration tests use temp databases. Time abstracted behind trait. Edge case tests mapped to spec. |
| III. UX Consistency | PASS | clap derive provides consistent `--help`. Subcommand pattern: `vibe-clock <resource> <action>`. ratatui for interactive tabular output and live clock display. Exit codes: 0/1/2. |
| IV. Performance | PASS | Rust compiled binary + SQLite indexed queries. DELETE journal mode for cloud-drive safety with atomic writes. Selective queries via SQL WHERE (no full-table scans for daily views). |

## Project Structure

### Documentation (this feature)

```text
specs/001-task-time-journal/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/
│   └── cli-commands.md  # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
src/
├── main.rs              # Entry point, CLI dispatch
├── cli/
│   ├── mod.rs           # clap App definition with subcommands
│   ├── project.rs       # Project subcommand handlers
│   ├── clock.rs         # Clock subcommand handlers
│   ├── task.rs          # Task subcommand handlers
│   ├── journal.rs       # Journal subcommand handler
│   └── report.rs        # Report subcommand handler
├── db/
│   ├── mod.rs           # Database connection, initialization, migrations
│   ├── project.rs       # Project CRUD queries
│   ├── task_entry.rs    # TaskEntry CRUD queries
│   └── clock_state.rs   # ClockState queries
├── models/
│   ├── mod.rs           # Re-exports
│   ├── project.rs       # Project struct
│   ├── task_entry.rs    # TaskEntry struct
│   └── clock_state.rs   # ClockState struct
├── services/
│   ├── mod.rs           # Re-exports
│   ├── project.rs       # Project business logic
│   ├── clock.rs         # Clock start/stop/status logic
│   ├── task.rs          # Task add/edit/delete logic
│   ├── journal.rs       # Daily journal aggregation
│   └── report.rs        # Report generation logic
├── tui/
│   ├── mod.rs           # TUI app state and event loop
│   ├── widgets.rs       # Custom ratatui widgets (task table, clock display)
│   ├── journal_view.rs  # Interactive daily journal view
│   ├── report_view.rs   # Interactive report view
│   └── input.rs         # Input handling and key bindings
├── formatting/
│   ├── mod.rs           # Re-exports
│   └── duration.rs      # Duration display formatting (e.g., "1h 30m")
├── crypto.rs            # Passphrase management (keyring + env var + prompt)
├── clock_trait.rs       # Clock trait for time abstraction (testability)
├── config.rs            # Database path resolution (default + VIBE_CLOCK_DB env)
└── error.rs             # Error types and exit code mapping

tests/
├── cli_project.rs       # Integration tests: project commands
├── cli_clock.rs         # Integration tests: clock commands
├── cli_task.rs          # Integration tests: task commands
├── cli_journal.rs       # Integration tests: journal command
├── cli_report.rs        # Integration tests: report command
├── common/
│   └── mod.rs           # Test helpers (temp DB setup, test clock)
└── edge_cases.rs        # Edge case tests from spec
```

**Structure Decision**: Single Rust project (cargo binary crate). The `src/` directory is organized by architectural layer: `cli/` for command parsing and dispatch, `services/` for business logic, `db/` for data access, `models/` for data structures, and `formatting/` for output rendering. This follows standard Rust project conventions and keeps modules small and single-responsibility per Constitution Principle I.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| `keyring` dependency for passphrase storage | Encryption key must persist between sessions without user re-entering it every time | Storing key in a file is insecure; env var alone is inconvenient for daily use |
| `crypto.rs` module with fallback chain (keyring → env var → prompt) | Different platforms have different keychain support; must work everywhere | Single approach would fail on some platforms |
