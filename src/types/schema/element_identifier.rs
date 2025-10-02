use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Trait for element identifiers (node labels, relationship types).
pub trait ElementIdentifier: Clone + Eq + Hash + Send + Sync {
    fn name(&self) -> &str;
    fn equals(&self, other: &Self) -> bool;
}

/// Node label identifier.
#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct NodeLabel(pub String);

impl NodeLabel {
    pub fn new(name: impl Into<String>) -> Self {
        NodeLabel(name.into())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn equals(&self, other: &NodeLabel) -> bool {
        self.0 == other.0
    }

    /// Special label representing all nodes.
    pub const ALL_NODES: &'static str = "*";

    pub fn all_nodes() -> Self {
        NodeLabel(Self::ALL_NODES.to_string())
    }

    pub fn is_all_nodes(&self) -> bool {
        self.0 == Self::ALL_NODES
    }
}

impl PartialEq for NodeLabel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for NodeLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl ElementIdentifier for NodeLabel {
    fn name(&self) -> &str {
        &self.0
    }

    fn equals(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Display for NodeLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Relationship type identifier.
#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct RelationshipType(pub String);

impl RelationshipType {
    pub fn new(name: impl Into<String>) -> Self {
        RelationshipType(name.into())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn equals(&self, other: &RelationshipType) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for RelationshipType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for RelationshipType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl ElementIdentifier for RelationshipType {
    fn name(&self) -> &str {
        &self.0
    }

    fn equals(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_label() {
        let label1 = NodeLabel::new("Person");
        let label2 = NodeLabel::new("Person");
        let label3 = NodeLabel::new("Company");

        assert_eq!(label1, label2);
        assert_ne!(label1, label3);
        assert_eq!(label1.name(), "Person");
        assert!(label1.equals(&label2));
    }

    #[test]
    fn test_relationship_type() {
        let rel1 = RelationshipType::new("KNOWS");
        let rel2 = RelationshipType::new("KNOWS");
        let rel3 = RelationshipType::new("LIKES");

        assert_eq!(rel1, rel2);
        assert_ne!(rel1, rel3);
        assert_eq!(rel1.name(), "KNOWS");
    }

    #[test]
    fn test_all_nodes_label() {
        let all = NodeLabel::all_nodes();
        assert!(all.is_all_nodes());
        assert_eq!(all.name(), NodeLabel::ALL_NODES);
    }
}
