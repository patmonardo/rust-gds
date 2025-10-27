use super::{
    Capabilities, DatabaseInfo, DeletionResult, GraphName, GraphStore, GraphStoreError,
    GraphStoreResult,
};
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph::{
    id_map::{IdMap, SimpleIdMap},
    DefaultGraph, Graph, GraphCharacteristics, GraphCharacteristicsBuilder, RelationshipTopology,
};
use crate::types::properties::graph::GraphPropertyValues;
use crate::types::ValueType;
use crate::config::GraphStoreConfig;

use crate::types::properties::node::NodePropertyValues;

use crate::types::properties::relationship::default_relationship_property_store::DefaultRelationshipPropertyStore;
use crate::types::properties::relationship::relationship_property::RelationshipProperty;
use crate::types::properties::relationship::RelationshipPropertyValues;
use crate::types::properties::relationship::{
    RelationshipPropertyStore, RelationshipPropertyStoreBuilder,
};

use crate::types::schema::{Direction, GraphSchema, PropertySchemaTrait};
use crate::projection::orientation::Orientation;
use crate::types::PropertyState;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// In-memory [`GraphStore`] backed by [`SimpleIdMap`] and [`RelationshipTopology`].
#[derive(Debug, Clone)]
pub struct DefaultGraphStore {
    config: Arc<GraphStoreConfig>,
    graph_name: GraphName,
    database_info: DatabaseInfo,
    schema: Arc<GraphSchema>,
    capabilities: Capabilities,
    creation_time: DateTime<Utc>,
    modification_time: DateTime<Utc>,
    id_map: Arc<SimpleIdMap>,
    relationship_topologies: HashMap<RelationshipType, Arc<RelationshipTopology>>,
    ordered_relationship_types: Vec<RelationshipType>,
    inverse_indexed_relationship_types: HashSet<RelationshipType>,
    relationship_count: usize,
    has_parallel_relationships: bool,
    graph_characteristics: GraphCharacteristics,
    graph_properties: HashMap<String, Arc<dyn GraphPropertyValues>>,
    node_properties: HashMap<String, Arc<dyn NodePropertyValues>>,
    node_properties_by_label: HashMap<String, HashSet<String>>,
    relationship_property_stores: HashMap<RelationshipType, DefaultRelationshipPropertyStore>,
    has_relationship_properties: bool,
}

