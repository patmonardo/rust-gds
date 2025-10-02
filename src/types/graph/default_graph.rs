use super::{Graph, GraphCharacteristics, GraphResult, RelationshipTopology};
use crate::projection::RelationshipType;
use crate::types::graph::characteristics::GraphCharacteristicsBuilder;
use crate::types::id_map::{
    BatchNodeIterable, FilteredIdMap, IdMap, MappedNodeId, NodeConsumer, NodeIdBatch,
    NodeIdIterator, NodeIterator, OriginalNodeId, PartialIdMap, SimpleIdMap,
};
use crate::types::properties::node::{NodePropertyContainer, NodePropertyValues};
use crate::types::properties::relationship::{
    PropertyValue, RelationshipConsumer, RelationshipCursorBox, RelationshipIterator,
    RelationshipPredicate, RelationshipProperties, RelationshipWithPropertyConsumer,
};
use crate::types::schema::{GraphSchema, NodeLabel, RelationshipType as SchemaRelationshipType};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Default in-memory graph implementation backed by [`SimpleIdMap`] and [`RelationshipTopology`].
#[derive(Debug, Clone)]
pub struct DefaultGraph {
    schema: Arc<GraphSchema>,
    id_map: Arc<SimpleIdMap>,
    characteristics: GraphCharacteristics,
    topologies: HashMap<RelationshipType, Arc<RelationshipTopology>>,
    ordered_types: Vec<RelationshipType>,
    inverse_indexed_types: HashSet<RelationshipType>,
    relationship_count: usize,
    has_parallel_edges: bool,
    node_properties: HashMap<String, Arc<dyn NodePropertyValues>>,
    has_relationship_properties: bool,
}

impl DefaultGraph {
    /// Creates a new graph instance from the provided components.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema: Arc<GraphSchema>,
        id_map: Arc<SimpleIdMap>,
        characteristics: GraphCharacteristics,
        topologies: HashMap<RelationshipType, Arc<RelationshipTopology>>,
        ordered_types: Vec<RelationshipType>,
        inverse_indexed_types: HashSet<RelationshipType>,
        relationship_count: usize,
        has_parallel_edges: bool,
        node_properties: HashMap<String, Arc<dyn NodePropertyValues>>,
        has_relationship_properties: bool,
    ) -> Self {
        Self {
            schema,
            id_map,
            characteristics,
            topologies,
            ordered_types,
            inverse_indexed_types,
            relationship_count,
            has_parallel_edges,
            node_properties,
            has_relationship_properties,
        }
    }

    /// Returns the topology associated with the provided relationship type, if present.
    fn topology_for(
        &self,
        relationship_type: &RelationshipType,
    ) -> Option<&Arc<RelationshipTopology>> {
        self.topologies.get(relationship_type)
    }

    /// Returns the set of relationship types present in this graph.
    pub fn relationship_types(&self) -> HashSet<RelationshipType> {
        self.ordered_types.iter().cloned().collect()
    }

    fn filtered_characteristics(&self, has_inverse_indices: bool) -> GraphCharacteristics {
        let mut builder = GraphCharacteristicsBuilder::new();
        if self.characteristics.is_directed() {
            builder = builder.directed();
        }
        if self.characteristics.is_undirected() {
            builder = builder.undirected();
        }
        if has_inverse_indices {
            builder = builder.inverse_indexed();
        }
        builder.build()
    }
}

impl Graph for DefaultGraph {
    fn schema(&self) -> &GraphSchema {
        &self.schema
    }

    fn characteristics(&self) -> GraphCharacteristics {
        self.characteristics
    }

    fn relationship_count(&self) -> usize {
        self.relationship_count
    }

    fn is_multi_graph(&self) -> bool {
        self.has_parallel_edges
    }

