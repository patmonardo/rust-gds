//! Canonical Partitioning strategy enum for the core partition utilities.
use core::fmt;

/// Partitioning strategies used by Pregel and other parallel algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Partitioning {
    Range,
    Degree,
    Auto,
}

impl Partitioning {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "RANGE" => Some(Partitioning::Range),
            "DEGREE" => Some(Partitioning::Degree),
            "AUTO" => Some(Partitioning::Auto),
            _ => None,
        }
    }

    pub fn to_string_upper(&self) -> &'static str {
        match self {
            Partitioning::Range => "RANGE",
            Partitioning::Degree => "DEGREE",
            Partitioning::Auto => "AUTO",
        }
    }
}

impl fmt::Display for Partitioning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_upper())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_display_roundtrip() {
        assert_eq!(Partitioning::parse("RANGE"), Some(Partitioning::Range));
        assert_eq!(Partitioning::parse("degree"), Some(Partitioning::Degree));
        assert_eq!(Partitioning::parse("auto"), Some(Partitioning::Auto));
        assert_eq!(Partitioning::Range.to_string(), "RANGE");
    }
}
