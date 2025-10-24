//! Integration test: Property Selection and Fallback Semantics
//!
//! Tests the behavior documented in ADR 0005: how relationship properties are
//! selected for traversal when multiple properties exist, and how fallback
//! values are used when properties are missing or not selected.
//!
//! Key scenarios:
//!   1. Single property auto-selection
//!   2. Multiple properties with no selector (no auto-select)
//!   3. Explicit selector pointing to valid property
//!   4. Fallback value usage when property is unselected
//!   5. Property selection per relationship type

use gds::projection::RelationshipType;
use gds::types::graph::id_map::SimpleIdMap;
use gds::types::graph::topology::RelationshipTopology;
use gds::types::graph::Graph;
use gds::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStore,
};
use gds::types::properties::relationship::{
    DefaultRelationshipPropertyValues, RelationshipIterator,
};
use gds::types::schema::GraphSchema;
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn test_single_property_auto_selection() {
    println!("\n=== Test: Single Property Auto-Selection ===\n");
    println!("When a relationship type has exactly ONE property, it should be auto-selected.\n");

    let mut store = build_simple_graph();
    let knows = RelationshipType::of("KNOWS");

    // Add single property
    let weight_values = Arc::new(DefaultRelationshipPropertyValues::new(
        vec![0.8, 0.6, 0.9],
        0.0,
        3,
    ));
    store
        .add_relationship_property(knows.clone(), "weight", weight_values)
        .expect("add weight");

    let graph = store.graph();

    println!("Store has 1 property for KNOWS: 'weight'");
    println!("Expected: auto-select 'weight' for traversal\n");

    // Traverse with fallback; auto-selected property should provide actual values
    const FALLBACK: f64 = -1.0f64;
    let mut values_seen = Vec::new();
    for cursor in graph.stream_relationships(0, FALLBACK) {
        values_seen.push(cursor.property());
    }

    println!("Values seen during traversal: {:?}", values_seen);
    println!("Fallback value: {}", FALLBACK);

    // With auto-selection, we expect actual property values, not fallback
    assert!(
        values_seen.iter().any(|&v| v != FALLBACK),
        "Auto-selected property should provide actual values"
    );
    assert!(graph.has_relationship_property());
}

#[test]
fn test_multiple_properties_no_selector() {
    println!("\n=== Test: Multiple Properties, No Selector ===\n");
    println!("When multiple properties exist and no selector is provided,");
    println!("NO property should be auto-selected (conservative policy).\n");

    let mut store = build_simple_graph();
    let knows = RelationshipType::of("KNOWS");

    // Add TWO properties
    let weight_values = Arc::new(DefaultRelationshipPropertyValues::new(
        vec![0.8, 0.6, 0.9],
        0.0,
        3,
    ));
    let capacity_values = Arc::new(DefaultRelationshipPropertyValues::new(
        vec![100.0, 50.0, 75.0],
        0.0,
        3,
    ));

    store
        .add_relationship_property(knows.clone(), "weight", weight_values)
        .expect("add weight");
    store
        .add_relationship_property(knows.clone(), "capacity", capacity_values)
        .expect("add capacity");

    let graph = store.graph();

    println!("Store has 2 properties for KNOWS: 'weight', 'capacity'");
    println!("Expected: NO auto-selection; fallback values should be returned\n");

    // Traverse with fallback; no selection means fallback should be used
    const FALLBACK: f64 = -999.0f64;
    let mut values_seen = Vec::new();
    for cursor in graph.stream_relationships(0, FALLBACK) {
        values_seen.push(cursor.property());
    }

    println!("Values seen during traversal: {:?}", values_seen);
    println!("Fallback value: {}", FALLBACK);

    // Without explicit selection, all values should be fallback
    // (This is the conservative behavior from ADR 0005)
    for &value in &values_seen {
        assert_eq!(
            value, FALLBACK,
            "With multiple properties and no selector, fallback should be used"
        );
    }
}