    fn relationship_type_filtered_graph(
        &self,
        relationship_types: &HashSet<RelationshipType>,
    ) -> GraphResult<Arc<dyn Graph>> {
        if relationship_types.is_empty() {
            return Ok(Arc::new(self.clone()));
        }

        let mut filtered_topologies: HashMap<RelationshipType, Arc<RelationshipTopology>> =
            HashMap::new();
        let mut ordered_types = Vec::new();
        let mut inverse_indexed_types = HashSet::new();
        let mut relationship_count = 0usize;
        let mut has_parallel_edges = false;

        for relationship_type in &self.ordered_types {
            if !relationship_types.contains(relationship_type) {
                continue;
            }

            if let Some(topology) = self.topology_for(relationship_type) {
                ordered_types.push(relationship_type.clone());
                filtered_topologies.insert(relationship_type.clone(), Arc::clone(topology));
                relationship_count += topology.relationship_count();
                if topology.is_inverse_indexed() {
                    inverse_indexed_types.insert(relationship_type.clone());
                }
                if topology.has_parallel_edges() {
                    has_parallel_edges = true;
                }
            }
        }

        let has_inverse_indices = !ordered_types.is_empty()
            && ordered_types
                .iter()
                .all(|rel_type| inverse_indexed_types.contains(rel_type));

        let filtered_characteristics = self.filtered_characteristics(has_inverse_indices);

        let filtered_schema = if relationship_types.is_empty() {
            Arc::clone(&self.schema)
        } else {
            let schema_types: HashSet<SchemaRelationshipType> = relationship_types
                .iter()
                .map(|rel_type| SchemaRelationshipType::new(rel_type.name()))
                .collect();
            Arc::new(self.schema.filter_relationship_types(&schema_types))
        };

        let filtered_graph = DefaultGraph::new(
            filtered_schema,
            Arc::clone(&self.id_map),
            filtered_characteristics,
            filtered_topologies,
            ordered_types,
            inverse_indexed_types,
            relationship_count,
            has_parallel_edges,
            self.node_properties.clone(),
            self.has_relationship_properties,
        );

        Ok(Arc::new(filtered_graph))
    }

    fn has_relationship_property(&self) -> bool {
        self.has_relationship_properties
    }

    fn concurrent_copy(&self) -> Arc<dyn Graph> {
        Arc::new(self.clone())
    }

    fn as_node_filtered_graph(&self) -> Option<Arc<dyn FilteredIdMap>> {
        None
    }
}

impl PartialIdMap for DefaultGraph {
    fn to_mapped_node_id(&self, original_node_id: OriginalNodeId) -> Option<MappedNodeId> {
        self.id_map.to_mapped_node_id(original_node_id)
    }

    fn root_node_count(&self) -> Option<usize> {
        self.id_map.root_node_count()
    }
}

impl NodeIterator for DefaultGraph {
    fn for_each_node(&self, consumer: &mut dyn NodeConsumer) {
        self.id_map.for_each_node(consumer)
    }

    fn iter(&self) -> NodeIdIterator<'_> {
        self.id_map.iter()
    }

    fn iter_with_labels<'a>(&'a self, labels: &'a HashSet<NodeLabel>) -> NodeIdIterator<'a> {
        self.id_map.iter_with_labels(labels)
    }
}

impl BatchNodeIterable for DefaultGraph {
    fn batch_iterables(&self, batch_size: usize) -> Vec<NodeIdBatch> {
        self.id_map.batch_iterables(batch_size)
    }
}

impl IdMap for DefaultGraph {
    fn type_id(&self) -> &str {
        self.id_map.type_id()
    }

    fn safe_to_mapped_node_id(&self, original_node_id: OriginalNodeId) -> Option<MappedNodeId> {
        self.id_map.safe_to_mapped_node_id(original_node_id)
    }

    fn to_original_node_id(&self, mapped_node_id: MappedNodeId) -> Option<OriginalNodeId> {
        self.id_map.to_original_node_id(mapped_node_id)
    }

    fn to_root_node_id(&self, mapped_node_id: MappedNodeId) -> Option<MappedNodeId> {
        self.id_map.to_root_node_id(mapped_node_id)
    }

    fn node_count(&self) -> usize {
        self.id_map.node_count()
    }

    fn node_count_for_label(&self, node_label: &NodeLabel) -> usize {
        self.id_map.node_count_for_label(node_label)
    }

    fn highest_original_id(&self) -> Option<OriginalNodeId> {
        self.id_map.highest_original_id()
    }

    fn node_labels(&self, mapped_node_id: MappedNodeId) -> HashSet<NodeLabel> {
        self.id_map.node_labels(mapped_node_id)
    }

    fn for_each_node_label(
        &self,
        mapped_node_id: MappedNodeId,
        consumer: &mut dyn crate::types::id_map::NodeLabelConsumer,
    ) {
        self.id_map.for_each_node_label(mapped_node_id, consumer)
    }

