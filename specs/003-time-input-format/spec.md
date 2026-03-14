# Feature Specification: Simplified Time Input Format

**Feature Branch**: `003-time-input-format`
**Created**: 2026-02-28
**Status**: Draft
**Input**: User description: "Time needs to be specified as hours and/or minutes since we only care about daily tasks."

## Clarifications

### Session 2026-02-28

- Q: Does this feature remove ISO 8601 full datetime support for `--start`/`--end`? → A: Yes. Remove ISO 8601 datetime; only accept `HH:MM`. Use `--date` for the day.
- Q: Should single-digit hours be accepted (e.g., `9:00` vs strict `09:00`)? → A: Yes. Accept both `H:MM` and `HH:MM`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Enter Start and End Times as Clock Times (Priority: P1)

As a user, I want to enter start and end times using simple clock-time notation (e.g., `9:00`, `14:30`) rather than full datetime strings, so that logging daily tasks is fast and natural.

**Why this priority**: Start/end time input is the most frequent time entry interaction. Simplifying it directly reduces friction for the most common use case.

**Independent Test**: Can be fully tested by adding a task with `--start 9:00 --end 10:30` and verifying the task is recorded with the correct times on today's date and a 90-minute duration.

**Acceptance Scenarios**:

1. **Given** a project exists, **When** the user adds a task with `--start 9:00 --end 10:30`, **Then** the system records the task for today with a start time of 09:00, end time of 10:30, and a duration of 1h 30m.
2. **Given** a project exists, **When** the user adds a task with `--start 9:00 --end 10:30 --date 2026-03-01`, **Then** the system records the task for 2026-03-01 with the specified times.
3. **Given** a project exists, **When** the user adds a task with `--start 14:00 --end 14:00` (zero duration), **Then** the system rejects the entry with an error indicating the end time must be after the start time.
4. **Given** a project exists, **When** the user adds a task with `--start 23:00 --end 01:00`, **Then** the system rejects the entry with an error, since tasks are daily and cannot span midnight via manual entry.

---

### User Story 2 - Enter Duration in Human-Friendly Format (Priority: P1)

As a user, I want to enter durations using a human-readable format like `1h30m`, `45m`, or `2h`, so that I do not have to convert everything to raw minutes.

**Why this priority**: Duration entry is equally as important as start/end times. A friendlier format reduces mental effort and input errors.

**Independent Test**: Can be fully tested by adding tasks with various duration formats and verifying each is parsed and stored correctly.

**Acceptance Scenarios**:

1. **Given** a project exists, **When** the user adds a task with `--duration 1h30m`, **Then** the system records a task with a duration of 90 minutes.
2. **Given** a project exists, **When** the user adds a task with `--duration 45m`, **Then** the system records a task with a duration of 45 minutes.
3. **Given** a project exists, **When** the user adds a task with `--duration 2h`, **Then** the system records a task with a duration of 120 minutes.
4. **Given** a project exists, **When** the user adds a task with `--duration 90` (plain number), **Then** the system treats it as 90 minutes for backward compatibility.
5. **Given** a project exists, **When** the user adds a task with `--duration 0m`, **Then** the system rejects the entry with an error indicating the duration must be greater than zero.
6. **Given** a project exists, **When** the user adds a task with `--duration abc`, **Then** the system rejects the entry with an error indicating the format is invalid and shows accepted formats.

---

### User Story 3 - Duration Display in Human-Friendly Format (Priority: P2)

As a user, I want durations displayed in the same human-friendly format (e.g., `1h 30m`) in all output (journal, reports, clock status), so that the input and output formats are consistent.

**Why this priority**: Consistency between input and output formats reduces cognitive load, but the feature works without this if durations are displayed in any readable format.

**Independent Test**: Can be fully tested by logging a task and verifying the journal and report views display durations in `Xh Ym` format.

**Acceptance Scenarios**:

1. **Given** a task with a duration of 90 minutes exists, **When** the user views the journal, **Then** the duration is displayed as `1h 30m`.
2. **Given** a task with a duration of 45 minutes exists, **When** the user views the journal, **Then** the duration is displayed as `45m`.
3. **Given** a task with a duration of 120 minutes exists, **When** the user views a report, **Then** the duration is displayed as `2h`.
4. **Given** a clock is running with 65 minutes elapsed, **When** the user checks clock status, **Then** the elapsed time is displayed as `1h 5m`.

---

### Edge Cases

- What happens when the user enters `--start 9` without minutes? The system rejects the input and displays the accepted format (`H:MM` or `HH:MM`).
- What happens when the user enters `--duration 1h90m` (minutes >= 60)? The system accepts it and normalizes to 2h 30m (150 minutes total).
- What happens when the user enters `--duration 0h0m`? The system rejects the entry since the duration must be greater than zero.
- What happens when the user edits a task's start or end time? The same simplified time format applies to the edit command.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST accept start and end times in `H:MM` or `HH:MM` format (24-hour clock), representing a time of day. Both single-digit and zero-padded hours are valid (e.g., `9:00` and `09:00`). ISO 8601 full datetime strings are no longer accepted for `--start`/`--end`.
- **FR-002**: System MUST accept durations in the following compact formats (no spaces): `Xh`, `Ym`, `XhYm`, `XhY` (trailing `m` is optional), or a plain number (interpreted as minutes). Spaced input (e.g., `1h 30m`) is not supported.
- **FR-003**: System MUST reject start/end times that are not in valid `H:MM` or `HH:MM` format with an error message showing the expected format.
- **FR-004**: System MUST reject durations that are zero or negative with an error message.
- **FR-005**: System MUST reject duration strings that do not match any accepted format with an error message listing accepted formats.
- **FR-006**: System MUST normalize durations with minutes >= 60 (e.g., `1h90m` becomes 150 minutes, displayed as `2h 30m`).
- **FR-007**: System MUST display all durations in `Xh Ym` format, omitting the hours component when it is zero (e.g., `45m`) and omitting the minutes component when it is zero (e.g., `2h`).
- **FR-008**: System MUST apply the same time input formats to both the add and edit commands for task entries.
- **FR-009**: Start and end times MUST be interpreted as times on the task's date (today by default, or the date specified via `--date`).
- **FR-010**: System MUST reject manual entries where start time equals or is after end time, since tasks are daily and cannot span midnight.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can log a task with start/end times using clock notation (`H:MM` or `HH:MM`) instead of full datetime strings.
- **SC-002**: Users can specify durations using natural notation (`1h30m`, `45m`, `2h`) in addition to plain minutes.
- **SC-003**: 100% of duration displays across the application (journal, reports, clock status) use the `Xh Ym` format consistently.
- **SC-004**: Invalid time or duration inputs produce error messages that include the accepted formats.
- **SC-005**: Backward compatibility is maintained: plain numbers for `--duration` continue to be interpreted as minutes.

## Assumptions

- Tasks are always within a single calendar day. Manual entries that would span midnight (start time after end time) are rejected.
- The `--date` option determines which day the start/end times belong to. It defaults to today.
- The 24-hour clock format is used for time input (`HH:MM`). 12-hour AM/PM format is not supported.
- Duration input is case-insensitive (`1H30M` is equivalent to `1h30m`).
- This feature modifies the existing time input parsing and duration display formatting. It does not change how time is stored internally.
- ISO 8601 full datetime format (e.g., `2026-02-28T09:00:00`) is removed for `--start`/`--end` inputs. Users specify the date separately via `--date` if needed.
