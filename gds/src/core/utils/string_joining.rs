//! String joining utilities
//!
//! Provides helpers for joining collections into delimited strings.

/// Join items with commas.
///
/// # Examples
/// ```
/// use gds::util::string_joining::join;
/// let items = vec!["a", "b", "c"];
/// assert_eq!(join(&items), "a, b, c");
/// ```
pub fn join<T: ToString>(items: &[T]) -> String {
    items
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Join items with a custom delimiter.
///
/// # Examples
/// ```
/// use gds::util::string_joining::join_with_delimiter;
/// let items = vec![1, 2, 3];
/// assert_eq!(join_with_delimiter(&items, " | "), "1 | 2 | 3");
/// ```
pub fn join_with_delimiter<T: ToString>(items: &[T], delimiter: &str) -> String {
    items
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(delimiter)
}

/// Format a list as `[a, b, c]`.
///
/// # Examples
/// ```
/// use gds::util::string_joining::format_list;
/// let items = vec!["x", "y", "z"];
/// assert_eq!(format_list(&items), "[x, y, z]");
/// ```
pub fn format_list<T: ToString>(items: &[T]) -> String {
    format!("[{}]", join(items))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join() {
        let items = vec!["a", "b", "c"];
        assert_eq!(join(&items), "a, b, c");
    }

    #[test]
    fn test_join_with_delimiter() {
        let items = vec![1, 2, 3];
        assert_eq!(join_with_delimiter(&items, " | "), "1 | 2 | 3");
    }

    #[test]
    fn test_format_list() {
        let items = vec!["x", "y", "z"];
        assert_eq!(format_list(&items), "[x, y, z]");
    }

    #[test]
    fn test_empty() {
        let items: Vec<String> = vec![];
        assert_eq!(join(&items), "");
        assert_eq!(format_list(&items), "[]");
    }
}
