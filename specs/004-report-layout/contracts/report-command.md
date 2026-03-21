# Contract: report Command

**Feature**: 004-report-layout
**Date**: 2026-03-21

## Overview

The `report` command interface is **unchanged** by this feature. Only the output layout changes.

## Command Schema

```
vibe-clock report --from <DATE> [--to <DATE>] [--pdf] [--output <PATH>]
```

### Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `--from` | DATE | Yes | Start date of the report period. Accepts `YYYY-MM-DD`, `today`, `yesterday`. |
| `--to` | DATE | No | End date (inclusive). Defaults to `--from` value if omitted. |
| `--pdf` | flag | No | Generate a PDF report in the current directory. |
| `--output` | PATH | No | Write PDF to a specific path or directory. Implies `--pdf`. |

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | User error (invalid date, `--from` after `--to`, nonexistent output directory) |
| 2 | System error (I/O failure writing PDF) |

## Output Contract (Terminal)

### When tasks exist

```
Report: <FROM> to <TO>

Project Summary
<aligned table: Project | Total>
...
TOTAL: <grand_total>

<FROM_DATE>
<aligned table: ID | Project | Description | Start | End | Duration>
...

<NEXT_DATE>
<aligned table: ID | Project | Description | Start | End | Duration>
...
```

**Constraints**:
- Project summary table appears first, always
- Date sections appear in ascending date order
- Only dates with tasks appear as sections
- Descriptions are never truncated; long descriptions wrap within the description column
- Start/End show `-` for tasks with no recorded time
- Duration uses `Xh Ym` format

### When no tasks exist

```
No tasks found between <FROM> and <TO>.
```

## Output Contract (PDF)

The PDF mirrors the terminal layout:
1. Header: "Time Report", date range, generation timestamp
2. Project summary table (Project | Total Hours)
3. Per-day sections (date heading + ID | Description | Project | Start | End | Duration table)
4. Grand Total

**No truncation** in either terminal or PDF output.

## Stability

This contract is stable. The command flags, argument names, and exit codes are not modified by this feature. Only the ordering and content of the terminal and PDF output changes (project-grouped layout removed; two-part layout is the only format).
