//! Integration tests for the Projection system.
//!
//! These tests demonstrate how to use NodeProjections and RelationshipProjections
//! to configure graph loading. The Projection system is the "Zod for Graphs" -
//! a runtime configuration layer that describes HOW to materialize a GraphStore.

use rust_gds::projection::{
    AbstractProjections, Aggregation, NodeLabel, NodeProjection, NodeProjections, Orientation,
    PropertyMapping, PropertyMappings, RelationshipProjection, RelationshipProjections,
    RelationshipType,
};
use std::sync::Arc;

/// Example: Simple node projection with a single label
#[test]
fn test_simple_node_projection() {
    let person_label = NodeLabel::of("Person");
    let projection = NodeProjection::of(person_label.clone());

    assert_eq!(projection.label().name(), person_label.name());
    assert!(projection.properties().is_empty());
}

/// Example: Node projection with properties using builder
#[test]
fn test_node_projection_with_properties() {
    let properties = PropertyMappings::builder()
        .add_property("age")
        .unwrap()
        .add_property("name")
        .unwrap()
        .build();

    let person = NodeProjection::builder()
        .label(NodeLabel::of("Person"))
        .properties(properties)
        .build();

    assert_eq!(person.label().name(), "Person");
    assert_eq!(person.properties().len(), 2);
    assert!(person.properties().contains_key("age"));
    assert!(person.properties().contains_key("name"));
}

/// Example: Project all nodes with all properties
#[test]
fn test_project_all_nodes() {
    let all_nodes = NodeProjection::all();

    assert_eq!(all_nodes.label().name(), "*");
    assert!(all_nodes.project_all());
}

/// Example: Multiple node projections (heterogeneous graph)
#[test]
fn test_multiple_node_projections() {
    let person = NodeProjection::builder()
        .label(NodeLabel::of("Person"))
        .properties(
            PropertyMappings::builder()
                .add_property("age")
                .unwrap()
                .build(),
        )
        .build();

    let company = NodeProjection::builder()
        .label(NodeLabel::of("Company"))
        .properties(
            PropertyMappings::builder()
                .add_property("revenue")
                .unwrap()
                .build(),
        )
        .build();

    let projections = NodeProjections::builder()
        .add(NodeLabel::of("Person"), Arc::new(person))
        .add(NodeLabel::of("Company"), Arc::new(company))
        .build();

    assert_eq!(projections.size(), 2);
    assert!(projections.get(&NodeLabel::of("Person")).is_some());
    assert!(projections.get(&NodeLabel::of("Company")).is_some());
}

/// Example: Simple relationship projection with natural orientation
#[test]
fn test_simple_relationship_projection() {
    let knows = RelationshipProjection::of(RelationshipType::of("KNOWS"));

    assert_eq!(knows.rel_type().name(), "KNOWS");
    assert_eq!(knows.orientation(), Orientation::Natural);
    assert_eq!(knows.aggregation(), Aggregation::Default);
    assert!(!knows.index_inverse());
}

/// Example: Undirected relationship projection
#[test]
fn test_undirected_relationship_projection() {
    let friends = RelationshipProjection::all_undirected();

    assert_eq!(friends.rel_type().name(), "*");
    assert_eq!(friends.orientation(), Orientation::Undirected);
    assert!(friends.project_all());
}

/// Example: Relationship projection with aggregation
#[test]
fn test_relationship_projection_with_aggregation() {
    let properties = PropertyMappings::builder()
        .add_property("weight")
        .unwrap()
        .build();

    let weighted_knows = RelationshipProjection::builder()
        .rel_type(RelationshipType::of("KNOWS"))
        .orientation(Orientation::Natural)
        .aggregation(Aggregation::Sum)
        .properties(properties)
        .build()
        .expect("valid projection");

    assert_eq!(weighted_knows.aggregation(), Aggregation::Sum);
    assert_eq!(weighted_knows.properties().len(), 1);
}

/// Example: Relationship projection with reverse orientation
#[test]
fn test_reverse_relationship_projection() {
    let follows = RelationshipProjection::builder()
        .rel_type(RelationshipType::of("FOLLOWS"))
        .orientation(Orientation::Reverse)
        .build()
        .expect("valid projection");

    assert_eq!(follows.orientation(), Orientation::Reverse);
}

/// Example: Relationship projection with inverse indexing
#[test]
fn test_relationship_projection_with_inverse_index() {
    let knows = RelationshipProjection::builder()
        .rel_type(RelationshipType::of("KNOWS"))
        .orientation(Orientation::Natural)
        .index_inverse(true)
        .build()
        .expect("valid projection");

    assert!(knows.index_inverse());
}

/// Example: INVALID - Cannot combine inverse index with undirected orientation
#[test]
fn test_invalid_inverse_index_with_undirected() {
    let result = RelationshipProjection::builder()
        .rel_type(RelationshipType::of("KNOWS"))
        .orientation(Orientation::Undirected)
        .index_inverse(true)
        .build();

    assert!(result.is_err(), "Should fail validation");
}

