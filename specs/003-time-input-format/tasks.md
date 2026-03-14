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

## Phase 1: Foundational (Blocking Prerequisites)

**Purpose**: Refactor `parse_time` signature to accept a date parameter, removing hidden coupling to `Local::now()`. This change affects all call sites and must complete before user story work.

- [ ] T001 Change `parse_time` signature in src/formatting/mod.rs from `parse_time(input: &str) -> Result<NaiveDateTime>` to `parse_time(input: &str, date: NaiveDate) -> Result<NaiveDateTime>`; remove ISO 8601 datetime parsing branches; use `%-H:%M` chrono format to accept both `H:MM` and `HH:MM`; update error message to `"Invalid time: '{input}'. Use HH:MM format (e.g., 9:00 or 14:30)"`
- [ ] T002 Update all call sites of `parse_time` in src/services/task.rs (`add_task` and `edit_task`) to pass the appropriate date: in `add_task`, parse `--date` parameter using `formatting::parse_date` (defaulting to today via `clock.now().date()`), pass parsed date to `parse_time` for both `--start` and `--end`; in `edit_task`, derive the date from the existing task's `start_time` date portion (or `created_at` if `start_time` is None) and pass it to `parse_time`
- [ ] T003 Update unit tests in src/formatting/mod.rs: remove `parse_time_iso_datetime` test; update `parse_time_hhmm` to pass a date; add test `parse_time_single_digit_hour` verifying `"9:00"` parses correctly; add test `parse_time_rejects_iso_datetime` verifying full datetime strings are rejected
- [ ] T004 Run `cargo test && cargo clippy` to verify all call sites compile and existing tests pass with the new signature

**Checkpoint**: `parse_time` refactored — user story implementation can begin

---

## Phase 2: User Story 1 - Enter Start/End Times as Clock Times (Priority: P1)

**Goal**: Users enter start and end times using `H:MM` or `HH:MM` notation; times are interpreted on the task's date (today or `--date`)

**Independent Test**: Add a task with `--start 9:00 --end 10:30`, verify it records with correct times and 90-minute duration on today's date

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T005 [P] [US1] Write integration test in tests/cli_task.rs: `adds_task_with_clock_time_start_end` — create project, run `task add --start 9:00 --end 10:30`, assert stdout contains "1h 30m"
- [ ] T006 [P] [US1] Write integration test in tests/cli_task.rs: `adds_task_with_date_and_clock_time` — create project, run `task add --start 9:00 --end 10:30 --date 2026-03-01`, verify task appears in journal for 2026-03-01
- [ ] T007 [P] [US1] Write integration test in tests/cli_task.rs: `rejects_iso_datetime_for_start` — run `task add --start 2026-03-01T09:00 --end 10:30`, assert stderr contains "Invalid time" and exit code 1
- [ ] T008 [P] [US1] Write integration test in tests/edge_cases.rs: `rejects_midnight_spanning_manual_entry` — run `task add --start 23:00 --end 01:00`, assert stderr contains error and exit code 1
- [ ] T009 [P] [US1] Write integration test in tests/edge_cases.rs: `rejects_equal_start_end_time` — run `task add --start 14:00 --end 14:00`, assert stderr contains "End time must be after start time" and exit code 1
- [ ] T010 [P] [US1] Write integration test in tests/edge_cases.rs: `rejects_start_without_colon` — run `task add --start 9 --end 10:00`, assert stderr contains "Invalid time" and exit code 1

### Implementation for User Story 1

- [ ] T011 [US1] Wire the `--date` parameter in src/services/task.rs `add_task`: parse `date` using `formatting::parse_date`, default to today via `clock.now().date()`; pass parsed date to `parse_time` for both `--start` and `--end`
- [ ] T012 [US1] Update src/cli/mod.rs help text for `TaskAction::Add`: change `--start` to `"Start time (HH:MM, 24-hour clock, e.g., 9:00 or 14:30)"`, change `--end` to `"End time (HH:MM, 24-hour clock, e.g., 17:30)"`, change `--date` to `"Date for the entry (YYYY-MM-DD, defaults to today)"`

**Checkpoint**: Users can enter start/end times with clock notation; ISO 8601 datetime is rejected

---

## Phase 3: User Story 2 - Enter Duration in Human-Friendly Format (Priority: P1)

**Goal**: Users enter durations using `1h30m`, `45m`, `2h` notation in addition to plain minutes

**Independent Test**: Add tasks with `--duration 1h30m`, `--duration 45m`, `--duration 2h`, and `--duration 90`; verify all record correct durations

### Tests for User Story 2

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T013 [P] [US2] Write unit test in src/formatting/duration.rs: `parse_duration_case_insensitive` — assert `parse_duration("1H30M")` returns 90
- [ ] T014 [P] [US2] Write unit test in src/formatting/duration.rs: `parse_duration_mixed_case` — assert `parse_duration("1H30m")` returns 90
- [ ] T015 [P] [US2] Write unit test in src/formatting/duration.rs: `parse_duration_normalizes_large_minutes` — assert `parse_duration("1h90m")` returns 150
- [ ] T016 [P] [US2] Write unit test in src/formatting/duration.rs: `parse_duration_rejects_zero_hm` — assert `parse_duration("0h0m")` returns error
- [ ] T017 [P] [US2] Write integration test in tests/cli_task.rs: `adds_task_with_human_duration` — create project, run `task add --duration 1h30m`, assert stdout contains "1h 30m"
- [ ] T018 [P] [US2] Write integration test in tests/cli_task.rs: `adds_task_with_hours_only_duration` — run `task add --duration 2h`, assert stdout contains "2h"
- [ ] T019 [P] [US2] Write integration test in tests/cli_task.rs: `adds_task_with_uppercase_duration` — create project, run `task add --duration 1H30M`, assert stdout contains "1h 30m" and exit code 0
- [ ] T020 [P] [US2] Write unit test in src/formatting/duration.rs: `parse_duration_rejects_spaced_input` — assert `parse_duration("1h 30m")` returns error (spaced format not supported per FR-002)
- [ ] T021 [P] [US2] Write unit test in src/formatting/duration.rs: `parse_duration_trailing_m_optional` — assert `parse_duration("1h30")` returns 90 (trailing `m` is optional per FR-002)

