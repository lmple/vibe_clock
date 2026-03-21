# Tasks: Two-Part Report Layout

**Input**: Design documents from `/specs/004-report-layout/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3)
- Exact file paths are included in each description

---

## Phase 1: Setup

**Purpose**: No new dependencies, no schema changes, no new modules — this is a refactor of existing code. No setup tasks required.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Restructure the `Report` data model. ALL user story rendering tasks depend on this being done first — the terminal renderer (US1, US2) and PDF renderer (US3) all consume the new `Report` struct.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [x] T001 Refactor `Report` struct in `src/services/report.rs`: remove `ProjectSection` and `project_sections` field; add `ProjectSummary { name: String, total: i64 }` struct, `DailyEntry { task: TaskEntry, project_name: String }` struct, and `DailySection { date: NaiveDate, entries: Vec<DailyEntry> }` struct; update `Report` to hold `project_summaries: Vec<ProjectSummary>` and `daily_sections: Vec<DailySection>`; update `generate_report()` to use a `BTreeMap<NaiveDate, Vec<DailyEntry>>` for grouping (provides ascending date order), sort each day's entries by `(start_time.unwrap_or(NaiveDateTime::MAX), task.id)`, and compute `grand_total` from all task durations

**Checkpoint**: `cargo build` succeeds (downstream renderers will fail to compile until updated in US1/US2/US3 — that is expected and acceptable here)

---

## Phase 3: User Story 1 - Project Summary Table (Priority: P1) 🎯 MVP

**Goal**: The first section of the terminal report displays a table showing each project and its total duration, plus a grand total row, replacing the old project-grouped task list header.

**Independent Test**: Run `cargo test report_shows_project_summary` and verify the summary table content is present in the output with correct project totals and a grand total.

### Implementation for User Story 1

- [x] T002 [US1] Add integration tests for the project summary table in `tests/cli_report.rs`: add `report_shows_project_summary_table` (multi-project: verifies each project name and its total appear before any date sections), `report_shows_grand_total_in_summary` (verifies "TOTAL" row equals sum of individual project totals), and `report_single_project_shows_summary_and_total` (single-project: one project row plus grand total)
- [x] T003 [US1] Rewrite the terminal renderer's first section in `src/cli/report.rs` to print the project summary table: print a "Project Summary" heading, print a two-column table header (`Project` | `Total`) with separator line, iterate `report.project_summaries` and print one row per project with `format_duration(summary.total)`, then print a separator line and a `TOTAL` row with `format_duration(report.grand_total)`; remove the old `for section in &report.project_sections` loop and the old `Grand Total` line at the end

**Checkpoint**: `cargo test` passes. `vibe-clock report --from <date> --to <date>` shows a project summary table followed by no per-day sections yet (US2). The old project-grouped format is fully removed.

---

## Phase 4: User Story 2 - Per-Day Task Breakdown (Priority: P1)

**Goal**: Below the project summary table, the terminal report shows one section per calendar date (ascending), each containing a table of complete, untruncated task entries with all fields visible.

**Independent Test**: Run `cargo test report_shows_per_day` and `cargo test report_no_truncation` and verify dated sections appear with full descriptions and correct ordering.

### Implementation for User Story 2

- [x] T004 [US2] Add integration tests for the per-day breakdown in `tests/cli_report.rs`: add `report_shows_per_day_sections` (verifies date headings `2026-02-25` and `2026-02-26` appear as sections), `report_shows_tasks_in_date_ascending_order` (verifies earlier date section appears before later date in stdout), `report_no_truncation_long_description` (adds a task with a 50-character description, verifies the full description appears and no `...` appears in stdout), `report_dash_for_tasks_with_no_start_end` (adds a duration-only task, verifies `-` appears in output for start and end), `report_empty_days_omitted` (verifies a date with no tasks does not appear as a section heading in the output), and `report_tasks_ordered_by_start_time_within_day` (adds two tasks on the same day with different start times, verifies the earlier-starting task appears before the later one in stdout)
- [x] T005 [US2] Add `wrap_description(desc: &str, width: usize) -> Vec<String>` helper function in `src/cli/report.rs` that splits a description string into chunks of at most `width` characters, splitting at word boundaries where possible (split at last space before the width limit; if no space exists, split at the width limit exactly)
- [x] T006 [US2] Implement the per-day sections renderer in `src/cli/report.rs`: after the project summary table, iterate `report.daily_sections` (already in ascending date order from the service); for each section print a blank line, the date heading (`section.date.format("%Y-%m-%d")`), a six-column table header (`ID` | `Project` | `Description` | `Start` | `End` | `Duration`) with a separator line, then for each `DailyEntry` call `wrap_description(&entry.task.description, 40)` and print the first chunk on the main row alongside `entry.task.id` (6), `entry.project_name` (18), `start` (7), `end` (7), `format_duration(entry.task.duration_min)` (8); print any additional description chunks on continuation rows with all other columns blank; format start/end as `HH:MM` or `-`

**Checkpoint**: `cargo test` passes. `vibe-clock report --from <date> --to <date>` shows the project summary table followed by per-day sections with full descriptions. No `...` appears in any task description.

---

## Phase 5: User Story 3 - PDF Export Reflects New Layout (Priority: P2)

**Goal**: The PDF export mirrors the new two-part terminal layout: project summary table first, then per-day sections with complete untruncated descriptions.

**Independent Test**: Run `cargo test generates_pdf_with_pdf_flag` and `cargo test pdf_contains_project_names` to verify the PDF is still generated successfully with the new Report structure.

### Implementation for User Story 3

- [x] T007 [US3] Update `render_pdf()` in `src/services/pdf.rs` to use the new `Report` struct: replace the `for section in &sorted_projects` loop that read `report.project_sections` with a loop over `report.project_summaries`; replace the internal `BTreeMap<String, Vec<_>>` date-grouping block (which re-derived dates from `project_sections`) with a direct loop over `report.daily_sections`; in the daily detail table, for each `DailyEntry` change `Text::new(desc)` to `Paragraph::new(entry.task.description.clone())` so genpdfi auto-wraps long descriptions within the column; remove the truncation logic (`if task.description.len() > 35 { format!("{}...", ...) }`)

**Checkpoint**: `cargo test` passes. All existing PDF tests pass. `vibe-clock report --from <date> --to <date> --pdf` generates a valid PDF with the project summary section followed by per-day dated sections.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Verification, formatting, and cleanup

- [x] T008 Run `cargo test && cargo clippy && cargo fmt --check` from repo root and fix all warnings and formatting issues
- [x] T009 Verify `generates_report_grouped_by_project`, `shows_day_by_day_breakdown`, and `report_single_date_terminal` existing tests still pass with the new layout (no changes expected — these tests check for content presence, not layout format)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Foundational (Phase 2)**: No dependencies — start immediately
- **US1 (Phase 3)**: Depends on T001 (Report struct)
- **US2 (Phase 4)**: Depends on T001 (Report struct) and T003 (summary table must exist for renderer to compile cleanly)
- **US3 (Phase 5)**: Depends on T001 (Report struct); can run in parallel with US1/US2 since it touches a different file (`src/services/pdf.rs`)
- **Polish (Phase 6)**: Depends on all user stories complete

### User Story Dependencies

- **US1 (P1)**: Starts after T001
- **US2 (P1)**: Starts after T003 (builds on the same renderer file)
- **US3 (P2)**: Starts after T001, independent of US1/US2 (different file)

### Within Each User Story

- Tests before implementation (write tests, verify they compile but fail against current code, then implement)
- T005 (wrap helper) before T006 (per-day renderer uses it)
- T002 before T003 (US1); T004 before T005/T006 (US2); T007 stands alone (US3)

### Parallel Opportunities

- US3 (T007) can run in parallel with US1/US2 — it touches only `src/services/pdf.rs`
- Within US2, T005 and T004 touch different files (cli/report.rs vs tests/cli_report.rs) and can run in parallel

---

## Parallel Example: User Story 2 + User Story 3

```bash
# Once T001 is done, these can run in parallel:
Task T004: Add US2 integration tests in tests/cli_report.rs
Task T007: Update render_pdf() in src/services/pdf.rs  # US3 independent
```

---

## Implementation Strategy

### MVP First (US1 + US2 — both P1)

1. Complete Phase 2: T001 (Report struct refactor) — CRITICAL, unblocks everything
2. Complete Phase 3: T002 → T003 (terminal summary table)
3. Complete Phase 4: T004 → T005 → T006 (terminal per-day breakdown)
4. **STOP and VALIDATE**: `cargo test` passes; manual test with real data looks correct
5. Add Phase 5 (US3 PDF) when ready

### Incremental Delivery

1. T001 → Foundation ready (Report struct compiles)
2. T002, T003 → Summary table visible in terminal (MVP summary)
3. T004, T005, T006 → Full two-part terminal layout (US1 + US2 complete)
4. T007 → PDF matches terminal layout (US3 complete)
5. T008, T009 → Polish and verify

---

## Notes

- T001 will cause compile errors in `src/cli/report.rs` and `src/services/pdf.rs` until T003 and T007 are done — this is expected
- The existing `ProjectSection` struct is fully removed; no backward-compatible shim needed
- Existing tests `generates_report_grouped_by_project`, `shows_day_by_day_breakdown`, `report_single_date_terminal` remain valid — they check for content, not format
- The `wrap_description` helper (T005) should be a private function in `src/cli/report.rs`, not exported
- `BTreeMap<NaiveDate, Vec<DailyEntry>>` in `generate_report()` provides ascending date ordering for free — no explicit sort needed for sections
- Per-task sorting: `entries.sort_by_key(|e| (e.task.start_time.unwrap_or(NaiveDateTime::MAX), e.task.id))`
