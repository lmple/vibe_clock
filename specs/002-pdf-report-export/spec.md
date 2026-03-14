# Feature Specification: PDF Report Export

**Feature Branch**: `002-pdf-report-export`
**Created**: 2026-02-28
**Status**: Draft
**Input**: User description: "Add PDF export for the report"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Export Report as PDF File (Priority: P1)

As a user, I want to export my time report to a PDF file so that I can share it with clients, managers, or stakeholders who do not have access to the application.

**Why this priority**: This is the core feature request. Without PDF export, the feature has no value.

**Independent Test**: Can be fully tested by generating a report for a date range and exporting it to a PDF file, then opening the PDF to verify it contains the correct data.

**Acceptance Scenarios**:

1. **Given** tasks exist within a date range, **When** the user generates a report with the PDF export option and specifies an output file path, **Then** a PDF file is created at the specified path containing two sections: a Project Summary (projects with total hours) and a Daily Detail section (chronological days with full task details).
2. **Given** tasks exist within a date range, **When** the user generates a PDF report, **Then** the PDF contains: (1) Project Summary section listing each project name with total hours, (2) Daily Detail section showing each day chronologically with all tasks including description, time, duration, and project name, and (3) a grand total at the end.
3. **Given** the user specifies a PDF export without an output file path, **When** the report is generated, **Then** the system saves the PDF to the current working directory with an auto-generated filename and displays the file path to the user.
4. **Given** no tasks exist within the selected date range, **When** the user requests a PDF export, **Then** the system displays a message indicating no data is available and does not create a PDF file.
5. **Given** the user wants a report for a single date, **When** the user runs the report command with only `--from` (omitting `--to`), **Then** the system generates a report for that single date only (e.g., `--from 2026-03-01 --pdf` or `--from today --pdf`).

---

### User Story 2 - Customizable PDF Output Path (Priority: P2)

As a user, I want to control where the PDF file is saved so that I can organize my exported reports in a location of my choosing.

**Why this priority**: File path control is important for usability but the feature works with a default path if this is not implemented.

**Independent Test**: Can be fully tested by exporting a report with a custom output path and verifying the file is created at the specified location.

**Acceptance Scenarios**:

1. **Given** the user provides an output path ending in `.pdf`, **When** the report is exported, **Then** the PDF is saved to that exact path.
2. **Given** the user provides an output path to a directory (no filename), **When** the report is exported, **Then** the PDF is saved in that directory with an auto-generated filename based on the date range (e.g., `report-2026-02-01-to-2026-02-28.pdf`).
3. **Given** the user provides an output path whose parent directory does not exist, **When** the report is exported, **Then** the system displays an error indicating the directory does not exist.
4. **Given** a file already exists at the output path, **When** the user exports a report, **Then** the system overwrites the existing file without prompting (the user explicitly chose the path).

---

### Edge Cases

- What happens when the report contains a very large number of entries (e.g., a full year)? The PDF is still generated correctly, spanning multiple pages as needed.
- What happens when the user does not have write permissions to the output path? The system displays an actionable error message indicating the file could not be written.
- What happens when the disk is full? The system displays an error message and does not leave a partial PDF file on disk.
- What happens when project or task names contain special characters (unicode, accented characters, emoji)? The PDF renders them correctly.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST allow users to export a time report as a PDF file by adding `--pdf` and/or `--output` options to the existing report command. Providing `--output` with a `.pdf` path implicitly triggers PDF generation without requiring `--pdf`. The `--to` flag is optional; if omitted, it defaults to the same date as `--from` (single-day report).
- **FR-002**: System MUST generate a PDF with two distinct sections: (1) Project Summary section listing each project with total hours spent, and (2) Daily Detail section showing a chronological list of days, where each day displays all tasks with complete information (task description, start/end time, duration, and project name). The PDF MUST include a grand total across all projects.
- **FR-003**: System MUST include a header in the PDF showing the report title, date range, and generation date.
- **FR-004**: When the user provides a file path with `--output`, the system MUST save the PDF to that path.
- **FR-005**: When the user provides a directory path with `--output`, the system MUST generate a filename based on the date range (format: `report-YYYY-MM-DD-to-YYYY-MM-DD.pdf`).
- **FR-006**: When no `--output` is provided but PDF export is requested via `--pdf`, the system MUST save the PDF to the current working directory with an auto-generated filename.
- **FR-007**: System MUST NOT create a PDF file when no tasks exist in the selected date range.
- **FR-008**: System MUST display the full path of the created PDF file upon successful export.
- **FR-009**: System MUST display an actionable error message if the PDF file cannot be written (permissions, invalid path, disk full).
- **FR-010**: System MUST handle multi-page reports when the data exceeds a single page.
- **FR-011**: System MUST render unicode and accented characters correctly in the PDF.
- **FR-012**: When PDF export is triggered, the system MUST still print the terminal report to stdout AND display the PDF file path after the report output.

### Key Entities

- **PDF Report**: A file representation of a time report for a given date range. Contains two distinct sections: (1) Project Summary - a list of projects with total hours spent on each, and (2) Daily Detail - a chronological list of days showing all tasks with complete details (description, start/end time, duration, project name). Includes a header with the report title, date range, and generation timestamp, plus a grand total at the end.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can export a report to PDF in a single command by adding one option to the existing report command.
- **SC-002**: The exported PDF contains 100% of the data shown in the terminal report for the same date range.
- **SC-003**: PDF generation for a full year of data (approximately 2,000 entries) completes within 5 seconds.
- **SC-004**: The exported PDF is a valid PDF file that opens correctly in standard PDF readers.
- **SC-005**: Error messages for export failures clearly indicate the cause and suggest a resolution.

## Clarifications

### Session 2026-02-28

- Q: When `--pdf` is used, should the terminal report still be printed? → A: Yes, print terminal report AND generate PDF, displaying the PDF file path at the end.
- Q: Should `--output file.pdf` implicitly trigger PDF generation without `--pdf`? → A: Yes, `--output` with a `.pdf` path implies PDF generation; `--pdf` flag is not required when `--output` is provided.

### Session 2026-03-01

- Q: What should be the exact structure of the PDF report? → A: Two sections: (1) Project Summary - list of projects with total hours; (2) Daily Detail - chronological list of days, each showing all tasks with full details (description, time, project)
- Q: How should users specify single-date reports (like today) more concisely? → A: Make `--to` optional; if omitted, use the same date as `--from` (single-day report). Supports: `--from 2026-03-01 --pdf` or `--from today --pdf`

## Assumptions

- The PDF export extends the existing `vibe-clock report` command rather than introducing a new top-level command.
- The PDF layout uses a simple, professional tabular format suitable for printing on A4/Letter paper.
- No interactive preview of the PDF is needed; the file is written directly to disk.
- The PDF does not need to support custom branding, logos, or color themes in this version.
- The system selects portrait or landscape orientation automatically based on content width.
- The `--pdf` flag triggers PDF export; `--output` optionally specifies the file path.
