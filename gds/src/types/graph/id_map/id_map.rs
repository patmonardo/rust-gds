use std::collections::HashSet;

use crate::types::{concurrency::Concurrency, schema::NodeLabel};

use super::{
    batch_node_iterable::BatchNodeIterable, filtered_id_map::FilteredIdMap,
    node_iterator::NodeIterator, partial_id_map::PartialIdMap, MappedNodeId, OriginalNodeId,
};

/// Lower bound of mapped identifiers.
pub const START_NODE_ID: MappedNodeId = 0;
/// Sentinel value mirroring the TypeScript `IdMap.NOT_FOUND` constant.
pub const NOT_FOUND: i64 = -1;
/// Type identifier used when an implementation does not have a dedicated type string.
pub const NO_TYPE: &str = "unsupported";

/// Trait alias that mirrors the TypeScript `IdMap.NodeLabelConsumer` interface.
pub trait NodeLabelConsumer {
    fn accept(&mut self, node_label: &NodeLabel) -> bool;
}

impl<F> NodeLabelConsumer for F
where
    F: FnMut(&NodeLabel) -> bool,
{
    fn accept(&mut self, node_label: &NodeLabel) -> bool {
        self(node_label)
    }
}

/// Complete bidirectional mapping between original node identifiers and the compact mapped
/// identifier space used inside the analytics engine.
pub trait IdMap: PartialIdMap + NodeIterator + BatchNodeIterable {
    /// Returns a stable identifier describing the concrete implementation.
    fn type_id(&self) -> &str {
        NO_TYPE
    }

    /// Safe variant of [`PartialIdMap::to_mapped_node_id`]. The default implementation simply
    /// delegates to [`PartialIdMap`], but concrete implementations may apply bounds checking or
    /// validation before returning the mapped identifier.
    fn safe_to_mapped_node_id(&self, original_node_id: OriginalNodeId) -> Option<MappedNodeId> {
        self.to_mapped_node_id(original_node_id)
    }

    /// Convenience helper exposing the sentinel-based variant used by the Java/TypeScript APIs.
    fn safe_to_mapped_or_not_found(&self, original_node_id: OriginalNodeId) -> i64 {
        self.safe_to_mapped_node_id(original_node_id)
            .unwrap_or(NOT_FOUND)
    }

    /// Maps a compact mapped identifier back to the original identifier (usually the database ID).
    fn to_original_node_id(&self, mapped_node_id: MappedNodeId) -> Option<OriginalNodeId>;

    /// Maps a mapped identifier from a filtered mapping back into the root mapped space.
    fn to_root_node_id(&self, mapped_node_id: MappedNodeId) -> Option<MappedNodeId> {
        Some(mapped_node_id)
    }

    /// Returns `true` when the original identifier is present in the mapping.
    fn contains_original_id(&self, original_node_id: OriginalNodeId) -> bool {
        self.to_mapped_node_id(original_node_id).is_some()
    }

    /// Number of mapped nodes in this mapping.
    fn node_count(&self) -> usize;

    /// Number of mapped nodes that carry the provided label.
    fn node_count_for_label(&self, node_label: &NodeLabel) -> usize;

    /// Highest original node identifier contained in this mapping, if known.
    fn highest_original_id(&self) -> Option<OriginalNodeId>;

    /// Returns the set of labels assigned to the given mapped node identifier.
    fn node_labels(&self, mapped_node_id: MappedNodeId) -> HashSet<NodeLabel>;

    /// Iterates over the labels for the provided node identifier.
    fn for_each_node_label(
        &self,
        mapped_node_id: MappedNodeId,
        consumer: &mut dyn NodeLabelConsumer,
    );

    /// Returns the set of all available node labels within the mapping.
    fn available_node_labels(&self) -> HashSet<NodeLabel>;

    /// Checks if a node carries the provided label.
    fn has_label(&self, mapped_node_id: MappedNodeId, label: &NodeLabel) -> bool;

    /// Registers a new label with the mapping without assigning it to any nodes yet.
    fn add_node_label(&mut self, node_label: NodeLabel);

    /// Associates the given node with the provided label.
    fn add_node_id_to_label(&mut self, node_id: MappedNodeId, node_label: NodeLabel);

    /// Returns the root mapping for nested filtered mappings. For root mappings this should
    /// return `self`.
    fn root_id_map(&self) -> &dyn IdMap;

    /// Creates a filtered view of the mapping limited to the given labels.
    fn with_filtered_labels(
        &self,
        node_labels: &HashSet<NodeLabel>,
        concurrency: Concurrency,
    ) -> Option<Box<dyn FilteredIdMap>>;
}
