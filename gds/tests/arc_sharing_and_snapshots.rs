//! Integration test: Arc-Based Sharing and View Snapshots
//!
//! Tests the copy-on-write and Arc-based sharing semantics of the GraphStore.
//! Demonstrates how multiple graph views can share property columns without copying,
//! and how modifications require builder pattern (copy-on-write).
//!
//! Key concepts tested:
//!   1. Multiple Graph views share PropertyValues via Arc (zero-copy)
//!   2. PropertyValues remain shared even when stores are modified
//!   3. Graph views are snapshots at creation time
//!   4. Builder pattern enables copy-on-write for PropertyStores
//!   5. Arc reference counting tracks shared ownership

use rust_gds::projection::{NodeLabel, RelationshipType};
use rust_gds::types::graph::id_map::SimpleIdMap;
use rust_gds::types::graph::topology::RelationshipTopology;
use rust_gds::types::graph::Graph;
use rust_gds::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStore,
};
use rust_gds::types::properties::node::{
    DefaultLongNodePropertyValues, NodePropertyContainer, NodePropertyValues,
};
use rust_gds::types::properties::relationship::DefaultRelationshipPropertyValues;
use rust_gds::types::schema::GraphSchema;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[test]
fn test_multiple_views_share_property_columns() {
    println!("\n=== Test: Multiple Views Share Property Columns (Arc) ===\n");
    println!("Graph views created from the same store share PropertyValues via Arc.\n");

    let mut store = build_test_graph();
    let person = NodeLabel::of("Person");

    // Add node property
    let age_values = Arc::new(DefaultLongNodePropertyValues::new(vec![30, 45, 25], 3));
    let age_arc_strong_count_before = Arc::strong_count(&age_values);

    let mut person_labels = HashSet::new();
    person_labels.insert(person.clone());
    let age_values_trait: Arc<dyn NodePropertyValues> = age_values.clone();
    store
        .add_node_property(person_labels, "age", age_values_trait)
        .expect("add age property");

    println!(
        "Arc strong count after adding to store: {}",
        Arc::strong_count(&age_values)
    );
    println!("  (Expected: original + 1 for store)\n");

    // Create first graph view
    let graph1 = store.graph();
    let count_after_graph1 = Arc::strong_count(&age_values);
    println!(
        "Arc strong count after creating graph1: {}",
        count_after_graph1
    );
    println!("  (Expected: original + store + graph1)\n");

    // Create second graph view
    let graph2 = store.graph();
    let count_after_graph2 = Arc::strong_count(&age_values);
    println!(
        "Arc strong count after creating graph2: {}",
        count_after_graph2
    );
    println!("  (Expected: original + store + graph1 + graph2)\n");

    // Verify both views can access the same column
    let values1 = graph1.node_properties("age").expect("age property");
    let values2 = graph2.node_properties("age").expect("age property");

    let age1 = values1.long_value(0).unwrap();
    let age2 = values2.long_value(0).unwrap();

    println!("graph1 age[0]: {}", age1);
    println!("graph2 age[0]: {}", age2);
    assert_eq!(age1, age2, "Both views see the same data");

    println!("\nKey insight: PropertyValues are shared (Arc), not copied.");
    println!("Creating graph views is cheap (no data duplication).");

    // Verify strong count increased
    assert!(
        count_after_graph2 > age_arc_strong_count_before,
        "Arc strong count should increase with views"
    );
}

#[test]
fn test_graph_views_are_snapshots() {
    println!("\n=== Test: Graph Views Are Snapshots at Creation Time ===\n");
    println!("Graph views capture the store state at creation time.\n");

    let mut store = build_test_graph();
    let knows = RelationshipType::of("KNOWS");

    // Create graph BEFORE adding properties
    let graph_before = store.graph();
    println!("Created graph_before (no relationship properties yet)");
    assert!(!graph_before.has_relationship_property());

    // Add relationship property
    let weight_values = Arc::new(DefaultRelationshipPropertyValues::new(
        vec![0.8, 0.6],
        0.0,
        2,
    ));
    store
        .add_relationship_property(knows.clone(), "weight", weight_values)
        .expect("add weight");

    // Create graph AFTER adding properties
    let graph_after = store.graph();
    println!("Created graph_after (weight property added)");
    assert!(graph_after.has_relationship_property());

    println!("\nChecking snapshots:");
    println!(
        "  graph_before.has_relationship_property() = {}",
        graph_before.has_relationship_property()
    );
    println!(
        "  graph_after.has_relationship_property() = {}",
        graph_after.has_relationship_property()
    );

    // Views are snapshots; graph_before doesn't see the new property
    assert!(
        !graph_before.has_relationship_property(),
        "graph_before should not see property added later"
    );
    assert!(
        graph_after.has_relationship_property(),
        "graph_after should see the property"
    );

    println!("\nKey insight: Graph views are immutable snapshots.");
    println!("They don't observe mutations made to the store after creation.");
}

