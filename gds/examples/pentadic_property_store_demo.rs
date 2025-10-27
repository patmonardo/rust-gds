//! PentadicPropertyStore Example: The Dyadic × Triadic Composition
//!
//! This example demonstrates the PentadicPropertyStore for Graph Algo Procedures
//! using the Dyadic × Triadic composition pattern.

use gds::types::properties::hyper::pentadic::*;
use gds::types::properties::hyper::dyadic::DyadicProperty;
use gds::types::properties::hyper::triadic::TriadicProperty;
use gds::types::properties::hyper::monadic::{MonadicProperty, MonadicLongPropertyValues};
use gds::types::properties::Property;
use gds::collections::backends::vec::VecLong;
use std::sync::Arc;

fn main() {
    println!("🎭 PentadicPropertyStore: The Dyadic × Triadic Composition");
    println!("=========================================================");
    
    // Example 1: Graph Algo Procedure - Input/Output × Meta/Node/Link
    println!("\n🕸️ Graph Algo Procedure Example:");
    graph_algo_procedure_example();
    
    // Example 2: Graph ML with Topology - Features/Labels × Meta/Node/Link
    println!("\n🧠 Graph ML with Topology Example:");
    graph_ml_topology_example();
}

fn graph_algo_procedure_example() {
    let mut graph_algo_store = GraphAlgoProcedurePropertyStore::new();
    
    // Create dyadic: Input/Output
    let input_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let input_prop = MonadicProperty::of("input", input_values);
    let output_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let output_prop = MonadicProperty::of("output", output_values);
    let dyadic_prop = DyadicProperty::new(input_prop, output_prop);
    
    // Create triadic: Meta/Node/Link
    let meta_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let meta_prop = MonadicProperty::of("meta", meta_values);
    let node_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let node_prop = MonadicProperty::of("node", node_values);
    let link_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let link_prop = MonadicProperty::of("link", link_values);
    let triadic_prop = TriadicProperty::new(meta_prop, node_prop, link_prop);
    
    // Create pentadic property: Dyadic × Triadic
    let graph_algo = PentadicProperty::new(dyadic_prop, triadic_prop);
    
    // Store the graph algorithm procedure
    graph_algo_store.insert("dijkstra".to_string(), graph_algo);
    
    // Retrieve and demonstrate
    if let Some(algo) = graph_algo_store.get("dijkstra") {
        println!("  ✅ Graph Algorithm stored:");
        println!("    Dyadic: {} → {}", 
            algo.dyadic().left().schema().key(), 
            algo.dyadic().right().schema().key()
        );
        println!("    Triadic: {} / {} / {}", 
            algo.triadic().meta().schema().key(),
            algo.triadic().node().schema().key(), 
            algo.triadic().link().schema().key()
        );
    }
    
    println!("  🕸️ Graph Algo Store size: {}", graph_algo_store.len());
}

fn graph_ml_topology_example() {
    let mut graph_ml_store = GraphMLWithTopologyPropertyStore::new();
    
    // Create Features/Labels × Meta/Node/Link procedures
    let procedures = vec![
        ("gnn_procedure", create_graph_ml_topology("features", "labels", "meta", "node", "link")),
        ("gat_procedure", create_graph_ml_topology("attention_features", "attention_labels", "attention_meta", "attention_node", "attention_link")),
    ];
    
    for (key, procedure) in procedures {
        graph_ml_store.insert(key.to_string(), procedure);
    }
    
    // Demonstrate iteration
    println!("  ✅ Graph ML Procedures stored:");
    for (key, procedure) in graph_ml_store.iter() {
        println!("    {}: {} → {} × {} / {} / {}", 
            key,
            procedure.dyadic().left().schema().key(), 
            procedure.dyadic().right().schema().key(),
            procedure.triadic().meta().schema().key(),
            procedure.triadic().node().schema().key(), 
            procedure.triadic().link().schema().key()
        );
    }
    
    println!("  🧠 Graph ML Store size: {}", graph_ml_store.len());
}

fn create_graph_ml_topology(features: &str, labels: &str, meta: &str, node: &str, link: &str) -> PentadicProperty {
    // Create dyadic: Features/Labels
    let features_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let features_prop = MonadicProperty::of(features, features_values);
    let labels_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let labels_prop = MonadicProperty::of(labels, labels_values);
    let dyadic_prop = DyadicProperty::new(features_prop, labels_prop);
    
    // Create triadic: Meta/Node/Link
    let meta_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let meta_prop = MonadicProperty::of(meta, meta_values);
    let node_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let node_prop = MonadicProperty::of(node, node_values);
    let link_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let link_prop = MonadicProperty::of(link, link_values);
    let triadic_prop = TriadicProperty::new(meta_prop, node_prop, link_prop);
    
    PentadicProperty::new(dyadic_prop, triadic_prop)
}
