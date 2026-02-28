# Data Model: Task Time Journal

**Branch**: `001-task-time-journal` | **Date**: 2026-02-28

## Entities

### Project

Represents a category or client for grouping tasks.

| Field      | Type    | Constraints                          |
|------------|---------|--------------------------------------|
| id         | INTEGER | PRIMARY KEY, AUTOINCREMENT           |
| name       | TEXT    | NOT NULL, UNIQUE                     |
| created_at | TEXT    | NOT NULL, ISO 8601 datetime          |
| updated_at | TEXT    | NOT NULL, ISO 8601 datetime          |

**Validation rules**:
- `name` MUST be non-empty and unique (case-sensitive).
- `name` MUST be trimmed of leading/trailing whitespace before storage.

### TaskEntry

Represents a unit of work performed, linked to a project.

| Field       | Type    | Constraints                                        |
|-------------|---------|----------------------------------------------------|
| id          | INTEGER | PRIMARY KEY, AUTOINCREMENT                         |
| project_id  | INTEGER | NOT NULL, FOREIGN KEY → Project(id) ON DELETE CASCADE |
| description | TEXT    | NOT NULL                                           |
| start_time  | TEXT    | NULL, ISO 8601 datetime (minute precision)         |
| end_time    | TEXT    | NULL, ISO 8601 datetime (minute precision)         |
| duration_min| INTEGER | NOT NULL, duration in minutes                      |
| created_at  | TEXT    | NOT NULL, ISO 8601 datetime                        |
| updated_at  | TEXT    | NOT NULL, ISO 8601 datetime                        |

**Validation rules**:
- `description` MUST be non-empty.
- If both `start_time` and `end_time` are provided, `end_time` MUST be after `start_time`.
- If `start_time` and `end_time` are provided, `duration_min` is calculated as the difference in minutes.
- If only `duration_min` is provided (manual entry without times), `start_time` and `end_time` are NULL.
- `duration_min` MUST be greater than 0.
- `project_id` MUST reference an existing Project.

**Date assignment**:
- A task's date is derived from `start_time` (date portion) when available.
- When `start_time` is NULL (duration-only entry), the date is derived from `created_at`.
- Tasks spanning midnight belong to the date of `start_time`.

### ClockState

Represents a currently running timer. At most one row exists in this table.

| Field       | Type    | Constraints                                        |
|-------------|---------|----------------------------------------------------|
| id          | INTEGER | PRIMARY KEY, CHECK(id = 1)                         |
| project_id  | INTEGER | NOT NULL, FOREIGN KEY → Project(id)                |
| description | TEXT    | NOT NULL                                           |
| start_time  | TEXT    | NOT NULL, ISO 8601 datetime                        |

**Validation rules**:
- Only one row can exist (enforced by `CHECK(id = 1)`).
- `project_id` MUST reference an existing Project.
- `description` MUST be non-empty.

**State transitions**:
- **Start clock**: INSERT into ClockState (fails if row exists → clock already running).
- **Stop clock**: DELETE from ClockState + INSERT into TaskEntry (within a single transaction).
- **Check status**: SELECT from ClockState. Empty result → no clock running.
- **App crash recovery**: On startup, if ClockState has a row, inform user and offer to stop or resume.

## Relationships

```text
Project 1 ──── * TaskEntry
Project 1 ──── 0..1 ClockState
```

- Deleting a Project cascades to all its TaskEntry rows (with user confirmation).
- Deleting a Project when a ClockState references it MUST stop the clock first (with user confirmation).

## SQL Schema

```sql
CREATE TABLE IF NOT EXISTS project (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS task_entry (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    start_time TEXT,
    end_time TEXT,
    duration_min INTEGER NOT NULL CHECK(duration_min > 0),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS clock_state (
    id INTEGER PRIMARY KEY CHECK(id = 1),
    project_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    start_time TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES project(id)
);

CREATE INDEX idx_task_entry_project_id ON task_entry(project_id);
CREATE INDEX idx_task_entry_start_time ON task_entry(start_time);
```

## Migration Strategy

For the initial version, the schema is created on first run via `CREATE TABLE IF NOT EXISTS`. Future migrations will use a `schema_version` table:

```sql
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY
);
```

Each migration is a numbered SQL script applied in order. The application checks the current version on startup and applies any pending migrations within a transaction.