    fn available_node_labels(&self) -> HashSet<NodeLabel> {
        self.id_map.available_node_labels()
    }

    fn has_label(&self, mapped_node_id: MappedNodeId, label: &NodeLabel) -> bool {
        self.id_map.has_label(mapped_node_id, label)
    }

    fn add_node_label(&mut self, node_label: NodeLabel) {
        Arc::make_mut(&mut self.id_map).add_node_label(node_label);
    }

    fn add_node_id_to_label(&mut self, node_id: MappedNodeId, node_label: NodeLabel) {
        Arc::make_mut(&mut self.id_map).add_node_id_to_label(node_id, node_label);
    }

    fn root_id_map(&self) -> &dyn IdMap {
        self.id_map.root_id_map()
    }

    fn with_filtered_labels(
        &self,
        node_labels: &HashSet<NodeLabel>,
        concurrency: crate::types::concurrency::Concurrency,
    ) -> Option<Box<dyn FilteredIdMap>> {
        self.id_map.with_filtered_labels(node_labels, concurrency)
    }
}

impl crate::types::graph::degrees::Degrees for DefaultGraph {
    fn degree(&self, node_id: MappedNodeId) -> usize {
        self.ordered_types
            .iter()
            .filter_map(|rel_type| self.topology_for(rel_type))
            .filter_map(|topology| topology.outgoing(node_id))
            .map(|neighbors| neighbors.len())
            .sum()
    }

    fn degree_inverse(&self, node_id: MappedNodeId) -> Option<usize> {
        if self.inverse_indexed_types.is_empty() {
            return None;
        }

        let total: usize = self
            .ordered_types
            .iter()
            .filter(|rel_type| self.inverse_indexed_types.contains(*rel_type))
            .filter_map(|rel_type| self.topology_for(rel_type))
            .filter_map(|topology| topology.incoming(node_id))
            .map(|neighbors| neighbors.len())
            .sum();
        Some(total)
    }

    fn degree_without_parallel_relationships(&self, node_id: MappedNodeId) -> usize {
        let mut unique = HashSet::new();
        for rel_type in &self.ordered_types {
            if let Some(topology) = self.topology_for(rel_type) {
                if let Some(neighbors) = topology.outgoing(node_id) {
                    unique.extend(neighbors.iter().copied());
                }
            }
        }
        unique.len()
    }
}

impl RelationshipPredicate for DefaultGraph {
    fn exists(&self, source_id: MappedNodeId, target_id: MappedNodeId) -> bool {
        self.ordered_types.iter().any(|rel_type| {
            self.topology_for(rel_type)
                .and_then(|topology| topology.outgoing(source_id))
                .map(|neighbors| neighbors.iter().any(|neighbor| *neighbor == target_id))
                .unwrap_or(false)
        })
    }
}

impl RelationshipIterator for DefaultGraph {
    fn for_each_relationship(
        &self,
        node_id: MappedNodeId,
        consumer: &mut dyn RelationshipConsumer,
    ) {
        for rel_type in &self.ordered_types {
            if let Some(topology) = self.topology_for(rel_type) {
                if let Some(neighbors) = topology.outgoing(node_id) {
                    for &target in neighbors {
                        if !consumer.accept(node_id, target) {
                            return;
                        }
                    }
                }
            }
        }
    }

    fn for_each_relationship_with_properties(
        &self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
        consumer: &mut dyn RelationshipWithPropertyConsumer,
    ) {
        for rel_type in &self.ordered_types {
            if let Some(topology) = self.topology_for(rel_type) {
                if let Some(neighbors) = topology.outgoing(node_id) {
                    for &target in neighbors {
                        if !consumer.accept(node_id, target, fallback_value) {
                            return;
                        }
                    }
                }
            }
        }
    }

    fn for_each_inverse_relationship(
        &self,
        node_id: MappedNodeId,
        consumer: &mut dyn RelationshipConsumer,
    ) {
        for rel_type in &self.ordered_types {
            if let Some(topology) = self.topology_for(rel_type) {
                if let Some(incoming) = topology.incoming(node_id) {
                    for &source in incoming {
                        if !consumer.accept(source, node_id) {
                            return;
                        }
                    }
                }
            }
        }
    }

