use rust_gds::projection::{NodeLabel, RelationshipType};
use rust_gds::types::graph::topology::RelationshipTopology;
use rust_gds::types::graph::Graph;
use rust_gds::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStore, GraphStoreResult,
};
use rust_gds::types::properties::graph::DefaultDoubleGraphPropertyValues;
use rust_gds::types::properties::node::DefaultLongNodePropertyValues;
use rust_gds::types::random::{RandomGraphConfig, RandomGraphResult, RandomRelationshipConfig};
use rust_gds::types::schema::{
    Direction, MutableGraphSchema, NodeLabel as SchemaNodeLabel,
    RelationshipType as SchemaRelationshipType,
};
use rust_gds::types::ValueType;
use rust_gds::types::{IdMap, SimpleIdMap};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n=== GraphStore Walkthrough ===\n");

    manual_walkthrough()?;
    println!("\n---\n");
    random_walkthrough()?;

    Ok(())
}

fn manual_walkthrough() -> GraphStoreResult<()> {
    println!("Manual construction of a small in-memory graph store\n");

    println!("1. Naming, database info, and capabilities");
    let graph_name = GraphName::new("neo4j-movies");
    let database_info = DatabaseInfo::new(
        DatabaseId::new("movies"),
        DatabaseLocation::remote("localhost", 7687, Some("neo4j".into()), None),
    );
    let mut capabilities = Capabilities::new();
    capabilities.add_feature("transient");
    capabilities.add_feature("write");

    println!("2. Schema with node labels, properties, and relationship types");
    let person = NodeLabel::of("Person");
    let movie = NodeLabel::of("Movie");
    let acted_in = RelationshipType::of("ACTED_IN");
    let person_schema = SchemaNodeLabel::new("Person");
    let movie_schema = SchemaNodeLabel::new("Movie");
    let acted_in_schema = SchemaRelationshipType::new("ACTED_IN");

    let mut schema_builder = MutableGraphSchema::empty();
    schema_builder.node_schema_mut().add_property(
        person_schema.clone(),
        "experience_years",
        ValueType::Long,
    );
    schema_builder.node_schema_mut().add_property(
        movie_schema.clone(),
        "release_year",
        ValueType::Long,
    );
    schema_builder.relationship_schema_mut().add_property(
        acted_in_schema.clone(),
        Direction::Directed,
        "role_count",
        ValueType::Long,
    );
    let schema = schema_builder.build();

    println!("3. ID mapping between original IDs and compact node IDs");
    let mut id_map = SimpleIdMap::from_original_ids([0, 1, 2, 3]);
    id_map.add_node_label(person_schema.clone());
    id_map.add_node_label(movie_schema.clone());
    id_map.add_node_id_to_label(0, person_schema.clone());
    id_map.add_node_id_to_label(1, person_schema.clone());
    id_map.add_node_id_to_label(2, movie_schema.clone());
    id_map.add_node_id_to_label(3, movie_schema.clone());

    println!("4. Relationship topology (adjacency lists)");
    let outgoing = vec![vec![2, 3], vec![3], vec![], vec![]];
    let topology = RelationshipTopology::new(outgoing, None);
    let mut relationship_topologies = HashMap::new();
    relationship_topologies.insert(acted_in.clone(), topology);

    println!("5. Assemble the DefaultGraphStore");
    let mut store = DefaultGraphStore::new(
        graph_name,
        database_info,
        schema,
        capabilities,
        id_map,
        relationship_topologies,
    );

    println!("6. Attach graph and node properties");
    let experience = Arc::new(DefaultLongNodePropertyValues::new(vec![20, 12, 0, 0], 4));
    let mut person_only = HashSet::new();
    person_only.insert(person.clone());
    store.add_node_property(person_only, "experience_years", experience)?;

    let density = Arc::new(DefaultDoubleGraphPropertyValues::singleton(0.5));
    store.add_graph_property("edge_density", density)?;

    println!("7. Inspect the graph view for validation");
    let graph = store.graph();
    println!("  Nodes: {}", graph.node_count());
    println!("  Relationships: {}", graph.relationship_count());
    println!("  Directed: {}", graph.characteristics().is_directed());
    println!("  Person nodes: {}", store.node_count_for_label(&person));
    println!("  Movie nodes: {}", store.node_count_for_label(&movie));

    println!("8. Schema summary");
    print_sorted_labels("  Node labels", &store.node_labels());
    print_sorted_relationships("  Relationship types", &store.relationship_types());

    Ok(())
}

fn random_walkthrough() -> RandomGraphResult<()> {
    println!("Random graph store generation via the new Random trait\n");

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
