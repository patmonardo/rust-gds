//! GraphStore trait - Central interface for managing and accessing graph data.

use super::{Capabilities, DatabaseInfo, DeletionResult};
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph::{Graph, GraphResult};
use crate::types::graph::id_map::IdMap;
use crate::projection::orientation::Orientation;
use crate::types::properties::graph::GraphPropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::relationship::RelationshipPropertyValues;
use crate::types::schema::GraphSchema;
use crate::types::ValueType;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Result type for GraphStore operations.
pub type GraphStoreResult<T> = Result<T, GraphStoreError>;

/// Errors that can occur during GraphStore operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum GraphStoreError {
    #[error("Node label not found: {0}")]
    NodeLabelNotFound(String),

    #[error("Relationship type not found: {0}")]
    RelationshipTypeNotFound(String),

    #[error("Property not found: {0}")]
    PropertyNotFound(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Schema error: {0}")]
    SchemaError(String),
}

/// Central interface for managing and accessing graph data.
///
/// GraphStore is the main orchestrator that provides methods for querying
/// and manipulating nodes, relationships, and their properties at both
/// individual and collection levels.
///
/// # Architecture
///
/// The GraphStore acts as a facade over several subsystems:
/// - **Schema**: Defines the structure (node labels, relationship types, properties)
/// - **IdMap**: Maps external node IDs to internal compact IDs
/// - **Properties**: Stores property values for nodes, relationships, and the graph
/// - **Topology**: Manages relationship connectivity
/// - **Graph**: Provides filtered views of the data
///
/// # Thread Safety
///
/// Implementations should be thread-safe, typically using Arc and RwLock
/// for shared mutable state.
pub trait GraphStore: Send + Sync {
    // =============================================================================
    // Database & Metadata
    // =============================================================================

    /// Returns information about the database this graph store was created from.
    fn database_info(&self) -> &DatabaseInfo;

    /// Returns the schema of this graph store.
    fn schema(&self) -> &GraphSchema;

    /// Returns the creation time of this graph store.
    fn creation_time(&self) -> chrono::DateTime<chrono::Utc>;

    /// Returns the last modification time of this graph store.
    fn modification_time(&self) -> chrono::DateTime<chrono::Utc>;

    /// Returns the capabilities of this graph store.
    fn capabilities(&self) -> &Capabilities;
    // =============================================================================
    // Core identity
    // =============================================================================

    /// Returns the node IdMap (original↔mapped ids) for this store.
    fn nodes(&self) -> Arc<dyn IdMap>;

    /// Returns the set of relationship types present in the store.
    fn relationships(&self) -> HashSet<RelationshipType> {
        self.relationship_types()
    }

    // =============================================================================
    // Graph Properties
    // =============================================================================

    /// Returns all graph property keys.
    fn graph_property_keys(&self) -> HashSet<String>;

    /// Checks if a graph property exists.
    fn has_graph_property(&self, property_key: &str) -> bool;

    /// Returns the value type of a graph property.
    fn graph_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType>;

    /// Returns graph property values.
    fn graph_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn GraphPropertyValues>>;

    /// Adds a graph property.
    fn add_graph_property(
        &mut self,
        _property_key: impl Into<String>,
        _property_values: Arc<dyn GraphPropertyValues>,
    ) -> GraphStoreResult<()>;

    /// Removes a graph property.
    fn remove_graph_property(&mut self, property_key: &str) -> GraphStoreResult<()>;

    // =============================================================================
    // Nodes
    // =============================================================================

    /// Returns the total number of nodes in the graph store.
    fn node_count(&self) -> usize;

    /// Returns the number of nodes with a specific label.
    fn node_count_for_label(&self, label: &NodeLabel) -> usize;

    /// Returns all node labels in the graph store.
    fn node_labels(&self) -> HashSet<NodeLabel>;

    /// Checks if a node label exists.
    fn has_node_label(&self, label: &NodeLabel) -> bool;

    /// Adds a new node label to the graph store.
    fn add_node_label(&mut self, node_label: NodeLabel) -> GraphStoreResult<()>;

    // =============================================================================
    // Node Properties
    // =============================================================================

