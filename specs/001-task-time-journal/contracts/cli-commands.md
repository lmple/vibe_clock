# CLI Command Contracts: Task Time Journal

**Branch**: `001-task-time-journal` | **Date**: 2026-02-28

All commands follow the pattern: `vibe-clock <resource> <action> [arguments] [options]`

## Project Commands

### `vibe-clock project add <name>`

Create a new project.

- **Arguments**: `name` (required) - project name
- **Stdout**: `Project '<name>' created.`
- **Stderr**: `Error: Project '<name>' already exists.` (exit 1)
- **Exit**: 0 on success, 1 on duplicate name

### `vibe-clock project list`

List all projects.

- **Stdout**: Tabular output with columns: ID, Name, Tasks (count), Created
- **Stdout** (empty): `No projects found. Create one with: vibe-clock project add <name>`
- **Exit**: 0

### `vibe-clock project edit <id> --name <new-name>`

Rename a project.

- **Arguments**: `id` (required) - project ID
- **Options**: `--name <new-name>` (required) - new project name
- **Stdout**: `Project renamed to '<new-name>'.`
- **Stderr**: `Error: Project with ID <id> not found.` (exit 1)
- **Stderr**: `Error: Project '<new-name>' already exists.` (exit 1)
- **Exit**: 0 on success, 1 on error

### `vibe-clock project delete <id> [--yes]`

Delete a project.

- **Arguments**: `id` (required) - project ID
- **Options**: `--yes` / `-y` - skip confirmation
- **Interactive**: If project has tasks and `--yes` not set, prompt: `Project '<name>' has <n> tasks. Delete project and all tasks? [y/N]`
- **Stdout**: `Project '<name>' deleted.`
- **Stderr**: `Error: Project with ID <id> not found.` (exit 1)
- **Exit**: 0 on success, 1 on error

## Clock Commands

### `vibe-clock clock start <project> <description>`

Start a time clock.

- **Arguments**:
  - `project` (required) - project name or ID
  - `description` (required) - task description
- **Stdout**: `Clock started for '<description>' on project '<project>' at <HH:MM>.`
- **Stderr**: `Error: Clock already running. Use 'vibe-clock clock stop' first, or 'vibe-clock clock status' to check.` (exit 1)
- **Stderr**: `Error: Project '<project>' not found.` (exit 1)
- **Exit**: 0 on success, 1 on error

### `vibe-clock clock stop`

Stop the running clock and save the task entry.

- **Stdout**: `Clock stopped. Logged <duration> for '<description>' on project '<project>'.`
- **Stderr**: `Error: No clock is running.` (exit 1)
- **Exit**: 0 on success, 1 on error

### `vibe-clock clock status`

Check if a clock is running.

- **Stdout** (running): `Clock running: '<description>' on project '<project>' since <HH:MM> (<elapsed> elapsed).`
- **Stdout** (not running): `No clock is running.`
- **Exit**: 0

## Task Commands

### `vibe-clock task add <project> <description> [time options]`

Manually add a task entry.

- **Arguments**:
  - `project` (required) - project name or ID
  - `description` (required) - task description
- **Options** (at least one time option required):
  - `--start <datetime>` - start time (ISO 8601 or HH:MM for today)
  - `--end <datetime>` - end time (ISO 8601 or HH:MM for today)
  - `--duration <minutes>` - duration in minutes (alternative to start/end)
  - `--date <date>` - date for the entry (defaults to today, used with --duration)
- **Stdout**: `Task logged: <duration> for '<description>' on project '<project>'.`
- **Stderr**: `Error: End time must be after start time.` (exit 1)
- **Stderr**: `Error: Provide either --start/--end or --duration.` (exit 1)
- **Exit**: 0 on success, 1 on error

### `vibe-clock task edit <id> [options]`

Edit an existing task entry.

- **Arguments**: `id` (required) - task entry ID
- **Options** (at least one required):
  - `--description <text>` - new description
  - `--project <name-or-id>` - move to different project
  - `--start <datetime>` - new start time
  - `--end <datetime>` - new end time
  - `--duration <minutes>` - new duration
- **Stdout**: `Task <id> updated.`
- **Stderr**: `Error: Task with ID <id> not found.` (exit 1)
- **Exit**: 0 on success, 1 on error

### `vibe-clock task delete <id> [--yes]`

Delete a task entry.

- **Arguments**: `id` (required) - task entry ID
- **Options**: `--yes` / `-y` - skip confirmation
- **Interactive**: If `--yes` not set, prompt: `Delete task '<description>' (<duration>)? [y/N]`
- **Stdout**: `Task <id> deleted.`
- **Stderr**: `Error: Task with ID <id> not found.` (exit 1)
- **Exit**: 0 on success, 1 on error

## Journal Commands

### `vibe-clock journal [date]`

View the daily task journal.

- **Arguments**: `date` (optional) - date to view (ISO 8601 or "today"/"yesterday", defaults to today)
- **Stdout**: Tabular output with columns: ID, Project, Description, Start, End, Duration
  - Followed by per-project totals and grand total
- **Stdout** (empty): `No tasks logged for <date>.`
- **Exit**: 0

## Report Commands

### `vibe-clock report --from <date> --to <date>`

Generate a time report for a date range.

- **Options**:
  - `--from <date>` (required) - start date (inclusive)
  - `--to <date>` (required) - end date (inclusive)
- **Stdout**: Report grouped by project, with day-by-day breakdown per project, project totals, and grand total
- **Stdout** (empty): `No tasks found between <from> and <to>.`
- **Stderr**: `Error: --from date must be before or equal to --to date.` (exit 1)
- **Exit**: 0 on success, 1 on error

## Global Options

- `--help` / `-h` - available on all commands and subcommands
- `--version` / `-V` - show application version

## Exit Codes

| Code | Meaning                    |
|------|----------------------------|
| 0    | Success                    |
| 1    | User error (bad input)     |
| 2    | System error (I/O failure) |
