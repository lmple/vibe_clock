# Research: Simplified Time Input Format

**Branch**: `003-time-input-format` | **Date**: 2026-02-28

## Decision 1: Single-Digit Hour Parsing

**Decision**: Use chrono's `%-H:%M` format specifier.

**Rationale**: Chrono 0.4 supports the `%-` modifier which suppresses padding requirements. `NaiveTime::parse_from_str(input, "%-H:%M")` accepts both `9:00` and `09:00` in a single call. This avoids a custom parser while handling all edge cases (hour range 0-23, minute range 0-59).

**Alternatives considered**:
- Custom split-on-colon parser: More code, manual validation of ranges, no advantage over chrono's built-in.
- Two-pass parsing (`%H:%M` then retry): Unnecessary since `%-H:%M` handles both cases.

## Decision 2: Case-Insensitive Duration Parsing

**Decision**: Convert duration input to lowercase before parsing.

**Rationale**: A single `.to_lowercase()` call at the top of `parse_duration` handles all case variations (`1H30M`, `1h30m`, `1H30m`) with zero additional complexity. This is applied after the plain-integer check (numbers are case-irrelevant).

**Alternatives considered**:
- Case-sensitive only: Unnecessarily strict; users naturally type mixed case.
- Regex-based parser: Overkill for this pattern; current char-search approach is simpler.

## Decision 3: Signature Change for `parse_time`

**Decision**: Change `parse_time(input: &str) -> Result<NaiveDateTime>` to `parse_time(input: &str, date: NaiveDate) -> Result<NaiveDateTime>`.

**Rationale**: The caller always knows the target date (today by default, or from `--date`). Making the date an explicit required parameter eliminates hidden coupling to `Local::now()` and improves testability. All call sites already have the date available or can default to today.

**Alternatives considered**:
- `Option<NaiveDate>` parameter with internal default: Hides the default behavior; explicit is better for a function used in multiple contexts (add, edit).
- Keep using `Local::now()` internally: Makes unit testing depend on wall clock time; violates Constitution Principle II (test isolation).