    /// Returns all node property keys in the graph store.
    fn node_property_keys(&self) -> HashSet<String>;

    /// Returns all property keys for a specific node label.
    fn node_property_keys_for_label(&self, label: &NodeLabel) -> HashSet<String>;

    /// Returns property keys common to all specified node labels.
    fn node_property_keys_for_labels(&self, labels: &HashSet<NodeLabel>) -> HashSet<String>;

    /// Checks if a node property exists.
    fn has_node_property(&self, property_key: &str) -> bool;

    /// Checks if a node property exists for a specific label.
    fn has_node_property_for_label(&self, label: &NodeLabel, property_key: &str) -> bool;

    /// Returns the value type of a node property.
    fn node_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType>;

    /// Returns node property values.
    fn node_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn NodePropertyValues>>;

    /// Adds a node property.
    fn add_node_property(
        &mut self,
        node_labels: HashSet<NodeLabel>,
        property_key: impl Into<String>,
        property_values: Arc<dyn NodePropertyValues>,
    ) -> GraphStoreResult<()>;

    /// Removes a node property.
    fn remove_node_property(&mut self, property_key: &str) -> GraphStoreResult<()>;

    // =============================================================================
    // Relationships
    // =============================================================================

    /// Returns the total number of relationships in the graph store.
    fn relationship_count(&self) -> usize;

    /// Returns the number of relationships of a specific type.
    fn relationship_count_for_type(&self, relationship_type: &RelationshipType) -> usize;

    /// Returns all relationship types in the graph store.
    fn relationship_types(&self) -> HashSet<RelationshipType>;

    /// Checks if a relationship type exists.
    fn has_relationship_type(&self, relationship_type: &RelationshipType) -> bool;

    /// Returns relationship types that have an inverse index.
    fn inverse_indexed_relationship_types(&self) -> HashSet<RelationshipType>;

    // =============================================================================
    // Relationship Properties
    // =============================================================================

    /// Returns all relationship property keys in the graph store.
    fn relationship_property_keys(&self) -> HashSet<String>;

    /// Returns all property keys for a specific relationship type.
    fn relationship_property_keys_for_type(&self, rel_type: &RelationshipType) -> HashSet<String>;

    /// Returns property keys common to all specified relationship types.
    fn relationship_property_keys_for_types(
        &self,
        rel_types: &HashSet<RelationshipType>,
    ) -> HashSet<String>;

    /// Checks if a relationship property exists for a specific relationship type.
    fn has_relationship_property(&self, rel_type: &RelationshipType, property_key: &str) -> bool;

    /// Returns the value type of a relationship property.
    fn relationship_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType>;

