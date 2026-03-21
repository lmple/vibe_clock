# Data Model: Two-Part Report Layout

**Feature**: 004-report-layout
**Date**: 2026-03-21

---

## Entities

### Report (refactored)

The `Report` struct is restructured to carry the two logical output parts natively.

**File**: `src/services/report.rs`

```
Report
├── from: NaiveDate              — report start date (inclusive)
├── to: NaiveDate                — report end date (inclusive)
├── project_summaries: Vec<ProjectSummary>   — one entry per project in the period
├── daily_sections: Vec<DailySection>        — one entry per date that has tasks (ascending)
└── grand_total: i64             — sum of all task durations in minutes
```

**Validation rules**:
- `from <= to` (enforced by CLI handler before calling `generate_report`)
- `grand_total` MUST equal the arithmetic sum of all `ProjectSummary.total` values
- `daily_sections` MUST be sorted ascending by `date`
- If no tasks exist in the period, `project_summaries` and `daily_sections` are both empty

---

### ProjectSummary (new)

Replaces the `ProjectSection` struct's role in the summary table. Does **not** carry task entries (those live in `DailySection`).

**File**: `src/services/report.rs`

```
ProjectSummary
├── name: String    — project name
└── total: i64      — total duration in minutes for this project in the report period
```

**Notes**:
- One `ProjectSummary` per project that has at least one task in the report period
- Order in `project_summaries` is insertion order (as tasks are encountered); presentation order is determined by the renderer

---

### DailySection (new)

Groups all task entries for a single calendar date.

**File**: `src/services/report.rs`

```
DailySection
├── date: NaiveDate           — the calendar date
└── entries: Vec<DailyEntry>  — tasks on this date, sorted by start_time ascending, then id
```

**Ordering rules**:
- `entries` sorted by `(start_time ascending, id ascending)`
- Tasks with `start_time = None` appear after all timed tasks, in `id` (creation) order

---

### DailyEntry (new)

A task entry together with its resolved project name, ready for display.

**File**: `src/services/report.rs`

```
DailyEntry
├── task: TaskEntry      — the raw task record (id, description, start_time, end_time, duration_min, created_at)
└── project_name: String — resolved project name for this task
```

**Display rules** (enforced by renderers):
- `task.id` → displayed as-is
- `task.description` → displayed in full, no truncation
  - Terminal: wraps within fixed column width (40 chars); continuation lines pad other columns with spaces
  - PDF: rendered as `Paragraph` element (auto word-wrap within column)
- `task.start_time` → `HH:MM` if `Some`, otherwise `-`
- `task.end_time` → `HH:MM` if `Some`, otherwise `-`
- `task.duration_min` → formatted with `format_duration()` (existing `Xh Ym` format)

---

## Removed Entity

### ProjectSection (removed)

The `ProjectSection` struct (which held `name`, `entries`, `total`) is replaced by the combination of `ProjectSummary` (for totals) and `DailyEntry` (for task records within `DailySection`). The `Report.project_sections` field is removed.

---

## State Transitions / Data Flow

```
Database query (list_tasks_for_date_range)
        │
        ▼
generate_report()
    ├── Accumulate project totals → Vec<ProjectSummary>
    ├── Group tasks by date → BTreeMap<NaiveDate, Vec<DailyEntry>>
    │       └── Sort each day's entries: (start_time, id)
    ├── Convert BTreeMap to Vec<DailySection> (BTreeMap preserves date order)
    └── Sum all durations → grand_total
        │
        ▼
Report { project_summaries, daily_sections, grand_total, from, to }
        │
        ├──► Terminal renderer (src/cli/report.rs)
        │       ├── Part 1: Project summary table
        │       └── Part 2: Per-day sections with wrapped descriptions
        │
        └──► PDF renderer (src/services/pdf.rs)
                ├── Part 1: Project summary table
                └── Part 2: Per-day sections with Paragraph-wrapped descriptions
```

---

## No Storage Changes

This feature makes no changes to the database schema or the `TaskEntry` model. All changes are in the service layer (how data is shaped after retrieval) and the rendering layer (how the shaped data is displayed).
