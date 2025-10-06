use super::{Graph, GraphCharacteristics, GraphResult, RelationshipTopology};
use crate::projection::RelationshipType;
use crate::types::graph::characteristics::GraphCharacteristicsBuilder;
use crate::types::graph::id_map::{
    BatchNodeIterable, FilteredIdMap, IdMap, MappedNodeId, NodeConsumer, NodeIdBatch,
    NodeIdIterator, NodeIterator, OriginalNodeId, PartialIdMap, SimpleIdMap,
};
use crate::types::properties::node::{NodePropertyContainer, NodePropertyValues};
use crate::types::properties::property::Property;
use crate::types::properties::relationship::{
    relationship_properties::RelationshipProperties,
    relationship_property_values::RelationshipPropertyValues, DefaultModifiableRelationshipCursor,
    DefaultRelationshipCursor, DefaultRelationshipPropertyStore, ModifiableRelationshipCursor,
    PropertyValue, RelationshipCursor, RelationshipCursorBox, RelationshipIterator,
    RelationshipPredicate, RelationshipStream,
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
    relationship_properties: HashMap<RelationshipType, DefaultRelationshipPropertyStore>,
    selected_relationship_properties: HashMap<RelationshipType, SelectedRelationshipProperty>,
    relationship_property_selectors: HashMap<RelationshipType, String>,
    topology_offsets: HashMap<RelationshipType, Arc<Vec<usize>>>,
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
        relationship_properties: HashMap<RelationshipType, DefaultRelationshipPropertyStore>,
        relationship_property_selectors: HashMap<RelationshipType, String>,
    ) -> Self {
        let topology_offsets = compute_topology_offsets(&topologies);
        let (selected_relationship_properties, effective_selectors) =
            build_selected_relationship_properties(
                &ordered_types,
                &relationship_properties,
                &relationship_property_selectors,
            );
        let has_relationship_properties = !selected_relationship_properties.is_empty();

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
            relationship_properties,
            selected_relationship_properties,
            relationship_property_selectors: effective_selectors,
            topology_offsets,
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

    fn selected_property(
        &self,
        relationship_type: &RelationshipType,
    ) -> Option<&SelectedRelationshipProperty> {
        self.selected_relationship_properties.get(relationship_type)
    }

    fn property_index(
        &self,
        relationship_type: &RelationshipType,
        source_id: MappedNodeId,
        neighbor_index: usize,
    ) -> Option<u64> {
        let offsets = self.topology_offsets.get(relationship_type)?;
        let source_index = source_id as usize;
        if source_index + 1 >= offsets.len() {
            return None;
        }
        let start = offsets[source_index];
        let end = offsets[source_index + 1];
        let degree = end.saturating_sub(start);
        if neighbor_index >= degree {
            return None;
        }
        Some((start + neighbor_index) as u64)
    }

    fn relationship_property_value_for(
        &self,
        relationship_type: &RelationshipType,
        source_id: MappedNodeId,
        neighbor_index: usize,
        fallback_value: PropertyValue,
    ) -> PropertyValue {
        let selected = match self.selected_property(relationship_type) {
            Some(selected) => selected,
            None => return fallback_value,
        };

        let index = match self.property_index(relationship_type, source_id, neighbor_index) {
            Some(index) => index,
            None => return fallback_value,
        };

        selected.value_at_or(index, fallback_value)
    }

    fn traverse_outgoing_relationships<F>(
        &self,
        node_id: MappedNodeId,
        mode: PropertyTraversalMode,
        mut callback: F,
    ) -> bool
    where
        F: FnMut(&dyn RelationshipCursor) -> bool,
    {
        let fallback = mode.fallback();
        let mut cursor = DefaultModifiableRelationshipCursor::new(node_id, node_id, fallback);
        cursor.set_source_id(node_id);

        for relationship_type in &self.ordered_types {
            let topology = match self.topology_for(relationship_type) {
                Some(topology) => topology,
                None => continue,
            };

            let neighbors = match topology.outgoing(node_id) {
                Some(neighbors) => neighbors,
                None => continue,
            };

            for (index, &target) in neighbors.iter().enumerate() {
                cursor.set_target_id(target);

                let property_value = if mode.requires_value() {
                    self.relationship_property_value_for(
                        relationship_type,
                        node_id,
                        index,
                        fallback,
                    )
                } else {
                    fallback
                };

                cursor.set_property(property_value);

                if !callback(&cursor as &dyn RelationshipCursor) {
                    return false;
                }
            }
        }

        true
    }

    fn traverse_inverse_relationships<F>(
        &self,
        node_id: MappedNodeId,
        mode: PropertyTraversalMode,
        mut callback: F,
    ) -> bool
    where
        F: FnMut(&dyn RelationshipCursor) -> bool,
    {
        let fallback = mode.fallback();
        let mut cursor = DefaultModifiableRelationshipCursor::new(node_id, node_id, fallback);
        cursor.set_target_id(node_id);

        for relationship_type in &self.ordered_types {
            let topology = match self.topology_for(relationship_type) {
                Some(topology) => topology,
                None => continue,
            };

            let incoming = match topology.incoming(node_id) {
                Some(incoming) => incoming,
                None => continue,
            };

            for &source in incoming.iter() {
                cursor.set_source_id(source);

                let property_value = if mode.requires_value() {
                    topology
                        .outgoing(source)
                        .and_then(|neighbors| {
                            neighbors
                                .iter()
                                .position(|&target| target == node_id)
                                .map(|index| {
                                    self.relationship_property_value_for(
                                        relationship_type,
                                        source,
                                        index,
                                        fallback,
                                    )
                                })
                        })
                        .unwrap_or(fallback)
                } else {
                    fallback
                };

                cursor.set_property(property_value);

                if !callback(&cursor as &dyn RelationshipCursor) {
                    return false;
                }
            }
        }

        true
    }
}

