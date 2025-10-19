//! Integration test: Property Key Distribution and Label/Type Projection
//!
//! This test demonstrates how PropertyValues (columns) are wrapped in Properties
//! and how property keys are organized and distributed across:
//!   1. Top-level GraphStore (all keys)
//!   2. NodeLabel-scoped keys (label-specific properties)
//!   3. RelationshipType-scoped keys (type-specific properties)
//!
//! Key architectural insight:
//!   - PropertyValues = the column (data)
//!   - Property = PropertyValues + schema metadata (key, type, default, state)
//!   - PropertyStore = HashMap<String, Property>
//!   - GraphStore manages multiple PropertyStores and exposes key projections

use gds::projection::{NodeLabel, RelationshipType};
use gds::types::graph::id_map::{IdMap, SimpleIdMap};
use gds::types::graph::topology::RelationshipTopology;
use gds::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStore,
};
use gds::types::properties::graph::DefaultDoubleGraphPropertyValues;
use gds::types::properties::node::DefaultLongNodePropertyValues;
use gds::types::properties::relationship::DefaultRelationshipPropertyValues;
use gds::types::schema::GraphSchema;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[test]
fn test_property_key_distribution_across_labels_and_types() {
    println!("\n=== Property Key Distribution Test ===\n");

    // Build a multi-label, multi-type graph with various properties
    let mut store = build_multi_label_graph();

    // --- Part 1: Node properties distributed by label ---
    println!("Part 1: Node Property Key Distribution by Label\n");

    let person = NodeLabel::of("Person");
    let company = NodeLabel::of("Company");
    let city = NodeLabel::of("City");

    // Add node properties with different label scopes
    let age_values = Arc::new(DefaultLongNodePropertyValues::new(vec![30, 45, 0, 0, 0], 5));
    let mut person_labels = HashSet::new();
    person_labels.insert(person.clone());
    store
        .add_node_property(person_labels, "age", age_values)
        .expect("add age property");

    let salary_values = Arc::new(DefaultLongNodePropertyValues::new(
        vec![75000, 120000, 0, 0, 0],
        5,
    ));
    let mut person_company_labels = HashSet::new();
    person_company_labels.insert(person.clone());
    person_company_labels.insert(company.clone());
    store
        .add_node_property(person_company_labels, "salary", salary_values)
        .expect("add salary property");

    let population_values = Arc::new(DefaultLongNodePropertyValues::new(
        vec![0, 0, 0, 0, 500000],
        5,
    ));
    let mut city_labels = HashSet::new();
    city_labels.insert(city.clone());
    store
        .add_node_property(city_labels, "population", population_values)
        .expect("add population property");

    // Inspect top-level keys
    let all_node_keys = store.node_property_keys();
    println!("Top-level node property keys (GraphStore view):");
    println!("  {:?}", all_node_keys);
    assert_eq!(all_node_keys.len(), 3);
    assert!(all_node_keys.contains("age"));
    assert!(all_node_keys.contains("salary"));
    assert!(all_node_keys.contains("population"));

    // Inspect label-scoped keys
    let person_keys = store.node_property_keys_for_label(&person);
    println!("\nPerson label keys:");
    println!("  {:?}", person_keys);
    assert_eq!(person_keys.len(), 2);
    assert!(person_keys.contains("age"));
    assert!(person_keys.contains("salary"));

    let company_keys = store.node_property_keys_for_label(&company);
    println!("\nCompany label keys:");
    println!("  {:?}", company_keys);
    assert_eq!(company_keys.len(), 1);
    assert!(company_keys.contains("salary"));

    let city_keys = store.node_property_keys_for_label(&city);
    println!("\nCity label keys:");
    println!("  {:?}", city_keys);
    assert_eq!(city_keys.len(), 1);
    assert!(city_keys.contains("population"));

    // Test label intersection (keys present on ALL specified labels)
    let mut intersection_labels = HashSet::new();
    intersection_labels.insert(person.clone());
    intersection_labels.insert(company.clone());
    let intersection_keys = store.node_property_keys_for_labels(&intersection_labels);
    println!("\nKeys present on BOTH Person AND Company:");
    println!("  {:?}", intersection_keys);
    assert_eq!(intersection_keys.len(), 1);
    assert!(intersection_keys.contains("salary"));

    // --- Part 2: Relationship properties distributed by type ---
    println!("\n\nPart 2: Relationship Property Key Distribution by Type\n");

    let works_at = RelationshipType::of("WORKS_AT");
    let knows = RelationshipType::of("KNOWS");

    // Add relationship properties with type-specific scope
    let tenure_values = Arc::new(DefaultRelationshipPropertyValues::new(
        vec![24.0, 36.0],
        0.0,
        2,
    ));
    store
        .add_relationship_property(works_at.clone(), "tenure_months", tenure_values)
        .expect("add tenure property");

    let weight_values = Arc::new(DefaultRelationshipPropertyValues::new(vec![0.8], 0.0, 1));
    store
        .add_relationship_property(knows.clone(), "weight", weight_values)
        .expect("add weight property");

    let confidence_values = Arc::new(DefaultRelationshipPropertyValues::new(
        vec![0.95, 0.88],
        0.0,
        2,
    ));
    store
        .add_relationship_property(works_at.clone(), "confidence", confidence_values)
        .expect("add confidence property");

    // Inspect top-level relationship keys
    let all_rel_keys = store.relationship_property_keys();
    println!("Top-level relationship property keys (GraphStore view):");
    println!("  {:?}", all_rel_keys);
    assert_eq!(all_rel_keys.len(), 3);
    assert!(all_rel_keys.contains("tenure_months"));
    assert!(all_rel_keys.contains("weight"));
    assert!(all_rel_keys.contains("confidence"));

    // Inspect type-scoped keys
    let works_at_keys = store.relationship_property_keys_for_type(&works_at);
    println!("\nWORKS_AT type keys:");
    println!("  {:?}", works_at_keys);
    assert_eq!(works_at_keys.len(), 2);
    assert!(works_at_keys.contains("tenure_months"));
    assert!(works_at_keys.contains("confidence"));

    let knows_keys = store.relationship_property_keys_for_type(&knows);
    println!("\nKNOWS type keys:");
    println!("  {:?}", knows_keys);
    assert_eq!(knows_keys.len(), 1);
    assert!(knows_keys.contains("weight"));

    // Test type union (keys present on ANY specified type)
    let mut type_set = HashSet::new();
    type_set.insert(works_at.clone());
    type_set.insert(knows.clone());
    let union_keys = store.relationship_property_keys_for_types(&type_set);
    println!("\nKeys present on WORKS_AT OR KNOWS:");
    println!("  {:?}", union_keys);
    assert_eq!(union_keys.len(), 3);

    // --- Part 3: Graph-level properties (no label/type scope) ---
    println!("\n\nPart 3: Graph-Level Property Keys\n");

    let density_values = Arc::new(DefaultDoubleGraphPropertyValues::singleton(0.3));
    store
        .add_graph_property("edge_density", density_values)
        .expect("add graph property");

    let component_values = Arc::new(DefaultDoubleGraphPropertyValues::singleton(2.0));
    store
        .add_graph_property("component_count", component_values)
        .expect("add graph property");

    let graph_keys = store.graph_property_keys();
    println!("Graph property keys (global, no label/type scope):");
    println!("  {:?}", graph_keys);
    assert_eq!(graph_keys.len(), 2);
    assert!(graph_keys.contains("edge_density"));
    assert!(graph_keys.contains("component_count"));

    // --- Part 4: PropertyValues → Property wrapping inspection ---
    println!("\n\nPart 4: PropertyValues Wrapping and Schema Metadata\n");

    // Retrieve a PropertyValues column directly
    let age_column = store
        .node_property_values("age")
        .expect("age property should exist");
    println!("Retrieved 'age' PropertyValues (the column):");
    println!("  ValueType: {:?}", age_column.value_type());
    println!("  Element count: {}", age_column.element_count());
    println!("  First value: {}", age_column.long_value(0).unwrap());

    // Demonstrate that Property = PropertyValues + schema
    println!("\nProperty wrapping adds:");
    println!("  - Key (string identifier)");
    println!("  - PropertyState (Normal/Transient/Persistent)");
    println!("  - DefaultValue (inferred from ValueType)");
    println!("  - ValueType (Long/Double/String/etc)");

    // Verify has_node_property_for_label checks label-scoped presence
    assert!(store.has_node_property_for_label(&person, "age"));
    assert!(!store.has_node_property_for_label(&company, "age"));
    assert!(!store.has_node_property_for_label(&city, "age"));

    assert!(store.has_relationship_property(&works_at, "tenure_months"));
    assert!(!store.has_relationship_property(&knows, "tenure_months"));

    println!("\nKey distribution test complete!");
    println!("Summary:");
    println!("  - GraphStore exposes top-level keys (all properties)");
    println!("  - node_property_keys_for_label(label) filters by label scope");
    println!("  - relationship_property_keys_for_type(type) filters by type scope");
    println!("  - PropertyValues = column; Property = column + metadata");
    println!("  - PropertyStore = HashMap<key, Property>; GraphStore orchestrates multiple stores");
}

