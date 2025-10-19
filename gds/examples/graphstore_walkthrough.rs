use gds::projection::{NodeLabel, RelationshipType};
use gds::types::graph::id_map::{IdMap, MappedNodeId, SimpleIdMap};
use gds::types::graph::topology::RelationshipTopology;
use gds::types::graph::Graph;
use gds::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStore, GraphStoreResult,
};
use gds::types::properties::graph::DefaultDoubleGraphPropertyValues;
use gds::types::properties::node::DefaultLongNodePropertyValues;
use gds::types::properties::relationship::PropertyValue;
use gds::types::random::{RandomGraphConfig, RandomGraphResult, RandomRelationshipConfig};
use gds::types::schema::{Direction, MutableGraphSchema};
use gds::types::value_type::ValueType;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n=== GraphStore Walkthrough ===\n");
    println!("This example demonstrates the core GraphStore construction and inspection APIs.");
    println!("We'll build a graph manually (showing all components) and via Random generation.\n");

    manual_walkthrough()?;
    println!("\n---\n");
    random_walkthrough()?;

    Ok(())
}

fn manual_walkthrough() -> GraphStoreResult<()> {
    println!("=== Manual Construction: Building a GraphStore from Scratch ===\n");
    println!("The GraphStore is the central container managing:");
    println!("  - Schema (node labels, relationship types, property definitions)");
    println!("  - Topology (adjacency structure via RelationshipTopology)");
    println!("  - Properties (columnar PropertyValues attached to graph/nodes/relationships)");
    println!("  - Metadata (capabilities, timestamps, database info)\n");

    println!("Step 1: Naming, database info, and capabilities");
    println!("  GraphName identifies this store; DatabaseInfo tracks provenance.");
    let graph_name = GraphName::new("neo4j-movies");
    let database_info = DatabaseInfo::new(
        DatabaseId::new("movies"),
        DatabaseLocation::remote("localhost", 7687, Some("neo4j".into()), None),
    );
    let mut capabilities = Capabilities::new();
    capabilities.add_feature("transient");
    capabilities.add_feature("write");

    println!("\nStep 2: Schema with node labels, properties, and relationship types");
    println!("  Schema defines the 'shape' of the graph: what labels/types exist,");
    println!("  what properties they can have, and their value types (Long/Double/etc).");
    let person = NodeLabel::of("Person");
    let movie = NodeLabel::of("Movie");
    let acted_in = RelationshipType::of("ACTED_IN");

    let mut schema_builder = MutableGraphSchema::empty();
    schema_builder.node_schema_mut().add_property(
        person.clone(),
        "experience_years",
        ValueType::Long,
    );
    schema_builder
        .node_schema_mut()
        .add_property(movie.clone(), "release_year", ValueType::Long);
    schema_builder.relationship_schema_mut().add_property(
        acted_in.clone(),
        Direction::Directed,
        "role_count",
        ValueType::Long,
    );
    let schema = schema_builder.build();

    println!("\nStep 3: ID mapping between original IDs and compact node IDs");
    println!("  IdMap translates original node IDs (e.g., from Neo4j) to compact 0..N range.");
    println!("  This enables efficient columnar storage and array-based topology.");
    let mut id_map = SimpleIdMap::from_original_ids([0, 1, 2, 3]);
    id_map.add_node_label(person.clone());
    id_map.add_node_label(movie.clone());
    id_map.add_node_id_to_label(0, person.clone());
    id_map.add_node_id_to_label(1, person.clone());
    id_map.add_node_id_to_label(2, movie.clone());
    id_map.add_node_id_to_label(3, movie.clone());

    println!("\nStep 4: Relationship topology (adjacency lists)");
    println!("  RelationshipTopology stores the graph structure as adjacency lists.");
    println!("  For directed graphs: outgoing[node_id] = [target_ids...]");
    println!("  Inverse indices (incoming edges) can be added for bidirectional traversal.");
    let outgoing = vec![vec![2, 3], vec![3], vec![], vec![]];
    let topology = RelationshipTopology::new(outgoing, None);
    let mut relationship_topologies = HashMap::new();
    relationship_topologies.insert(acted_in.clone(), topology);

    println!("\nStep 5: Assemble the DefaultGraphStore");
    println!("  DefaultGraphStore::new combines all components into a mutable store.");
    println!("  At this point, the graph structure is fixed; properties can be added/removed.");
    let mut store = DefaultGraphStore::new(
        graph_name,
        database_info,
        schema,
        capabilities,
        id_map,
        relationship_topologies,
    );

    println!("\nStep 6: Attach graph and node properties");
    println!("  Properties are columnar: PropertyValues = Arc<dyn *PropertyValues>.");
    println!("  Node properties are label-scoped; graph properties are global scalars.");
    println!(
        "  The store holds Arc clones; multiple views can share the same columns (zero-copy)."
    );
    let experience = Arc::new(DefaultLongNodePropertyValues::new(vec![20, 12, 0, 0], 4));
    let mut person_only = HashSet::new();
    person_only.insert(person.clone());
    store.add_node_property(person_only, "experience_years", experience)?;

    let density = Arc::new(DefaultDoubleGraphPropertyValues::singleton(0.5));
    store.add_graph_property("edge_density", density)?;

    println!("\nStep 7: Inspect the graph view for validation");
    println!("  GraphStore::graph() creates a lightweight, immutable Graph view.");
    println!("  The Graph trait provides traversal, degree queries, and property access.");
    println!("  Views share topology/properties via Arc; creating views is cheap.");
    let graph = store.graph();
    println!("  Nodes: {}", graph.node_count());
    println!("  Relationships: {}", graph.relationship_count());
    println!("  Directed: {}", graph.characteristics().is_directed());
    println!("  Person nodes: {}", store.node_count_for_label(&person));
    println!("  Movie nodes: {}", store.node_count_for_label(&movie));
    print_relationship_sample(&*graph, 0, "  Sample outgoing relationships from node 0");

    println!("8. Schema summary");
    print_sorted_labels("  Node labels", &store.node_labels());
    print_sorted_relationships("  Relationship types", &store.relationship_types());

    Ok(())
}

