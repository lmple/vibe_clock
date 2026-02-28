# Feature Specification: Task Time Journal

**Feature Branch**: `001-task-time-journal`
**Created**: 2026-02-28
**Status**: Draft
**Input**: User description: "Build an application that can help me keep a daily journal of tasks with time spent for each tasks. Time spent can be either clocked with a starting time and an ending time or either edited manually. Each task is linked to a project that can be configured before. The application should be able to create report based on dates selection. Finally, the application is CLI based."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Manage Projects (Priority: P1)

As a user, I want to create and manage projects so that I can categorize my tasks under meaningful groups. Before logging any time, I need at least one project to exist. I can list, create, edit, and delete projects from the command line.

**Why this priority**: Projects are the foundational entity. Tasks depend on projects, so project management must exist first for the application to be usable.

**Independent Test**: Can be fully tested by creating, listing, editing, and deleting projects from the CLI, and verifying they persist between sessions.

**Acceptance Scenarios**:

1. **Given** no projects exist, **When** the user creates a project with a name, **Then** the project is saved and appears in the project list.
2. **Given** projects exist, **When** the user lists projects, **Then** all projects are displayed with their names.
3. **Given** a project exists, **When** the user edits its name, **Then** the updated name is reflected in the project list and in any associated tasks.
4. **Given** a project has no associated tasks, **When** the user deletes it, **Then** the project is removed from the list.
5. **Given** a project has associated tasks, **When** the user attempts to delete it, **Then** the system warns the user and asks for confirmation before deleting the project and its associated tasks.

---

### User Story 2 - Log Tasks with Clocked Time (Priority: P1)

As a user, I want to start a timer when I begin working on a task and stop it when I finish, so that my time is tracked accurately without manual calculation.

**Why this priority**: Clock-based time tracking is the core feature of the application and the primary way users will log time.

**Independent Test**: Can be fully tested by starting a clock on a task, performing work, stopping the clock, and verifying the elapsed time is recorded correctly.

**Acceptance Scenarios**:

1. **Given** a project exists, **When** the user starts a clock with a task description linked to that project, **Then** the system records the start time and indicates the clock is running.
2. **Given** a clock is running, **When** the user stops the clock, **Then** the system records the end time, calculates the duration, and saves the task entry.
3. **Given** a clock is running, **When** the user checks the status, **Then** the system shows the currently running task, its project, and elapsed time so far.
4. **Given** a clock is already running, **When** the user tries to start another clock, **Then** the system informs the user that a clock is already running and asks whether to stop the current one first.

---

### User Story 3 - Log Tasks with Manual Time Entry (Priority: P1)

As a user, I want to manually enter a task with a specific start time, end time, or duration, so that I can log work I forgot to clock or that happened outside the application.

**Why this priority**: Manual entry is essential for correcting missed clock sessions and ensuring complete daily records.

**Independent Test**: Can be fully tested by creating a task entry with manually specified times and verifying it appears correctly in the task list.

**Acceptance Scenarios**:

1. **Given** a project exists, **When** the user creates a task with a description, start time, and end time, **Then** the system calculates the duration and saves the entry.
2. **Given** a project exists, **When** the user creates a task with a description and a duration only, **Then** the system saves the entry with the specified duration.
3. **Given** a task entry exists, **When** the user edits the start time, end time, or duration, **Then** the system updates the entry and recalculates accordingly.

---

### User Story 4 - View Daily Task Journal (Priority: P2)

As a user, I want to view all tasks logged for a specific day so that I can review what I worked on and how much time I spent.

**Why this priority**: Viewing daily entries is necessary for self-review and is a prerequisite for the reporting feature.

**Independent Test**: Can be fully tested by logging several tasks across different days and viewing the journal for a specific day.

**Acceptance Scenarios**:

1. **Given** tasks exist for today, **When** the user views today's journal, **Then** all tasks for today are listed with project name, description, start/end times, and duration.
2. **Given** no tasks exist for a specific date, **When** the user views that day's journal, **Then** the system shows a message indicating no tasks were logged.
3. **Given** tasks exist, **When** the user views the journal without specifying a date, **Then** the system defaults to showing today's tasks.
4. **Given** tasks exist across multiple projects, **When** the user views the journal, **Then** the total time per project and the grand total for the day are displayed.

---

### User Story 5 - Generate Reports by Date Range (Priority: P2)

As a user, I want to generate time reports for a selected date range so that I can review how I spent my time over a period and share summaries with stakeholders.

**Why this priority**: Reporting provides the analytical value of the application and is a key differentiator from simple note-taking.

**Independent Test**: Can be fully tested by logging tasks over multiple days and generating a report for a date range, verifying totals and groupings.