    fn for_each_inverse_relationship_with_properties(
        &self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
        consumer: &mut dyn RelationshipWithPropertyConsumer,
    ) {
        for rel_type in &self.ordered_types {
            if let Some(topology) = self.topology_for(rel_type) {
                if let Some(incoming) = topology.incoming(node_id) {
                    for &source in incoming {
                        if !consumer.accept(source, node_id, fallback_value) {
                            return;
                        }
                    }
                }
            }
        }
    }

    fn stream_relationships<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
    ) -> Box<dyn Iterator<Item = RelationshipCursorBox> + Send + 'a> {
        let mut cursors: Vec<RelationshipCursorBox> = Vec::new();
        for rel_type in &self.ordered_types {
            if let Some(topology) = self.topology_for(rel_type) {
                if let Some(neighbors) = topology.outgoing(node_id) {
                    cursors.extend(neighbors.iter().map(|&target| {
                        Box::new(StaticRelationshipCursor {
                            source: node_id,
                            target,
                            property: fallback_value,
                        }) as RelationshipCursorBox
                    }));
                }
            }
        }
        Box::new(cursors.into_iter())
    }

    fn concurrent_copy(&self) -> Box<dyn RelationshipIterator> {
        Box::new(self.clone())
    }
}

impl RelationshipProperties for DefaultGraph {
    fn default_property_value(&self) -> PropertyValue {
        0.0
    }

    fn relationship_property(
        &self,
        _source_id: MappedNodeId,
        _target_id: MappedNodeId,
        fallback_value: PropertyValue,
    ) -> PropertyValue {
        fallback_value
    }
}

impl NodePropertyContainer for DefaultGraph {
    fn node_properties(&self, property_key: &str) -> Option<Arc<dyn NodePropertyValues>> {
        self.node_properties.get(property_key).cloned()
    }

    fn available_node_properties(&self) -> HashSet<String> {
        self.node_properties.keys().cloned().collect()
    }
}

#[derive(Debug, Clone)]
struct StaticRelationshipCursor {
    source: MappedNodeId,
    target: MappedNodeId,
    property: PropertyValue,
}

impl crate::types::properties::relationship::RelationshipCursor for StaticRelationshipCursor {
    fn source_id(&self) -> MappedNodeId {
        self.source
    }

    fn target_id(&self) -> MappedNodeId {
        self.target
    }

    fn property(&self) -> PropertyValue {
        self.property
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph::degrees::Degrees;
    use crate::types::graph::Graph;

    fn build_graph() -> DefaultGraph {
        let schema = Arc::new(GraphSchema::empty());
        let id_map = Arc::new(SimpleIdMap::from_original_ids([0, 1, 2]));

        let topology = RelationshipTopology::new(vec![vec![1, 2], vec![2], vec![]], None);
        let relationship_count = topology.relationship_count();
        let has_parallel_edges = topology.has_parallel_edges();

        let mut topologies = HashMap::new();
        let rel_type = RelationshipType::of("KNOWS");
        topologies.insert(rel_type.clone(), Arc::new(topology));

        DefaultGraph::new(
            schema,
            id_map,
            GraphCharacteristicsBuilder::new().directed().build(),
            topologies,
            vec![rel_type],
            HashSet::new(),
            relationship_count,
            has_parallel_edges,
            HashMap::new(),
            false,
        )
    }

    #[test]
    fn computes_degrees_and_relationship_counts() {
        let graph = build_graph();
        assert_eq!(graph.relationship_count(), 3);
        assert_eq!(graph.degree(0), 2);
        assert_eq!(graph.degree(1), 1);
        assert_eq!(graph.degree_without_parallel_relationships(0), 2);
        assert!(graph.exists(0, 2));
        assert!(!graph.exists(2, 0));
        assert_eq!(graph.nth_target(0, 1), Some(2));
    }

    #[test]
    fn filters_relationship_types() {
        let graph = build_graph();
        let rel_type = RelationshipType::of("KNOWS");

        let mut filter = HashSet::new();
        filter.insert(rel_type.clone());
        let filtered = graph.relationship_type_filtered_graph(&filter).unwrap();
        assert_eq!(filtered.relationship_count(), 3);

        let empty_filter = HashSet::new();
        let no_filter = graph
            .relationship_type_filtered_graph(&empty_filter)
            .unwrap();
        assert_eq!(no_filter.relationship_count(), graph.relationship_count());
    }
}
