# Implementation Plan: PDF Report Export

**Branch**: `002-pdf-report-export` | **Date**: 2026-03-01 | **Spec**: `specs/002-pdf-report-export/spec.md`
**Input**: Feature specification from `/specs/002-pdf-report-export/spec.md`

## Summary

Add PDF export to the existing `vibe-clock report` command via `--pdf` and `--output` flags. The PDF contains two distinct sections: (1) Project Summary listing projects with total hours, and (2) Daily Detail showing chronological days with full task information. Uses the `genpdfi` crate (pure Rust, high-level API with TableLayout, multi-page, font embedding) to render reports. Terminal output is preserved when PDF is generated.

## Technical Context

**Language/Version**: Rust 2024 edition, MSRV 1.85.0
**Primary Dependencies**: genpdfi (PDF generation), chrono (dates), clap v4 (CLI)
**Storage**: No changes — reads from existing SQLite/SQLCipher database
**Testing**: cargo test (unit + integration via assert_cmd)
**Target Platform**: Linux (primary), macOS, Windows
**Project Type**: CLI application
**Performance Goals**: PDF generation for 2,000 entries within 5 seconds (SC-003)
**Constraints**: Pure Rust dependencies only; no external runtime tools
**Scale/Scope**: Reports up to ~2,000 task entries, multi-page PDF output

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Gate | Status | Notes |
|-----------|------|--------|-------|
| I. Code Quality | Single responsibility, no magic numbers | PASS | PDF generation isolated in `src/services/pdf.rs`; path resolution in separate function; two-section rendering logic cleanly separated |
| I. Code Quality | New dependency justified | PASS | `genpdfi` is required — no existing dep can generate PDFs. Pure Rust, no system deps. |
| II. Testing Standards | Full command path tests | PASS | Integration tests verify PDF file creation, two-section structure, content validation |
| II. Testing Standards | TDD workflow | PASS | Tests written before implementation per task ordering |
| II. Testing Standards | Edge case tests | PASS | Permission errors, no-data, unicode, large datasets all have corresponding tests |
| III. UX Consistency | Subcommand pattern | PASS | Extends existing `report` command with `--pdf`/`--output` options |
| III. UX Consistency | Errors to stderr | PASS | All PDF write errors go to stderr via existing AppError |
| III. UX Consistency | Destructive op confirmation | PASS | Overwrite is intentional (user chose the path); no confirmation needed per spec |
| IV. Performance | 200ms typical, 1s report | PASS | PDF generation is additive; terminal report stays under 1s; PDF adds bounded I/O (target: 5s for 2,000 entries) |

## Project Structure

### Documentation (this feature)

```text
specs/002-pdf-report-export/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── cli-commands.md  # CLI contract changes
└── tasks.md             # Phase 2 output (created by /speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── cli/
│   ├── mod.rs           # Add --pdf and --output args to Report command
│   └── report.rs        # Wire PDF generation after terminal report
├── services/
│   ├── report.rs        # Existing report generation (unchanged)
│   └── pdf.rs           # NEW: PDF rendering with two-section structure
└── formatting/
    └── mod.rs           # Unchanged

tests/
├── cli_report.rs        # Integration tests for --pdf/--output
└── edge_cases.rs        # PDF edge case tests

assets/
└── fonts/
    └── LiberationSans-Regular.ttf  # Bundled font for unicode support
```

**Structure Decision**: Single new file `src/services/pdf.rs` handles all PDF generation with clear separation between Project Summary and Daily Detail rendering. CLI changes are minimal additions to existing `src/cli/mod.rs` and `src/cli/report.rs`. Font bundled in `assets/fonts/` and included via `include_bytes!`.

## Key Design Decisions

1. **Two-Section PDF Structure (FR-002 Clarification)**:
   - **Section 1: Project Summary** — Table listing each project name with total hours spent. Provides high-level overview for quick reference.
   - **Section 2: Daily Detail** — Chronological listing by date. Each date shows all tasks with complete details (ID, description, project, start time, end time, duration). Provides audit trail and detailed breakdown.
   - **Rationale**: Separates summary view (for executives/clients) from detail view (for verification/billing). Matches typical time-tracking report patterns.

2. **`Report` struct reuse**: The existing `Report` struct from `src/services/report.rs` contains project sections and task data. The PDF renderer adapts this to the two-section format: extract project totals for summary, reorganize tasks by date for daily detail.

3. **PDF rendering pipeline**: `render_pdf(report: &Report, output_path: &Path) -> Result<()>` in `src/services/pdf.rs`. Constructs a `genpdfi::Document`, adds header, renders Project Summary table, renders Daily Detail section (grouped by date), adds grand total, writes to file.

4. **Path resolution**: `resolve_pdf_path(output: Option<&str>, pdf_flag: bool, from: NaiveDate, to: NaiveDate) -> Result<Option<PathBuf>, AppError>` determines if/where to write PDF. Returns `None` if no PDF requested, `Err` for invalid paths.

5. **Atomic write**: Write to temp file (`.tmp` suffix), rename on success. Prevents partial files on disk failure (edge case requirement).

6. **Font embedding**: Bundle Liberation Sans TTF via `include_bytes!` macro. Single regular weight is sufficient for tabular reports.

## Code Changes Summary

| File | Change | Rationale |
|------|--------|-----------|
| `Cargo.toml` | Add `genpdfi` dependency | PDF generation library |
| `src/cli/mod.rs` | Add `--pdf` and `--output` to `Report` command | FR-001 |
| `src/cli/report.rs` | Call PDF generation after terminal output | FR-012, FR-008 |
| `src/services/pdf.rs` | NEW: PDF rendering with two sections | FR-002 (Project Summary + Daily Detail), FR-003, FR-010, FR-011 |
| `src/services/mod.rs` | Add `pub mod pdf;` | Module declaration |
| `tests/cli_report.rs` | Add PDF integration tests | Constitution II |
| `tests/edge_cases.rs` | Add PDF edge case tests | Constitution II |
| `assets/fonts/` | Bundle Liberation Sans TTF | FR-011 |

## Implementation Notes

### Project Summary Section (FR-002 Part 1)

- Render as a simple table: `| Project | Total Hours |`
- Extract project names and totals from `report.project_sections`
- Use genpdfi `TableLayout` with two columns
- Sort alphabetically by project name for consistency

### Daily Detail Section (FR-002 Part 2)

- Group tasks chronologically by date (extract from task `start_time` or `created_at`)
- For each date:
  - Heading: `## YYYY-MM-DD`
  - Table: `| ID | Description | Project | Start | End | Duration |`
- Use genpdfi `TableLayout` with six columns
- Preserve all task details for audit trail

### Multi-Page Support (FR-010)

- genpdfi handles pagination automatically when content exceeds page height
- No special handling needed beyond setting appropriate margins and fonts

### Error Handling (FR-009)

- Parent directory validation: `parent().exists()` check before write
- Permission errors: Caught by `std::io::Error`, wrapped in `AppError::SystemError`
- Disk full: Atomic write + temp file deletion ensures no partial PDFs
- All errors include actionable messages per Constitution I

