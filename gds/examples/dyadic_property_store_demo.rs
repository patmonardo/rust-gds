//! DyadicPropertyStore Example: The Binary Dance
//!
//! This example demonstrates the DyadicPropertyStore for ML Features/Labels
//! and Graph Source/Target relationships.

use gds::types::properties::hyper::dyadic::*;
use gds::types::properties::hyper::monadic::{MonadicProperty, MonadicLongPropertyValues};
use gds::types::properties::Property;
use gds::collections::backends::vec::VecLong;
use std::sync::Arc;

fn main() {
    println!("🎭 DyadicPropertyStore: The Binary Dance");
    println!("=========================================");
    
    // Example 1: ML Features/Labels
    println!("\n📊 ML Features/Labels Example:");
    ml_features_labels_example();
    
    // Example 2: Graph Source/Target
    println!("\n🕸️ Graph Source/Target Example:");
    graph_source_target_example();
}

fn ml_features_labels_example() {
    let mut ml_store = InputOutputPropertyStore::new();
    
    // Create features (input) property
    let features_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let features_prop = MonadicProperty::of("features", features_values);
    
    // Create labels (output) property  
    let labels_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let labels_prop = MonadicProperty::of("labels", labels_values);
    
    // Create dyadic property combining features and labels
    let features_labels = DyadicProperty::new(features_prop, labels_prop);
    
    // Store the ML training data
    ml_store.insert("training_data".to_string(), features_labels);
    
    // Retrieve and demonstrate
    if let Some(data) = ml_store.get("training_data") {
        println!("  ✅ Training data stored:");
        println!("    Features: {}", data.left().schema().key());
        println!("    Labels: {}", data.right().schema().key());
    }
    
    println!("  📈 ML Store size: {}", ml_store.len());
}

fn graph_source_target_example() {
    let mut graph_store = SourceTargetPropertyStore::new();
    
    // Create source node property
    let source_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let source_prop = MonadicProperty::of("source", source_values);
    
    // Create target node property
    let target_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let target_prop = MonadicProperty::of("target", target_values);
    
    // Create dyadic property for edge
    let edge = DyadicProperty::new(source_prop, target_prop);
    
    // Store multiple edges
    let edges = vec![
        ("edge_1", edge),
        ("edge_2", DyadicProperty::new(
            MonadicProperty::of("source", Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0))),
            MonadicProperty::of("target", Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0)))
        )),
    ];
    
    for (key, edge) in edges {
        graph_store.insert(key.to_string(), edge);
    }
    
    // Demonstrate iteration
    println!("  ✅ Graph edges stored:");
    for (key, edge) in graph_store.iter() {
        println!("    {}: {} → {}", 
            key, 
            edge.left().schema().key(), 
            edge.right().schema().key()
        );
    }
    
    println!("  🕸️ Graph Store size: {}", graph_store.len());
}
