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
/// Supported formats:
/// - Plain integer (e.g., "90") → 90 minutes
/// - "Xh Ym" (e.g., "1h 30m") → 90 minutes
/// - "Xh" (e.g., "2h") → 120 minutes
/// - "Ym" (e.g., "45m") → 45 minutes
pub fn parse_duration(input: &str) -> Result<i64> {
    let input = input.trim();

    // Try plain integer first
    if let Ok(mins) = input.parse::<i64>() {
        if mins <= 0 {
            bail!("Duration must be greater than 0");
        }
        return Ok(mins);
    }

    let mut total = 0i64;
    let mut found = false;

    // Parse hours component
    if let Some(h_pos) = input.find('h') {
        let hours: i64 = input[..h_pos]
            .trim()
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid hours in duration: '{input}'"))?;
        total += hours * 60;
        found = true;
    }

    // Parse minutes component
    if let Some(m_pos) = input.find('m') {
        let start = input.find('h').map(|p| p + 1).unwrap_or(0);
        let mins: i64 = input[start..m_pos]
            .trim()
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid minutes in duration: '{input}'"))?;
        total += mins;
        found = true;
    }

    if !found || total <= 0 {
        bail!(
            "Invalid duration format: '{input}'. Use minutes (e.g., '90') or 'Xh Ym' (e.g., '1h 30m')"
        );
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn parse_hours_and_minutes() {
        assert_eq!(parse_duration("1h 30m").unwrap(), 90);
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