    /// Returns relationship property values.
    fn relationship_property_values(
        &self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn RelationshipPropertyValues>>;

    /// Adds relationship property values for the given relationship type.
    fn add_relationship_property(
        &mut self,
        relationship_type: RelationshipType,
        property_key: impl Into<String>,
        property_values: Arc<dyn RelationshipPropertyValues>,
    ) -> GraphStoreResult<()>;

    /// Removes a relationship property from the given relationship type.
    fn remove_relationship_property(
        &mut self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<()>;

    // =============================================================================
    // Operations
    // =============================================================================

    /// Deletes relationships of a specific type.
    fn delete_relationships(
        &mut self,
        relationship_type: &RelationshipType,
    ) -> GraphStoreResult<DeletionResult>;

    // =============================================================================
    // Graph Views
    // =============================================================================

    /// Returns an unfiltered graph view over all nodes and relationships.
    /// This is the primary method for obtaining a Graph instance from the store.
    fn get_graph(&self) -> Arc<dyn Graph>;

    /// Returns a graph view filtered to the provided relationship types.
    fn get_graph_with_types(
        &self,
        relationship_types: &HashSet<RelationshipType>,
    ) -> GraphResult<Arc<dyn Graph>>;

    /// Returns a graph view filtered to the provided relationship types and using
    /// the provided relationship property selectors per type.
    ///
    /// When a selector is not provided for a type, the implementation may auto-select
    /// a property if exactly one exists, otherwise no property is selected for that type.
    fn get_graph_with_types_and_selectors(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        relationship_property_selectors: &HashMap<RelationshipType, String>,
    ) -> GraphResult<Arc<dyn Graph>>;

    /// Returns a graph view filtered to the provided relationship types and orientation.
    fn get_graph_with_types_and_orientation(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        orientation: Orientation,
    ) -> GraphResult<Arc<dyn Graph>>;

    /// Returns a graph view filtered by types, with property selectors, and orientation.
    fn get_graph_with_types_selectors_and_orientation(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        relationship_property_selectors: &HashMap<RelationshipType, String>,
        orientation: Orientation,
    ) -> GraphResult<Arc<dyn Graph>>;
}

/// Base implementation for GraphStore adapters.
///
/// This provides delegation to an underlying GraphStore, useful for
/// creating decorators or proxies that add functionality.
pub struct GraphStoreAdapter<G: GraphStore> {
    graph_store: Arc<G>,
}

impl<G: GraphStore> GraphStoreAdapter<G> {
    /// Creates a new GraphStoreAdapter.
    pub fn new(graph_store: Arc<G>) -> Self {
        Self { graph_store }
    }

    /// Returns a reference to the underlying graph store.
    pub fn inner(&self) -> &G {
        &self.graph_store
    }

    /// Check whether a property key is present anywhere in this store:
    /// - graph-level properties
    /// - node-level properties (any label)
    /// - relationship-level properties (any relationship type)
    pub fn is_property_key_used_anywhere(&self, key: &str) -> bool {
        if self.graph_store.has_graph_property(key) {
            return true;
        }
        if self.graph_store.has_node_property(key) {
            return true;
        }
        self.graph_store.relationship_property_keys().contains(key)
    }

    /// Return where a property key appears:
    /// (is_graph_level, node_labels_with_key, relationship_types_with_key)
    pub fn property_key_locations(
        &self,
        key: &str,
    ) -> (bool, HashSet<NodeLabel>, HashSet<RelationshipType>) {
        let mut node_labels_with = HashSet::new();
        for label in self.graph_store.node_labels() {
            if self.graph_store.has_node_property_for_label(&label, key) {
                node_labels_with.insert(label);
            }
        }

        let mut rel_types_with = HashSet::new();
        for rel_type in self.graph_store.relationship_types() {
            if self.graph_store.has_relationship_property(&rel_type, key) {
                rel_types_with.insert(rel_type);
            }
        }

        (
            self.graph_store.has_graph_property(key),
            node_labels_with,
            rel_types_with,
        )
    }
}

impl<G: GraphStore> GraphStore for GraphStoreAdapter<G> {
    fn database_info(&self) -> &DatabaseInfo {
        self.graph_store.database_info()
    }

    fn schema(&self) -> &GraphSchema {
        self.graph_store.schema()
    }

    fn creation_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.graph_store.creation_time()
    }

    fn modification_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.graph_store.modification_time()
    }

    fn capabilities(&self) -> &Capabilities {
        self.graph_store.capabilities()
    }

    fn nodes(&self) -> Arc<dyn IdMap> {
        self.graph_store.nodes()
    }

    fn graph_property_keys(&self) -> HashSet<String> {
        self.graph_store.graph_property_keys()
    }

    fn has_graph_property(&self, property_key: &str) -> bool {
        self.graph_store.has_graph_property(property_key)
    }

