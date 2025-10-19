use gds::projection::RelationshipType;
use gds::types::graph::id_map::SimpleIdMap;
use gds::types::graph::topology::RelationshipTopology;
use gds::types::graph::Graph;
use gds::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStore,
};
use gds::types::properties::relationship::relationship_property_values::RelationshipPropertyValues;
use gds::types::properties::relationship::{DefaultRelationshipPropertyValues, PropertyValue};
use gds::types::schema::GraphSchema;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

fn main() {
    println!("\n=== Relationship Property Filtered View ===\n");
    println!("This example shows how to create filtered graph views by relationship type.");
    println!("Graph::relationship_type_filtered_graph(types) returns a new Graph view");
    println!("that includes only the specified relationship types.\n");
    println!("Key insight: filtered views share topology/properties via Arc (cheap projection).");
    println!("Property stores are cloned per type, but PropertyValues remain shared.\n");

    let mut store = build_store_with_multiple_relationship_types();

    let knows = RelationshipType::of("KNOWS");
    let trusts = RelationshipType::of("TRUSTS");

    let knows_weights: Arc<dyn RelationshipPropertyValues> = Arc::new(
        DefaultRelationshipPropertyValues::new(vec![0.6, 0.85], 0.0, 2),
    );
    let trust_scores: Arc<dyn RelationshipPropertyValues> = Arc::new(
        DefaultRelationshipPropertyValues::new(vec![0.95, 0.4], 0.0, 2),
    );

    store
        .add_relationship_property(knows.clone(), "weight", Arc::clone(&knows_weights))
        .expect("failed to register KNOWS weight property");
    store
        .add_relationship_property(trusts.clone(), "trust_score", Arc::clone(&trust_scores))
        .expect("failed to register TRUSTS score property");

    println!("Property keys per type:");
    println!(
        "  KNOWS => {:?}",
        store.relationship_property_keys_for_type(&knows)
    );
    println!(
        "  TRUSTS => {:?}\n",
        store.relationship_property_keys_for_type(&trusts)
    );

    let graph = store.graph();
    print_graph("Full graph", &*graph);

    let mut filter = HashSet::new();
    filter.insert(trusts.clone());
    let filtered = graph
        .relationship_type_filtered_graph(&filter)
        .expect("filtered graph should preserve relationship properties");
    print_graph("Filtered graph (TRUSTS only)", filtered.as_ref());
}

fn build_store_with_multiple_relationship_types() -> DefaultGraphStore {
    let graph_name = GraphName::new("relationship_filtered_demo");
    let database_info = DatabaseInfo::new(
        DatabaseId::new("demo-db"),
        DatabaseLocation::remote("localhost", 7687, None, None),
    );
    let schema = GraphSchema::empty();
    let capabilities = Capabilities::default();
    let id_map = SimpleIdMap::from_original_ids([0, 1, 2]);

    let knows_topology = RelationshipTopology::new(vec![vec![1], vec![2], vec![]], None);
    let trusts_topology = RelationshipTopology::new(vec![vec![2], vec![], vec![1]], None);

    let mut topologies = HashMap::new();
    topologies.insert(RelationshipType::of("KNOWS"), knows_topology);
    topologies.insert(RelationshipType::of("TRUSTS"), trusts_topology);

    DefaultGraphStore::new(
        graph_name,
        database_info,
        schema,
        capabilities,
        id_map,
        topologies,
    )
}

fn print_graph(title: &str, graph: &dyn Graph) {
    println!("{title}:");
    const FALLBACK: PropertyValue = -1.0;

    for node_id in 0..graph.node_count() as u64 {
        println!("  node {node_id} outgoing:");
        let mut count = 0usize;
        for cursor in graph.stream_relationships(node_id, FALLBACK) {
            count += 1;
            println!(
                "    {} -> {} (property {:.3})",
                cursor.source_id(),
                cursor.target_id(),
                cursor.property()
            );
        }
        if count == 0 {
            println!("    (no outgoing relationships)");
        }
    }
    println!();
}