**Acceptance Scenarios**:

1. **Given** tasks exist within a date range, **When** the user generates a report for that range, **Then** the report shows tasks grouped by project with total time per project and overall total.
2. **Given** tasks exist within a date range, **When** the user generates a report for that range, **Then** the report shows a day-by-day breakdown within each project.
3. **Given** no tasks exist within the selected date range, **When** the user generates a report, **Then** the system displays a message indicating no data is available.
4. **Given** tasks exist, **When** the user generates a report for a single day, **Then** the output matches the daily journal view for that day.

---

### User Story 6 - Edit and Delete Task Entries (Priority: P3)

As a user, I want to edit or delete previously logged task entries so that I can correct mistakes or remove erroneous entries.

**Why this priority**: Data correction is important for accuracy but is not needed for initial usability.

**Independent Test**: Can be fully tested by creating task entries, editing their details, and deleting them, then verifying the changes persist.

**Acceptance Scenarios**:

1. **Given** a task entry exists, **When** the user edits its description, project, or time values, **Then** the changes are saved and reflected in journal views and reports.
2. **Given** a task entry exists, **When** the user deletes it, **Then** the entry is removed and no longer appears in journal views or reports.
3. **Given** a task entry exists, **When** the user changes its associated project, **Then** the entry moves to the new project in all views and reports.

---

### Edge Cases

- What happens when the user starts a clock and the application is terminated unexpectedly? The running clock state is persisted so it can be resumed or stopped on next launch.
- What happens when the user enters an end time earlier than the start time? The system rejects the entry and displays an error message.
- What happens when the user enters a date in the future for a task? The system allows it (the user may be pre-planning) but displays a warning.
- What happens when the user deletes a project that has tasks? The system warns and requires confirmation, then deletes both the project and its tasks.
- What happens when the clock crosses midnight? The task is recorded with the correct start and end times spanning the day boundary, and appears in the journal for the start date.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST allow users to create, list, edit, and delete projects with a unique name.
- **FR-002**: System MUST allow users to start a time clock on a task linked to an existing project.
- **FR-003**: System MUST allow only one clock to run at a time.
- **FR-004**: System MUST allow users to stop a running clock and record the task entry with calculated duration.
- **FR-005**: System MUST allow users to check the status of a running clock (task, project, elapsed time).
- **FR-006**: System MUST allow users to manually create task entries with a description, project, and either start/end times or a duration.
- **FR-007**: System MUST validate that end time is after start time for manual entries.
- **FR-008**: System MUST allow users to edit any field of an existing task entry (description, project, start time, end time, duration).
- **FR-009**: System MUST allow users to delete task entries.
- **FR-010**: System MUST display a daily journal showing all tasks for a given date with project, description, times, and durations.
- **FR-011**: System MUST default to today's date when no date is specified for journal views.
- **FR-012**: System MUST display total time per project and grand total in journal views.
- **FR-013**: System MUST generate reports for a user-specified date range showing tasks grouped by project with time totals.
- **FR-014**: System MUST include day-by-day breakdown within project groupings in reports.
- **FR-015**: System MUST persist all data between sessions.
- **FR-016**: System MUST persist clock state so that an interrupted clock can be recovered on next launch.
- **FR-017**: System MUST be operated entirely via command-line interface.

### Key Entities

- **Project**: Represents a category or client for grouping tasks. Has a unique name. Can contain zero or more task entries.
- **Task Entry**: Represents a unit of work performed. Has a description, a reference to a project, a start time, an end time, and a calculated duration. Can be created via clock or manual entry.
- **Clock State**: Represents a currently running timer. Has a reference to a task description, a project, and a start time. Only one can exist at a time.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can create a project and log their first clocked task within 2 minutes of first use.
- **SC-002**: Users can start and stop a clock in 2 commands or fewer.
- **SC-003**: Users can manually log a task entry in a single command.
- **SC-004**: Users can view their daily journal in a single command.
- **SC-005**: Users can generate a report for any date range in a single command.
- **SC-006**: All task data persists across application restarts without data loss.
- **SC-007**: The application provides clear error messages for all invalid inputs (wrong date format, missing project, overlapping times).
- **SC-008**: Reports accurately reflect 100% of logged task entries within the selected date range.

## Assumptions

- The application is single-user (no multi-user or authentication needed).
- Data is stored locally on the user's machine.
- Time precision is to the minute (seconds are not tracked).
- Date and time formats follow the user's locale or a sensible default (ISO 8601).
- The CLI uses a subcommand pattern (e.g., `app project list`, `app clock start`, `app report`).
- No export format is required for reports at this stage (plain text output to terminal).
