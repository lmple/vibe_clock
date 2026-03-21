# Tasks: Simplified Time Input Format

**Input**: Design documents from `/specs/003-time-input-format/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, contracts/

**Tests**: Included per Constitution Principle II (TDD is NON-NEGOTIABLE).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: No new project structure needed. This feature modifies existing files only — no new modules or directories are created.

- [X] T001 Confirm all affected files compile cleanly before changes: run `cargo test && cargo clippy` from repo root; record any pre-existing failures

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Refactor `parse_time` signature to accept an explicit date parameter, removing the hidden dependency on `Local::now()` and fixing the root cause bug (`_date` ignored in `add_task`). This change affects all call sites and **must complete before any user story work begins**.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T002 Change `parse_time` signature in `src/formatting/mod.rs` from `parse_time(input: &str) -> Result<NaiveDateTime>` to `parse_time(input: &str, date: NaiveDate) -> Result<NaiveDateTime>`; remove both ISO 8601 datetime parsing branches (`%Y-%m-%dT%H:%M:%S` and `%Y-%m-%dT%H:%M`); replace `NaiveTime::parse_from_str(input, "%H:%M")` with `"%-H:%M"` to accept both `H:MM` and `HH:MM`; combine parsed time with the passed-in `date` argument instead of `Local::now().date_naive()`; update error message to `"Invalid time: '<input>'. Use HH:MM format (e.g., 9:00 or 14:30)"`
- [X] T003 Update unit tests in `src/formatting/mod.rs`: remove `parse_time_iso_datetime` test; update `parse_time_hhmm` to pass a date argument (e.g., `parse_time("14:30", NaiveDate::from_ymd_opt(2026,3,21).unwrap())`); add `parse_time_single_digit_hour` asserting `parse_time("9:00", date)` returns `09:00` on that date; add `parse_time_rejects_iso_datetime` asserting `parse_time("2026-03-01T09:00", date).is_err()`
- [X] T004 Update all call sites of `parse_time` in `src/services/task.rs`: in `add_task`, parse the `_date` parameter (rename to `date`) using `formatting::parse_date`, defaulting to `clock.now().date_naive()` when `None`; pass the resolved date to both `parse_time(start_str, task_date)` and `parse_time(end_str, task_date)`; in `edit_task`, derive the date from the existing task's `start_time.map(|t| t.date()).unwrap_or_else(|| clock.now().date_naive())` and pass it to `parse_time`
- [X] T005 Run `cargo test && cargo clippy` to verify all call sites compile and existing (updated) tests pass

**Checkpoint**: `parse_time` refactored — user story implementation can begin

---

## Phase 3: User Story 1 - Enter Start/End Times as Clock Times (Priority: P1) 🎯 MVP

**Goal**: Users enter start/end times using `H:MM`/`HH:MM` notation; times are interpreted on the task's date (today or via `--date` on both `task add` and `task edit`)

**Independent Test**: `vibe-clock task add "Acme" "Standup" --start 9:00 --end 10:30` records a task with a 90-minute duration on today's date; `--date 2026-03-01` makes it record for that date instead; `task edit 1 --date yesterday` moves the task's date

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T006 [P] [US1] Write integration test `adds_task_with_clock_time_start_end` in `tests/cli_task.rs`: create project, run `task add "proj" "desc" --start 9:00 --end 10:30`, assert exit code 0 and stdout contains "1h 30m"
- [X] T007 [P] [US1] Write integration test `adds_task_with_date_and_clock_time` in `tests/cli_task.rs`: create project, run `task add "proj" "desc" --start 9:00 --end 10:30 --date 2026-03-01`, then run `journal 2026-03-01`, assert stdout contains the task description
- [X] T008 [P] [US1] Write integration test `adds_task_date_shortcut_yesterday` in `tests/cli_task.rs`: run `task add "proj" "desc" --duration 30 --date yesterday`, then run `journal yesterday`, assert stdout contains the task description
- [X] T009 [P] [US1] Write integration test `rejects_iso_datetime_for_start` in `tests/cli_task.rs`: run `task add "proj" "desc" --start 2026-03-01T09:00 --end 10:30`, assert exit code 1 and stderr contains "Invalid time"
- [X] T010 [P] [US1] Write integration test `rejects_equal_start_end_time` in `tests/edge_cases.rs`: run `task add "proj" "desc" --start 14:00 --end 14:00`, assert exit code 1 and stderr contains "End time must be after start time"
- [X] T011 [P] [US1] Write integration test `rejects_midnight_spanning_entry` in `tests/edge_cases.rs`: run `task add "proj" "desc" --start 23:00 --end 01:00`, assert exit code 1 and stderr contains error
- [X] T012 [P] [US1] Write integration test `rejects_start_time_without_colon` in `tests/edge_cases.rs`: run `task add "proj" "desc" --start 9 --end 10:00`, assert exit code 1 and stderr contains "Invalid time"
- [X] T013 [P] [US1] Write integration test `edits_task_with_date_flag` in `tests/cli_task.rs`: add a task, run `task edit <id> --date yesterday`, then run `journal yesterday`, assert task appears there; verify it no longer appears in today's journal
- [X] T014 [P] [US1] Write integration test `edits_task_start_end_uses_existing_date` in `tests/cli_task.rs`: add a task with `--date 2026-03-01 --start 9:00 --end 10:00`, edit with `--end 11:00` (no `--date`), assert duration becomes 2h and task date is still 2026-03-01

### Implementation for User Story 1

- [X] T015 [US1] Add `--date` field to `TaskAction::Edit` in `src/cli/mod.rs`: `#[arg(long)] date: Option<String>` with help text `"Move to a different date (YYYY-MM-DD, 'today', or 'yesterday')"`; update `--start` help to `"New start time (HH:MM, 24-hour clock, e.g., 9:00 or 14:30)"`; update `--end` help to `"New end time (HH:MM, 24-hour clock)"`
- [X] T016 [US1] Update `src/cli/mod.rs` `TaskAction::Add` help strings: change `--start` help to `"Start time (HH:MM, 24-hour clock, e.g., 9:00 or 14:30)"`; change `--end` help to `"End time (HH:MM, 24-hour clock)"`;  change `--date` help to `"Date for the entry (YYYY-MM-DD, 'today', or 'yesterday'; defaults to today)"`
- [X] T017 [US1] Update `edit_task` signature in `src/services/task.rs` to accept `date: Option<&str>`; when `--date` is provided, resolve it via `formatting::parse_date`; use that date to reinterpret `--start`/`--end` times via `parse_time`; when only `--date` is provided without new `--start`/`--end`, update start/end timestamps to move the existing times to the new date (preserve time-of-day, change date component); update `duration_min` only when times change
- [X] T018 [US1] Thread `--date` through the `task edit` call in `src/cli/task.rs`: pass `date.as_deref()` to `edit_task()` alongside existing parameters

