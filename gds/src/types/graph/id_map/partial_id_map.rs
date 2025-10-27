use super::{MappedNodeId, OriginalNodeId};

/// Partial mapping between original node identifiers and mapped identifiers.
///
/// This trait mirrors the TypeScript `PartialIdMap` interface. It is used by the
/// relationship loading code paths where only a subset of the full
/// [`IdMap`](crate::types::graph::id_map::IdMap)
/// functionality is required.
pub trait PartialIdMap: Send + Sync {
    /// Maps an original node identifier into the mapped node identifier space.
    /// Implementations should return `None` when the node is unmapped.
    fn to_mapped_node_id(&self, original_node_id: OriginalNodeId) -> Option<MappedNodeId>;

    /// Number of nodes in the root mapping (before any filtering is applied).
    ///
    /// Nested/filtered mappings can use this to expose the size of their parent mapping.
    fn root_node_count(&self) -> Option<usize> {
        None
    }

    /// Convenience helper that mirrors the TypeScript `NOT_FOUND` sentinel.
    /// Most Rust call-sites should use [`PartialIdMap::to_mapped_node_id`], but the
    /// sentinel is provided for compatibility with existing logic.
    fn to_mapped_or_not_found(&self, original_node_id: OriginalNodeId) -> i64 {
        self.to_mapped_node_id(original_node_id)
            .unwrap_or(super::NOT_FOUND)
    }
}

/// Empty partial mapping used as a safe default. It never maps any identifiers.
#[derive(Debug, Clone, Copy, Default)]
pub struct EmptyPartialIdMap;

impl PartialIdMap for EmptyPartialIdMap {
    fn to_mapped_node_id(&self, _original_node_id: OriginalNodeId) -> Option<MappedNodeId> {
        None
    }

    fn root_node_count(&self) -> Option<usize> {
        None
    }
}
