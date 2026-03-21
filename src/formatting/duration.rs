use anyhow::{Result, bail};

/// Format a duration in minutes as "Xh Ym" string.
pub fn format_duration(minutes: i64) -> String {
    let hours = minutes / 60;
    let mins = minutes % 60;
    if hours > 0 && mins > 0 {
        format!("{hours}h {mins}m")
    } else if hours > 0 {
        format!("{hours}h")
    } else {
        format!("{mins}m")
    }
}

/// Parse a duration string into minutes.
///
/// Supported formats (case-insensitive):
/// - Plain integer (e.g., "90") → 90 minutes
/// - "XhYm" (e.g., "1h30m") → 90 minutes
/// - "Xh" (e.g., "2h") → 120 minutes
/// - "Ym" (e.g., "45m") → 45 minutes
/// - "XhY" (trailing 'm' optional, e.g., "1h30") → 90 minutes
///
/// Space-separated input (e.g., "1h 30m") is not supported.
pub fn parse_duration(input: &str) -> Result<i64> {
    let trimmed = input.trim();

    // Try plain integer first (case-irrelevant, no letters)
    if let Ok(mins) = trimmed.parse::<i64>() {
        if mins <= 0 {
            bail!("Duration must be greater than 0");
        }
        return Ok(mins);
    }

    // Reject internal whitespace (e.g., "1h 30m")
    if trimmed.contains(char::is_whitespace) {
        bail!(
            "Invalid duration: '{trimmed}'. Use Xh, Ym, XhYm, or minutes (e.g., 1h30m, 45m, 2h, 90)"
        );
    }

    // Normalise to lowercase for case-insensitive parsing
    let s = trimmed.to_lowercase();

    let mut total = 0i64;
    let mut found = false;

    // Parse hours component
    if let Some(h_pos) = s.find('h') {
        let hours: i64 = s[..h_pos]
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid hours in duration: '{trimmed}'"))?;
        total += hours * 60;
        found = true;

        // Parse optional minutes after 'h': accept both "XhYm" and "XhY"
        let after_h = &s[h_pos + 1..];
        if !after_h.is_empty() {
            let mins_str = after_h.trim_end_matches('m');
            let mins: i64 = mins_str
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid minutes in duration: '{trimmed}'"))?;
            total += mins;
        }
    } else if let Some(m_pos) = s.find('m') {
        // Minutes-only: e.g., "45m"
        let mins: i64 = s[..m_pos]
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid minutes in duration: '{trimmed}'"))?;
        total += mins;
        found = true;
    }

    if !found || total <= 0 {
        bail!(
            "Invalid duration: '{trimmed}'. Use Xh, Ym, XhYm, or minutes (e.g., 1h30m, 45m, 2h, 90)"
        );
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_duration_consistency_check() {
        assert_eq!(format_duration(90), "1h 30m");
        assert_eq!(format_duration(45), "45m");
        assert_eq!(format_duration(120), "2h");
        assert_eq!(format_duration(65), "1h 5m");
        assert_eq!(format_duration(0), "0m");
    }

    #[test]
    fn format_hours_and_minutes() {
        assert_eq!(format_duration(90), "1h 30m");
    }

    #[test]
    fn format_hours_only() {
        assert_eq!(format_duration(120), "2h");
    }

    #[test]
    fn format_minutes_only() {
        assert_eq!(format_duration(45), "45m");
    }

    #[test]
    fn format_zero() {
        assert_eq!(format_duration(0), "0m");
    }

    #[test]
    fn parse_plain_integer() {
        assert_eq!(parse_duration("90").unwrap(), 90);
    }

    #[test]
    fn parse_duration_rejects_spaced_input() {
        assert!(
            parse_duration("1h 30m").is_err(),
            "space-separated format not supported"
        );
    }

    #[test]
    fn parse_duration_case_insensitive() {
        assert_eq!(parse_duration("1H30M").unwrap(), 90);
    }

    #[test]
    fn parse_duration_mixed_case() {
        assert_eq!(parse_duration("1H30m").unwrap(), 90);
    }

    #[test]
    fn parse_duration_normalizes_large_minutes() {
        assert_eq!(parse_duration("1h90m").unwrap(), 150);
    }

    #[test]
    fn parse_duration_rejects_zero_hm() {
        assert!(parse_duration("0h0m").is_err());
    }

    #[test]
    fn parse_duration_trailing_m_optional() {
        assert_eq!(parse_duration("1h30").unwrap(), 90);
    }

    #[test]
    fn parse_hours_only() {
        assert_eq!(parse_duration("2h").unwrap(), 120);
    }

    #[test]
    fn parse_minutes_only() {
        assert_eq!(parse_duration("45m").unwrap(), 45);
    }

    #[test]
    fn parse_rejects_zero() {
        assert!(parse_duration("0").is_err());
    }

    #[test]
    fn parse_rejects_negative() {
        assert!(parse_duration("-5").is_err());
    }

    #[test]
    fn parse_rejects_garbage() {
        assert!(parse_duration("abc").is_err());
    }
}
