//! Integration test: GraphStore as Projected Data Container
//!
//! Tests layer 4: GraphStore/Graph as the container that holds projected data.
//! This explores the boundary between:
//!   - Layer 4: GraphStore as Container (what it *is*)
//!   - Layer 5: Projection in Execution (what you can *do* with it)
//!
//! Key distinctions:
//!   - GraphStore = Mutable container managing schema, topology, properties
//!   - Graph = Immutable view exposing the contained structure
//!   - ResultStore = GraphStore in the context of IO/Writing (output of pipeline)
//!
//! Tests focus on:
//!   1. GraphStore as schema holder (what labels/types exist)
//!   2. GraphStore as topology container (adjacency structure)
//!   3. GraphStore as property manager (column orchestration)
//!   4. Graph as immutable projection of container contents
//!   5. ResultStore semantics (GraphStore as pipeline output)

use rust_gds::projection::{NodeLabel, RelationshipType};
use rust_gds::types::graph::id_map::{IdMap, SimpleIdMap};
use rust_gds::types::graph::topology::RelationshipTopology;
use rust_gds::types::graph::Graph;
use rust_gds::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStore,
};
use rust_gds::types::properties::graph::DefaultDoubleGraphPropertyValues;
use rust_gds::types::properties::node::DefaultLongNodePropertyValues;
use rust_gds::types::properties::relationship::DefaultRelationshipPropertyValues;
use rust_gds::types::schema::GraphSchema;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[test]
fn test_graphstore_as_schema_holder() {
    println!("\n=== Test: GraphStore as Schema Holder ===\n");
    println!("GraphStore contains the schema metadata: what labels/types are possible.\n");

    let mut store = build_empty_store();

    println!("Initial state:");
    println!("  Node labels: {:?}", store.node_labels());
    println!("  Relationship types: {:?}", store.relationship_types());
    assert_eq!(store.node_labels().len(), 0);
    assert_eq!(store.relationship_types().len(), 0);

    // Add labels and types to the container
    let person = NodeLabel::of("Person");
    let company = NodeLabel::of("Company");

    store
        .add_node_label(person.clone())
        .expect("add Person label");
    store
        .add_node_label(company.clone())
        .expect("add Company label");

    println!("\nAfter adding labels:");
    println!("  Node labels: {:?}", store.node_labels());
    assert_eq!(store.node_labels().len(), 2);
    assert!(store.has_node_label(&person));
    assert!(store.has_node_label(&company));

    println!("\nKey insight: GraphStore *is* the schema container.");
    println!("It tracks what categories (Labels/Types) exist in the projected graph.");
}

#[test]
fn test_graphstore_as_topology_container() {
    println!("\n=== Test: GraphStore as Topology Container ===\n");
    println!("GraphStore holds the adjacency structure (who connects to whom).\n");

    let store = build_graph_with_topology();

    println!("Topology inspection:");
    println!("  Total nodes: {}", store.node_count());
    println!("  Total relationships: {}", store.relationship_count());

    let knows = RelationshipType::of("KNOWS");
    let works_at = RelationshipType::of("WORKS_AT");

    println!(
        "  KNOWS relationships: {}",
        store.relationship_count_for_type(&knows)
    );
    println!(
        "  WORKS_AT relationships: {}",
        store.relationship_count_for_type(&works_at)
    );

    assert_eq!(store.node_count(), 4);
    assert_eq!(store.relationship_count(), 3);
    assert_eq!(store.relationship_count_for_type(&knows), 1);
    assert_eq!(store.relationship_count_for_type(&works_at), 2);

    println!("\nKey insight: GraphStore manages topology per relationship type.");
    println!("It's a multi-graph container: each type has its own adjacency structure.");
}

#[test]
fn test_graphstore_as_property_manager() {
    println!("\n=== Test: GraphStore as Property Manager ===\n");
    println!("GraphStore orchestrates property columns across graph/node/relationship levels.\n");

    let mut store = build_empty_store();
    let person = NodeLabel::of("Person");

    println!("Initial property state:");
    println!("  Graph properties: {:?}", store.graph_property_keys());
    println!("  Node properties: {:?}", store.node_property_keys());
    println!(
        "  Relationship properties: {:?}",
        store.relationship_property_keys()
    );

    // Add properties at each level
    let density = Arc::new(DefaultDoubleGraphPropertyValues::singleton(0.3));
    store
        .add_graph_property("edge_density", density)
        .expect("add graph property");

    let age_values = Arc::new(DefaultLongNodePropertyValues::new(vec![30, 45, 25, 50], 4));
    let mut person_labels = HashSet::new();
    person_labels.insert(person.clone());
    store
        .add_node_property(person_labels, "age", age_values)
        .expect("add node property");

    let knows = RelationshipType::of("KNOWS");
    let weight_values = Arc::new(DefaultRelationshipPropertyValues::new(vec![0.8], 0.0, 1));
    store
        .add_relationship_property(knows.clone(), "weight", weight_values)
        .expect("add relationship property");

    println!("\nAfter adding properties:");
    println!("  Graph properties: {:?}", store.graph_property_keys());
    println!("  Node properties: {:?}", store.node_property_keys());
    println!(
        "  Relationship properties: {:?}",
        store.relationship_property_keys()
    );

    assert_eq!(store.graph_property_keys().len(), 1);
    assert_eq!(store.node_property_keys().len(), 1);
    assert_eq!(store.relationship_property_keys().len(), 1);

    println!("\nKey insight: GraphStore is a triadic property orchestrator.");
    println!("It manages three separate property namespaces (graph/node/relationship).");
}

