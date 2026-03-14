# Research: PDF Report Export

**Date**: 2026-02-28

## Decision 1: PDF Library

**Decision**: Use `genpdfi` (v0.2.7) — an actively maintained fork of `genpdf`.

**Rationale**: `genpdfi` is a high-level, pure-Rust PDF generator built on `printpdf` and `rusttype`. It provides automatic page layout, text alignment, multi-page support, and a `TableLayout` element — all required for this feature. Being pure Rust means no system dependencies, which aligns with the project's existing approach (bundled SQLCipher, etc.).

**Alternatives considered**:
- `printpdf`: Too low-level. Would require manual text measurement, line wrapping, page breaking, and table layout. Significantly more code for the same result.
- `genpdf` (original): Unmaintained — no commits in 3+ years. `genpdfi` is its actively maintained fork (updated January 2026).
- HTML-to-PDF (headless Chrome, WeasyPrint): Requires external runtime dependencies. Violates the pure-Rust, zero-external-deps approach.

## Decision 2: Font for Unicode Support

**Decision**: Bundle a TTF font that supports Latin + extended Unicode (e.g., Liberation Sans or Noto Sans). Load it at PDF generation time.

**Rationale**: `genpdfi` requires explicit font loading for rendering text. The default PDF fonts (Helvetica, etc.) only support basic Latin. To satisfy FR-011 (unicode, accented characters), we need an embedded font with broad glyph coverage. Liberation Sans or Noto Sans are open-source (SIL OFL), widely compatible, and provide good coverage.

**Alternatives considered**:
- System font detection: Platform-dependent, unreliable in CI/containers.
- PDF built-in fonts only: Would fail for unicode characters (accented names, emoji). Violates FR-011.

## Decision 3: Output Path Resolution

**Decision**: Implement path resolution in a dedicated `resolve_pdf_path` function with three cases:
1. `--output path.pdf` → use as-is (validate parent directory exists)
2. `--output /some/dir/` (directory) → append auto-generated filename
3. `--pdf` only (no `--output`) → current working directory + auto-generated filename

Auto-generated filename format: `report-YYYY-MM-DD-to-YYYY-MM-DD.pdf`

**Rationale**: Follows FR-004, FR-005, FR-006 directly. Separating path resolution into its own function makes it independently testable and keeps the PDF generation logic clean.

## Decision 4: Atomic Write Strategy

**Decision**: Write PDF to a temporary file in the same directory, then rename atomically on success. Delete temp file on failure.

**Rationale**: Satisfies the edge case requirement that disk-full or write errors must not leave a partial PDF file. Same-directory temp file ensures the rename is atomic (same filesystem).
