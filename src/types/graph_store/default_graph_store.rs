use super::{
    Capabilities, DatabaseInfo, DeletionResult, GraphName, GraphStore, GraphStoreError,
    GraphStoreResult,
};
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph::{
    DefaultGraph, GraphCharacteristics, GraphCharacteristicsBuilder, RelationshipTopology,
};
use crate::types::id_map::{IdMap, SimpleIdMap};
use crate::types::properties::graph::GraphPropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::relationship::RelationshipPropertyValues;
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
    schema: GraphSchema,
    capabilities: Capabilities,
    creation_time: DateTime<Utc>,
    modification_time: DateTime<Utc>,
    id_map: SimpleIdMap,
    relationship_topologies: HashMap<RelationshipType, RelationshipTopology>,
    graph_properties: HashMap<String, Arc<dyn GraphPropertyValues>>,
    node_properties: HashMap<String, Arc<dyn NodePropertyValues>>,
    node_properties_by_label: HashMap<String, HashSet<String>>,
    relationship_property_values:
        HashMap<RelationshipType, HashMap<String, Arc<dyn RelationshipPropertyValues>>>,
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
        Self {
            graph_name,
            database_info,
            schema,
            capabilities,
            creation_time: now,
            modification_time: now,
            id_map,
            relationship_topologies,
            graph_properties: HashMap::new(),
            node_properties: HashMap::new(),
            node_properties_by_label: HashMap::new(),
            relationship_property_values: HashMap::new(),
        }
    }

    /// Builds a [`DefaultGraph`] view over the current store contents.
    pub fn graph(&self) -> Arc<DefaultGraph> {
        let mut ordered_types: Vec<RelationshipType> =
            self.relationship_topologies.keys().cloned().collect();
        ordered_types.sort_by(|left, right| left.name().cmp(right.name()));

        let mut topologies = HashMap::new();
        let mut inverse_indexed_types = HashSet::new();
        let mut relationship_count = 0usize;
        let mut has_parallel_edges = false;

        for relationship_type in &ordered_types {
            if let Some(topology) = self.relationship_topologies.get(relationship_type) {
                if topology.is_inverse_indexed() {
                    inverse_indexed_types.insert(relationship_type.clone());
                }
                if topology.has_parallel_edges() {
                    has_parallel_edges = true;
                }
                relationship_count += topology.relationship_count();
                topologies.insert(relationship_type.clone(), Arc::new(topology.clone()));
            }
        }

        let mut characteristics_builder = GraphCharacteristicsBuilder::new();
        match self.schema.direction() {
            Direction::Directed => {
                characteristics_builder = characteristics_builder.directed();
            }
            Direction::Undirected => {
                characteristics_builder = characteristics_builder.undirected();
            }
        }

        let inverse_indexed = !ordered_types.is_empty()
            && ordered_types
                .iter()
                .all(|rel_type| inverse_indexed_types.contains(rel_type));
        if inverse_indexed {
            characteristics_builder = characteristics_builder.inverse_indexed();
        }

        let characteristics: GraphCharacteristics = characteristics_builder.build();

        Arc::new(DefaultGraph::new(
            Arc::new(self.schema.clone()),
            Arc::new(self.id_map.clone()),
            characteristics,
            topologies,
            ordered_types,
            inverse_indexed_types,
            relationship_count,
            has_parallel_edges,
            self.node_properties.clone(),
            self.relationship_property_values
                .values()
                .any(|properties| !properties.is_empty()),
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
        self.id_map.add_node_label(schema_label);
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
                .or_insert_with(HashSet::new)
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
        self.relationship_topologies
            .values()
            .map(RelationshipTopology::relationship_count)
            .sum()
    }

    fn relationship_count_for_type(&self, relationship_type: &RelationshipType) -> usize {
        self.relationship_topologies
            .get(relationship_type)
            .map(RelationshipTopology::relationship_count)
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
        self.relationship_topologies
            .iter()
            .filter_map(|(rel_type, topology)| {
                if topology.is_inverse_indexed() {
                    Some(rel_type.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn relationship_property_keys(&self) -> HashSet<String> {
        self.relationship_property_values
            .values()
            .flat_map(|props| props.keys().cloned())
            .collect()
    }

    fn relationship_property_keys_for_type(&self, rel_type: &RelationshipType) -> HashSet<String> {
        self.relationship_property_values
            .get(rel_type)
            .map(|props| props.keys().cloned().collect())
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
        self.relationship_property_values
            .get(rel_type)
            .map(|props| props.contains_key(property_key))
            .unwrap_or(false)
    }

    fn relationship_property_type(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<crate::types::property::ValueType> {
        if let Some(value_type) = self
            .relationship_property_values
            .values()
            .find_map(|props| props.get(property_key))
            .map(|values| values.value_type())
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
        self.relationship_property_values
            .get(relationship_type)
            .and_then(|props| props.get(property_key))
            .cloned()
            .ok_or_else(|| GraphStoreError::PropertyNotFound(property_key.to_string()))
    }

    fn delete_relationships(
        &mut self,
        relationship_type: &RelationshipType,
    ) -> GraphStoreResult<DeletionResult> {
        if let Some(topology) = self.relationship_topologies.remove(relationship_type) {
            let removed_count = topology.relationship_count();
            self.relationship_property_values.remove(relationship_type);
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
}
