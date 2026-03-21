# Implementation Plan: Simplified Time Input Format

**Branch**: `003-fix-date-input` | **Date**: 2026-03-21 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/003-time-input-format/spec.md`

## Summary

Fix a bug where `--date` is silently ignored when using `--start`/`--end` for `task add`, and implement the full simplified time-input format: drop ISO 8601 datetime from `--start`/`--end` (keep date-only ISO 8601 on `--date`/`--from`/`--to`), drop space-separated duration (`"1h 30m"`), add `--date` to `task edit`, and ensure all durations display consistently in `Xh Ym` format.

## Technical Context

**Language/Version**: Rust 2024 edition, MSRV 1.85.0
**Primary Dependencies**: clap v4 (CLI), chrono (date/time), rusqlite (bundled-sqlcipher-vendored-openssl), anyhow (errors)
**Storage**: SQLite with SQLCipher encryption — no schema changes required
**Testing**: `cargo test` (unit + integration via `assert_cmd`)
**Target Platform**: Linux/macOS CLI binary
**Project Type**: CLI tool
**Performance Goals**: All commands < 200 ms (Principle IV) — parsing changes have no performance impact
**Constraints**: No new dependencies; changes must compile with MSRV 1.85.0
**Scale/Scope**: Parsing layer only — 6 files touched, no new files created

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Code Quality — single responsibility, named constants, no dead code | ✅ PASS | Removing ISO 8601 branches reduces code. Removing `_date` prefix fixes dead parameter. |
| II. Testing Standards — TDD, isolated unit tests, integration tests, edge cases | ✅ PASS | tasks.md has failing-test-first ordering; all FRs map to tests |
| III. UX Consistency — ISO 8601 date inputs MUST always work | ⚠️ VIOLATION (justified) | ISO 8601 *datetime* removed from `--start`/`--end`. See justification below. ISO 8601 *date* (`YYYY-MM-DD`) still accepted everywhere (`--date`, `--from`, `--to`, `journal`). |
| IV. Performance — < 200 ms | ✅ PASS | Parsing change is O(1) string ops |

**Constitution Violation Justification** (Principle III):

> The constitution states: "Date and time inputs MUST accept ISO 8601 format."
>
> This feature removes `YYYY-MM-DDTHH:MM` (ISO 8601 *datetime*) from `--start`/`--end`, replacing it with `HH:MM` + a separate `--date` flag. All *date* inputs (`--date`, `--from`, `--to`, `journal [date]`) still accept `YYYY-MM-DD` (ISO 8601 date). The removal applies only to the *time-of-day* arguments.
>
> The prior clarification session (2026-02-28) explicitly approved this tradeoff. The combined datetime form created the bug being fixed: `parse_time("09:00")` ignores `--date` by defaulting to today, while `parse_time("2026-01-15T09:00")` embeds the date in the string, bypassing `--date` entirely.

## Project Structure

### Documentation (this feature)

```text
specs/003-time-input-format/
├── plan.md              ← this file
├── research.md          ← root cause + decisions
├── contracts/
│   └── cli-commands.md  ← before/after command signatures
└── tasks.md             ← implementation tasks
```

### Source Code (affected files only)

```text
src/
├── formatting/
│   ├── mod.rs           # parse_time(): signature change + remove ISO 8601 branches
│   └── duration.rs      # parse_duration(): remove space-separated, add case-insensitive
├── services/
│   └── task.rs          # add_task(): wire _date; edit_task(): accept + apply date
└── cli/
    ├── mod.rs           # TaskAction::Edit: add --date; update help strings
    └── task.rs          # thread --date through edit_task() call

README.md                # remove ISO 8601 example; update duration example
tests/
├── cli_task.rs          # new integration tests (US1, US2, edit)
└── edge_cases.rs        # midnight spanning, equal start/end, missing colon
```

**Structure Decision**: Single project, no new files. All changes are confined to the parsing layer and CLI definitions.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|--------------------------------------|
| Removes ISO 8601 datetime from `--start`/`--end` (Principle III) | Eliminating ambiguity between `--date` and embedded datetime; fixes the root cause bug | Keeping both formats means `--date 2026-01-15 --start 2026-03-21T09:00` is ambiguous; the bug cannot be fixed cleanly without removing the combined form |

## Phase 0: Research

**Status**: Complete — see [research.md](research.md)

Key findings:
1. **Root cause confirmed**: `_date` in `add_task()` is intentionally dead (underscore prefix); fix is the `parse_time()` signature change.
2. `parse_time()` must accept a `date: NaiveDate` parameter — eliminates hidden `Local::now()` dependency.
3. `parse_duration()` space-separated branch removed; `.to_lowercase()` added for case-insensitivity.
4. `--date` added to `task edit`; reuses existing `parse_date()` which already supports all required shortcuts.
5. No new dependencies needed; no schema changes.

## Phase 1: Design & Contracts

**Status**: Complete

### Data Model

No entity changes. `TaskEntry` fields (`start_time`, `end_time`, `duration_min`) remain as-is. Internal storage format unchanged.

Key invariants preserved:
- `start_time` and `end_time` stored as `NaiveDateTime` (date + time)
- `duration_min` stored as `i64` (minutes)
- Date is now always the explicit `--date` value (or today), not inferred from the time string

### Contracts

See [contracts/cli-commands.md](contracts/cli-commands.md).

Summary of changes:
- `task add --start`/`--end`: `YYYY-MM-DDTHH:MM` removed; `H:MM`/`HH:MM` only
- `task add --duration`: `"1h 30m"` (spaced) removed; compact + plain integer only; case-insensitive
- `task add --date`: now applies to `--start`/`--end` too (not just `--duration`); accepts `today`/`yesterday`
- `task edit --date`: new option — moves task to a different date; accepts `YYYY-MM-DD`, `today`, `yesterday`
- All error messages updated to show accepted formats

### Agent Context

Will be updated by the agent context script below.