fn build_multi_label_graph() -> DefaultGraphStore {
    let graph_name = GraphName::new("multi-label-test");
    let database_info = DatabaseInfo::new(
        DatabaseId::new("test-db"),
        DatabaseLocation::remote("localhost", 7687, None, None),
    );
    let schema = GraphSchema::empty();
    let capabilities = Capabilities::default();

    // 5 nodes: 0,1=Person; 2=Company; 3=Company; 4=City
    let mut id_map = SimpleIdMap::from_original_ids([100, 101, 200, 201, 300]);
    let person = NodeLabel::of("Person");
    let company = NodeLabel::of("Company");
    let city = NodeLabel::of("City");

    id_map.add_node_label(person.clone());
    id_map.add_node_label(company.clone());
    id_map.add_node_label(city.clone());

    id_map.add_node_id_to_label(0, person.clone());
    id_map.add_node_id_to_label(1, person);
    id_map.add_node_id_to_label(2, company.clone());
    id_map.add_node_id_to_label(3, company);
    id_map.add_node_id_to_label(4, city);

    // Relationships: 0->2 (WORKS_AT), 1->3 (WORKS_AT), 0->1 (KNOWS)
    let works_at_topology =
        RelationshipTopology::new(vec![vec![2], vec![3], vec![], vec![], vec![]], None);
    let knows_topology =
        RelationshipTopology::new(vec![vec![1], vec![], vec![], vec![], vec![]], None);

    let mut topologies = HashMap::new();
    topologies.insert(RelationshipType::of("WORKS_AT"), works_at_topology);
    topologies.insert(RelationshipType::of("KNOWS"), knows_topology);

    DefaultGraphStore::new(
        graph_name,
        database_info,
        schema,
        capabilities,
        id_map,
        topologies,
    )
}