#[derive(Debug, Clone)]
struct SelectedRelationshipProperty {
    values: Arc<dyn RelationshipPropertyValues>,
    fallback: PropertyValue,
}

impl SelectedRelationshipProperty {
    fn new(values: Arc<dyn RelationshipPropertyValues>, fallback: PropertyValue) -> Self {
        Self { values, fallback }
    }

    fn value_at_or(&self, index: u64, fallback: PropertyValue) -> PropertyValue {
        self.values.double_value(index).unwrap_or(fallback)
    }
}

#[derive(Debug, Clone, Copy)]
struct PropertyTraversalMode {
    fallback: PropertyValue,
    include_value: bool,
}

impl PropertyTraversalMode {
    fn with_value(fallback: PropertyValue) -> Self {
        Self {
            fallback,
            include_value: true,
        }
    }

    fn fallback(self) -> PropertyValue {
        self.fallback
    }

    fn requires_value(self) -> bool {
        self.include_value
    }
}

fn compute_topology_offsets(
    topologies: &HashMap<RelationshipType, Arc<RelationshipTopology>>,
) -> HashMap<RelationshipType, Arc<Vec<usize>>> {
    let mut offsets = HashMap::new();
    for (rel_type, topology) in topologies {
        let capacity = topology.node_capacity();
        let mut prefix = Vec::with_capacity(capacity + 1);
        let mut total = 0usize;
        prefix.push(total);
        for node in 0..capacity {
            let mapped_id = node as MappedNodeId;
            let degree = topology
                .outgoing(mapped_id)
                .map(|neighbors| neighbors.len())
                .unwrap_or(0);
            total += degree;
            prefix.push(total);
        }
        offsets.insert(rel_type.clone(), Arc::new(prefix));
    }
    offsets
}

