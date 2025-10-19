//! String formatting utilities
//!
//! Provides formatWithLocale-style helpers for string interpolation.
//! Keeps behavior simple and deterministic (no actual locale plumbing).

/// Replace `%s` tokens in format string with provided arguments sequentially.
/// Missing args are rendered as `<missing>`.
///
/// # Examples
/// ```
/// use gds::util::string_formatting::format_with_locale;
/// let msg = format_with_locale("Hello %s, you have %s messages", &["Alice", "5"]);
/// assert_eq!(msg, "Hello Alice, you have 5 messages");
/// ```
pub fn format_with_locale(fmt: &str, args: &[impl ToString]) -> String {
    let mut out = String::new();
    let mut parts = fmt.split("%s");

    if let Some(first) = parts.next() {
        out.push_str(first);
    }

    for (i, part) in parts.enumerate() {
        let replacement = args
            .get(i)
            .map(|a| a.to_string())
            .unwrap_or_else(|| "<missing>".to_string());
        out.push_str(&replacement);
        out.push_str(part);
    }

    out
}

/// Format a number with underscore as thousands separator.
///
/// # Examples
/// ```
/// use gds::util::string_formatting::format_number;
/// assert_eq!(format_number(1000), "1_000");
/// assert_eq!(format_number(1234567), "1_234_567");
/// ```
pub fn format_number(number: i64) -> String {
    let s = number.abs().to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push('_');
        }
        result.push(*ch);
    }

    if number < 0 {
        format!("-{}", result)
    } else {
        result
    }
}

/// Convert string to lowercase using English locale semantics.
pub fn to_lower_case_with_locale(s: &str) -> String {
    s.to_lowercase()
}

/// Convert string to uppercase using English locale semantics.
pub fn to_upper_case_with_locale(s: &str) -> String {
    s.to_uppercase()
}

/// Check if a string is empty or whitespace-only.
pub fn is_empty(s: Option<&str>) -> bool {
    s.is_none_or(|s| s.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_with_locale() {
        let msg = format_with_locale("Hello %s, you have %s messages", &["Alice", "5"]);
        assert_eq!(msg, "Hello Alice, you have 5 messages");
    }

    #[test]
    fn test_format_with_locale_missing_args() {
        let msg = format_with_locale("X %s Y %s Z", &["one"]);
        assert_eq!(msg, "X one Y <missing> Z");
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1000), "1_000");
        assert_eq!(format_number(1234567), "1_234_567");
        assert_eq!(format_number(-1000), "-1_000");
    }

    #[test]
    fn test_is_empty() {
        assert!(is_empty(None));
        assert!(is_empty(Some("")));
        assert!(is_empty(Some("   ")));
        assert!(!is_empty(Some("hello")));
    }
}
