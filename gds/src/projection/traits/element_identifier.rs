use crate::projection::{NodeLabel, RelationshipType};
use std::hash::Hash;

/// Trait for element identifiers (node labels, relationship types).
///
/// This trait provides a common interface for types that identify graph elements.
/// Equality and hashing are provided by the implementing types' PartialEq/Eq/Hash impls.
pub trait ElementIdentifier: Clone + Eq + Hash + Send + Sync {
    /// Returns the name of this element identifier.
    fn name(&self) -> &str;
}

impl ElementIdentifier for NodeLabel {
    fn name(&self) -> &str {
        self.name()
    }
}

impl ElementIdentifier for RelationshipType {
    fn name(&self) -> &str {
        self.name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_label_as_identifier() {
        let label = NodeLabel::of("Person");
        assert_eq!(label.name(), "Person");

        let all = NodeLabel::all_nodes();
        assert_eq!(all.name(), "__ALL__");
    }

    #[test]
    fn test_relationship_type_as_identifier() {
        let rel_type = RelationshipType::of("KNOWS");
        assert_eq!(rel_type.name(), "KNOWS");
    }
}
