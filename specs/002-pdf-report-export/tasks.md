# Tasks: PDF Report Export

**Input**: Design documents from `/specs/002-pdf-report-export/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/cli-commands.md

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: Add genpdfi dependency and bundle font asset

- [X] T001 Add `genpdfi = "0.2"` dependency to Cargo.toml
- [X] T002 Download Liberation Sans Regular TTF and place in assets/fonts/LiberationSans-Regular.ttf (create directory if needed)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Create the PDF rendering module skeleton and path resolution utility that both user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T003 Create src/services/pdf.rs with module skeleton: `pub fn render_pdf(report: &Report, output_path: &Path) -> Result<(), AppError>` that creates a minimal genpdfi Document, loads the bundled font via `include_bytes!("../../assets/fonts/LiberationSans-Regular.ttf")`, adds a single line "Report", and writes to the output path. Also add `pub fn resolve_pdf_path(output: Option<&str>, pdf_flag: bool, from: NaiveDate, to: NaiveDate) -> Result<Option<PathBuf>, AppError>` returning `None` when neither `--pdf` nor `--output` is provided. Register module in src/services/mod.rs with `pub mod pdf;`
- [X] T004 Add `--pdf` (bool flag) and `--output` (Option<String>) arguments to the `Report` variant in src/cli/mod.rs; update help text per contracts/cli-commands.md
- [X] T005 Update src/cli/report.rs `handle_report` signature to accept `pdf: bool, output: Option<&str>`; after printing terminal report, call `resolve_pdf_path` and if `Some`, call `render_pdf` and print "PDF report saved to {path}"; update the match arm in src/main.rs to pass the new arguments
- [X] T006 Write integration test in tests/cli_report.rs: `generates_pdf_with_pdf_flag` — add tasks, run report with `--pdf`, assert exit code 0, assert stdout contains "PDF report saved to", assert the PDF file exists and is non-empty

**Checkpoint**: Foundation ready — minimal PDF generation works end-to-end

---

## Phase 3: User Story 1 — Export Report as PDF File (Priority: P1) 🎯 MVP

**Goal**: Generate a complete PDF with two distinct sections: (1) Project Summary listing projects with total hours, and (2) Daily Detail showing chronological days with full task information

**Independent Test**: Run `vibe-clock report --from YYYY-MM-DD --to YYYY-MM-DD --pdf`, open the generated PDF, verify it contains both the Project Summary section and Daily Detail section with all data

### Tests for User Story 1

- [X] T007 [P] [US1] Write integration test `pdf_contains_two_sections` in tests/cli_report.rs: generate PDF with tasks from multiple projects and dates, verify the PDF file exists and is valid
- [X] T008 [P] [US1] Write integration test `pdf_not_created_when_no_tasks` in tests/cli_report.rs: run report with `--pdf` for a date range with no tasks, assert stdout contains "No tasks found", assert no PDF file was created in the temp directory
- [X] T009 [P] [US1] Write integration test `pdf_contains_project_names` in tests/cli_report.rs: add tasks for two projects, generate PDF, verify PDF file is created and non-empty
- [X] T010 [P] [US1] Write integration test `pdf_handles_unicode_text` in tests/cli_report.rs: add a task with description "Réunion café" and project "Développement", generate PDF, assert success and file is non-empty

### Implementation for User Story 1

- [X] T011 [US1] Implement PDF header in src/services/pdf.rs `render_pdf`: add document title "Time Report", date range "{from} to {to}", and generation date using genpdfi Paragraph elements with bold styling for the title
- [X] T012 [US1] Implement Project Summary section in src/services/pdf.rs `render_pdf`: extract project names and totals from `report.project_sections`, create a two-column TableLayout (`| Project | Total Hours |`), sort alphabetically by project name, render with heading "Project Summary"
- [X] T013 [US1] Implement Daily Detail section in src/services/pdf.rs `render_pdf`: flatten all tasks from `report.project_sections.entries`, group by date (from `start_time` or `created_at`), for each date render a heading (YYYY-MM-DD) and a six-column TableLayout (`| ID | Description | Project | Start | End | Duration |`), sort chronologically by date then by start time within each day
- [X] T014 [US1] Implement grand total in src/services/pdf.rs `render_pdf`: after all sections, add a separator line and "Grand Total: {formatted_duration}" paragraph
- [X] T015 [US1] Implement no-data guard in src/cli/report.rs: when `report.project_sections.is_empty()` and PDF was requested, skip PDF generation (print "No tasks found" message only, do not call render_pdf)
- [X] T016 [US1] Implement atomic write in src/services/pdf.rs: write PDF to a temporary file (same directory, `.tmp` suffix) then rename to final path on success; on failure, delete temp file and return actionable error via `AppError::SystemError`
- [X] T017 [US1] Verify all US1 tests pass: run `cargo test --test cli_report` and confirm T007-T010 tests all pass

**Checkpoint**: User Story 1 complete — PDF export produces a full two-section report (Project Summary + Daily Detail) matching the specification

---

## Phase 4: User Story 2 — Customizable PDF Output Path (Priority: P2)

**Goal**: Allow users to control where the PDF is saved via `--output` with support for file paths, directory paths, and auto-generated filenames

**Independent Test**: Run report with `--output /path/to/file.pdf`, verify file is created at that exact path; run with `--output /some/dir/`, verify auto-named file is created in that directory

### Tests for User Story 2

- [X] T018 [P] [US2] Write integration test `pdf_output_to_specific_path` in tests/cli_report.rs: run report with `--output {tmp}/custom.pdf`, assert file exists at that exact path
- [X] T019 [P] [US2] Write integration test `pdf_output_to_directory` in tests/cli_report.rs: run report with `--output {tmp_dir}/`, assert a file matching `report-*-to-*.pdf` pattern exists in that directory
- [X] T020 [P] [US2] Write integration test `pdf_output_implies_pdf_flag` in tests/cli_report.rs: run report with `--output {tmp}/out.pdf` (no `--pdf` flag), assert PDF file is created (verifies `--output` implies `--pdf`)
- [X] T021 [P] [US2] Write integration test `pdf_rejects_nonexistent_directory` in tests/cli_report.rs: run report with `--output /nonexistent/dir/report.pdf`, assert exit code 1, assert stderr contains "does not exist"

### Implementation for User Story 2

- [X] T022 [US2] Implement full `resolve_pdf_path` logic in src/services/pdf.rs: handle three cases — (1) `--output` ending in `.pdf`: use as-is, validate parent dir exists; (2) `--output` as directory: append auto-generated `report-{from}-to-{to}.pdf` filename; (3) `--pdf` only: use current working directory with auto-generated filename. Return `None` if neither flag is set. Return `Err` for invalid paths (non-existent parent directory).
- [X] T023 [US2] Implement overwrite behavior: no special handling needed (genpdfi file write + atomic rename naturally overwrites); add integration test `pdf_overwrites_existing_file` in tests/cli_report.rs to verify
- [X] T024 [US2] Verify all US2 tests pass: run `cargo test --test cli_report` and confirm T018-T021 and T023 tests all pass

**Checkpoint**: User Story 2 complete — custom output paths work correctly

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Edge case handling, error messages, and final validation

- [X] T025 Write edge case test `pdf_write_permission_error` in tests/edge_cases.rs: attempt to write PDF to a read-only directory, assert exit code is non-zero and stderr contains an actionable error message
- [X] T026 Update CLI help text in src/cli/mod.rs: ensure `--pdf` and `--output` descriptions match contracts/cli-commands.md examples
- [X] T027 Run full validation: `cargo test && cargo clippy -- -D warnings && cargo fmt --check`

---

## Phase 6: Single-Date Report Shortcut (Enhancement)

**Purpose**: Allow users to omit `--to` flag for single-day reports (defaults to `--from` date)

**Goal**: Simplify the most common use case (today's report or any single date) by making `--to` optional

**Independent Test**: Run `vibe-clock report --from today --pdf` or `vibe-clock report --from 2026-03-01 --pdf` and verify report is generated for that single date only

### Tests for Single-Date Shortcut

- [X] T02- [ ] T028 [P] Write integration test `report_single_date_with_pdf` in tests/cli_report.rs: run report with `--from 2026-02-25 --pdf` (no `--to`), assert PDF is created for single date, verify filename contains single date (not range)
- [X] T02- [ ] T029 [P] Write integration test `report_today_shortcut` in tests/cli_report.rs: run report with `--from today --pdf`, assert PDF is created with today's date
- [X] T030 [P] Write integration test `report_single_date_terminal` in tests/cli_report.rs: run report with `--from 2026-02-25` (no PDF, no `--to`), assert terminal output shows single-date report

### Implementation for Single-Date Shortcut

- [X] T031 Make `--to` optional in src/cli/mod.rs: change `to: String` to `to: Option<String>` in the Report variant, update help text to indicate `--to` is optional and defaults to `--from` value
- [X] T032 Update src/cli/report.rs `handle_report`: change signature to accept `to: Option<&str>`, implement default logic: `let to_date = to.unwrap_or(from);` before parsing dates, ensuring single-date reports work correctly
- [X] T033 Update auto-generated PDF filename logic in src/services/pdf.rs: when from == to, use format `report-YYYY-MM-DD.pdf` instead of `report-YYYY-MM-DD-to-YYYY-MM-DD.pdf` for cleaner single-date filenames
- [X] T034 Update src/main.rs Report match arm: change destructuring to include `to: Option<String>` and pass `to.as_deref()` to handle_report
- [X] T035 Verify all single-date shortcut tests pass: run `cargo test --test cli_report` and confirm T028-T030 tests all pass

**Checkpoint**: Single-date shortcut complete — users can omit `--to` for cleaner single-day report syntax

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational phase completion
- **User Story 2 (Phase 4)**: Depends on Foundational phase completion — can run in parallel with US1 but US1 is recommended first
- **Polish (Phase 5)**: Depends on US1 and US2 completion
- **Single-Date Shortcut (Phase 6)**: Depends on Foundational phase completion — can be implemented independently or after Phase 5

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Phase 2 — core two-section PDF rendering (Project Summary + Daily Detail)
- **User Story 2 (P2)**: Can start after Phase 2 — path resolution is mostly independent but builds on the `resolve_pdf_path` skeleton from Phase 2

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Implementation tasks are sequential within each story
- Story complete before moving to next priority

### Parallel Opportunities

- T007, T008, T009, T010 can all run in parallel (different test functions, same file but no conflicts)
- T018, T019, T020, T021 can all run in parallel
- T028, T029, T030 can all run in parallel (different test functions for single-date shortcut)
- US1 and US2 could theoretically proceed in parallel after Phase 2, but sequential is recommended since US2 refines `resolve_pdf_path` from Phase 2

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Write test pdf_contains_two_sections in tests/cli_report.rs"
Task: "Write test pdf_not_created_when_no_tasks in tests/cli_report.rs"
Task: "Write test pdf_contains_project_names in tests/cli_report.rs"
Task: "Write test pdf_handles_unicode_text in tests/cli_report.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (add genpdfi, bundle font)
2. Complete Phase 2: Foundational (skeleton + CLI args + wiring)
3. Complete Phase 3: User Story 1 (full two-section PDF content)
4. **STOP and VALIDATE**: Generate a PDF and open it — verify Project Summary section shows all projects with totals, and Daily Detail section shows chronological days with all task details

### Incremental Delivery

1. Setup + Foundational → Minimal PDF generation works
2. User Story 1 → Full two-section report in PDF (Project Summary + Daily Detail) → MVP complete
3. User Story 2 → Custom output paths → Feature complete
4. Polish → Error handling, edge cases → Production ready
5. Single-Date Shortcut (Phase 6) → UX enhancement for common use case (optional but recommended)

### Key Implementation Notes

**Two-Section PDF Structure (Critical)**:
- **Project Summary Section**: Simple 2-column table listing each project with total hours (alphabetical order)
- **Daily Detail Section**: For each date (chronological), show heading + 6-column table with all tasks including full details (ID, description, project, start, end, duration)
- Both sections must be clearly separated with appropriate headings
- Grand total appears after all sections

**Data Flow**:
- `Report` struct → `render_pdf` function
- Project Summary: Extract from `project_sections` → name + total
- Daily Detail: Flatten all `entries` → group by date → render chronologically
- Use genpdfi's `TableLayout` for both sections

**Testing Focus**:
- Verify PDF file creation and validity
- Verify both sections are present (through file size / structure checks)
- Edge cases: unicode, no data, permissions, large datasets
