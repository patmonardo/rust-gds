use rust_gds::projection::RelationshipType;
use rust_gds::types::graph::topology::RelationshipTopology;
use rust_gds::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStore,
};
use rust_gds::types::properties::relationship::{
    DefaultRelationshipPropertyValues, PropertyValue, RelationshipIterator,
    RelationshipPropertyValues,
};
use rust_gds::types::schema::GraphSchema;
use rust_gds::types::{IdMap, SimpleIdMap};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n=== Relationship Cursor Traversal ===\n");

    let mut store = build_sample_store();
    let relationship_type = RelationshipType::of("KNOWS");

    let weight_values: Arc<dyn RelationshipPropertyValues> = Arc::new(
        DefaultRelationshipPropertyValues::new(vec![0.9, 0.7, 0.4], 0.0, 3),
    );
    store.add_relationship_property(
        relationship_type.clone(),
        "weight",
        Arc::clone(&weight_values),
    )?;

    println!(
        "Weight property default value: {}\n",
        weight_values.default_value()
    );

    let graph = store.graph();
    const FALLBACK: PropertyValue = -1.0;

    for node_id in 0..graph.node_count() as u64 {
        println!("Node {node_id} outgoing:");
        let mut count = 0usize;
        for cursor in graph.stream_relationships(node_id, FALLBACK) {
            count += 1;
            println!(
                "  {} -> {} (weight {:.3})",
                cursor.source_id(),
                cursor.target_id(),
                cursor.property()
            );
        }
        if count == 0 {
            println!("  (no outgoing relationships)");
        }
    }

    let target_node = 2u64;
    println!("\nIncoming relationships to node {target_node}:");
    let mut incoming = 0usize;
    for cursor in graph.stream_inverse_relationships(target_node, FALLBACK) {
        incoming += 1;
        println!(
            "  {} -> {} (weight {:.3})",
            cursor.source_id(),
            cursor.target_id(),
            cursor.property()
        );
    }
    if incoming == 0 {
        println!("  (no incoming relationships)");
    }

    let values = store.relationship_property_values(&relationship_type, "weight")?;
    println!(
        "\nThe store reports {} weight entries for {} relationships",
        values.relationship_count(),
        store.relationship_count_for_type(&relationship_type)
    );

    Ok(())
}

fn build_sample_store() -> DefaultGraphStore {
    let graph_name = GraphName::new("relationship_cursor_demo");
    let database_info = DatabaseInfo::new(
        DatabaseId::new("demo-db"),
        DatabaseLocation::remote("localhost", 7687, None, None),
    );
    let schema = GraphSchema::empty();
    let capabilities = Capabilities::default();
    let id_map = SimpleIdMap::from_original_ids([0, 1, 2]);

    let knows_topology = RelationshipTopology::new(vec![vec![1, 2], vec![2], vec![]], None);

    let mut topologies = HashMap::new();
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
