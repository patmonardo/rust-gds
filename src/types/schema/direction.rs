use serde::{Deserialize, Serialize};

/// Represents the direction of relationships in a graph.
/// Mirrors the TypeScript Direction enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    /// Relationships have a specific direction from source to target.
    Directed,
    /// Relationships have no specific direction, can be traversed both ways.
    Undirected,
}

impl Direction {
    /// Checks if this direction is undirected.
    pub fn is_undirected(&self) -> bool {
        matches!(self, Direction::Undirected)
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Directed => write!(f, "DIRECTED"),
            Direction::Undirected => write!(f, "UNDIRECTED"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_undirected() {
        assert!(!Direction::Directed.is_undirected());
        assert!(Direction::Undirected.is_undirected());
    }
}