#[test]
fn test_graph_as_immutable_projection_of_store() {
    println!("\n=== Test: Graph as Immutable Projection of GraphStore ===\n");
    println!("Graph views expose the container's contents without mutation capability.\n");

    let mut store = build_graph_with_topology();
    let person = NodeLabel::of("Person");

    // Add properties to store
    let age_values = Arc::new(DefaultLongNodePropertyValues::new(vec![30, 45, 25, 50], 4));
    let mut person_labels = HashSet::new();
    person_labels.insert(person.clone());
    store
        .add_node_property(person_labels, "age", age_values)
        .expect("add age");

    // Create graph view
    let graph = store.graph();

    println!("Graph exposes container contents:");
    println!("  graph.node_count() = {}", graph.node_count());
    println!(
        "  graph.relationship_count() = {}",
        graph.relationship_count()
    );
    println!("  graph.characteristics() = {:?}", graph.characteristics());

    // Graph provides read-only access
    assert_eq!(graph.node_count(), store.node_count());
    assert_eq!(graph.relationship_count(), store.relationship_count());

    println!("\nGraph cannot mutate container:");
    println!("  - No graph.add_node_property() method");
    println!("  - No graph.add_relationship() method");
    println!("  - Graph = immutable lens over GraphStore contents");

    println!("\nKey insight: Graph is the *projection* (in the geometric sense).");
    println!("It's a read-only 2D shadow of the 3D GraphStore container.");
}

#[test]
fn test_graphstore_as_resultstore_semantics() {
    println!("\n=== Test: GraphStore as ResultStore (Pipeline Output) ===\n");
    println!("GraphStore can serve as the output of a projection pipeline.\n");

    // Simulate a pipeline: Source → Transform → ResultStore
    let source_store = build_source_graph();
    println!("Source GraphStore:");
    println!("  Nodes: {}", source_store.node_count());
    println!("  Relationships: {}", source_store.relationship_count());
    println!("  Node labels: {:?}", source_store.node_labels());

    // Transform: filter by relationship type (projection-like operation)
    let knows = RelationshipType::of("KNOWS");
    let source_graph = source_store.graph();
    let mut filter_set = HashSet::new();
    filter_set.insert(knows.clone());

    let filtered_graph = source_graph
        .relationship_type_filtered_graph(&filter_set)
        .expect("filter graph");

    println!("\nFiltered Graph (projection result):");
    println!("  Relationships: {}", filtered_graph.relationship_count());

    // In a full pipeline, we'd construct a new GraphStore from filtered_graph
    // That new store would be the "ResultStore"
    println!("\nResultStore semantics:");
    println!("  - ResultStore = GraphStore produced by a pipeline");
    println!("  - It's a container like any other GraphStore");
    println!("  - Can be written to disk, passed to algorithms, or queried");
    println!("  - The *context* (pipeline output) adds meaning, not the type");

    println!("\nKey insight: GraphStore is polymorphic in context.");
    println!("Source, intermediate, and result stores are all GraphStore instances.");
    println!("The IO/Writing system determines which role a store plays.");
}

#[test]
fn test_container_metadata_and_provenance() {
    println!("\n=== Test: Container Metadata and Provenance ===\n");
    println!("GraphStore tracks metadata about the contained graph.\n");

    let store = build_graph_with_metadata();

    println!("Metadata inspection:");
    println!("  Graph name: {}", store.database_info().database_id());
    println!("  Creation time: {}", store.creation_time());
    println!("  Modification time: {}", store.modification_time());
    println!("  Capabilities: {:?}", store.capabilities());

    println!("\nSchema metadata:");
    println!("  Direction: {:?}", store.schema().direction());
    println!("  Node labels: {:?}", store.node_labels());
    println!("  Relationship types: {:?}", store.relationship_types());

    println!("\nKey insight: GraphStore is more than data — it's data + context.");
    println!("Metadata enables provenance tracking through projection pipelines.");
}

// Helpers

fn build_empty_store() -> DefaultGraphStore {
    let id_map = SimpleIdMap::from_original_ids([0, 1, 2, 3]);
    let topologies = HashMap::new();

    DefaultGraphStore::new(
        GraphName::new("empty-test"),
        DatabaseInfo::new(
            DatabaseId::new("test"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        ),
        GraphSchema::empty(),
        Capabilities::default(),
        id_map,
        topologies,
    )
}

fn build_graph_with_topology() -> DefaultGraphStore {
    let id_map = SimpleIdMap::from_original_ids([0, 1, 2, 3]);

    let knows_topology = RelationshipTopology::new(vec![vec![1], vec![], vec![], vec![]], None);
    let works_topology = RelationshipTopology::new(vec![vec![], vec![2, 3], vec![], vec![]], None);

    let mut topologies = HashMap::new();
    topologies.insert(RelationshipType::of("KNOWS"), knows_topology);
    topologies.insert(RelationshipType::of("WORKS_AT"), works_topology);

    DefaultGraphStore::new(
        GraphName::new("topology-test"),
        DatabaseInfo::new(
            DatabaseId::new("test"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        ),
        GraphSchema::empty(),
        Capabilities::default(),
        id_map,
        topologies,
    )
}

fn build_source_graph() -> DefaultGraphStore {
    build_graph_with_topology()
}

fn build_graph_with_metadata() -> DefaultGraphStore {
    let mut store = build_graph_with_topology();
    store.add_node_label(NodeLabel::of("Person")).ok();
    store.add_node_label(NodeLabel::of("Company")).ok();
    store
}
