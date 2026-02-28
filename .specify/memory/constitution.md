<!--
  Sync Impact Report
  ===================
  Version change: 0.0.0 → 1.0.0 (initial ratification)

  Added principles:
    - I. Code Quality
    - II. Testing Standards (NON-NEGOTIABLE)
    - III. User Experience Consistency
    - IV. Performance Requirements

  Added sections:
    - Development Workflow
    - Quality Gates
    - Governance

  Removed sections: None (initial version)

  Templates requiring updates:
    - .specify/templates/plan-template.md — ✅ No changes needed (Constitution Check section already generic)
    - .specify/templates/spec-template.md — ✅ No changes needed (spec is technology-agnostic by design)
    - .specify/templates/tasks-template.md — ✅ No changes needed (test-first ordering already present)
    - README.md — ⚠️ Pending: consider adding constitution reference once project has more content

  Follow-up TODOs: None
-->

# Vibe Clock Constitution

## Core Principles

### I. Code Quality

- All code MUST follow a single, consistent style enforced by automated formatting and linting tools configured at the project root.
- Functions and modules MUST have a single, clear responsibility. A function that requires more than one sentence to describe its purpose MUST be split.
- Public interfaces (CLI commands, module exports) MUST include type annotations or equivalent type safety mechanisms provided by the chosen language.
- Magic numbers, hardcoded strings, and duplicated logic MUST be extracted into named constants or shared utilities.
- Dead code, commented-out code, and unused imports MUST be removed before merging. No "just in case" code is permitted.
- Dependencies MUST be explicitly declared and version-pinned. New dependencies MUST be justified by a concrete need that cannot be met with existing dependencies or reasonable custom code.
- Error messages MUST be actionable: they MUST tell the user what went wrong and what they can do about it.

### II. Testing Standards (NON-NEGOTIABLE)

- Every user-facing feature MUST have at least one automated test that exercises the full command path (input to output).
- Test-Driven Development (TDD) is the default workflow: write failing tests first, then implement until tests pass, then refactor. Deviations MUST be justified in the commit message.
- Unit tests MUST be isolated: no file system access, no real clocks, no shared mutable state between tests.
- Integration tests MUST verify data persistence round-trips: write data, restart the application (or reload from storage), read data back, and assert correctness.
- Edge cases identified in the specification MUST each have a corresponding test.
- Test names MUST describe the behavior being verified, not the implementation detail (e.g., "rejects_end_time_before_start_time" not "test_validate_method").
- All tests MUST pass before any code is merged. A failing test suite is a blocking issue.

### III. User Experience Consistency

- All CLI commands MUST follow a consistent subcommand pattern: `<app> <resource> <action> [arguments] [options]`.
- All commands MUST provide `--help` output describing usage, arguments, and options.
- Output for listing operations MUST use a consistent tabular format with aligned columns.
- Error output MUST go to stderr. Normal output MUST go to stdout.
- Destructive operations (delete, overwrite) MUST require explicit confirmation unless a `--yes` or `-y` flag is provided.
- Date and time inputs MUST accept ISO 8601 format. Other convenient formats (e.g., "today", "yesterday", relative dates) SHOULD be supported but ISO 8601 MUST always work.
- Success operations MUST provide brief, human-readable confirmation messages (e.g., "Project 'Acme' created." not silent success).
- Exit codes MUST follow convention: 0 for success, 1 for user errors (bad input), 2 for system errors (I/O failure).

### IV. Performance Requirements

- All CLI commands MUST respond within 200 milliseconds for typical workloads (up to 1,000 task entries and 50 projects).
- Data storage operations MUST be atomic: a crash or interruption MUST NOT corrupt the data file or leave it in a partial state.
- The application MUST NOT load the entire data set into memory when only a subset is needed (e.g., querying a single day SHOULD NOT require reading all historical entries if the storage format allows selective access).
- Report generation for a full year of data (approximately 2,000 entries) MUST complete within 1 second.
- The application MUST start and be ready to accept input without a perceptible delay (under 100 milliseconds cold start).

## Quality Gates

- **Pre-commit**: Formatting and linting checks MUST pass automatically. Code that fails formatting MUST be rejected.
- **Pre-merge**: All tests (unit, integration, edge case) MUST pass. Test coverage for new code MUST meet or exceed the project average.
- **Specification compliance**: Every functional requirement (FR-XXX) in the spec MUST map to at least one test. Untested requirements MUST be flagged during review.
- **Performance validation**: Commands that interact with data MUST be benchmarked against the performance thresholds defined in Principle IV before release.

## Development Workflow

- Commits MUST be atomic: one logical change per commit. A commit that mixes unrelated changes MUST be split.
- Commit messages MUST follow conventional format: `type: description` where type is one of `feat`, `fix`, `refactor`, `test`, `docs`, `chore`.
- Branches MUST follow the naming convention established by speckit: `NNN-short-name`.
- Code review (or self-review for solo development) MUST verify compliance with all four core principles before merging.
- When a principle is intentionally violated, the violation MUST be documented inline with a comment explaining the rationale and referencing the principle number (e.g., `// Violates Principle IV: acceptable because this is a one-time migration`).

## Governance

- This constitution supersedes all ad-hoc practices. When a principle conflicts with convenience, the principle wins unless a documented exception is recorded.
- Amendments to this constitution MUST include: the change description, the rationale, and an update to the version number following semantic versioning (MAJOR for principle removals or redefinitions, MINOR for new principles or material expansions, PATCH for clarifications and wording fixes).
- All pull requests and code reviews MUST verify compliance with the active constitution version.
- Complexity beyond what is strictly required MUST be justified against Principle I (Code Quality) and Principle IV (Performance). "It might be useful later" is not a valid justification.

**Version**: 1.0.0 | **Ratified**: 2026-02-28 | **Last Amended**: 2026-02-28
