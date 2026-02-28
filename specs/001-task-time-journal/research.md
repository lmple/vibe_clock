# Research: Task Time Journal

**Branch**: `001-task-time-journal` | **Date**: 2026-02-28

## R-001: Rust Edition and Toolchain

**Decision**: Rust 2024 edition, MSRV 1.85.0
**Rationale**: The 2024 edition was stabilized with Rust 1.85.0 (Feb 2025). As a greenfield project, there is no reason to use an older edition. The 2024 edition includes improved RPIT lifetime capture rules, `unsafe_op_in_unsafe_fn` lint enabled by default, and various ergonomic improvements.
**Alternatives considered**: Rust 2021 edition (still supported but misses 2024 improvements); nightly toolchain (unnecessary).

## R-002: CLI Framework

**Decision**: `clap` v4 with derive macros for command parsing + `ratatui` + `crossterm` for reactive TUI
**Rationale**: clap is the de facto standard for Rust CLI argument parsing. The derive API provides type-safe subcommand definitions with automatic `--help` generation, satisfying Constitution Principle III (UX Consistency). For the "reactive" requirement, `ratatui` (with `crossterm` as the terminal backend) provides a full terminal UI framework with live-updating widgets, keyboard navigation, and interactive table editing. This enables a live clock display (elapsed time ticking), navigable task tables for the journal view, and inline editing of task entries. clap handles initial command dispatch; certain commands (journal, report, clock status) launch a ratatui interactive session.
**Alternatives considered**:
- `dialoguer`/`inquire`: Prompt-based libraries. Cannot show live-updating clock timers, navigate tables, or build a reactive editing experience. Too limited for the "reactive" requirement.
- `cursive`: Another TUI framework, less actively maintained than ratatui.
- `structopt`: Superseded by clap v4 derive.

## R-003: SQLite Library

**Decision**: `rusqlite` with bundled SQLCipher
**Rationale**: rusqlite is the most mature SQLite binding for Rust. It supports bundling SQLCipher directly via the `bundled-sqlcipher-vendored-openssl` feature flag, which means encryption is handled at the SQLite level transparently. This avoids a separate encryption layer. rusqlite provides simple, direct SQL access appropriate for a small schema with 3 tables.
**Alternatives considered**:
- `diesel`: Full ORM with migration support, but heavy for 3 tables. Adds compile-time complexity and a DSL learning curve. SQLCipher integration is less straightforward.
- `sqlx`: Async-first design is unnecessary overhead for a synchronous CLI. Compile-time query checking requires a running database during `cargo check`, complicating CI.
- `sea-orm`: Even heavier than diesel. Not appropriate for this scale.

## R-004: SQLite Encryption

**Decision**: SQLCipher via rusqlite's `bundled-sqlcipher-vendored-openssl` feature
**Rationale**: SQLCipher is the industry standard for SQLite encryption (AES-256-CBC). The `bundled-sqlcipher-vendored-openssl` feature compiles SQLCipher from source as part of the Rust build, producing a single statically-linked binary. Encryption is transparent to the application - just provide a key via `PRAGMA key` after opening the connection.
**Key management**: The passphrase is prompted on first use and stored in the OS keychain via the `keyring` crate (uses libsecret on Linux, Keychain on macOS, Credential Manager on Windows). If no keychain is available, the user can provide the passphrase via an environment variable (`VIBE_CLOCK_KEY`) or be prompted each session.
**Alternatives considered**:
- Application-level encryption (encrypt before writing to SQLite): Breaks SQL query capability. Cannot use WHERE clauses on encrypted data.
- `sqlcipher` crate directly: Less maintained than rusqlite's bundled approach.
- AES encryption of the entire file: Breaks SQLite's ACID properties and prevents selective reads.

## R-005: Cloud Drive Compatibility

