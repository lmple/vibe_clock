# CLI Command Contract Changes: Simplified Time Input Format

**Branch**: `003-time-input-format` | **Date**: 2026-02-28

This document describes changes to existing CLI command contracts. Only modified options are listed.

## `vibe-clock task add` — Updated Options

| Option | Before | After |
|--------|--------|-------|
| `--start` | Start time (YYYY-MM-DDTHH:MM or HH:MM) | Start time (HH:MM, 24-hour clock, e.g., 9:00 or 09:00) |
| `--end` | End time (YYYY-MM-DDTHH:MM or HH:MM) | End time (HH:MM, 24-hour clock, e.g., 17:30) |
| `--duration` | Duration in minutes (alternative to start/end) | Duration (e.g., 1h30m, 45m, 2h, or 90 for minutes) |
| `--date` | Date for the entry (defaults to today, used with --duration) | Date for the entry (YYYY-MM-DD, defaults to today; applies to --start/--end and --duration) |

**Behavioral changes**:
- `--start`/`--end` no longer accept ISO 8601 datetime (e.g., `2026-02-28T14:30`). Use `--date 2026-02-28 --start 14:30` instead.
- `--date` now applies to `--start`/`--end` entries as well (previously only documented for `--duration`).

## `vibe-clock task edit` — Updated Options

| Option | Before | After |
|--------|--------|-------|
| `--start` | New start time | New start time (HH:MM, 24-hour clock) |
| `--end` | New end time | New end time (HH:MM, 24-hour clock) |
| `--duration` | New duration in minutes | New duration (e.g., 1h30m, 45m, 2h, or 90 for minutes) |
| `--date` | *(not present)* | Move task to a different date (YYYY-MM-DD, `today`, or `yesterday`) |

**Behavioral changes**:
- When `--date` is provided with `--start`/`--end`, the new times are applied on that date.
- When only `--date` is provided (no `--start`/`--end`), the existing start/end times are moved to the new date (timestamps updated, duration preserved).
- When editing start/end times without `--date`, the time is applied to the existing task's date (unchanged from before).

## `--date` Accepted Values (task add and task edit)

Matches `journal` shortcut behavior (clarification 2026-03-21):

| Value | Resolves to |
|-------|-------------|
| `YYYY-MM-DD` | The specified date |
| `today` | Current calendar day |
| `yesterday` | Previous calendar day |

## Error Messages

| Condition | Message |
|-----------|---------|
| Invalid time format | `Invalid time: '<input>'. Use HH:MM format (e.g., 9:00 or 14:30)` |
| Invalid duration format | `Invalid duration: '<input>'. Use Xh, Ym, XhYm, or minutes (e.g., 1h30m, 45m, 2h, 90)` |
| Zero/negative duration | `Duration must be greater than 0` |
| End time <= start time | `End time must be after start time.` |