impl DefaultGraphStore {
    /// Creates a new store from the provided components.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        config: GraphStoreConfig,
        graph_name: GraphName,
        database_info: DatabaseInfo,
        schema: GraphSchema,
        capabilities: Capabilities,
        id_map: SimpleIdMap,
        relationship_topologies: HashMap<RelationshipType, RelationshipTopology>,
    ) -> Self {
        let now = Utc::now();
        let config = Arc::new(config);
        let schema = Arc::new(schema);
        let id_map = Arc::new(id_map);
        let relationship_topologies = relationship_topologies
            .into_iter()
            .map(|(rel_type, topology)| (rel_type, Arc::new(topology)))
            .collect();

        let mut store = Self {
            config: Arc::clone(&config),
            graph_name,
            database_info,
            schema,
            capabilities,
            creation_time: now,
            modification_time: now,
            id_map,
            relationship_topologies,
            ordered_relationship_types: Vec::new(),
            inverse_indexed_relationship_types: HashSet::new(),
            relationship_count: 0,
            has_parallel_relationships: false,
            graph_characteristics: GraphCharacteristicsBuilder::new().build(),
            graph_properties: HashMap::new(),
            node_properties: HashMap::new(),
            node_properties_by_label: HashMap::new(),
            relationship_property_stores: HashMap::new(),
            has_relationship_properties: false,
        };

        store.rebuild_relationship_metadata();
        store.refresh_relationship_property_state();
        store
    }

    /// Builds a [`DefaultGraph`] view over the current store contents.
    /// Returns the concrete DefaultGraph type for backwards compatibility.
    pub fn graph(&self) -> Arc<DefaultGraph> {
        // Create DefaultGraph directly for backwards compatibility
        let topologies = self
            .relationship_topologies
            .iter()
            .map(|(rel_type, topology)| (rel_type.clone(), Arc::clone(topology)))
            .collect::<HashMap<_, _>>();

        Arc::new(DefaultGraph::new(
            Arc::clone(&self.config),
            Arc::clone(&self.schema),
            Arc::clone(&self.id_map),
            self.graph_characteristics,
            topologies,
            self.ordered_relationship_types.clone(),
            self.inverse_indexed_relationship_types.clone(),
            self.relationship_count,
            self.has_parallel_relationships,
            self.node_properties.clone(),
            self.relationship_property_stores.clone(),
            HashMap::new(),
        ))
    }

    fn set_modified(&mut self) {
        self.modification_time = Utc::now();
    }

    fn schema_labels(&self) -> HashSet<NodeLabel> {
        self.id_map
            .available_node_labels()
            .into_iter()
            .map(|label| NodeLabel::of(label.name()))
            .collect()
    }

    // === Property Management with Config ===

    /// Add a node property with i64 values using the store's config for backend selection.
    pub fn add_node_property_i64(
        &mut self,
        key: String,
        values: Vec<i64>,
    ) -> Result<(), GraphStoreError> {
        let node_count = self.id_map.node_count();
        
        // Use config to create CollectionsConfig
        let collections_config = self.config.node_collections_config::<i64>(node_count);
        
        // Create property using config
        use crate::types::properties::node::impls::default_node_property_values::DefaultLongNodePropertyValues;
        use crate::collections::backends::vec::VecLong;
        
        let backend = crate::collections::backends::factory::create_long_backend_from_config(&collections_config, values);
        let pv = Arc::new(DefaultLongNodePropertyValues::<VecLong>::from_collection(backend, node_count));
        
        self.node_properties.insert(key, pv);
        self.set_modified();
        Ok(())
    }

    /// Add a node property with f64 values using the store's config for backend selection.
    pub fn add_node_property_f64(
        &mut self,
        key: String,
        values: Vec<f64>,
    ) -> Result<(), GraphStoreError> {
        let node_count = self.id_map.node_count();
        
        // Use config to create CollectionsConfig
        let collections_config = self.config.node_collections_config::<f64>(node_count);
        
        // Create property using config
        use crate::types::properties::node::impls::default_node_property_values::DefaultDoubleNodePropertyValues;
        use crate::collections::backends::vec::VecDouble;
        
        let backend = crate::collections::backends::factory::create_double_backend_from_config(&collections_config, values);
        let pv = Arc::new(DefaultDoubleNodePropertyValues::<VecDouble>::from_collection(backend, node_count));
        
        self.node_properties.insert(key, pv);
        self.set_modified();
        Ok(())
    }

    /// Add a graph property with i64 values using the store's config for backend selection.
    pub fn add_graph_property_i64(
        &mut self,
        key: String,
        values: Vec<i64>,
    ) -> Result<(), GraphStoreError> {
        // Use config to create CollectionsConfig (graph properties don't scale with node count)
        let collections_config = self.config.graph_collections_config::<i64>(values.len());
        
        // Create property using config
        use crate::types::properties::graph::impls::default_graph_property_values::DefaultLongGraphPropertyValues;
        use crate::collections::backends::vec::VecLong;
        
        let backend = crate::collections::backends::factory::create_long_backend_from_config(&collections_config, values);
        let pv = Arc::new(DefaultLongGraphPropertyValues::<VecLong>::from_collection(backend));
        
        self.graph_properties.insert(key, pv);
        self.set_modified();
        Ok(())
    }

    /// Add a graph property with f64 values using the store's config for backend selection.
    pub fn add_graph_property_f64(
        &mut self,
        key: String,
        values: Vec<f64>,
    ) -> Result<(), GraphStoreError> {
        // Use config to create CollectionsConfig
        let collections_config = self.config.graph_collections_config::<f64>(values.len());
        
        // Create property using config
        use crate::types::properties::graph::impls::default_graph_property_values::DefaultDoubleGraphPropertyValues;
        use crate::collections::backends::vec::VecDouble;
        
        let backend = crate::collections::backends::factory::create_double_backend_from_config(&collections_config, values);
        let pv = Arc::new(DefaultDoubleGraphPropertyValues::<VecDouble>::from_collection(backend));
        
        self.graph_properties.insert(key, pv);
        self.set_modified();
        Ok(())
    }

    fn to_schema_label(label: &NodeLabel) -> NodeLabel {
        NodeLabel::of(label.name())
    }

    fn label_key(label: &NodeLabel) -> String {
        label.name().to_string()
    }

    fn rebuild_relationship_metadata(&mut self) {
        let mut ordered: Vec<RelationshipType> =
            self.relationship_topologies.keys().cloned().collect();
        ordered.sort_by(|left, right| left.name().cmp(right.name()));

        let mut inverse_indexed = HashSet::new();
        let mut relationship_count = 0usize;
        let mut has_parallel = false;

        for rel_type in &ordered {
            if let Some(topology) = self.relationship_topologies.get(rel_type) {
                if topology.is_inverse_indexed() {
                    inverse_indexed.insert(rel_type.clone());
                }
                if topology.has_parallel_edges() {
                    has_parallel = true;
                }
                relationship_count += topology.relationship_count();
            }
        }

        let all_inverse_indexed = !ordered.is_empty()
            && ordered
                .iter()
                .all(|rel_type| inverse_indexed.contains(rel_type));

        let mut characteristics_builder = GraphCharacteristicsBuilder::new();
        match self.schema.direction() {
            Direction::Directed => {
                characteristics_builder = characteristics_builder.directed();
            }
            Direction::Undirected => {
                characteristics_builder = characteristics_builder.undirected();
            }
        }

        if all_inverse_indexed {
            characteristics_builder = characteristics_builder.inverse_indexed();
        }

        self.ordered_relationship_types = ordered;
        self.inverse_indexed_relationship_types = inverse_indexed;
        self.relationship_count = relationship_count;
        self.has_parallel_relationships = has_parallel;
        self.graph_characteristics = characteristics_builder.build();
    }

    fn refresh_relationship_property_state(&mut self) {
        self.has_relationship_properties = self
            .relationship_property_stores
            .values()
            .any(|store| !store.is_empty());
    }
}

