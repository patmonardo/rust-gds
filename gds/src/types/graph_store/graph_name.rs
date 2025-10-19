//! GraphName - Immutable representation of a graph name.

use std::fmt;

/// Immutable representation of a graph name.
///
/// GraphName is a simple wrapper around a String that provides
/// semantic meaning and ensures consistency in graph naming.
#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct GraphName {
    value: String,
}

impl GraphName {
    /// Creates a new GraphName.
    ///
    /// # Arguments
    /// * `value` - The graph name value
    pub fn new<S: AsRef<str>>(value: S) -> Self {
        Self {
            value: value.as_ref().to_string(),
        }
    }

    /// Parses a string into a GraphName.
    ///
    /// This is an alias for `new()` to match the TypeScript API.
    pub fn parse<S: AsRef<str>>(graph_name: S) -> Self {
        Self::new(graph_name)
    }

    /// Returns the graph name value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the graph name value (alias for `value()`).
    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for GraphName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<String> for GraphName {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for GraphName {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl AsRef<str> for GraphName {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_name_creation() {
        let name = GraphName::new("test-graph");
        assert_eq!(name.value(), "test-graph");
        assert_eq!(name.get_value(), "test-graph");
    }

    #[test]
    fn test_graph_name_parse() {
        let name = GraphName::parse("parsed-graph");
        assert_eq!(name.value(), "parsed-graph");
    }

    #[test]
    fn test_graph_name_display() {
        let name = GraphName::new("my-graph");
        assert_eq!(format!("{}", name), "my-graph");
    }

    #[test]
    fn test_graph_name_from_string() {
        let name: GraphName = "test".to_string().into();
        assert_eq!(name.value(), "test");
    }

    #[test]
    fn test_graph_name_from_str() {
        let name: GraphName = "test".into();
        assert_eq!(name.value(), "test");
    }

    #[test]
    fn test_graph_name_equality() {
        let name1 = GraphName::new("graph");
        let name2 = GraphName::new("graph");
        let name3 = GraphName::new("other");

        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
    }

    #[test]
    fn test_graph_name_ordering() {
        let name1 = GraphName::new("a");
        let name2 = GraphName::new("b");

        assert!(name1 < name2);
    }
}
