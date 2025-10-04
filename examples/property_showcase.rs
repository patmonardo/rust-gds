//! Property-centric walkthrough for the Rust GDS playground.
//!
//! This example focuses on how graph, node, and relationship properties can be
//! constructed and inspected using the `DefaultGraphStore`. It mirrors the
//! structure of the TypeScript API while taking advantage of Rust's strong
//! typing around property containers.

use rust_gds::projection::{NodeLabel, RelationshipType};
use rust_gds::types::graph::topology::RelationshipTopology;
use rust_gds::types::graph::Graph;
use rust_gds::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStore, GraphStoreResult,
};
use rust_gds::types::properties::graph::DefaultDoubleGraphPropertyValues;
use rust_gds::types::properties::node::{
    DefaultDoubleNodePropertyValues, DefaultLongNodePropertyValues, NodePropertyContainer,
    NodePropertyValues,
};
use rust_gds::types::properties::relationship::PropertyValue;
use rust_gds::types::schema::{
    Direction, MutableGraphSchema, NodeLabel as SchemaNodeLabel,
    RelationshipType as SchemaRelationshipType,
};
use rust_gds::types::ValueType;
use rust_gds::types::{IdMap, MappedNodeId, SimpleIdMap};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n=== Property Showcase ===\n");

    let store = build_property_showcase()?;

    explore_graph_properties(&store)?;
    println!("\n---\n");
    explore_node_properties(&store)?;
    println!("\n---\n");
    explore_graph_view(&store)?;

    Ok(())
}

fn build_property_showcase() -> GraphStoreResult<DefaultGraphStore> {
    let graph_name = GraphName::new("property-demo");
    let database_info = DatabaseInfo::new(
        DatabaseId::new("sandbox"),
        DatabaseLocation::remote("localhost", 7687, None, None),
    );

    let mut capabilities = Capabilities::new();
    capabilities.add_feature("write");
    capabilities.add_feature("properties");

    let person = NodeLabel::of("Person");
    let company = NodeLabel::of("Company");
    let works_at = RelationshipType::of("WORKS_AT");

    let person_schema = SchemaNodeLabel::new("Person");
    let company_schema = SchemaNodeLabel::new("Company");
    let works_at_schema = SchemaRelationshipType::new("WORKS_AT");

    let mut schema_builder = MutableGraphSchema::empty();
    schema_builder.node_schema_mut().add_property(
        person_schema.clone(),
        "experience_years",
        ValueType::Long,
    );
    schema_builder.node_schema_mut().add_property(
        company_schema.clone(),
        "headcount",
        ValueType::Long,
    );
    schema_builder.relationship_schema_mut().add_property(
        works_at_schema,
        Direction::Directed,
        "tenure_months",
        ValueType::Long,
    );
    let schema = schema_builder.build();

    let mut id_map = SimpleIdMap::from_original_ids([100, 101, 200, 201]);
    id_map.add_node_label(person_schema.clone());
    id_map.add_node_label(company_schema.clone());
    id_map.add_node_id_to_label(0, person_schema.clone());
    id_map.add_node_id_to_label(1, person_schema);
    id_map.add_node_id_to_label(2, company_schema.clone());
    id_map.add_node_id_to_label(3, company_schema);

    let outgoing = vec![vec![2], vec![3], vec![], vec![]];
    let topology = RelationshipTopology::new(outgoing, None);

    let mut relationship_topologies: HashMap<RelationshipType, RelationshipTopology> =
        HashMap::new();
    relationship_topologies.insert(works_at.clone(), topology);

    let mut store = DefaultGraphStore::new(
        graph_name,
        database_info,
        schema,
        capabilities,
        id_map,
        relationship_topologies,
    );

    // --- Node properties ---------------------------------------------------
    let experience_years = Arc::new(DefaultLongNodePropertyValues::new(vec![5, 11, 0, 0], 4));
    let mut person_labels = HashSet::new();
    person_labels.insert(person.clone());
    store.add_node_property(person_labels, "experience_years", experience_years)?;

    let performance_rating = Arc::new(DefaultDoubleNodePropertyValues::new(
        vec![4.6, 3.9, 4.2, 4.8],
        4,
    ));
    let mut rated_labels = HashSet::new();
    rated_labels.insert(person.clone());
    rated_labels.insert(company.clone());
    store.add_node_property(rated_labels, "performance_rating", performance_rating)?;

    // --- Graph properties --------------------------------------------------
    let headcount = Arc::new(DefaultDoubleGraphPropertyValues::singleton(2.0));
    store.add_graph_property("company_count", headcount)?;

    Ok(store)
}

fn explore_graph_properties(store: &DefaultGraphStore) -> GraphStoreResult<()> {
    println!(
        "Graph properties available: {:?}",
        store.graph_property_keys()
    );

    if let Ok(values) = store.graph_property_values("company_count") {
        let counts: Vec<f64> = values.double_values().collect();
        println!("  company_count -> {:?}", counts);
    }

    Ok(())
}

fn explore_node_properties(store: &DefaultGraphStore) -> GraphStoreResult<()> {
    println!("Node property keys: {:?}", store.node_property_keys());

    let person_label = NodeLabel::of("Person");
    println!(
        "Properties mapped to Person: {:?}",
        store.node_property_keys_for_label(&person_label)
    );

    let experience_values = store.node_property_values("experience_years")?;
    print_scalar_series("experience_years", &experience_values, store.node_count());

    let rating_values = store.node_property_values("performance_rating")?;
    print_scalar_series("performance_rating", &rating_values, store.node_count());

    Ok(())
}

fn explore_graph_view(store: &DefaultGraphStore) -> GraphStoreResult<()> {
    let graph = store.graph();
    println!("Graph view characteristics: {:?}", graph.characteristics());

    if let Some(values) = graph.node_properties("performance_rating") {
        println!("Graph access to performance ratings:");
        print_scalar_series("performance_rating", &values, graph.node_count());
    }

    print_relationship_sample(&*graph, 0, "Cursor sample from node 0");

    Ok(())
}

fn print_scalar_series(name: &str, values: &Arc<dyn NodePropertyValues>, node_count: usize) {
    match values.value_type() {
        ValueType::Long => {
            let mut series = Vec::new();
            for node_id in 0..node_count {
                match values.long_value(node_id as u64) {
                    Ok(value) => series.push(value.to_string()),
                    Err(_) => series.push("(missing)".into()),
                }
            }
            println!("  {} -> [ {} ]", name, series.join(", "));
        }
        ValueType::Double => {
            let mut series = Vec::new();
            for node_id in 0..node_count {
                match values.double_value(node_id as u64) {
                    Ok(value) => series.push(format!("{:.2}", value)),
                    Err(_) => series.push("(missing)".into()),
                }
            }
            println!("  {} -> [ {} ]", name, series.join(", "));
        }
        other => {
            println!(
                "  {} -> unsupported dump for value type {:?} (dimension {:?})",
                name,
                other,
                values.dimension()
            );
        }
    }
}

fn print_relationship_sample(graph: &dyn Graph, node_id: MappedNodeId, title: &str) {
    const FALLBACK: PropertyValue = 0.0;
    println!("{}", title);

    let mut count = 0usize;
    let mut more = false;
    for (index, cursor) in graph.stream_relationships(node_id, FALLBACK).enumerate() {
        if index < 5 {
            println!(
                "    {} -> {} (property {:.3})",
                cursor.source_id(),
                cursor.target_id(),
                cursor.property()
            );
        } else {
            more = true;
            break;
        }
        count += 1;
    }

    if count == 0 {
        println!("    (no outgoing relationships)");
    } else if more {
        println!("    ...");
    }
}