/// Example: Multiple relationship projections with different configurations
#[test]
fn test_multiple_relationship_projections() {
    let knows = RelationshipProjection::builder()
        .rel_type(RelationshipType::of("KNOWS"))
        .orientation(Orientation::Natural)
        .aggregation(Aggregation::None)
        .build()
        .expect("valid");

    let friends = RelationshipProjection::builder()
        .rel_type(RelationshipType::of("FRIENDS_WITH"))
        .orientation(Orientation::Undirected)
        .aggregation(Aggregation::Max)
        .properties(
            PropertyMappings::builder()
                .add_property("strength")
                .unwrap()
                .build(),
        )
        .build()
        .expect("valid");

    let follows = RelationshipProjection::builder()
        .rel_type(RelationshipType::of("FOLLOWS"))
        .orientation(Orientation::Reverse)
        .index_inverse(true)
        .build()
        .expect("valid");

    let projections = RelationshipProjections::builder()
        .add(RelationshipType::of("KNOWS"), Arc::new(knows))
        .add(RelationshipType::of("FRIENDS_WITH"), Arc::new(friends))
        .add(RelationshipType::of("FOLLOWS"), Arc::new(follows))
        .build();

    assert_eq!(projections.size(), 3);
}

/// Example: Fluent API for relationship projection
#[test]
fn test_relationship_projection_fluent_api() {
    let projection = RelationshipProjection::of(RelationshipType::of("KNOWS"))
        .with_orientation(Orientation::Undirected)
        .with_aggregation(Aggregation::Sum)
        .with_index_inverse(false);

    assert_eq!(projection.orientation(), Orientation::Undirected);
    assert_eq!(projection.aggregation(), Aggregation::Sum);
    assert!(!projection.index_inverse());
}

/// Example: Property mapping with source field
#[test]
fn test_property_mapping_with_source() {
    let mapping = PropertyMapping::with_source("nodeAge", "age").expect("valid mapping");

    assert_eq!(mapping.property_key(), "nodeAge");
    assert_eq!(mapping.neo_property_key(), "age");
}

/// Example: Complete graph projection configuration
#[test]
fn test_complete_graph_projection_config() {
    // Define node projections
    let person = NodeProjection::builder()
        .label(NodeLabel::of("Person"))
        .properties(
            PropertyMappings::builder()
                .add_property("age")
                .unwrap()
                .add_property("name")
                .unwrap()
                .build(),
        )
        .build();

    let company = NodeProjection::builder()
        .label(NodeLabel::of("Company"))
        .properties(
            PropertyMappings::builder()
                .add_property("revenue")
                .unwrap()
                .build(),
        )
        .build();

    let node_projections = NodeProjections::builder()
        .add(NodeLabel::of("Person"), Arc::new(person))
        .add(NodeLabel::of("Company"), Arc::new(company))
        .build();

    // Define relationship projections
    let works_at = RelationshipProjection::builder()
        .rel_type(RelationshipType::of("WORKS_AT"))
        .orientation(Orientation::Natural)
        .properties(
            PropertyMappings::builder()
                .add_property("since")
                .unwrap()
                .build(),
        )
        .build()
        .expect("valid");

    let knows = RelationshipProjection::builder()
        .rel_type(RelationshipType::of("KNOWS"))
        .orientation(Orientation::Undirected)
        .aggregation(Aggregation::Sum)
        .properties(
            PropertyMappings::builder()
                .add_property("weight")
                .unwrap()
                .build(),
        )
        .build()
        .expect("valid");

    let rel_projections = RelationshipProjections::builder()
        .add(RelationshipType::of("WORKS_AT"), Arc::new(works_at))
        .add(RelationshipType::of("KNOWS"), Arc::new(knows))
        .build();

    // This configuration is ready to be passed to NativeFactory
    assert_eq!(node_projections.size(), 2);
    assert_eq!(rel_projections.size(), 2);
}

/// Example: Orientation conversions and queries
#[test]
fn test_orientation_operations() {
    // Parse from string
    assert_eq!(Orientation::parse("NATURAL"), Some(Orientation::Natural));
    assert_eq!(Orientation::parse("REVERSE"), Some(Orientation::Reverse));
    assert_eq!(
        Orientation::parse("UNDIRECTED"),
        Some(Orientation::Undirected)
    );

    // Inverse operation
    assert_eq!(Orientation::Natural.inverse(), Orientation::Reverse);
    assert_eq!(Orientation::Reverse.inverse(), Orientation::Natural);
    assert_eq!(Orientation::Undirected.inverse(), Orientation::Undirected);

    // Type checks
    assert!(Orientation::Undirected.is_undirected());
    assert!(Orientation::Natural.is_directed());
    assert!(Orientation::Reverse.is_directed());
}

/// Example: NodeLabel interning (memory efficiency)
#[test]
fn test_node_label_interning() {
    let label1 = NodeLabel::of("Person");
    let label2 = NodeLabel::of("Person");
    let label3 = NodeLabel::of("Company");

    // Same string → same interned instance
    assert_eq!(label1, label2);
    assert_ne!(label1, label3);

    // Efficient comparisons
    assert_eq!(label1.name(), "Person");
    assert_eq!(label3.name(), "Company");
}

/// Example: RelationshipType interning
#[test]
fn test_relationship_type_interning() {
    let knows1 = RelationshipType::of("KNOWS");
    let knows2 = RelationshipType::of("KNOWS");
    let follows = RelationshipType::of("FOLLOWS");

    assert_eq!(knows1, knows2);
    assert_ne!(knows1, follows);
    assert_eq!(knows1.name(), "KNOWS");
}