**Checkpoint**: Users can enter start/end times with clock notation; `--date` works on both `task add` and `task edit`; ISO 8601 datetime is rejected

---

## Phase 4: User Story 2 - Enter Duration in Human-Friendly Format (Priority: P1)

**Goal**: Users enter durations using compact notation (`1h30m`, `45m`, `2h`) or plain minutes; case-insensitive; space-separated format rejected

**Independent Test**: `vibe-clock task add "Acme" "desc" --duration 1h30m` records a 90-minute task; `--duration 1H30M` works identically; `--duration "1h 30m"` fails with error

### Tests for User Story 2

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T019 [P] [US2] Write unit test `parse_duration_rejects_spaced_input` in `src/formatting/duration.rs`: assert `parse_duration("1h 30m").is_err()` (space-separated format not supported per FR-002 / clarification 2026-03-21)
- [X] T020 [P] [US2] Write unit test `parse_duration_case_insensitive` in `src/formatting/duration.rs`: assert `parse_duration("1H30M").unwrap() == 90`
- [X] T021 [P] [US2] Write unit test `parse_duration_mixed_case` in `src/formatting/duration.rs`: assert `parse_duration("1H30m").unwrap() == 90`
- [X] T022 [P] [US2] Write unit test `parse_duration_normalizes_large_minutes` in `src/formatting/duration.rs`: assert `parse_duration("1h90m").unwrap() == 150`
- [X] T023 [P] [US2] Write unit test `parse_duration_rejects_zero_hm` in `src/formatting/duration.rs`: assert `parse_duration("0h0m").is_err()`
- [X] T024 [P] [US2] Write unit test `parse_duration_trailing_m_optional` in `src/formatting/duration.rs`: assert `parse_duration("1h30").unwrap() == 90`
- [X] T025 [P] [US2] Write integration test `adds_task_with_human_duration` in `tests/cli_task.rs`: create project, run `task add "proj" "desc" --duration 1h30m`, assert exit code 0 and stdout contains "1h 30m"
- [X] T026 [P] [US2] Write integration test `adds_task_with_uppercase_duration` in `tests/cli_task.rs`: run `task add "proj" "desc" --duration 1H30M`, assert exit code 0 and stdout contains "1h 30m"
- [X] T027 [P] [US2] Write integration test `rejects_spaced_duration` in `tests/cli_task.rs`: run `task add "proj" "desc" --duration "1h 30m"`, assert exit code 1 and stderr contains "Invalid duration"

