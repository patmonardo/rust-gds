//! Projection examples showing how to configure graph projections.
//!
//! Run this example with:
//! ```sh
//! cargo run --example projection_showcase
//! ```

use gds::projection::{
    AbstractProjections, Aggregation, NodeLabel, NodeProjection, NodeProjections, Orientation,
    PropertyMappings, RelationshipProjection, RelationshipProjections, RelationshipType,
};
use std::sync::Arc;

fn main() {
    println!("=== Projection Showcase ===\n");

    basic_node_projections();
    println!();

    basic_relationship_projections();
    println!();

    complete_projection_config();
    println!();

    orientation_examples();
}

/// Basic node projection examples
fn basic_node_projections() {
    println!("1. NODE PROJECTIONS");
    println!("-------------------");

    // Simple node projection
    let person = NodeProjection::of(NodeLabel::of("Person"));
    println!("✓ Simple projection: {}", person.label().name());

    // Project all nodes
    let all_nodes = NodeProjection::all();
    println!(
        "✓ Project all: {} (is_project_all: {})",
        all_nodes.label().name(),
        all_nodes.project_all()
    );

    // Node projection with properties
    let properties = PropertyMappings::builder()
        .add_property("age")
        .unwrap()
        .add_property("name")
        .unwrap()
        .build();

    let person_with_props = NodeProjection::builder()
        .label(NodeLabel::of("Person"))
        .properties(properties)
        .build();

    println!(
        "✓ Person with {} properties",
        person_with_props.properties().len()
    );
}

/// Basic relationship projection examples
fn basic_relationship_projections() {
    println!("2. RELATIONSHIP PROJECTIONS");
    println!("---------------------------");

    // Simple relationship
    let knows = RelationshipProjection::of(RelationshipType::of("KNOWS"));
    println!(
        "✓ KNOWS: orientation={}, aggregation={}",
        knows.orientation().as_str(),
        knows.aggregation().as_str()
    );

    // Undirected relationship
    let friends = RelationshipProjection::all_undirected();
    println!(
        "✓ All undirected: {} (is_undirected: {})",
        friends.rel_type().name(),
        friends.is_undirected()
    );

    // Reverse relationship
    let follows_reverse = RelationshipProjection::of(RelationshipType::of("FOLLOWS"))
        .with_orientation(Orientation::Reverse);
    println!(
        "✓ FOLLOWS reversed: orientation={}",
        follows_reverse.orientation().as_str()
    );

    // Relationship with aggregation
    let weighted = RelationshipProjection::of(RelationshipType::of("RATED"))
        .with_orientation(Orientation::Natural)
        .with_aggregation(Aggregation::Sum);
    println!(
        "✓ RATED with aggregation: {}",
        weighted.aggregation().as_str()
    );
}

/// Complete projection configuration
fn complete_projection_config() {
    println!("3. COMPLETE GRAPH CONFIGURATION");
    println!("--------------------------------");

    // Build node projections
    let person_props = PropertyMappings::builder()
        .add_property("age")
        .unwrap()
        .add_property("name")
        .unwrap()
        .build();

    let person = NodeProjection::builder()
        .label(NodeLabel::of("Person"))
        .properties(person_props)
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

    println!("✓ Node projections: {} types", node_projections.size());

    // Build relationship projections
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
        .expect("valid projection");

    let knows = RelationshipProjection::builder()
        .rel_type(RelationshipType::of("KNOWS"))
        .orientation(Orientation::Undirected)
        .aggregation(Aggregation::Max)
        .properties(
            PropertyMappings::builder()
                .add_property("strength")
                .unwrap()
                .build(),
        )
        .build()
        .expect("valid projection");

    let rel_projections = RelationshipProjections::builder()
        .add(RelationshipType::of("WORKS_AT"), Arc::new(works_at))
        .add(RelationshipType::of("KNOWS"), Arc::new(knows))
        .build();

    println!(
        "✓ Relationship projections: {} types",
        rel_projections.size()
    );
    println!("\n→ Ready for NativeFactory::build(config)");
}

/// Orientation transformation examples
fn orientation_examples() {
    println!("4. ORIENTATION TRANSFORMATIONS");
    println!("------------------------------");

    let natural = Orientation::Natural;
    println!("✓ Natural inverse: {}", natural.inverse().as_str());

    let reverse = Orientation::Reverse;
    println!("✓ Reverse inverse: {}", reverse.inverse().as_str());

    let undirected = Orientation::Undirected;
    println!("✓ Undirected inverse: {}", undirected.inverse().as_str());
    println!("✓ Is undirected? {}", undirected.is_undirected());

    // Parse from strings
    if let Some(parsed) = Orientation::parse("UNDIRECTED") {
        println!("✓ Parsed 'UNDIRECTED': {}", parsed.as_str());
    }
}
