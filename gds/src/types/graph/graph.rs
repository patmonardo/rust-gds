use super::{characteristics::GraphCharacteristics, degrees::Degrees};
use crate::projection::RelationshipType;
use crate::types::graph::{
	characteristics::GraphCharacteristicsBuilder,
	id_map::{FilteredIdMap, IdMap, MappedNodeId, NOT_FOUND},
};
use crate::types::properties::{
	node::traits::node_property_container::{NodePropertyContainer, NodePropertyContainerExt},
	relationship::{relationship_properties::RelationshipProperties, traits::RelationshipIterator},
};
use crate::types::schema::GraphSchema;
use std::collections::HashSet;
use std::sync::Arc;

/// Result alias used by graph operations that may fail during construction of filtered views.
pub type GraphResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Core graph interface combining topology, properties, and ID mapping access.
pub trait Graph:
	IdMap
	+ NodePropertyContainer
	+ Degrees
	+ RelationshipIterator
	+ RelationshipProperties
	+ Send
	+ Sync
{
	/// Returns the schema describing this graph instance.
	fn schema(&self) -> &GraphSchema;

	/// Returns the advertised characteristics of this graph instance.
	fn characteristics(&self) -> GraphCharacteristics;

	/// Returns true when the graph contains no nodes.
	fn is_empty(&self) -> bool {
		self.node_count() == 0
	}

	/// Returns the total number of relationships present in the graph.
	fn relationship_count(&self) -> usize;

	/// Returns `true` when parallel relationships may exist in the graph.
	fn is_multi_graph(&self) -> bool;

	/// Produces a filtered view limited to the given relationship types.
	fn relationship_type_filtered_graph(
		&self,
		relationship_types: &HashSet<RelationshipType>,
	) -> GraphResult<Arc<dyn Graph>>;

	/// Returns whether any relationship property values are present.
	fn has_relationship_property(&self) -> bool;

	/// Creates a thread-safe copy for concurrent use.
	fn concurrent_copy(&self) -> Arc<dyn Graph>;

	/// Returns the filtered node mapping used to create this graph, if one exists.
	fn as_node_filtered_graph(&self) -> Option<Arc<dyn FilteredIdMap>>;

	/// Returns the `offset`-th target of the outgoing adjacency for `source_id` if present.
	fn nth_target(&self, source_id: MappedNodeId, offset: usize) -> Option<MappedNodeId> {
        if offset >= self.degree(source_id) {
			return None;
		}

		self.stream_relationships(source_id, self.default_property_value())
			.nth(offset)
			.map(|cursor| cursor.target_id())
	}
}

/// Extension helpers that mirror convenience functions from the TypeScript API.
pub trait GraphExt: Graph {
	/// Returns true when the graph exposes the given node property key.
	fn has_node_property(&self, property_key: &str) -> bool {
		NodePropertyContainerExt::has_node_property(self, property_key)
	}

	/// Returns the characteristics whittled down by the provided builder configuration.
	fn characteristics_with(
		&self,
		configure: impl FnOnce(GraphCharacteristicsBuilder) -> GraphCharacteristicsBuilder,
	) -> GraphCharacteristics {
		let requested = configure(GraphCharacteristics::builder()).build();
		self.characteristics().intersect(requested)
	}

	/// Helper that replicates the TypeScript static `Graph.nthTarget` convenience function.
	fn nth_target_or_not_found(&self, source_id: MappedNodeId, offset: usize) -> i64 {
		self.nth_target(source_id, offset)
			.map(|id| id as i64)
			.unwrap_or(NOT_FOUND)
	}
}

impl<T: Graph + ?Sized> GraphExt for T {}