    fn graph_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        self.graph_store.graph_property_type(property_key)
    }

    fn graph_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn GraphPropertyValues>> {
        self.graph_store.graph_property_values(property_key)
    }

    fn add_graph_property(
        &mut self,
        _property_key: impl Into<String>,
        _property_values: Arc<dyn GraphPropertyValues>,
    ) -> GraphStoreResult<()> {
        // Note: This requires Arc::get_mut which won't work with shared ownership
        // Real implementation would need interior mutability (RwLock)
        Err(GraphStoreError::InvalidOperation(
            "Cannot mutate through adapter".to_string(),
        ))
    }

    fn remove_graph_property(&mut self, _property_key: &str) -> GraphStoreResult<()> {
        Err(GraphStoreError::InvalidOperation(
            "Cannot mutate through adapter".to_string(),
        ))
    }

    fn node_count(&self) -> usize {
        self.graph_store.node_count()
    }

    fn node_count_for_label(&self, label: &NodeLabel) -> usize {
        self.graph_store.node_count_for_label(label)
    }

    fn node_labels(&self) -> HashSet<NodeLabel> {
        self.graph_store.node_labels()
    }

    fn has_node_label(&self, label: &NodeLabel) -> bool {
        self.graph_store.has_node_label(label)
    }

    fn add_node_label(&mut self, _node_label: NodeLabel) -> GraphStoreResult<()> {
        Err(GraphStoreError::InvalidOperation(
            "Cannot mutate through adapter".to_string(),
        ))
    }

    fn node_property_keys(&self) -> HashSet<String> {
        self.graph_store.node_property_keys()
    }

    fn node_property_keys_for_label(&self, label: &NodeLabel) -> HashSet<String> {
        self.graph_store.node_property_keys_for_label(label)
    }

    fn node_property_keys_for_labels(&self, labels: &HashSet<NodeLabel>) -> HashSet<String> {
        self.graph_store.node_property_keys_for_labels(labels)
    }

    fn has_node_property(&self, property_key: &str) -> bool {
        self.graph_store.has_node_property(property_key)
    }

    fn has_node_property_for_label(&self, label: &NodeLabel, property_key: &str) -> bool {
        self.graph_store
            .has_node_property_for_label(label, property_key)
    }

    fn node_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        self.graph_store.node_property_type(property_key)
    }

    fn node_property_values(
        &self,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn NodePropertyValues>> {
        self.graph_store.node_property_values(property_key)
    }

    fn add_node_property(
        &mut self,
        _node_labels: HashSet<NodeLabel>,
        _property_key: impl Into<String>,
        _property_values: Arc<dyn NodePropertyValues>,
    ) -> GraphStoreResult<()> {
        Err(GraphStoreError::InvalidOperation(
            "Cannot mutate through adapter".to_string(),
        ))
    }

    fn remove_node_property(&mut self, _property_key: &str) -> GraphStoreResult<()> {
        Err(GraphStoreError::InvalidOperation(
            "Cannot mutate through adapter".to_string(),
        ))
    }

    fn relationship_count(&self) -> usize {
        self.graph_store.relationship_count()
    }

    fn relationship_count_for_type(&self, relationship_type: &RelationshipType) -> usize {
        self.graph_store
            .relationship_count_for_type(relationship_type)
    }

    fn relationship_types(&self) -> HashSet<RelationshipType> {
        self.graph_store.relationship_types()
    }

    fn has_relationship_type(&self, relationship_type: &RelationshipType) -> bool {
        self.graph_store.has_relationship_type(relationship_type)
    }

    fn inverse_indexed_relationship_types(&self) -> HashSet<RelationshipType> {
        self.graph_store.inverse_indexed_relationship_types()
    }

    fn relationship_property_keys(&self) -> HashSet<String> {
        self.graph_store.relationship_property_keys()
    }

    fn relationship_property_keys_for_type(&self, rel_type: &RelationshipType) -> HashSet<String> {
        self.graph_store
            .relationship_property_keys_for_type(rel_type)
    }

    fn relationship_property_keys_for_types(
        &self,
        rel_types: &HashSet<RelationshipType>,
    ) -> HashSet<String> {
        self.graph_store
            .relationship_property_keys_for_types(rel_types)
    }

    fn has_relationship_property(&self, rel_type: &RelationshipType, property_key: &str) -> bool {
        self.graph_store
            .has_relationship_property(rel_type, property_key)
    }

    fn relationship_property_type(&self, property_key: &str) -> GraphStoreResult<ValueType> {
        self.graph_store.relationship_property_type(property_key)
    }

    fn relationship_property_values(
        &self,
        relationship_type: &RelationshipType,
        property_key: &str,
    ) -> GraphStoreResult<Arc<dyn RelationshipPropertyValues>> {
        self.graph_store
            .relationship_property_values(relationship_type, property_key)
    }

    fn add_relationship_property(
        &mut self,
        _relationship_type: RelationshipType,
        _property_key: impl Into<String>,
        _property_values: Arc<dyn RelationshipPropertyValues>,
    ) -> GraphStoreResult<()> {
        Err(GraphStoreError::InvalidOperation(
            "Cannot mutate through adapter".to_string(),
        ))
    }

    fn remove_relationship_property(
        &mut self,
        _relationship_type: &RelationshipType,
        _property_key: &str,
    ) -> GraphStoreResult<()> {
        Err(GraphStoreError::InvalidOperation(
            "Cannot mutate through adapter".to_string(),
        ))
    }

    fn delete_relationships(
        &mut self,
        _relationship_type: &RelationshipType,
    ) -> GraphStoreResult<DeletionResult> {
        Err(GraphStoreError::InvalidOperation(
            "Cannot mutate through adapter".to_string(),
        ))
    }

    fn get_graph(&self) -> Arc<dyn Graph> {
        self.graph_store.get_graph()
    }

    fn get_graph_with_types(
        &self,
        relationship_types: &HashSet<RelationshipType>,
    ) -> GraphResult<Arc<dyn Graph>> {
        self.graph_store.get_graph_with_types(relationship_types)
    }

    fn get_graph_with_types_and_selectors(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        relationship_property_selectors: &HashMap<RelationshipType, String>,
    ) -> GraphResult<Arc<dyn Graph>> {
        self.graph_store.get_graph_with_types_and_selectors(
            relationship_types,
            relationship_property_selectors,
        )
    }

    fn get_graph_with_types_and_orientation(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        orientation: Orientation,
    ) -> GraphResult<Arc<dyn Graph>> {
        self.graph_store
            .get_graph_with_types_and_orientation(relationship_types, orientation)
    }

    fn get_graph_with_types_selectors_and_orientation(
        &self,
        relationship_types: &HashSet<RelationshipType>,
        relationship_property_selectors: &HashMap<RelationshipType, String>,
        orientation: Orientation,
    ) -> GraphResult<Arc<dyn Graph>> {
        self.graph_store.get_graph_with_types_selectors_and_orientation(
            relationship_types,
            relationship_property_selectors,
            orientation,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph_store::default_graph_store::DefaultGraphStore;

    use crate::types::graph::id_map::SimpleIdMap;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph_store::{DatabaseId, DatabaseLocation, GraphName};
    use crate::types::properties::relationship::DefaultRelationshipPropertyValues;
    use std::collections::HashMap;
    use std::sync::Arc;

    fn make_sample_store() -> DefaultGraphStore {
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
    fn graph_store_adapter_reflects_relationship_property_usage() {
        let mut store = make_sample_store();

        let rel_type = RelationshipType::of("KNOWS");
        let values = Arc::new(DefaultRelationshipPropertyValues::with_default(
            vec![1.0, 2.0, 3.0],
            3,
        ));

        // add a relationship property and verify adapter finds it
        store
            .add_relationship_property(rel_type.clone(), "weight", values)
            .expect("add relationship property");

        let adapter = GraphStoreAdapter::new(Arc::new(store));

        // relationship-level key should be detected
        assert!(adapter.is_property_key_used_anywhere("weight"));

        // absent key should not be detected anywhere
        assert!(!adapter.is_property_key_used_anywhere("absent_key"));

        // locations should report that the key appears under the KNOWS relationship type
        let (is_graph, node_labels, rel_types) = adapter.property_key_locations("weight");
        assert!(!is_graph);
        assert!(node_labels.is_empty());
        assert!(rel_types.contains(&RelationshipType::of("KNOWS")));
    }

    #[test]
    fn test_graph_store_error_variants() {
        let error = GraphStoreError::NodeLabelNotFound("Person".to_string());
        assert!(error.to_string().contains("Person"));

        let error = GraphStoreError::RelationshipTypeNotFound("KNOWS".to_string());
        assert!(error.to_string().contains("KNOWS"));

        let error = GraphStoreError::PropertyNotFound("age".to_string());
        assert!(error.to_string().contains("age"));
    }
}