### Implementation for User Story 2

- [X] T028 [US2] Update `parse_duration` in `src/formatting/duration.rs`: after the plain-integer check, reject input containing internal whitespace (after trim) with error `"Invalid duration: '<input>'. Use Xh, Ym, XhYm, or minutes (e.g., 1h30m, 45m, 2h, 90)"`; add `.to_lowercase()` conversion before parsing; when input contains `h` and digits follow with no trailing `m`, treat them as minutes (e.g., `1h30` → 90); update the error message for all invalid format paths
- [X] T029 [US2] Update `--duration` help text in `src/cli/mod.rs` for both `TaskAction::Add` and `TaskAction::Edit`: `"Duration (e.g., 1h30m, 45m, 2h, or 90 for minutes)"`

**Checkpoint**: Users can enter durations in compact human-friendly format; space-separated format is rejected; plain integers still work

---

## Phase 5: User Story 3 - Duration Display Consistency (Priority: P2)

**Goal**: Verify all duration displays across the application use `Xh Ym` format; `format_duration()` already outputs this — this phase is an audit

**Independent Test**: Log a 90-minute task; verify journal shows `1h 30m`, report shows `1h 30m`, clock status shows elapsed in `Xh Ym` format

### Tests for User Story 3

- [X] T030 [P] [US3] Write unit test `format_duration_consistency_check` in `src/formatting/duration.rs`: assert `format_duration(90) == "1h 30m"`, `format_duration(45) == "45m"`, `format_duration(120) == "2h"`, `format_duration(65) == "1h 5m"` (verifies existing behavior matches spec SC-003)

### Implementation for User Story 3

- [X] T031 [US3] Audit all duration display call sites: search for all uses of `format_duration` in `src/cli/clock.rs`, `src/cli/task.rs`, `src/cli/journal.rs`, `src/cli/report.rs`, `src/formatting/mod.rs` and any TUI files — confirm every display path routes through `format_duration()`; if any raw minute value is printed without `format_duration`, fix it

**Checkpoint**: All duration displays verified consistent — `format_duration()` is the single source of truth

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation update and final validation

- [X] T032 [P] Update `README.md`: replace `--start 2026-02-28T09:00 --end 2026-02-28T09:15` example with `--start 09:00 --end 09:15 --date 2026-02-28`; replace `--duration "1h 30m"` example with `--duration 1h30m`; add `--date` option to the task edit usage example showing `--date yesterday`
- [X] T033 Run full test suite and linting: `cargo test && cargo clippy -- -D warnings && cargo fmt --check`; fix any failures

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Setup — **BLOCKS all user stories**
- **User Story 1 (Phase 3)**: Depends on Foundational (T002–T005)
- **User Story 2 (Phase 4)**: Depends on Foundational only — can proceed **in parallel with US1** (different file: `duration.rs` vs `mod.rs`)
- **User Story 3 (Phase 5)**: Depends on Foundational only — audit task, no dependencies on US1/US2
- **Polish (Phase 6)**: Depends on US1 and US2 completion

### User Story Dependencies

