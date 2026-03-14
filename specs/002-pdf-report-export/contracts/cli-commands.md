# CLI Contract: PDF Report Export

**Date**: 2026-02-28

## `vibe-clock report` Changes

### New Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `--pdf` | flag | No | Triggers PDF export to current directory with auto-generated filename |
| `--output` | `String` | No | Output path for PDF file. If path ends in `.pdf`, used as-is. If directory, auto-generates filename. Implies `--pdf`. |

### Behavior Matrix

| `--pdf` | `--output` | Result |
|---------|------------|--------|
| absent | absent | Terminal report only (current behavior) |
| present | absent | Terminal report + PDF in current directory |
| absent | `file.pdf` | Terminal report + PDF at specified path |
| absent | `/some/dir/` | Terminal report + PDF in directory with auto name |
| present | `file.pdf` | Terminal report + PDF at specified path |
| present | `/some/dir/` | Terminal report + PDF in directory with auto name |

### Auto-Generated Filename Format

```
report-YYYY-MM-DD-to-YYYY-MM-DD.pdf
```

Example: `report-2026-02-01-to-2026-02-28.pdf`

### Success Output

After the terminal report, an additional line is printed:

```
PDF report saved to /absolute/path/to/report-2026-02-01-to-2026-02-28.pdf
```

### Error Messages

| Condition | Message | Exit Code |
|-----------|---------|-----------|
| Parent directory doesn't exist | `Error: Directory '/path/to' does not exist.` | 1 |
| Permission denied | `Error: Cannot write to '/path/to/file.pdf': permission denied.` | 2 |
| Disk full / I/O error | `Error: Failed to write PDF: <system error>. No partial file was created.` | 2 |
| No tasks in range (with --pdf) | `No tasks found between YYYY-MM-DD and YYYY-MM-DD.` (no PDF created) | 0 |

### Examples

```bash
# Generate report + PDF in current directory
vibe-clock report --from 2026-02-01 --to 2026-02-28 --pdf

# Generate report + PDF at specific path
vibe-clock report --from 2026-02-01 --to 2026-02-28 --output ~/reports/february.pdf

# --output implies --pdf
vibe-clock report --from 2026-02-01 --to 2026-02-28 --output ~/reports/

# No PDF, just terminal (unchanged)
vibe-clock report --from 2026-02-01 --to 2026-02-28
```
