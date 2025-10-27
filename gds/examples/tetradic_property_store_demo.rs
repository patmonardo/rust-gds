//! TetradicPropertyStore Example: The Dyadic Plane (Dyadic²)
//!
//! This example demonstrates the TetradicPropertyStore for ML Pipeline
//! using the Dyadic² composition pattern.

use gds::types::properties::hyper::tetradic::*;
use gds::types::properties::hyper::dyadic::DyadicProperty;
use gds::types::properties::hyper::monadic::{MonadicProperty, MonadicLongPropertyValues};
use gds::types::properties::Property;
use gds::collections::backends::vec::VecLong;
use std::sync::Arc;

fn main() {
    println!("🎭 TetradicPropertyStore: The Dyadic Plane (Dyadic²)");
    println!("==================================================");
    
    // Example 1: ML Pipeline - Input/Output × Training/Validation
    println!("\n📊 ML Pipeline Example:");
    ml_pipeline_example();
    
    // Example 2: Graph ML Pipeline - Features/Labels × Source/Target
    println!("\n🕸️ Graph ML Pipeline Example:");
    graph_ml_pipeline_example();
}

fn ml_pipeline_example() {
    let mut ml_pipeline_store = MLPipelinePropertyStore::new();
    
    // Create primary dyadic: Input/Output
    let input_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let input_prop = MonadicProperty::of("input", input_values);
    let output_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let output_prop = MonadicProperty::of("output", output_values);
    let primary_dyadic = DyadicProperty::new(input_prop, output_prop);
    
    // Create secondary dyadic: Training/Validation
    let train_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let train_prop = MonadicProperty::of("train", train_values);
    let val_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let val_prop = MonadicProperty::of("validation", val_values);
    let secondary_dyadic = DyadicProperty::new(train_prop, val_prop);
    
    // Create tetradic property: Dyadic²
    let ml_pipeline = TetradicProperty::new(primary_dyadic, secondary_dyadic);
    
    // Store the ML pipeline
    ml_pipeline_store.insert("ml_pipeline".to_string(), ml_pipeline);
    
    // Retrieve and demonstrate
    if let Some(pipeline) = ml_pipeline_store.get("ml_pipeline") {
        println!("  ✅ ML Pipeline stored:");
        println!("    Primary: {} → {}", 
            pipeline.primary().left().schema().key(), 
            pipeline.primary().right().schema().key()
        );
        println!("    Secondary: {} → {}", 
            pipeline.secondary().left().schema().key(), 
            pipeline.secondary().right().schema().key()
        );
    }
    
    println!("  📈 ML Pipeline Store size: {}", ml_pipeline_store.len());
}

fn graph_ml_pipeline_example() {
    let mut graph_ml_store = GraphMLPipelinePropertyStore::new();
    
    // Create Features/Labels × Source/Target pipelines
    let pipelines = vec![
        ("pipeline_1", create_graph_ml_pipeline("features_1", "labels_1", "source_1", "target_1")),
        ("pipeline_2", create_graph_ml_pipeline("features_2", "labels_2", "source_2", "target_2")),
    ];
    
    for (key, pipeline) in pipelines {
        graph_ml_store.insert(key.to_string(), pipeline);
    }
    
    // Demonstrate iteration
    println!("  ✅ Graph ML Pipelines stored:");
    for (key, pipeline) in graph_ml_store.iter() {
        println!("    {}: {} → {} × {} → {}", 
            key,
            pipeline.primary().left().schema().key(), 
            pipeline.primary().right().schema().key(),
            pipeline.secondary().left().schema().key(), 
            pipeline.secondary().right().schema().key()
        );
    }
    
    println!("  🕸️ Graph ML Store size: {}", graph_ml_store.len());
}

fn create_graph_ml_pipeline(features: &str, labels: &str, source: &str, target: &str) -> TetradicProperty {
    // Create primary dyadic: Features/Labels
    let features_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let features_prop = MonadicProperty::of(features, features_values);
    let labels_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let labels_prop = MonadicProperty::of(labels, labels_values);
    let primary_dyadic = DyadicProperty::new(features_prop, labels_prop);
    
    // Create secondary dyadic: Source/Target
    let source_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let source_prop = MonadicProperty::of(source, source_values);
    let target_values = Arc::new(MonadicLongPropertyValues::new(VecLong::new(), 0));
    let target_prop = MonadicProperty::of(target, target_values);
    let secondary_dyadic = DyadicProperty::new(source_prop, target_prop);
    
    TetradicProperty::new(primary_dyadic, secondary_dyadic)
}