### Implementation for User Story 2

- [ ] T022 [US2] Update `parse_duration` in src/formatting/duration.rs: add `.to_lowercase()` on the input after the plain-integer check; reject input containing whitespace (after trim) with an error; when input contains `h` and digits follow but no `m` is present, treat trailing digits as minutes (e.g., `1h30` → 90); update the error message to `"Invalid duration: '{input}'. Use Xh, Ym, XhYm, or minutes (e.g., 1h30m, 45m, 2h, 90)"`
- [ ] T023 [US2] Update src/cli/mod.rs help text for `TaskAction::Add` and `TaskAction::Edit`: change `--duration` to `"Duration (e.g., 1h30m, 1h30, 45m, 2h, or 90 for minutes)"`

**Checkpoint**: Users can enter durations in human-friendly format; case-insensitive; backward compatible with plain numbers

---

## Phase 4: User Story 3 - Duration Display Consistency (Priority: P2)

**Goal**: Verify that all duration displays across the application use `Xh Ym` format consistently

**Independent Test**: Log tasks and verify journal, reports, and clock status all display durations in `Xh Ym` format

### Tests for User Story 3

- [ ] T024 [P] [US3] Write unit test in src/formatting/duration.rs: `format_duration_consistency` — assert `format_duration(90)` returns `"1h 30m"`, `format_duration(45)` returns `"45m"`, `format_duration(120)` returns `"2h"` (verify existing behavior matches spec)

### Implementation for User Story 3

- [ ] T025 [US3] Audit all duration display call sites in src/cli/clock.rs, src/cli/task.rs, src/tui/journal_view.rs, src/tui/report_view.rs, src/formatting/mod.rs to confirm all use `format_duration()` — no changes expected; document confirmation

**Checkpoint**: All duration displays verified consistent

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Edit command consistency and final validation

- [ ] T026 [P] Write integration test in tests/cli_task.rs: `edits_task_times_with_clock_format` — create task with start/end, edit end time with `--end 11:00`, verify duration is recalculated
- [ ] T027 [P] Update src/cli/mod.rs help text for `TaskAction::Edit`: change `--start` to `"New start time (HH:MM, 24-hour clock)"`, change `--end` to `"New end time (HH:MM, 24-hour clock)"`, change `--duration` to `"New duration (e.g., 1h30m, 45m, 2h, or 90 for minutes)"`
- [ ] T028 Run `cargo test && cargo clippy -- -D warnings && cargo fmt --check` and fix any issues

---

## Dependencies & Execution Order

### Phase Dependencies

- **Foundational (Phase 1)**: No dependencies — start immediately. BLOCKS all user stories.
- **User Story 1 (Phase 2)**: Depends on Foundational (Phase 1) — `parse_time` signature must be refactored first
- **User Story 2 (Phase 3)**: Depends on Foundational only — can proceed in parallel with US1 (different file: duration.rs vs mod.rs)
- **User Story 3 (Phase 4)**: Depends on Foundational only — audit task, no code changes expected
- **Polish (Phase 5)**: Depends on US1 and US2 completion (edit command uses both parse_time and parse_duration)

### User Story Dependencies

- **US1 (Clock Time Input)**: Foundational only — first to implement
- **US2 (Human Duration Input)**: Foundational only — can run in parallel with US1
- **US3 (Display Consistency)**: Foundational only — audit task, no dependencies on US1/US2

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Implementation tasks in dependency order
- Story complete before checkpoint

### Parallel Opportunities

- T005–T010 can all run in parallel (US1 tests, different test functions)
- T013–T021 can all run in parallel (US2 tests, different test functions)
- US1 (Phase 2) and US2 (Phase 3) can proceed in parallel after Phase 1 (different files)
- T026 and T027 can run in parallel (Phase 5)

---

## Implementation Strategy

### MVP First (User Story 1 + 2)

1. Complete Phase 1: Foundational (`parse_time` refactor)
2. Complete Phase 2: US1 (clock time input) — in parallel with Phase 3
3. Complete Phase 3: US2 (human duration input) — in parallel with Phase 2
4. **STOP and VALIDATE**: `task add --start 9:00 --end 10:30` and `task add --duration 1h30m` both work
5. Complete Phase 4: US3 (display audit — quick verification)
6. Complete Phase 5: Polish (edit command + final checks)

### Incremental Delivery

1. Foundational → `parse_time` accepts `H:MM`/`HH:MM` only
2. US1 → Clock time input works for task add → MVP for time input!
3. US2 → Duration input (`1h30m`) works → Complete input format!
4. US3 → Display consistency verified
5. Polish → Edit command updated, all tests pass

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- US1 and US2 are both P1 priority and can proceed in parallel since they modify different files (mod.rs vs duration.rs)
- US3 is primarily an audit/verification task — `format_duration` already outputs `Xh Ym` format
- The `_date` parameter in `add_task` is currently unused — this feature properly wires it
- No new files created; all changes modify existing functions
