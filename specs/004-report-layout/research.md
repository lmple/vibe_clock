# Research: Two-Part Report Layout

**Feature**: 004-report-layout
**Date**: 2026-03-21

---

## Finding 1: Current Report Structure

**Decision**: The `Report` struct in `src/services/report.rs` uses `project_sections: Vec<ProjectSection>`, where each section owns its task entries. The PDF service (`src/services/pdf.rs`) already implements a two-part layout (project summary + daily detail) by re-grouping entries from `project_sections` into a `BTreeMap<String, Vec<_>>` by date. The terminal renderer (`src/cli/report.rs`) only renders the project-grouped layout with 30-character truncation.

**Rationale**: The cleanest fix is to restructure `Report` to carry the two logical parts natively — a flat `Vec<ProjectSummary>` for the summary table and a `Vec<DailySection>` for the breakdown — rather than having both renderers derive these shapes from `project_sections`. This removes duplication and makes the PDF renderer simpler.

**Alternatives considered**: Keep `project_sections` and add a parallel `daily_sections` field. Rejected because it duplicates data and `project_sections` with its `entries` field would become dead weight for the terminal renderer.

---

## Finding 2: Terminal Description Wrapping

**Decision**: Implement a helper that prints each task row with a fixed-width description column. When the description exceeds the column width, it is split into chunks; the first chunk appears on the main row alongside ID, project, start, end, duration; subsequent chunks are printed on continuation rows with the description cell left-padded and all other cells empty.

**Rationale**: Standard Rust `println!` with `{:<N}` format strings does not support automatic line-wrapping within a cell. The simplest correct approach is to split the string manually. A fixed column width of 40 characters is chosen: wide enough to reduce wrapping for typical descriptions, narrow enough to fit a standard 80-column terminal alongside the other columns (ID=6, Project=18, Start=7, End=7, Duration=8 ≈ 86 total with separators — fits in 100 columns comfortably).

**Alternatives considered**: Dynamic terminal width detection via `termsize` or `crossterm::terminal::size()`. Rejected because it adds a new dependency for marginal benefit; a fixed column width is sufficient for the acceptance criteria and keeps the dependency list stable.

---

## Finding 3: PDF Description Wrapping

**Decision**: Replace `genpdfi::elements::Text::new(desc)` with `genpdfi::elements::Paragraph::new(desc)` in the daily detail table. `genpdfi` `Paragraph` elements support word-wrapping within their allocated column width automatically. No change to table column weights needed (current `vec![1, 3, 2, 1, 1, 1]` gives description 3/9 of page width, sufficient).

**Rationale**: The existing PDF already uses `Paragraph` for headings and free text. `Text` is a single-line element; replacing it with `Paragraph` is a one-line change per cell that enables wrapping without additional dependencies or layout changes.

**Alternatives considered**: Keep `Text` and increase the description column weight. Rejected: still truncates if text is very long; wrapping is the correct solution.

---

## Finding 4: Task Ordering within Daily Sections

**Decision**: When building `DailySection.entries`, sort tasks by `start_time` ascending (NaiveDateTime comparison). Tasks with `start_time = None` sort after all timed tasks, ordered by `id` (insertion order, which matches creation order for SQLite autoincrement).

**Rationale**: `start_time` is `Option<NaiveDateTime>`. Rust's `Option<T: Ord>` sorts `None` after `Some(...)` when using `Option::partial_cmp` with a `None`-last convention, which can be achieved by mapping `None` to `NaiveDateTime::MAX` for comparison purposes. Sorting by `id` for the tie-break among untimed tasks is O(1) to implement (just use `.sort_by_key(|e| (e.task.start_time.map(...), e.task.id))`).

**Alternatives considered**: SQL-level ORDER BY in the DB query. Viable but couples rendering order concerns to the data layer; the service layer is the right place for sort logic.

---

## Finding 5: Impact on Existing Tests

**Decision**: Three existing report tests check output that will change format:
- `generates_report_grouped_by_project` checks for `"Acme"`, `"Beta"`, `"Grand Total"` — these will still appear in the new two-part layout.
- `shows_day_by_day_breakdown` checks for task names and dates — these will still appear.
- `report_single_date_terminal` checks for `"Acme"` and `"Day 1 work"` — both appear in new layout.

No tests check for the old project-heading format (`## Acme (2h)`) or the truncation pattern (`...`), so the existing tests remain valid. New tests are needed for: no-truncation assertion, per-day section headers, summary table, task ordering within a day.

**Alternatives considered**: None — the existing tests are intentionally written to check for content, not format, so they are resilient to layout changes.