impl GraphStore for DefaultGraphStore {
    fn database_info(&self) -> &DatabaseInfo {
        &self.database_info
    }

    fn schema(&self) -> &GraphSchema {
        &self.schema
    }

    fn creation_time(&self) -> DateTime<Utc> {
        self.creation_time
    }

    fn modification_time(&self) -> DateTime<Utc> {
        self.modification_time
    }

    fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }

    fn nodes(&self) -> Arc<dyn IdMap> {
        Arc::clone(&self.id_map) as Arc<dyn IdMap>
    }

    fn graph_property_keys(&self) -> HashSet<String> {
        self.graph_properties.keys().cloned().collect()
    }

    fn has_graph_property(&self, property_key: &str) -> bool {
        self.graph_properties.contains_key(property_key)
    }

    fn graph_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        // First check the actual property stores
        if let Some(property_values) = self.graph_properties.get(property_key) {
            return Ok(property_values.value_type());
        }

        // Fall back to schema if property not found in stores
        if let Some(property_schema) = self.schema.graph_properties().get(property_key) {
            return Ok(property_schema.value_type());
        }

        Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn graph_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn GraphPropertyValues>> {
        self.graph_properties
            .get(property_key)
            .cloned()
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn add_graph_property(
        &mut self,
        property_key: impl Into<String>,
        property_values: Arc<dyn GraphPropertyValues>,
    ) -> GraphStoreResult<()> {
        let key = property_key.into();
        self.graph_properties.insert(key, property_values);
        self.set_modified();
        Ok(())
    }

    fn remove_graph_property(&mut self, property_key: &str) -> GraphStoreResult<()> {
        if self.graph_properties.remove(property_key).is_some() {
            self.set_modified();
            Ok(())
        } else {
            Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
        }
    }

    fn node_count(&self) -> usize {
        self.id_map.node_count()
    }

    fn node_count_for_label(&self, label: &NodeLabel) -> usize {
        let schema_label = Self::to_schema_label(label);
        self.id_map.node_count_for_label(&schema_label)
    }

    fn node_labels(&self) -> HashSet<NodeLabel> {
        self.schema_labels()
    }

    fn has_node_label(&self, label: &NodeLabel) -> bool {
        let schema_label = Self::to_schema_label(label);
        self.schema_labels().contains(&schema_label)
    }

    fn add_node_label(&mut self, node_label: NodeLabel) -> GraphStoreResult<()> {
        let schema_label = Self::to_schema_label(&node_label);
        Arc::make_mut(&mut self.id_map).add_node_label(schema_label);
        self.set_modified();
        Ok(())
    }

    fn node_property_keys(&self) -> HashSet<String> {
        self.node_properties.keys().cloned().collect()
    }

    fn node_property_keys_for_label(&self, label: &NodeLabel) -> HashSet<String> {
        self.node_properties_by_label
            .get(&Self::label_key(label))
            .cloned()
            .unwrap_or_default()
    }

    fn node_property_keys_for_labels(&self, labels: &HashSet<NodeLabel>) -> HashSet<String> {
        if labels.is_empty() {
            return self.node_property_keys();
        }

        let mut iter = labels.iter();
        let first = iter.next().unwrap();
        let mut intersection = self.node_property_keys_for_label(first);

        for label in iter {
            let keys = self.node_property_keys_for_label(label);
            intersection = intersection
                .intersection(&keys)
                .cloned()
                .collect::<HashSet<_>>();
        }

        intersection
    }

    fn has_node_property(&self, property_key: &str) -> bool {
        self.node_properties.contains_key(property_key)
    }

    fn has_node_property_for_label(&self, label: &NodeLabel, property_key: &str) -> bool {
        self.node_properties_by_label
            .get(&Self::label_key(label))
            .map(|keys| keys.contains(property_key))
            .unwrap_or(false)
    }

    fn node_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        // First check the actual property stores
        if let Some(property_values) = self.node_properties.get(property_key) {
            return Ok(property_values.value_type());
        }

        // Fall back to schema if property not found in stores
        for entry in self.schema.node_schema().entries() {
            if let Some(property_schema) = entry.properties().get(property_key) {
                return Ok(property_schema.value_type());
            }
        }

        Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn node_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn NodePropertyValues>> {
        self.node_properties
            .get(property_key)
            .cloned()
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn add_node_property(
        &mut self,
        node_labels: HashSet<NodeLabel>,
        property_key: impl Into<String>,
        property_values: Arc<dyn NodePropertyValues>,
    ) -> GraphStoreResult<()> {
        let key = property_key.into();
        self.node_properties.insert(key.clone(), property_values);

        for label in node_labels {
            let label_key = Self::label_key(&label);
            self.node_properties_by_label
                .entry(label_key)
                .or_default()
                .insert(key.clone());
        }

        self.set_modified();
        Ok(())
    }

    fn remove_node_property(&mut self, property_key: &str) -> GraphStoreResult<()> {
        if self.node_properties.remove(property_key).is_some() {
            for keys in self.node_properties_by_label.values_mut() {
                keys.remove(property_key);
            }
            self.set_modified();
            Ok(())
        } else {
            Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
        }
    }

    fn relationship_count(&self) -> usize {
        self.relationship_count
    }

    fn relationship_count_for_type(&self, relationship_type: &RelationshipType) -> usize {
        self.relationship_topologies
            .get(relationship_type)
            .map(|topology| topology.relationship_count())
            .unwrap_or(0)
    }

    fn relationship_types(&self) -> HashSet<RelationshipType> {
        let mut types: HashSet<RelationshipType> =
            self.relationship_topologies.keys().cloned().collect();

        for schema_type in self.schema.relationship_schema().available_types() {
            types.insert(RelationshipType::of(schema_type.name()));
        }

        types
    }

    fn has_relationship_type(&self, relationship_type: &RelationshipType) -> bool {
        self.relationship_types().contains(relationship_type)
    }

    fn inverse_indexed_relationship_types(&self) -> HashSet<RelationshipType> {
        self.inverse_indexed_relationship_types.clone()
    }

    fn relationship_property_keys(&self) -> HashSet<String> {
        self.relationship_property_stores
            .values()
            .flat_map(|store| store.relationship_properties().keys().cloned())
            .collect()
    }

    fn relationship_property_keys_for_type(&self, rel_type: &RelationshipType) -> HashSet<String> {
        self.relationship_property_stores
            .get(rel_type)
            .map(|store| store.relationship_properties().keys().cloned().collect())
            .unwrap_or_default()
    }

    fn relationship_property_keys_for_types(
        &self,
        rel_types: &HashSet<RelationshipType>,
    ) -> HashSet<String> {
        rel_types
            .iter()
            .flat_map(|rel_type| self.relationship_property_keys_for_type(rel_type))
            .collect()
    }

    fn has_relationship_property(&self, rel_type: &RelationshipType, property_key: &str) -> bool {
        self.relationship_property_stores
            .get(rel_type)
            .map(|store| store.contains_key(property_key))
            .unwrap_or(false)
    }

    fn relationship_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        // First check the actual property stores
        for store in self.relationship_property_stores.values() {
            if let Some(property) = store.get(property_key) {
                return Ok(property.property_schema().value_type());
            }
        }

        // Fall back to schema if property not found in stores
        for entry in self.schema.relationship_schema().entries() {
            if let Some(property_schema) = entry.properties().get(property_key) {
                return Ok(property_schema.value_type());
            }
        }

        Err(GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn relationship_property_values(
        &self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn RelationshipPropertyValues>> {
        self.relationship_property_stores
            .get(relationship_type)
            .and_then(|store| store.get(property_key))
            .map(|property| property.values_arc())
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn add_relationship_property(
        &mut self,
        relationship_type: RelationshipType,
        property_key: impl Into<String>,
        property_values: Arc<dyn RelationshipPropertyValues>,
    ) -> GraphStoreResult<()> {
        let key = property_key.into();
        let property = RelationshipProperty::with_state(
            key.clone(),
            PropertyState::Persistent,
            property_values,
        );

        let store = self
            .relationship_property_stores
            .remove(&relationship_type)
            .unwrap_or_else(RelationshipPropertyStore::empty);

        let updated_store = store.to_builder().put(key, property).build();
        self.relationship_property_stores
            .insert(relationship_type, updated_store);

        self.refresh_relationship_property_state();
        self.set_modified();
        Ok(())
    }

    fn remove_relationship_property(
        &mut self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<()> {
        let store = self
            .relationship_property_stores
            .remove(relationship_type)
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))?;

        if !store.contains_key(property_key) {
            // Restore the store since the property wasn't found
            self.relationship_property_stores
                .insert(relationship_type.clone(), store);
            return Err(GraphStoreError::PropertyNotFound(property_key.to_string()));
        }

        let updated_store = store.to_builder().remove_property(property_key).build();

        if !updated_store.is_empty() {
            self.relationship_property_stores
                .insert(relationship_type.clone(), updated_store);
        }

        self.refresh_relationship_property_state();
        self.set_modified();
        Ok(())
    }

    fn delete_relationships(
        &mut self,
        relationship_type: &RelationshipType,
    ) -> GraphStoreResult<DeletionResult> {
        if let Some(topology) = self.relationship_topologies.remove(relationship_type) {
            let removed_count = topology.relationship_count();
            self.relationship_property_stores.remove(relationship_type);
            self.rebuild_relationship_metadata();
            self.refresh_relationship_property_state();
            self.set_modified();
            Ok(DeletionResult::with_counts(
                self.graph_name.clone(),
                0,
                removed_count,
            ))
        } else {
            Err(GraphStoreError::RelationshipTypeNotFound(
                relationship_type.name().to_string(),
            ))
        }
    }

    fn get_graph(&self) -> Arc<dyn Graph> {
        let topologies = self
            .relationship_topologies
            .iter()
            .map(|(rel_type, topology)| (rel_type.clone(), Arc::clone(topology)))
            .collect::<HashMap<_, _>>();

        Arc::new(DefaultGraph::new(
            Arc::clone(&self.config),
            Arc::clone(&self.schema),
            Arc::clone(&self.id_map),
            self.graph_characteristics,
            topologies,
            self.ordered_relationship_types.clone(),
            self.inverse_indexed_relationship_types.clone(),
            self.relationship_count,
            self.has_parallel_relationships,
            self.node_properties.clone(),
            self.relationship_property_stores.clone(),
            HashMap::new(),
        ))
    }

    fn get_graph_with_types(
        &self,
        relationship_types: &HashSet<RelationshipType>,
    ) -> crate::types::graph::GraphResult<Arc<dyn Graph>> {
        self.graph()
            .relationship_type_filtered_graph(relationship_types)
    }

    fn get_graph_with_types_and_selectors(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        relationship_property_selectors: &HashMap<RelationshipType, String>,
    ) -> crate::types::graph::GraphResult<Arc<dyn Graph>> {
        // Build a DefaultGraph then let it select properties based on provided selectors
        let selectors = relationship_property_selectors.clone();

        // If selector missing and exactly one property exists for a type, allow DefaultGraph::new to auto-select
        // by passing selectors as-is; DefaultGraph::new handles auto-selection.
        let topologies = self
            .relationship_topologies
            .iter()
            .filter(|(rel_type, _)| relationship_types.contains(*rel_type))
            .map(|(rel_type, topology)| (rel_type.clone(), Arc::clone(topology)))
            .collect::<HashMap<_, _>>();

        let mut ordered_types = self
            .ordered_relationship_types
            .iter()
            .filter(|rel_type| relationship_types.contains(*rel_type))
            .cloned()
            .collect::<Vec<_>>();

        let mut inverse_indexed_types = self
            .inverse_indexed_relationship_types
            .iter()
            .filter(|rel_type| relationship_types.contains(*rel_type))
            .cloned()
            .collect::<HashSet<_>>();

        let relationship_count: usize = topologies
            .values()
            .map(|top| top.relationship_count())
            .sum();

        let has_parallel_edges = topologies
            .values()
            .any(|top| top.has_parallel_edges());

        // Characteristics: preserve directed/undirected; inverse_indexed only if all selected are inverse indexed
        let all_inverse_indexed = !ordered_types.is_empty()
            && ordered_types
                .iter()
                .all(|t| inverse_indexed_types.contains(t));
        let mut characteristics_builder = GraphCharacteristicsBuilder::new();
        match self.schema.direction() {
            Direction::Directed => {
                characteristics_builder = characteristics_builder.directed();
            }
            Direction::Undirected => {
                characteristics_builder = characteristics_builder.undirected();
            }
        }
        if all_inverse_indexed {
            characteristics_builder = characteristics_builder.inverse_indexed();
        }
        let filtered_characteristics = characteristics_builder.build();

        // Filter relationship properties and apply selectors
        let filtered_relationship_properties = ordered_types
            .iter()
            .filter_map(|rel_type| {
                self.relationship_property_stores
                    .get(rel_type)
                    .map(|store| (rel_type.clone(), store.clone()))
            })
            .collect::<HashMap<_, _>>();

        // DefaultGraph::new expects selectors keyed by type name
        let filtered_selectors = selectors
            .into_iter()
            .filter(|(rel_type, _)| ordered_types.contains(rel_type))
            .collect::<HashMap<_, _>>();

        let filtered_graph = DefaultGraph::new(
            Arc::clone(&self.config),
            Arc::clone(&self.schema),
            Arc::clone(&self.id_map),
            filtered_characteristics,
            topologies,
            std::mem::take(&mut ordered_types),
            std::mem::take(&mut inverse_indexed_types),
            relationship_count,
            has_parallel_edges,
            self.node_properties.clone(),
            filtered_relationship_properties,
            filtered_selectors,
        );

        Ok(Arc::new(filtered_graph))
    }

    fn get_graph_with_types_and_orientation(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        _orientation: Orientation,
    ) -> crate::types::graph::GraphResult<Arc<dyn Graph>> {
        // Orientation informs traversal; return a filtered view by types.
        self.graph()
            .relationship_type_filtered_graph(relationship_types)
    }

    fn get_graph_with_types_selectors_and_orientation(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        relationship_property_selectors: &HashMap<RelationshipType, String>,
        orientation: Orientation,
    ) -> crate::types::graph::GraphResult<Arc<dyn Graph>> {
        let view = self.get_graph_with_types_and_selectors(
            relationship_types,
            relationship_property_selectors,
        )?;
        let _ = orientation; // reserved for future orientation-aware views
        Ok(view)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph::degrees::Degrees;
    use crate::types::graph::Graph;
    use crate::types::graph_store::{DatabaseId, DatabaseLocation};
    use crate::types::properties::relationship::impls::default_relationship_property_values::DefaultRelationshipPropertyValues;
    use std::sync::Arc;

    fn sample_store() -> DefaultGraphStore {
        let graph_name = GraphName::new("g");
        let database_info = DatabaseInfo::new(
            DatabaseId::new("db"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        );
        let schema = GraphSchema::empty();
        let capabilities = Capabilities::default();
        let id_map = SimpleIdMap::from_original_ids([0, 1, 2]);

        let topology = RelationshipTopology::new(vec![vec![1, 2], vec![2], vec![]], None);

        let mut relationship_topologies = HashMap::new();
        relationship_topologies.insert(RelationshipType::of("KNOWS"), topology);

        DefaultGraphStore::new(
            crate::config::GraphStoreConfig::default(),
            graph_name,
            database_info,
            schema,
            capabilities,
            id_map,
            relationship_topologies,
        )
    }

    #[test]
    fn graph_view_reflects_store_data() {
        let store = sample_store();
        assert_eq!(store.node_count(), 3);
        assert_eq!(store.relationship_count(), 3);

        let graph = store.graph();
        assert_eq!(graph.relationship_count(), 3);
        assert!(graph.characteristics().is_undirected());
        assert_eq!(graph.degree(0), 2);
    }

    #[test]
    fn test_add_node_property_with_config() {
        use crate::config::GraphStoreConfig;
        
        // Create config with specific backend
        let config = GraphStoreConfig::default();
        
        let mut store = DefaultGraphStore::new(
            config,
            GraphName::new("test"),
            DatabaseInfo::new(
                DatabaseId::new("test"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            GraphSchema::empty(),
            Capabilities::default(),
            SimpleIdMap::from_original_ids([0, 1, 2]),
            HashMap::new(),
        );
        
        // Add property - should use Vec backend from config
        store.add_node_property_i64("age".to_string(), vec![1, 2, 3]).unwrap();
        
        // Verify property exists
        assert!(store.node_properties.contains_key("age"));
        assert_eq!(store.node_properties.len(), 1);
    }

    #[test]
    fn test_add_graph_property_with_config() {
        use crate::config::GraphStoreConfig;
        
        let config = GraphStoreConfig::default();
        
        let mut store = DefaultGraphStore::new(
            config,
            GraphName::new("test"),
            DatabaseInfo::new(
                DatabaseId::new("test"),
                DatabaseLocation::remote("localhost", 7687, None, None),
            ),
            GraphSchema::empty(),
            Capabilities::default(),
            SimpleIdMap::from_original_ids([0, 1, 2]),
            HashMap::new(),
        );
        
        // Add graph property
        store.add_graph_property_f64("density".to_string(), vec![0.5]).unwrap();
        
        // Verify property exists
        assert!(store.graph_properties.contains_key("density"));
        assert_eq!(store.graph_properties.len(), 1);
    }

    #[test]
    fn manages_relationship_properties() {
        let mut store = sample_store();
        let rel_type = RelationshipType::of("KNOWS");

        let values = Arc::new(DefaultRelationshipPropertyValues::with_default(
            vec![1.0, 2.0, 3.0],
            3,
        ));

        store
            .add_relationship_property(rel_type.clone(), "weight", values)
            .expect("add relationship property");

        assert!(store.has_relationship_property(&rel_type, "weight"));
        assert!(store.relationship_property_keys().contains("weight"));
        let retrieved = store
            .relationship_property_values(&rel_type, "weight")
            .expect("retrieve property");
        assert_eq!(retrieved.double_value(1).unwrap(), 2.0);
        assert!(store.graph().has_relationship_property());

        store
            .remove_relationship_property(&rel_type, "weight")
            .expect("remove relationship property");
        assert!(!store.has_relationship_property(&rel_type, "weight"));
        assert!(!store.graph().has_relationship_property());
    }
}