#[test]
fn test_property_selection_per_relationship_type() {
    println!("\n=== Test: Property Selection Per Relationship Type ===\n");
    println!("Different relationship types can have different properties selected.\n");

    let mut store = build_multi_type_graph();
    let knows = RelationshipType::of("KNOWS");
    let works_at = RelationshipType::of("WORKS_AT");

    // Add properties to each type
    let weight_values = Arc::new(DefaultRelationshipPropertyValues::new(vec![0.8], 0.0, 1));
    store
        .add_relationship_property(knows.clone(), "weight", weight_values)
        .expect("add weight to KNOWS");

    let tenure_values = Arc::new(DefaultRelationshipPropertyValues::new(
        vec![24.0, 36.0],
        0.0,
        2,
    ));
    store
        .add_relationship_property(works_at.clone(), "tenure_months", tenure_values)
        .expect("add tenure to WORKS_AT");

    let graph = store.graph();

    println!("KNOWS has 1 property: 'weight' (auto-selected)");
    println!("WORKS_AT has 1 property: 'tenure_months' (auto-selected)\n");

    const FALLBACK: f64 = -1.0f64;

    // Traverse KNOWS relationships (node 0 -> 1)
    let mut knows_values = Vec::new();
    for cursor in graph.stream_relationships(0, FALLBACK) {
        knows_values.push(cursor.property());
    }
    println!("KNOWS traversal values: {:?}", knows_values);

    // Traverse WORKS_AT relationships (node 0 -> 2, 3)
    let mut works_values = Vec::new();
    for cursor in graph.stream_relationships(1, FALLBACK) {
        works_values.push(cursor.property());
    }
    println!("WORKS_AT traversal values: {:?}", works_values);

    // Both should have auto-selected properties (not fallback)
    assert!(knows_values.iter().any(|&v| v != FALLBACK));
    assert!(works_values.iter().any(|&v| v != FALLBACK));
}

#[test]
fn test_fallback_when_property_missing() {
    println!("\n=== Test: Fallback When Property Missing ===\n");
    println!("When a relationship type has NO properties, fallback is used.\n");

    let store = build_simple_graph(); // No properties added
    let graph = store.graph();

    const FALLBACK: f64 = 42.0f64;
    let mut values_seen = Vec::new();
    for cursor in graph.stream_relationships(0, FALLBACK) {
        values_seen.push(cursor.property());
    }

    println!("No properties attached; fallback = {}", FALLBACK);
    println!("Values seen: {:?}", values_seen);

    for &value in &values_seen {
        assert_eq!(
            value, FALLBACK,
            "When no property exists, fallback should be used"
        );
    }

    assert!(!graph.has_relationship_property());
}

// Helper: simple graph with 3 nodes, 2 edges (0->1, 0->2)
fn build_simple_graph() -> DefaultGraphStore {
    let id_map = SimpleIdMap::from_original_ids([0, 1, 2]);
    let topology = RelationshipTopology::new(vec![vec![1, 2], vec![], vec![]], None);

    let mut topologies = HashMap::new();
    topologies.insert(RelationshipType::of("KNOWS"), topology);

    DefaultGraphStore::new(
        GraphName::new("simple-test"),
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

// Helper: graph with multiple relationship types
fn build_multi_type_graph() -> DefaultGraphStore {
    let id_map = SimpleIdMap::from_original_ids([0, 1, 2, 3]);

    // 0->1 (KNOWS), 1->2 (WORKS_AT), 1->3 (WORKS_AT)
    let knows_topology = RelationshipTopology::new(vec![vec![1], vec![], vec![], vec![]], None);
    let works_topology = RelationshipTopology::new(vec![vec![], vec![2, 3], vec![], vec![]], None);

    let mut topologies = HashMap::new();
    topologies.insert(RelationshipType::of("KNOWS"), knows_topology);
    topologies.insert(RelationshipType::of("WORKS_AT"), works_topology);

    DefaultGraphStore::new(
        GraphName::new("multi-type-test"),
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