fn build_selected_relationship_properties(
    ordered_types: &[RelationshipType],
    stores: &HashMap<RelationshipType, DefaultRelationshipPropertyStore>,
    selectors: &HashMap<RelationshipType, String>,
) -> (
    HashMap<RelationshipType, SelectedRelationshipProperty>,
    HashMap<RelationshipType, String>,
) {
    let mut selected = HashMap::new();
    let mut effective = HashMap::new();

    for rel_type in ordered_types {
        let store = match stores.get(rel_type) {
            Some(store) if !store.is_empty() => store,
            _ => continue,
        };

        let chosen_key = selectors
            .get(rel_type)
            .cloned()
            .or_else(|| auto_select_property_key(store));

        if let Some(key) = chosen_key {
            if let Some(property) = store.get(&key) {
                let values_arc = property.values();
                // SAFETY: By construction, RelationshipProperty stores RelationshipPropertyValues
                let rel_values = unsafe {
                    std::mem::transmute::<
                        Arc<dyn crate::types::properties::property_values::PropertyValues>,
                        Arc<dyn RelationshipPropertyValues>,
                    >(values_arc)
                };
                let selection = SelectedRelationshipProperty::new(
                    rel_values.clone(),
                    rel_values.default_value(),
                );
                selected.insert(rel_type.clone(), selection);
                effective.insert(rel_type.clone(), key);
            }
        }
    }

    (selected, effective)
}

fn auto_select_property_key(store: &DefaultRelationshipPropertyStore) -> Option<String> {
    if store.len() == 1 {
        store.relationship_properties().keys().next().cloned()
    } else {
        None
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

        let filtered_relationship_properties = ordered_types
            .iter()
            .filter_map(|rel_type| {
                self.relationship_properties
                    .get(rel_type)
                    .map(|store| (rel_type.clone(), store.clone()))
            })
            .collect::<HashMap<_, _>>();

        let filtered_selectors = ordered_types
            .iter()
            .filter_map(|rel_type| {
                self.relationship_property_selectors
                    .get(rel_type)
                    .map(|key| (rel_type.clone(), key.clone()))
            })
            .collect::<HashMap<_, _>>();

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
            filtered_relationship_properties,
            filtered_selectors,
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
        consumer: &mut dyn crate::types::graph::id_map::NodeLabelConsumer,
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
                .map(|neighbors| neighbors.contains(&target_id))
                .unwrap_or(false)
        })
    }
}

impl RelationshipIterator for DefaultGraph {
    fn stream_relationships<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
    ) -> RelationshipStream<'a> {
        let mut cursors: Vec<RelationshipCursorBox> = Vec::new();
        let _ = self.traverse_outgoing_relationships(
            node_id,
            PropertyTraversalMode::with_value(fallback_value),
            |cursor| {
                let snapshot = DefaultRelationshipCursor::new(
                    cursor.source_id(),
                    cursor.target_id(),
                    cursor.property(),
                );
                cursors.push(Box::new(snapshot) as RelationshipCursorBox);
                true
            },
        );
        Box::new(cursors.into_iter())
    }

    fn stream_inverse_relationships<'a>(
        &'a self,
        node_id: MappedNodeId,
        fallback_value: PropertyValue,
    ) -> RelationshipStream<'a> {
        let mut cursors: Vec<RelationshipCursorBox> = Vec::new();
        let _ = self.traverse_inverse_relationships(
            node_id,
            PropertyTraversalMode::with_value(fallback_value),
            |cursor| {
                let snapshot = DefaultRelationshipCursor::new(
                    cursor.source_id(),
                    cursor.target_id(),
                    cursor.property(),
                );
                cursors.push(Box::new(snapshot) as RelationshipCursorBox);
                true
            },
        );
        Box::new(cursors.into_iter())
    }

    fn concurrent_copy(&self) -> Box<dyn RelationshipIterator> {
        Box::new(self.clone())
    }
}

impl RelationshipProperties for DefaultGraph {
    fn default_property_value(&self) -> PropertyValue {
        self.selected_relationship_properties
            .values()
            .next()
            .map(|selection| selection.fallback)
            .unwrap_or(0.0)
    }

    fn relationship_property(
        &self,
        source_id: MappedNodeId,
        target_id: MappedNodeId,
        fallback_value: PropertyValue,
    ) -> PropertyValue {
        if self.selected_relationship_properties.is_empty() {
            return fallback_value;
        }

        let mut property = fallback_value;
        let mut found = false;
        let _ = self.traverse_outgoing_relationships(
            source_id,
            PropertyTraversalMode::with_value(fallback_value),
            |cursor| {
                if cursor.target_id() == target_id {
                    property = cursor.property();
                    found = true;
                    false
                } else {
                    true
                }
            },
        );

        if found {
            property
        } else {
            fallback_value
        }
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
            HashMap::new(),
            HashMap::new(),
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
