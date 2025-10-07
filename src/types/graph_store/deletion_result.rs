//! DeletionResult - Result information from deletion operations.

use super::GraphName;

/// Result of a graph deletion operation.
///
/// Contains information about which graph was deleted and
/// optionally what was removed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeletionResult {
    graph_name: GraphName,
    deleted_node_count: Option<usize>,
    deleted_relationship_count: Option<usize>,
}

impl DeletionResult {
    /// Creates a new DeletionResult.
    ///
    pub fn new(graph_name: GraphName) -> Self {
        Self {
            graph_name,
            deleted_node_count: None,
            deleted_relationship_count: None,
        }
    }

    /// Creates a DeletionResult with count information.
    ///
    pub fn with_counts(
        graph_name: GraphName,
        node_count: usize,
        relationship_count: usize,
    ) -> Self {
        Self {
            graph_name,
            deleted_node_count: Some(node_count),
            deleted_relationship_count: Some(relationship_count),
        }
    }

    /// Returns the graph name.
    pub fn graph_name(&self) -> &GraphName {
        &self.graph_name
    }

    /// Returns the number of deleted nodes, if available.
    pub fn deleted_node_count(&self) -> Option<usize> {
        self.deleted_node_count
    }

    /// Returns the number of deleted relationships, if available.
    pub fn deleted_relationship_count(&self) -> Option<usize> {
        self.deleted_relationship_count
    }

    /// Sets the deleted node count.
    pub fn set_deleted_node_count(&mut self, count: usize) {
        self.deleted_node_count = Some(count);
    }

    /// Sets the deleted relationship count.
    pub fn set_deleted_relationship_count(&mut self, count: usize) {
        self.deleted_relationship_count = Some(count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deletion_result_new() {
        let graph_name = GraphName::new("test-graph");
        let result = DeletionResult::new(graph_name.clone());

        assert_eq!(result.graph_name(), &graph_name);
        assert_eq!(result.deleted_node_count(), None);
        assert_eq!(result.deleted_relationship_count(), None);
    }

    #[test]
    fn test_deletion_result_with_counts() {
        let graph_name = GraphName::new("test-graph");
        let result = DeletionResult::with_counts(graph_name.clone(), 100, 200);

        assert_eq!(result.graph_name(), &graph_name);
        assert_eq!(result.deleted_node_count(), Some(100));
        assert_eq!(result.deleted_relationship_count(), Some(200));
    }

    #[test]
    fn test_set_deleted_counts() {
        let graph_name = GraphName::new("test-graph");
        let mut result = DeletionResult::new(graph_name);

        result.set_deleted_node_count(50);
        result.set_deleted_relationship_count(75);

        assert_eq!(result.deleted_node_count(), Some(50));
        assert_eq!(result.deleted_relationship_count(), Some(75));
    }
}
