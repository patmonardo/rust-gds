use super::{
    Capabilities, DatabaseInfo, DeletionResult, GraphName, GraphStore, GraphStoreError,
    GraphStoreResult,
};
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph::{
    id_map::{IdMap, SimpleIdMap},
    DefaultGraph, GraphCharacteristics, GraphCharacteristicsBuilder, RelationshipTopology,
};
use crate::types::properties::graph::graph_property_values::GraphPropertyValues;
use crate::types::value_type::ValueType;

// FIXED: use explicit module path for node property values (re-exports removed)
use crate::types::properties::node::node_property_values::NodePropertyValues;

// FIXED: expand relationship module imports explicitly (no broad pub use now)
use crate::types::properties::relationship::impls::default_relationship_property_store::DefaultRelationshipPropertyStore;
use crate::types::properties::relationship::relationship_property::RelationshipProperty;
use crate::types::properties::relationship::relationship_property_store::{
    RelationshipPropertyStore, RelationshipPropertyStoreBuilder,
};
use crate::types::properties::relationship::relationship_property_values::RelationshipPropertyValues;

use crate::types::property_state::PropertyState;
use crate::types::schema::{
    Direction, GraphSchema, NodeLabel as SchemaNodeLabel, PropertySchemaTrait,
};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// In-memory [`GraphStore`] backed by [`SimpleIdMap`] and [`RelationshipTopology`].
#[derive(Debug, Clone)]
pub struct DefaultGraphStore {
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
        graph_name: GraphName,
        database_info: DatabaseInfo,
        schema: GraphSchema,
        capabilities: Capabilities,
        id_map: SimpleIdMap,
        relationship_topologies: HashMap<RelationshipType, RelationshipTopology>,
    ) -> Self {
        let now = Utc::now();
        let schema = Arc::new(schema);
        let id_map = Arc::new(id_map);
        let relationship_topologies = relationship_topologies
            .into_iter()
            .map(|(rel_type, topology)| (rel_type, Arc::new(topology)))
            .collect();

        let mut store = Self {
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
    pub fn graph(&self) -> Arc<DefaultGraph> {
        let topologies = self
            .relationship_topologies
            .iter()
            .map(|(rel_type, topology)| (rel_type.clone(), Arc::clone(topology)))
            .collect::<HashMap<_, _>>();

        Arc::new(DefaultGraph::new(
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

    fn schema_labels(&self) -> HashSet<SchemaNodeLabel> {
        self.id_map.available_node_labels()
    }

    fn to_schema_label(label: &NodeLabel) -> SchemaNodeLabel {
        SchemaNodeLabel::new(label.name())
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

    fn graph_property_keys(&self) -> HashSet<String> {
        self.graph_properties.keys().cloned().collect()
    }

    fn has_graph_property(&self, property_key: &str) -> bool {
        self.graph_properties.contains_key(property_key)
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
            .into_iter()
            .map(|label| NodeLabel::of(label.name()))
            .collect()
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
        if let Some(value_type) = self
            .relationship_property_stores
            .values()
            .find_map(|store| store.get(property_key))
            .map(|property| property.values.value_type())
        {
            return Ok(value_type);
        }

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
            .map(|property| {
                // Cast Arc<dyn PropertyValues> to Arc<dyn RelationshipPropertyValues>
                // SAFETY: By construction, RelationshipProperty only stores RelationshipPropertyValues
                let arc_copy = Arc::clone(&property.values);
                unsafe {
                    std::mem::transmute::<
                        Arc<dyn crate::types::properties::property_values::PropertyValues>,
                        Arc<dyn RelationshipPropertyValues>,
                    >(arc_copy)
                }
            })
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn add_relationship_property(
        &mut self,
        relationship_type: RelationshipType,
        property_key: impl Into<String>,
        property_values: Arc<dyn RelationshipPropertyValues>,
    ) -> GraphStoreResult<()> {
        let key = property_key.into();
        let property =
            RelationshipProperty::of(key.clone(), PropertyState::Persistent, property_values);

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph::degrees::Degrees;
    use crate::types::graph::Graph;
    use crate::types::graph_store::{DatabaseId, DatabaseLocation};
    use crate::types::properties::relationship::DefaultRelationshipPropertyValues;
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
