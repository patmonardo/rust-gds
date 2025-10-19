/// Orientation of relationships in a graph.
///
/// Defines how relationships are interpreted during graph construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Orientation {
    /// Use the natural direction of the relationship (source → target).
    #[default]
    Natural,
    /// Use the reverse direction of the relationship (target → source).
    Reverse,
    /// Treat the relationship as undirected (bidirectional).
    Undirected,
}

impl Orientation {
    /// Parses an orientation from a string (case-insensitive).
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "NATURAL" => Some(Orientation::Natural),
            "REVERSE" => Some(Orientation::Reverse),
            "UNDIRECTED" => Some(Orientation::Undirected),
            _ => None,
        }
    }

    /// Returns the string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Orientation::Natural => "NATURAL",
            Orientation::Reverse => "REVERSE",
            Orientation::Undirected => "UNDIRECTED",
        }
    }

    /// Returns the inverse orientation.
    ///
    /// NATURAL ↔ REVERSE, UNDIRECTED → UNDIRECTED
    pub fn inverse(&self) -> Self {
        match self {
            Orientation::Natural => Orientation::Reverse,
            Orientation::Reverse => Orientation::Natural,
            Orientation::Undirected => Orientation::Undirected,
        }
    }

    /// Checks if this orientation is undirected.
    pub fn is_undirected(&self) -> bool {
        matches!(self, Orientation::Undirected)
    }

    /// Checks if this orientation is directed (NATURAL or REVERSE).
    pub fn is_directed(&self) -> bool {
        !self.is_undirected()
    }
}

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Orientation::parse("NATURAL"), Some(Orientation::Natural));
        assert_eq!(Orientation::parse("natural"), Some(Orientation::Natural));
        assert_eq!(Orientation::parse("REVERSE"), Some(Orientation::Reverse));
        assert_eq!(
            Orientation::parse("UNDIRECTED"),
            Some(Orientation::Undirected)
        );
        assert_eq!(Orientation::parse("invalid"), None);
    }

    #[test]
    fn test_inverse() {
        assert_eq!(Orientation::Natural.inverse(), Orientation::Reverse);
        assert_eq!(Orientation::Reverse.inverse(), Orientation::Natural);
        assert_eq!(Orientation::Undirected.inverse(), Orientation::Undirected);
    }

    #[test]
    fn test_is_undirected() {
        assert!(!Orientation::Natural.is_undirected());
        assert!(!Orientation::Reverse.is_undirected());
        assert!(Orientation::Undirected.is_undirected());
    }
}
