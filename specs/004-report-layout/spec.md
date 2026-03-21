# Feature Specification: Two-Part Report Layout

**Feature Branch**: `004-report-layout`
**Created**: 2026-03-21
**Status**: Draft
**Input**: User description: "The report should be created as two part. First part is a table with the name of the project and hours spent for the time specified of the report. below this table another new tables containing per day all tasks registered. I need to see the complete entry."

## Clarifications

### Session 2026-03-21

- Q: Should daily sections appear oldest-first or newest-first? → A: Oldest first (ascending date order).
- Q: Should long descriptions wrap to the next line or expand the column width? → A: Wrap to next line; other columns (start, end, duration) remain aligned.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Project Summary Table (Priority: P1)

As a user, I want the report to open with a concise summary table showing each project and its total time for the period, so I can immediately see how time was distributed across projects without scanning individual task entries.

**Why this priority**: The summary is the highest-value information in a time report — users and stakeholders need totals at a glance before diving into details. It is independently useful and forms the first section of the report.

**Independent Test**: Can be fully tested by running a report over a date range with tasks in multiple projects and verifying the summary table lists each project with its correct total and a grand total row.

**Acceptance Scenarios**:

1. **Given** tasks exist across multiple projects in the report period, **When** the user runs a report, **Then** the first section displays a table with one row per project (project name + total duration) and a grand total row at the bottom.
2. **Given** all tasks belong to a single project, **When** the user runs a report, **Then** the summary table shows one project row and a grand total row equal to that project's total.
3. **Given** no tasks exist in the report period, **When** the user runs a report, **Then** no summary table is shown and a "no tasks found" message is displayed instead.

---

### User Story 2 - Per-Day Task Breakdown (Priority: P1)

As a user, I want the report to include a day-by-day breakdown of all tasks in the period, with every field of each task entry fully visible — project name, complete description without any truncation, start time, end time, and duration — so I can review exactly what was done on each day.

**Why this priority**: The per-day breakdown is the primary audit and review tool. Users need untruncated, chronologically organized task entries to understand the full picture of their work. Both the summary and breakdown are P1 because neither alone delivers the complete feature value.

**Independent Test**: Can be fully tested by running a report with tasks spanning multiple days and verifying that each day appears as its own section with all task fields shown in full (no `...` truncation in any field).

**Acceptance Scenarios**:

1. **Given** tasks exist on multiple different dates within the period, **When** the user runs a report, **Then** below the summary table each date with tasks appears as a separate section heading, each containing a table with columns: ID, Project, Description, Start, End, Duration.
2. **Given** a task has a description longer than 30 characters, **When** it appears in the per-day breakdown, **Then** the full description is displayed without any truncation or ellipsis.
3. **Given** a task was logged with only a duration (no start/end times), **When** it appears in the per-day breakdown, **Then** the Start and End columns show a dash (`-`) for that task.
4. **Given** multiple tasks exist on the same day, **When** that day's section is displayed, **Then** tasks are ordered by start time ascending; tasks with no start time appear after timed tasks in creation order.
5. **Given** tasks exist on only some days within the report range, **When** the report is displayed, **Then** only days with at least one task appear as sections — empty days are omitted.

---

### User Story 3 - PDF Export Reflects New Layout (Priority: P2)

As a user, I want the PDF export to match the new two-part structure — summary table followed by per-day sections with complete entries — so that printed or shared reports are consistent with the terminal output.

**Why this priority**: PDF is a secondary output channel used for sharing and archiving. Terminal output must work first; the PDF follows the same structure.

**Independent Test**: Can be fully tested by generating a PDF with `--pdf` and verifying it contains the project summary section followed by dated daily sections with complete, untruncated task entries.

**Acceptance Scenarios**:

1. **Given** the user runs a report with `--pdf` or `--output path.pdf`, **Then** the generated PDF contains a project summary table (project name + total) followed by one section per day with complete task entries.
2. **Given** a task has a long description, **When** it appears in the PDF per-day section, **Then** the full description is shown word-wrapped within its cell, never truncated with `...`.

---

### Edge Cases

- What happens when the report period spans a single day? One day section appears in the breakdown below the summary table; the layout is identical.
- What happens when a project name is very long? The project name displays in full in both the summary table and the per-day breakdown.
- What happens when there are many projects (e.g., 20+)? All projects appear in the summary table without pagination or row truncation.
- What happens when the report range covers many days but only a few have tasks? Only the days with tasks appear as sections; the summary table is unaffected.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The report MUST display a project summary table as the first section, with one row per project showing the project name and total duration for the report period, plus a grand total row.
- **FR-002**: The report MUST display a per-day breakdown below the summary table, with one section per date that contains at least one task. Dates with no tasks MUST be omitted.
- **FR-003**: Each per-day section MUST contain a table with the following columns for every task: task ID, project name, full description, start time, end time, and duration.
- **FR-004**: Task descriptions MUST be displayed in full without any truncation or ellipsis in both the terminal output and the PDF. In the terminal, descriptions that exceed the column width MUST wrap onto the next line within the same row; all other columns (start, end, duration) remain aligned on a single line.
- **FR-005**: Within each day's table, tasks MUST be ordered by start time ascending. Tasks with no start time MUST appear after timed tasks, ordered by creation time.
- **FR-010**: Daily sections MUST be ordered by date ascending (oldest date first).
- **FR-006**: Tasks with no start or end time MUST show a dash (`-`) in the Start and End columns.
- **FR-007**: The PDF export MUST reflect the same two-part structure: project summary table followed by per-day sections with complete task entries.
- **FR-008**: The project summary grand total row MUST equal the arithmetic sum of all individual task durations in the period.
- **FR-009**: The existing command interface (`--from`, `--to`, `--pdf`, `--output`) MUST remain unchanged.

### Key Entities

- **Project Summary Row**: One entry per project in the report period, showing project name and total duration across all tasks.
- **Grand Total Row**: A single summary row at the bottom of the project summary table showing the sum of all project totals.
- **Daily Section**: All task entries for a single calendar date within the period, presented as a table under a date heading.
- **Complete Task Entry**: A task record displayed with all fields — ID, project name, full untruncated description, start time (or `-`), end time (or `-`), and duration.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The report's first visible section is always the project summary table (when tasks exist), and the per-day breakdown always follows it.
- **SC-002**: Every task description in the per-day breakdown is displayed in its entirety — no `...` truncation characters appear in any task description field in either terminal or PDF output.
- **SC-003**: Every date within the report period that has at least one task appears as a distinct dated section in the breakdown; dates with no tasks produce no section.
- **SC-004**: The grand total in the project summary matches the sum of all task durations visible in the per-day breakdown sections.
- **SC-005**: The PDF output generated with `--pdf` or `--output` contains both the project summary table and the per-day sections with complete task entries.

## Assumptions

- The two-part layout replaces the current project-grouped layout entirely; the old format (tasks grouped under project headings with truncated descriptions) is removed.
- Duration values are displayed using the existing `Xh Ym` format (e.g., `2h 30m`, `45m`, `2h`), consistent with the rest of the application.
- Daily section headings use `YYYY-MM-DD` date format (e.g., `2026-02-28`), consistent with existing date display conventions.
- Start and end times are displayed as `HH:MM` (e.g., `09:00`, `14:30`).
- All tasks belong to a project by design; a "no project" case does not exist.
- The terminal and PDF outputs always share the same two-part structure; there is no option to revert to the old layout.