fn random_walkthrough() -> RandomGraphResult<()> {
    println!("=== Random Generation: Testing with Synthetic Graphs ===\n");
    println!("The Random trait provides deterministic graph generation for testing.");
    println!("RandomGraphConfig specifies: node count, labels, relationship types, density.");
    println!("Seeded RNG ensures reproducibility across runs.\n");

    let config = RandomGraphConfig {
        graph_name: "demo-random".into(),
        database_name: "in-memory".into(),
        node_count: 24,
        node_labels: vec!["Person".into(), "Movie".into(), "Genre".into()],
        relationships: vec![
            RandomRelationshipConfig::new("ACTED_IN", 0.18),
            RandomRelationshipConfig::new("FRIENDS_WITH", 0.12),
        ],
        directed: true,
        inverse_indexed: true,
        seed: Some(7),
    };

    let store = DefaultGraphStore::random(&config)?;
    let graph = store.graph();

    println!("Config summary:");
    println!("  Nodes: {}", config.node_count);
    println!(
        "  Relationships configured: {}",
        config
            .relationships
            .iter()
            .map(|r| r.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );

    println!("Generated store snapshot:");
    println!("  Stored nodes: {}", store.node_count());
    println!("  Stored relationships: {}", store.relationship_count());
    println!("  Node property keys: {:?}", store.node_property_keys());
    println!(
        "  Graph properties: {:?}",
        store.graph_property_keys().into_iter().collect::<Vec<_>>()
    );
    println!(
        "  Directed graph: {}",
        graph.characteristics().is_directed()
    );

    if let Ok(edge_density) = store.graph_property_values("edge_density") {
        let values: Vec<f64> = edge_density.double_values().collect();
        if let Some(value) = values.first() {
            println!("  Edge density (graph property): {:.3}", value);
        }
    }

    print_sorted_labels("  Node labels", &store.node_labels());
    print_sorted_relationships("  Relationship types", &store.relationship_types());
    print_relationship_sample(&*graph, 0, "  Sample outgoing relationships from node 0");

    Ok(())
}

fn print_sorted_labels(title: &str, labels: &HashSet<NodeLabel>) {
    let mut names: Vec<_> = labels
        .iter()
        .map(|label| label.name().to_string())
        .collect();
    names.sort();
    println!("{}: {}", title, names.join(", "));
}

fn print_sorted_relationships(title: &str, relationships: &HashSet<RelationshipType>) {
    let mut names: Vec<_> = relationships
        .iter()
        .map(|rel_type| rel_type.name().to_string())
        .collect();
    names.sort();
    println!("{}: {}", title, names.join(", "));
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
