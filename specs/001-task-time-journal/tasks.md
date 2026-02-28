# Tasks: Task Time Journal

**Input**: Design documents from `/specs/001-task-time-journal/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

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

**Purpose**: Project initialization, Cargo workspace, and tooling configuration

- [X] T001 Initialize Rust project with `cargo init --name vibe-clock` and set edition = "2024", rust-version = "1.85.0" in Cargo.toml
- [X] T002 Add all dependencies to Cargo.toml: clap v4 (derive), ratatui, crossterm, rusqlite (bundled-sqlcipher-vendored-openssl), chrono, keyring, dirs, anyhow; dev-dependencies: assert_cmd, predicates, tempfile
- [X] T003 [P] Configure release profile in Cargo.toml: strip = true, lto = true, codegen-units = 1
- [X] T004 [P] Create directory structure: src/cli/, src/db/, src/models/, src/services/, src/tui/, src/formatting/, tests/common/
- [X] T005 [P] Add rustfmt.toml and .clippy.toml with project formatting and lint rules
- [X] T006 [P] Configure git pre-commit hook running `cargo fmt --check && cargo clippy -- -D warnings` per Constitution Quality Gates
- [X] T007 Create src/error.rs with AppError enum (UserError, SystemError) mapping to exit codes 1 and 2, implementing std::error::Error and From<rusqlite::Error>

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**CRITICAL**: No user story work can begin until this phase is complete

- [X] T008 Create src/clock_trait.rs with Clock trait (fn now() -> NaiveDateTime) and SystemClock implementation
- [X] T009 Create src/config.rs with database path resolution: check VIBE_CLOCK_DB env var, fall back to dirs::data_dir()/vibe-clock/vibe-clock.db, create parent directories if needed
- [X] T010 Create src/crypto.rs with passphrase management: try keyring first, fall back to VIBE_CLOCK_KEY env var, fall back to terminal prompt; store new passphrase in keyring on first use
- [X] T011 Create src/db/mod.rs with Database struct: open connection, apply PRAGMA key, set journal_mode=DELETE, locking_mode=EXCLUSIVE, synchronous=FULL, busy_timeout=5000, foreign_keys=ON; run schema migrations
- [X] T012 Create SQL schema initialization in src/db/mod.rs: CREATE TABLE IF NOT EXISTS for project, task_entry, clock_state, schema_version tables with indexes per data-model.md
- [X] T013 [P] Create src/models/project.rs with Project struct (id, name, created_at, updated_at) deriving Debug, Clone
- [X] T014 [P] Create src/models/task_entry.rs with TaskEntry struct (id, project_id, description, start_time, end_time, duration_min, created_at, updated_at) deriving Debug, Clone
- [X] T015 [P] Create src/models/clock_state.rs with ClockState struct (id, project_id, description, start_time) deriving Debug, Clone
- [X] T016 Create src/models/mod.rs re-exporting Project, TaskEntry, ClockState
- [X] T017 Create src/formatting/duration.rs with format_duration(minutes: i64) -> String returning "Xh Ym" format, and parse_duration(input: &str) -> Result<i64>
- [X] T018 Create src/formatting/mod.rs with date parsing helpers: parse_date(input) supporting ISO 8601, "today", "yesterday"; parse_time(input) supporting ISO 8601 and HH:MM (assumes today); re-export duration formatting per Constitution Principle III
- [X] T019 Add non-TUI text output functions in src/formatting/mod.rs: format_task_table(tasks) -> String for plain-text tabular output with aligned columns, format_totals(per_project, grand) -> String; used when stdout is not a terminal (piped output) and by integration tests
- [X] T020 Create src/cli/mod.rs with top-level clap App definition using derive macros: Cli struct with subcommands for Project, Clock, Task, Journal, Report; include --version and --help
- [X] T021 Create src/main.rs entry point: parse CLI args via clap, resolve DB path, open encrypted database, dispatch to subcommand handlers
- [X] T022 Create tests/common/mod.rs with test helpers: create_test_db() returning (TempDir, Database) with unencrypted temp SQLite, FakeClock struct implementing Clock trait
- [X] T023 [P] Write integration test in tests/common/mod.rs or tests/crypto_test.rs: opens_db_with_env_key (set VIBE_CLOCK_KEY env var, open encrypted DB, write and read back a project, assert success)
- [X] T024 [P] Write integration test in tests/common/mod.rs or tests/crypto_test.rs: rejects_wrong_key (create encrypted DB with key A, try opening with key B, assert error)

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Manage Projects (Priority: P1)

**Goal**: Users can create, list, edit, and delete projects from the CLI

**Independent Test**: Create, list, edit, and delete projects from the CLI; verify persistence across process restarts

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T025 [P] [US1] Write integration test in tests/cli_project.rs: creates_project_and_appears_in_list (run `vibe-clock project add "Acme"`, then `vibe-clock project list`, assert stdout contains "Acme")
- [X] T026 [P] [US1] Write integration test in tests/cli_project.rs: rejects_duplicate_project_name (create "Acme" twice, assert stderr contains "already exists", exit code 1)
- [X] T027 [P] [US1] Write integration test in tests/cli_project.rs: edits_project_name (create "Acme", edit to "Beta", list shows "Beta" not "Acme")
- [X] T028 [P] [US1] Write integration test in tests/cli_project.rs: deletes_project_without_tasks (create project, delete with --yes, list shows empty)
- [X] T029 [P] [US1] Write integration test in tests/cli_project.rs: deletes_project_with_tasks_warns_and_cascades (create project, add task, delete project with --yes, assert both project and task gone; without --yes assert confirmation prompt)
- [X] T030 [P] [US1] Write integration test in tests/cli_project.rs: lists_empty_projects_with_help_message (run list with no projects, assert stdout contains "No projects found")

### Implementation for User Story 1

- [X] T031 [US1] Create src/db/project.rs with ProjectRepo: insert(name) -> Result<Project>, list() -> Result<Vec<Project>>, find_by_id(id) -> Result<Option<Project>>, find_by_name(name) -> Result<Option<Project>>, update_name(id, new_name) -> Result<()>, delete(id) -> Result<()>, count_tasks(id) -> Result<i64>
- [X] T032 [US1] Create src/services/project.rs with ProjectService: create(name), list(), rename(id, new_name), delete(id, force) with validation (non-empty name, unique name, trim whitespace, cascade warning)
- [X] T033 [US1] Create src/cli/project.rs with clap subcommands: ProjectCommand enum (Add, List, Edit, Delete) with args per contracts/cli-commands.md; handle_project() dispatching to ProjectService
- [X] T034 [US1] Wire project subcommand in src/cli/mod.rs and src/main.rs dispatch

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Log Tasks with Clocked Time (Priority: P1)

**Goal**: Users can start a timer, stop it, and have the elapsed time recorded as a task entry; clock state persists across crashes

**Independent Test**: Start a clock on a project, stop it, verify the task entry is saved with correct duration; kill process while clock running, restart, verify clock state recoverable

### Tests for User Story 2

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T035 [P] [US2] Write integration test in tests/cli_clock.rs: starts_clock_and_shows_confirmation (create project, start clock, assert stdout contains "Clock started")
- [X] T036 [P] [US2] Write integration test in tests/cli_clock.rs: stops_clock_and_logs_task (start clock, stop clock, assert stdout contains "Clock stopped" and duration)
- [X] T037 [P] [US2] Write integration test in tests/cli_clock.rs: shows_clock_status_when_running (start clock, check status, assert stdout contains project name, description, and elapsed time)
- [X] T038 [P] [US2] Write integration test in tests/cli_clock.rs: rejects_second_clock_start (start clock, try starting another, assert stderr contains "already running", exit code 1)
- [X] T039 [P] [US2] Write integration test in tests/cli_clock.rs: reports_no_clock_running_on_stop (stop without starting, assert stderr contains "No clock is running", exit code 1)
- [X] T040 [P] [US2] Write integration test in tests/cli_clock.rs: recovers_clock_state_after_crash (start clock, exit without stopping, relaunch app, assert warning about running clock with elapsed time; verify clock can be stopped and task is recorded correctly) [FR-016]

### Implementation for User Story 2

- [X] T041 [US2] Create src/db/clock_state.rs with ClockStateRepo: insert(project_id, description, start_time) -> Result<()>, get() -> Result<Option<ClockState>>, delete() -> Result<()>
- [X] T042 [US2] Create src/db/task_entry.rs with TaskEntryRepo: insert(project_id, description, start_time, end_time, duration_min) -> Result<TaskEntry> (used by clock stop and manual add)
- [X] T043 [US2] Create src/services/clock.rs with ClockService: start(project, description, clock) -> Result<()>, stop(clock) -> Result<TaskEntry>, status(clock) -> Result<Option<ClockStatus>>, recover() -> Result<Option<ClockState>>; start validates no clock running and project exists; stop calculates duration and wraps DELETE+INSERT in transaction; recover checks for existing clock state on startup [FR-016]
- [X] T044 [US2] Create src/cli/clock.rs with clap subcommands: ClockCommand enum (Start, Stop, Status) with args per contracts/cli-commands.md; handle_clock() dispatching to ClockService
- [X] T045 [US2] Implement clock crash recovery in src/main.rs: on startup, check ClockStateRepo for existing row via ClockService::recover(); if found, print warning with task description, project, and elapsed time; offer to stop or continue [FR-016]
- [X] T046 [US2] Wire clock subcommand in src/cli/mod.rs and src/main.rs dispatch

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Log Tasks with Manual Time Entry (Priority: P1)

**Goal**: Users can manually log task entries with start/end times or duration only

**Independent Test**: Create a task with --start/--end, create another with --duration only, verify both appear correctly

### Tests for User Story 3

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T047 [P] [US3] Write integration test in tests/cli_task.rs: adds_task_with_start_and_end (create project, add task with --start 09:00 --end 10:30, assert stdout contains "1h 30m")
- [X] T048 [P] [US3] Write integration test in tests/cli_task.rs: adds_task_with_duration_only (create project, add task with --duration 45, assert stdout contains "45m")
- [X] T049 [P] [US3] Write integration test in tests/cli_task.rs: rejects_end_before_start (add task with --start 10:00 --end 09:00, assert stderr contains "End time must be after start time", exit code 1)
- [X] T050 [P] [US3] Write integration test in tests/cli_task.rs: rejects_nonexistent_project (add task for unknown project, assert stderr contains "not found", exit code 1)

### Implementation for User Story 3

- [X] T051 [US3] Create src/services/task.rs with TaskService: add(project, description, start, end, duration, date, clock) -> Result<TaskEntry> with validation (end > start, duration > 0, project exists, compute duration from times or accept duration-only). Note: US6 will extend this service with edit() and delete() methods.
- [X] T052 [US3] Create src/cli/task.rs with clap subcommands: TaskCommand enum (Add with --start, --end, --duration, --date options) per contracts/cli-commands.md; handle_task() dispatching to TaskService
- [X] T053 [US3] Wire task subcommand in src/cli/mod.rs and src/main.rs dispatch

**Checkpoint**: All P1 user stories should now be independently functional

---

## Phase 6: TUI Infrastructure

**Purpose**: Shared TUI components required by US4 (Journal) and US5 (Reports)

**CRITICAL**: Must complete before Phase 7 and Phase 8

- [X] T054 Create src/tui/mod.rs with TUI app scaffold: Terminal setup/teardown with crossterm (enable/disable raw mode, alternate screen), event loop reading crossterm events, App state struct
- [X] T055 Create src/tui/input.rs with key binding handler: q/Esc to quit, up/down for navigation, common key mappings shared across views
- [X] T056 Create src/tui/widgets.rs with reusable ratatui widgets: TaskTable widget (renders Vec<TaskEntry> as Table with column widths), ClockStatusBar widget (shows running clock with live elapsed time), DurationCell widget (formats minutes as "Xh Ym")

**Checkpoint**: TUI infrastructure ready for journal and report views

---

## Phase 7: User Story 4 - View Daily Task Journal (Priority: P2)

**Goal**: Users can view all tasks for a specific day with per-project totals and grand total

**Independent Test**: Log tasks across multiple projects and days, view journal for a specific day, verify table output with correct totals

**Depends on**: TUI Infrastructure (Phase 6)

### Tests for User Story 4

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T057 [P] [US4] Write integration test in tests/cli_journal.rs: shows_todays_tasks_by_default (add tasks for today, run `vibe-clock journal`, assert output contains task descriptions, project names, durations)
- [X] T058 [P] [US4] Write integration test in tests/cli_journal.rs: shows_tasks_for_specific_date (add tasks for 2026-02-25, run `vibe-clock journal 2026-02-25`, assert correct tasks shown)
- [X] T059 [P] [US4] Write integration test in tests/cli_journal.rs: shows_empty_message_for_date_with_no_tasks (run journal for a date with no tasks, assert stdout contains "No tasks logged")
- [X] T060 [P] [US4] Write integration test in tests/cli_journal.rs: shows_per_project_totals_and_grand_total (add tasks across 2 projects, assert output contains totals for each project and grand total)

### Implementation for User Story 4

- [X] T061 [US4] Add query methods to src/db/task_entry.rs: list_by_date(date: NaiveDate) -> Result<Vec<TaskEntry>> using WHERE on start_time date portion or created_at for duration-only entries
- [X] T062 [US4] Create src/services/journal.rs with JournalService: get_daily_journal(date, clock) -> Result<DailyJournal> returning tasks grouped by project with per-project and grand totals
- [X] T063 [US4] Create src/tui/journal_view.rs with ratatui-based interactive journal table: columns for ID, Project, Description, Start, End, Duration; summary row with totals; keyboard navigation with up/down arrows; q to quit
- [X] T064 [US4] Create src/cli/journal.rs with clap args: optional date argument (defaults to today); handle_journal() that launches TUI journal view or falls back to text output (via src/formatting) if stdout is not a terminal
- [X] T065 [US4] Wire journal subcommand in src/cli/mod.rs and src/main.rs dispatch

**Checkpoint**: At this point, User Stories 1-4 should all work independently

---

## Phase 8: User Story 5 - Generate Reports by Date Range (Priority: P2)

**Goal**: Users can generate time reports for a selected date range showing tasks grouped by project with day-by-day breakdown

**Independent Test**: Log tasks over multiple days, generate report for the range, verify groupings, daily breakdowns, and totals

**Depends on**: TUI Infrastructure (Phase 6)

### Tests for User Story 5

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T066 [P] [US5] Write integration test in tests/cli_report.rs: generates_report_grouped_by_project (add tasks across projects and dates, run report, assert output groups by project with totals)
- [X] T067 [P] [US5] Write integration test in tests/cli_report.rs: shows_day_by_day_breakdown (add tasks across multiple days, assert report shows per-day entries within each project)
- [X] T068 [P] [US5] Write integration test in tests/cli_report.rs: shows_empty_message_for_range_with_no_data (run report for empty range, assert stdout contains "No tasks found")
- [X] T069 [P] [US5] Write integration test in tests/cli_report.rs: rejects_from_after_to (run report with --from after --to, assert stderr error, exit code 1)

### Implementation for User Story 5

- [X] T070 [US5] Add query methods to src/db/task_entry.rs: list_by_date_range(from: NaiveDate, to: NaiveDate) -> Result<Vec<TaskEntry>>
- [X] T071 [US5] Create src/services/report.rs with ReportService: generate(from, to) -> Result<Report> returning tasks grouped by project, each project containing day-by-day breakdown with subtotals and grand total
- [X] T072 [US5] Create src/tui/report_view.rs with ratatui-based interactive report: collapsible project sections, day-by-day rows, totals row; keyboard navigation; q to quit
- [X] T073 [US5] Create src/cli/report.rs with clap args: --from and --to (required), handle_report() launching TUI report view or text fallback
- [X] T074 [US5] Wire report subcommand in src/cli/mod.rs and src/main.rs dispatch

**Checkpoint**: At this point, User Stories 1-5 should all work independently

---

## Phase 9: User Story 6 - Edit and Delete Task Entries (Priority: P3)

**Goal**: Users can edit or delete previously logged task entries

**Independent Test**: Create task entries, edit their fields, delete some, verify changes persist in journal and reports

### Tests for User Story 6

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T075 [P] [US6] Write integration test in tests/cli_task.rs: edits_task_description (create task, edit description, verify journal shows updated description)
- [X] T076 [P] [US6] Write integration test in tests/cli_task.rs: edits_task_times (create task with start/end, edit end time, verify duration recalculated)
- [X] T077 [P] [US6] Write integration test in tests/cli_task.rs: moves_task_to_different_project (create task in project A, edit --project to B, verify task appears under project B)
- [X] T078 [P] [US6] Write integration test in tests/cli_task.rs: deletes_task_with_yes_flag (create task, delete with --yes, verify task gone from journal)
- [X] T079 [P] [US6] Write integration test in tests/cli_task.rs: rejects_edit_nonexistent_task (edit task with invalid ID, assert stderr contains "not found", exit code 1)

### Implementation for User Story 6

- [X] T080 [US6] Add query methods to src/db/task_entry.rs: find_by_id(id) -> Result<Option<TaskEntry>>, update(id, fields) -> Result<()>, delete(id) -> Result<()>
- [X] T081 [US6] Add methods to src/services/task.rs: edit(id, description, project, start, end, duration) -> Result<TaskEntry> with validation and duration recalculation; delete(id, force) -> Result<()> with confirmation
- [X] T082 [US6] Add Edit and Delete subcommands to src/cli/task.rs with args per contracts/cli-commands.md; dispatch to TaskService

**Checkpoint**: All user stories should now be independently functional

---

## Phase 10: Edge Cases

**Purpose**: Tests for edge cases identified in the spec

- [X] T083 [P] Write edge case test in tests/edge_cases.rs: clock_state_persists_after_crash (start clock, drop connection without stopping, reopen DB, assert clock state recoverable via ClockService::recover)
- [ ] T084 [P] Write edge case test in tests/edge_cases.rs: warns_on_future_date_task (add task with future date, assert warning in stderr but exit code 0)
- [X] T085 [P] Write edge case test in tests/edge_cases.rs: delete_project_with_running_clock_stops_clock_first (create project, start clock, delete project with --yes, assert clock stopped and project deleted)
- [X] T086 [P] Write edge case test in tests/edge_cases.rs: clock_crossing_midnight_records_correct_times (start clock at 23:55, stop at 00:05 next day, verify start_time date = original day, duration = 10 minutes)
- [ ] T087 [P] Write edge case test in tests/edge_cases.rs: duration_only_task_appears_in_correct_date_journal (add task with --duration 60 --date 2026-03-01, verify it appears in journal for 2026-03-01 not today)

---

## Phase 11: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T088 [P] Add --help descriptions to all clap subcommands and arguments per contracts/cli-commands.md
- [X] T089 [P] Add performance benchmark test in tests/bench.rs: seed database with 2,000 task entries and 50 projects, measure journal query time (<200ms) and yearly report generation time (<1s) per Constitution Principle IV
- [ ] T090 Run quickstart.md validation: execute the quickstart workflow end-to-end and verify all commands produce expected output
- [X] T091 [P] Run `cargo clippy -- -D warnings` and `cargo fmt --check` and fix any issues

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational - BLOCKS User Stories 2-6 (Project entity needed)
- **User Story 2 (Phase 4)**: Depends on US1 (needs Project to exist) + Foundational
- **User Story 3 (Phase 5)**: Depends on US1 (needs Project) + US2 (shares TaskEntryRepo)
- **TUI Infrastructure (Phase 6)**: Depends on Foundational only; can proceed in parallel with US1-US3. BLOCKS US4 and US5.
- **User Story 4 (Phase 7)**: Depends on US1 + TaskEntryRepo + TUI Infrastructure (Phase 6)
- **User Story 5 (Phase 8)**: Depends on US4 (extends journal concept to date ranges) + TUI Infrastructure (Phase 6)
- **User Story 6 (Phase 9)**: Depends on US3 (extends task add with edit/delete)
- **Edge Cases (Phase 10)**: Can start after US2 (most edge cases involve clock)
- **Polish (Phase 11)**: Depends on all desired user stories being complete

### User Story Dependencies

- **US1 (Manage Projects)**: Foundation only - first story to implement
- **US2 (Clock Time)**: Depends on US1 (needs projects + TaskEntryRepo)
- **US3 (Manual Time)**: Depends on US1 (needs projects) + uses TaskEntryRepo from US2
- **US4 (Daily Journal)**: Depends on US1 + TaskEntryRepo + TUI Infrastructure
- **US5 (Reports)**: Depends on US4 (extends query patterns) + TUI Infrastructure
- **US6 (Edit/Delete)**: Depends on US3 (extends TaskService and CLI)

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Models/DB layer before services
- Services before CLI handlers
- CLI wiring last
- Story complete before moving to next priority

### Parallel Opportunities

- T003, T004, T005, T006 can run in parallel (Phase 1)
- T008, T009, T010 can run in parallel (Phase 2 - no dependencies between them)
- T013, T014, T015 can run in parallel (model structs are independent)
- T023, T024 can run in parallel (crypto tests)
- All test tasks within a user story (T025-T030, T035-T040, etc.) can run in parallel
- TUI Infrastructure (Phase 6) can proceed in parallel with US1-US3
- Edge case tests (T083-T087) can all run in parallel
- T088, T089, T091 can run in parallel (Phase 11)

---

## Parallel Example: User Story 1

```bash
# Launch all tests for US1 together:
Task: "Write integration test creates_project_and_appears_in_list in tests/cli_project.rs"
Task: "Write integration test rejects_duplicate_project_name in tests/cli_project.rs"
Task: "Write integration test edits_project_name in tests/cli_project.rs"
Task: "Write integration test deletes_project_without_tasks in tests/cli_project.rs"
Task: "Write integration test deletes_project_with_tasks_warns_and_cascades in tests/cli_project.rs"
Task: "Write integration test lists_empty_projects_with_help_message in tests/cli_project.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Manage Projects)
4. **STOP and VALIDATE**: Test project CRUD independently
5. Demo: `vibe-clock project add/list/edit/delete` working

### Incremental Delivery

1. Complete Setup + Foundational -> Foundation ready
2. Add User Story 1 (Projects) -> Test independently -> MVP!
3. Add User Story 2 (Clock) -> Test independently -> Core time tracking with crash recovery!
4. Add User Story 3 (Manual Entry) -> Test independently -> Complete input methods
5. Add TUI Infrastructure -> Enable interactive views
6. Add User Story 4 (Journal) -> Test independently -> Daily review capability
7. Add User Story 5 (Reports) -> Test independently -> Analytical value
8. Add User Story 6 (Edit/Delete) -> Test independently -> Data correction
9. Edge Cases + Polish -> Production ready

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Tests MUST fail before implementing (TDD per Constitution Principle II)
- Commit after each task or logical group using conventional format: `type: description`
- Stop at any checkpoint to validate story independently
- For integration tests: use VIBE_CLOCK_DB env var pointing to tempfile, VIBE_CLOCK_KEY env var to skip keyring in tests