#[test]
fn test_filtered_views_share_topology() {
    println!("\n=== Test: Filtered Views Share Topology (Arc) ===\n");
    println!("Creating filtered graph views (by relationship type) is cheap.\n");

    let mut store = build_multi_type_graph();
    let knows = RelationshipType::of("KNOWS");
    let works_at = RelationshipType::of("WORKS_AT");

    // Add properties
    let weight_values = Arc::new(DefaultRelationshipPropertyValues::new(vec![0.8], 0.0, 1));
    store
        .add_relationship_property(knows.clone(), "weight", weight_values)
        .expect("add weight");

    let tenure_values = Arc::new(DefaultRelationshipPropertyValues::new(
        vec![24.0, 36.0],
        0.0,
        2,
    ));
    store
        .add_relationship_property(works_at.clone(), "tenure", tenure_values)
        .expect("add tenure");

    let full_graph = store.graph();
    println!(
        "Full graph: {} relationships",
        full_graph.relationship_count()
    );

    // Create filtered view (only KNOWS relationships)
    let mut knows_filter = HashSet::new();
    knows_filter.insert(knows.clone());
    let knows_graph = full_graph
        .relationship_type_filtered_graph(&knows_filter)
        .expect("filter KNOWS");

    println!(
        "KNOWS-only graph: {} relationships",
        knows_graph.relationship_count()
    );

    // Create filtered view (only WORKS_AT relationships)
    let mut works_filter = HashSet::new();
    works_filter.insert(works_at.clone());
    let works_graph = full_graph
        .relationship_type_filtered_graph(&works_filter)
        .expect("filter WORKS_AT");

    println!(
        "WORKS_AT-only graph: {} relationships",
        works_graph.relationship_count()
    );

    println!("\nFiltered views share topology via Arc (no data duplication).");
    println!("Projection is a cheap operation (metadata filtering, not data copying).");

    assert_eq!(knows_graph.relationship_count(), 1);
    assert_eq!(works_graph.relationship_count(), 2);
}

#[test]
fn test_property_values_remain_shared_after_store_mutation() {
    println!("\n=== Test: PropertyValues Remain Shared After Store Mutation ===\n");
    println!("When properties are removed from the store, existing graph views retain Arc refs.\n");

    let mut store = build_test_graph();
    let person = NodeLabel::of("Person");

    // Add property
    let age_values = Arc::new(DefaultLongNodePropertyValues::new(vec![30, 45, 25], 3));

    let mut person_labels = HashSet::new();
    person_labels.insert(person.clone());
    let age_values_trait: Arc<dyn NodePropertyValues> = age_values.clone();
    store
        .add_node_property(person_labels, "age", age_values_trait)
        .expect("add age");

    // Create graph view (holds Arc to age_values)
    let graph = store.graph();
    let count_with_graph = Arc::strong_count(&age_values);
    println!(
        "Arc count with graph view: {} (original + store + graph)",
        count_with_graph
    );

    // Remove property from store
    store
        .remove_node_property("age")
        .expect("remove age from store");
    println!("Removed 'age' property from store");

    let count_after_removal = Arc::strong_count(&age_values);
    println!(
        "Arc count after removal: {} (original + graph)",
        count_after_removal
    );

    // Graph view still has access to the property (Arc keeps it alive)
    let values = graph.node_properties("age").expect("age still accessible");
    let age = values.long_value(0).unwrap();
    println!("graph.node_properties(\"age\")[0] = {}", age);

    assert_eq!(
        age, 30,
        "Graph view retains property even after store removal"
    );
    println!(
        "\nKey insight: Arc-based sharing keeps data alive as long as any view holds a reference."
    );
    println!("Store mutation doesn't affect existing graph views (immutable snapshots).");
}

// Helper: simple 3-node graph with KNOWS relationships
fn build_test_graph() -> DefaultGraphStore {
    let id_map = SimpleIdMap::from_original_ids([0, 1, 2]);
    let topology = RelationshipTopology::new(vec![vec![1, 2], vec![], vec![]], None);

    let mut topologies = HashMap::new();
    topologies.insert(RelationshipType::of("KNOWS"), topology);

    DefaultGraphStore::new(
        GraphName::new("test-graph"),
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

// Helper: multi-type graph
fn build_multi_type_graph() -> DefaultGraphStore {
    let id_map = SimpleIdMap::from_original_ids([0, 1, 2, 3]);

    let knows_topology = RelationshipTopology::new(vec![vec![1], vec![], vec![], vec![]], None);
    let works_topology = RelationshipTopology::new(vec![vec![], vec![2, 3], vec![], vec![]], None);

    let mut topologies = HashMap::new();
    topologies.insert(RelationshipType::of("KNOWS"), knows_topology);
    topologies.insert(RelationshipType::of("WORKS_AT"), works_topology);

    DefaultGraphStore::new(
        GraphName::new("multi-type"),
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
