//! Traversal-focused demo for exploring the Rust GDS graph API.
//!
//! This example highlights how to navigate relationships, compute degree based
//! metrics, and materialize filtered sub-graphs using the `Graph` trait.

use gds::projection::RelationshipType;
use gds::types::graph::id_map::MappedNodeId;
use gds::types::graph::{Graph, GraphExt, GraphResult};
use gds::types::graph_store::DefaultGraphStore;
use gds::types::graph::PropertyValue;
use gds::types::random::{RandomGraphConfig, RandomGraphResult, RandomRelationshipConfig};
use std::collections::HashSet;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("\n=== Traversal Inspector ===\n");
    println!("This example explores the Graph traversal API in depth.");
    println!("We'll demonstrate:");
    println!("  - Degree queries (out/in degree, degree_without_parallel_relationships)");
    println!("  - Cursor-based neighbor enumeration (stream_relationships)");
    println!("  - Relationship type filtering (creating sub-graph projections)");
    println!("  - Inverse traversal (stream_inverse_relationships)\n");
    println!(
        "Key insight: Graph = read-only view; all traversal is zero-allocation where possible.\n"
    );

    let store =
        generate_sample_graph().map_err(|err| -> Box<dyn Error + Send + Sync> { Box::new(err) })?;
    let graph = store.graph();

    summarize_graph(&*graph);
    println!("\n---\n");
    sample_reachable_nodes(&*graph, 4);
    println!("\n---\n");
    inspect_relationship_filters(&*graph)?;

    Ok(())
}

fn generate_sample_graph() -> RandomGraphResult<DefaultGraphStore> {
    let config = RandomGraphConfig {
        graph_name: "traversal-demo".into(),
        database_name: "sessions".into(),
        node_count: 18,
        node_labels: vec!["Person".into(), "City".into(), "Office".into()],
        relationships: vec![
            RandomRelationshipConfig::new("WORKS_WITH", 0.14),
            RandomRelationshipConfig::new("VISITED", 0.18),
            RandomRelationshipConfig::new("REPORTS_TO", 0.12),
        ],
        directed: true,
        inverse_indexed: true,
        seed: Some(17),
    };

    DefaultGraphStore::random(&config)
}

fn summarize_graph(graph: &dyn Graph) {
    println!("Graph summary");
    println!("  Nodes: {}", graph.node_count());
    println!("  Relationships: {}", graph.relationship_count());
    println!("  Directed: {}", graph.characteristics().is_directed());
    println!(
        "  Inverse indexed: {}",
        graph.characteristics().is_inverse_indexed()
    );
    println!(
        "  Has relationship property data: {}",
        graph.has_relationship_property()
    );
}

fn sample_reachable_nodes(graph: &dyn Graph, node_limit: usize) {
    const FALLBACK_REL_PROPERTY: PropertyValue = 0.0;
    println!("Sampling degrees and reachable neighbors via RelationshipCursor");

    let limit = node_limit.min(graph.node_count());
    for node in 0..limit {
        let node_id = node as MappedNodeId;
        let out_degree = graph.degree(node_id);
        let in_degree = graph.degree_inverse(node_id).unwrap_or_default();
        println!(
            "  Node {node_id}: out={out_degree}, in={in_degree}, nth_target(0)={:?}",
            graph.nth_target(node_id, 0)
        );

        let mut neighbors = Vec::new();
        let mut more_neighbors = false;
        let relationships = graph.stream_relationships(node_id, FALLBACK_REL_PROPERTY);

        for (idx, cursor) in relationships.enumerate() {
            if idx < 5 {
                neighbors.push((cursor.target_id(), cursor.property()));
            } else {
                more_neighbors = true;
                break;
            }
        }

        if neighbors.is_empty() {
            println!("    No outgoing relationships");
        } else {
            let display: Vec<String> = neighbors
                .iter()
                .map(|(target, property)| format!("target={target}, property={property:.3}"))
                .collect();
            if more_neighbors {
                println!("    -> [{} ...]", display.join(", "));
            } else {
                println!("    -> [{}]", display.join(", "));
            }
        }
    }
}

fn inspect_relationship_filters(graph: &dyn Graph) -> GraphResult<()> {
    println!("Filtering by relationship types");

    let mut focus = HashSet::new();
    focus.insert(RelationshipType::of("WORKS_WITH"));

    let filtered = graph.relationship_type_filtered_graph(&focus)?;
    println!(
        "  WORKS_WITH only: {} relationships over {} nodes",
        filtered.relationship_count(),
        filtered.node_count()
    );

    let mut city_focus = HashSet::new();
    city_focus.insert(RelationshipType::of("VISITED"));
    city_focus.insert(RelationshipType::of("REPORTS_TO"));

    let filtered_multi = graph.relationship_type_filtered_graph(&city_focus)?;
    println!(
        "  VISITED + REPORTS_TO: {} relationships; sample nth target {:?}",
        filtered_multi.relationship_count(),
        filtered_multi.nth_target_or_not_found(0, 1)
    );

    Ok(())
}