**Decision**: Use SQLite in DELETE journal mode with exclusive locking; document single-device usage recommendation
**Rationale**: Cloud drive sync tools (Dropbox, OneDrive, Google Drive) may upload the main DB, WAL, and SHM files at different times, causing corruption on other machines. DELETE mode avoids this by using a single short-lived journal file that is deleted after each transaction. From the cloud drive's perspective, only one file changes.
**Configuration**:
- `PRAGMA journal_mode=DELETE` for cloud-drive safety.
- `PRAGMA locking_mode=EXCLUSIVE` since this is single-user, prevents the sync tool from reading a partially-written file.
- `PRAGMA synchronous=FULL` to ensure all writes are fully flushed to disk.
- `PRAGMA busy_timeout=5000` to handle brief cloud sync locks.
- Document that concurrent access from multiple machines is not supported.
**Alternatives considered**:
- WAL mode: Better performance and crash recovery, but creates persistent `-wal` and `-shm` files that can desync on cloud drives. Not safe for this use case.
- Disabling cloud drive support: User explicitly requested it.

## R-006: Testing Strategy

**Decision**: `cargo test` with `tempfile` for integration tests, `mockall` or manual trait-based mocking for time abstraction
**Rationale**: Rust's built-in test framework is sufficient. Each integration test creates a temporary SQLite database using `tempfile::NamedTempFile`. Time is abstracted behind a `Clock` trait so tests can inject fixed timestamps. Unit tests operate on pure functions and domain logic without database access.
**Test organization**:
- `tests/` directory for integration tests (full command paths)
- `#[cfg(test)] mod tests` within source files for unit tests
- `assert_cmd` + `predicates` crates for CLI binary testing (spawn the binary and assert on stdout/stderr/exit code)
**Alternatives considered**:
- `rstest` for parameterized tests: Useful but adds a dependency for marginal benefit at this scale.
- In-memory SQLite for unit tests: Blurs the line between unit and integration tests. Better to keep unit tests database-free.

## R-007: Cross-Platform Standalone Binary

**Decision**: Static linking via `bundled-sqlcipher-vendored-openssl` feature; distribute platform-specific binaries
**Rationale**: The `bundled-sqlcipher-vendored-openssl` feature compiles SQLCipher (and therefore OpenSSL's libcrypto subset) from source, producing a statically-linked binary on all platforms. No runtime dependencies on SQLCipher or OpenSSL system libraries. The binary is self-contained.
**Build targets**:
- Linux: `x86_64-unknown-linux-gnu` (dynamically linked to glibc) or `x86_64-unknown-linux-musl` (fully static)
- macOS: `x86_64-apple-darwin` and `aarch64-apple-darwin` (universal binary possible)
- Windows: `x86_64-pc-windows-msvc`
**Alternatives considered**:
- Dynamic linking to system SQLCipher: Requires users to install SQLCipher separately. Defeats the "standalone" requirement.
- Cross-compilation via `cross`: Viable for CI but not required for initial development.

## R-008: Output Formatting

**Decision**: ratatui's built-in `Table` widget for tabular output in journal and report views
**Rationale**: Since ratatui is already a dependency for the reactive TUI, its built-in Table widget provides aligned, styled tables with column constraints, borders, and highlighting. No additional dependency needed. For non-TUI output (e.g., piped to a file), a simple text formatter using `format!` with fixed-width columns is sufficient.
**Alternatives considered**:
- `comfy-table`: Good standalone table library, but redundant when ratatui is already present.
- `tabled`: Similar to comfy-table, also redundant.
- `prettytable-rs`: Unmaintained.

## R-009: Date/Time Handling

**Decision**: `chrono` for date/time types and parsing
**Rationale**: chrono is the standard Rust date/time library. Provides `NaiveDateTime`, `NaiveDate`, `NaiveTime` types appropriate for local time tracking (no timezone complexity needed for a personal journal). Parsing supports ISO 8601 out of the box.
**Alternatives considered**:
- `time` crate: Growing alternative but chrono remains more widely used and has better formatting/parsing support for this use case.
- `jiff`: Newer but less battle-tested.