- **US1 (Clock Time Input)**: Foundational only — first to implement
- **US2 (Human Duration Input)**: Foundational only — parallel with US1
- **US3 (Display Consistency)**: Foundational only — audit/verify task

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Implementation tasks in dependency order within the phase
- Story complete before Polish phase begins

### Parallel Opportunities

- T006–T014 (US1 tests) can all run in parallel — different test functions
- T019–T027 (US2 tests) can all run in parallel — different test/unit functions
- US1 (Phase 3) and US2 (Phase 4) can proceed in parallel after Phase 2 — different files
- T032 and T033 can start once US1+US2 are done

---

## Parallel Example: User Story 1

```bash
# Write all US1 tests in parallel (different functions):
Task T006: adds_task_with_clock_time_start_end in tests/cli_task.rs
Task T007: adds_task_with_date_and_clock_time in tests/cli_task.rs
Task T008: adds_task_date_shortcut_yesterday in tests/cli_task.rs
Task T009: rejects_iso_datetime_for_start in tests/cli_task.rs
Task T010: rejects_equal_start_end_time in tests/edge_cases.rs
Task T011: rejects_midnight_spanning_entry in tests/edge_cases.rs
Task T012: rejects_start_time_without_colon in tests/edge_cases.rs
Task T013: edits_task_with_date_flag in tests/cli_task.rs
Task T014: edits_task_start_end_uses_existing_date in tests/cli_task.rs

# Then implement sequentially (shared file dependencies):
Task T015: Add --date to TaskAction::Edit in src/cli/mod.rs
Task T016: Update TaskAction::Add help strings in src/cli/mod.rs
Task T017: Update edit_task() in src/services/task.rs
Task T018: Thread --date through src/cli/task.rs
```

## Parallel Example: User Story 2

```bash
# Write all US2 tests in parallel:
Task T019: parse_duration_rejects_spaced_input in src/formatting/duration.rs
Task T020: parse_duration_case_insensitive in src/formatting/duration.rs
Task T021: parse_duration_mixed_case in src/formatting/duration.rs
Task T022: parse_duration_normalizes_large_minutes in src/formatting/duration.rs
Task T023: parse_duration_rejects_zero_hm in src/formatting/duration.rs
Task T024: parse_duration_trailing_m_optional in src/formatting/duration.rs
Task T025: adds_task_with_human_duration in tests/cli_task.rs
Task T026: adds_task_with_uppercase_duration in tests/cli_task.rs
Task T027: rejects_spaced_duration in tests/cli_task.rs

# Then implement:
Task T028: Update parse_duration() in src/formatting/duration.rs
Task T029: Update --duration help text in src/cli/mod.rs
```

---

## Implementation Strategy

### MVP First (User Story 1 + 2 — both P1)

1. Complete Phase 1: Setup (T001)
2. Complete Phase 2: Foundational (T002–T005) — **blocking**
3. Complete Phase 3 (US1) and Phase 4 (US2) in **parallel** — different files
4. **STOP and VALIDATE**: `task add --start 9:00 --end 10:30`, `task add --duration 1h30m`, `task edit <id> --date yesterday` all work
5. Complete Phase 5: US3 (audit — quick verification)
6. Complete Phase 6: Polish (README + final checks)

### Incremental Delivery

1. Foundational → `parse_time` accepts `H:MM`/`HH:MM` only; `--date` is wired
2. US1 → Clock time input + `task edit --date` → Core bug fixed!
3. US2 → Compact duration input → Complete input format simplification
4. US3 → Display consistency confirmed
5. Polish → README updated; all tests green

---

## Notes

- [P] tasks = different files, no dependencies within that phase
- [Story] label maps task to specific user story for traceability
- US1 and US2 are both P1 and can proceed **in parallel** after Phase 2 (different files: `mod.rs` vs `duration.rs`)
- US3 is an audit — `format_duration` already outputs `Xh Ym`; no code changes expected
- The `_date` underscore prefix bug (root cause) is fixed in T004 as part of Foundational
- `task edit --date` is new functionality added in this session (clarification 2026-03-21); T013–T014 and T015–T018 cover it
- No new files created; all changes modify existing functions
