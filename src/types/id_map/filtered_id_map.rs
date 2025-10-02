use super::{IdMap, MappedNodeId};

/// Extension of [`IdMap`] that adds support for nested/filtered mappings.
pub trait FilteredIdMap: IdMap {
    /// Maps a root mapped node identifier to the filtered identifier space.
    fn to_filtered_node_id(&self, root_node_id: MappedNodeId) -> Option<MappedNodeId>;

    /// Checks if the given root mapped identifier is present in the filtered mapping.
    fn contains_root_node_id(&self, root_node_id: MappedNodeId) -> bool;
}
