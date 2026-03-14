# Data Model: PDF Report Export

**Date**: 2026-02-28

## No New Entities

This feature does not introduce new data entities or modify existing ones. The PDF is generated from the existing `Report` struct defined in `src/services/report.rs`.

## Existing Entities Used

### Report (read-only)

| Field | Type | Description |
|-------|------|-------------|
| `from` | `NaiveDate` | Report start date |
| `to` | `NaiveDate` | Report end date |
| `project_sections` | `Vec<ProjectSection>` | Tasks grouped by project |
| `grand_total` | `i64` | Total minutes across all projects |

### ProjectSection (read-only)

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Project name |
| `entries` | `Vec<TaskEntry>` | Task entries for this project |
| `total` | `i64` | Total minutes for this project |

### TaskEntry (read-only)

| Field | Type | Description |
|-------|------|-------------|
| `id` | `i64` | Task entry ID |
| `description` | `String` | Task description |
| `start_time` | `Option<NaiveDateTime>` | Start time (if set) |
| `end_time` | `Option<NaiveDateTime>` | End time (if set) |
| `duration_min` | `i64` | Duration in minutes |
| `created_at` | `NaiveDateTime` | Creation timestamp |

## Output Artifact

The PDF file is a pure output artifact — it is not stored in the database and has no lifecycle beyond file creation. It is generated on-demand from the `Report` struct and written to disk.

### PDF Structure Mapping

The PDF renderer transforms the `Report` struct into a two-section document:

**Section 1: Project Summary**
- Source: `report.project_sections` → extract `name` and `total` fields
- Format: Two-column table (`| Project | Total Hours |`)
- Sorting: Alphabetical by project name

**Section 2: Daily Detail**
- Source: `report.project_sections` → flatten all `entries`, group by date (extracted from `start_time` or fallback to `created_at`)
- Format: For each date, render heading + six-column table (`| ID | Description | Project | Start | End | Duration |`)
- Sorting: Chronological by date, then by start time within each day

**Grand Total**
- Source: `report.grand_total`
- Format: Final paragraph after all sections
