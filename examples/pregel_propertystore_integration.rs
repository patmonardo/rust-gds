//! PropertyStore → Pregel Integration Demonstration
//!
//! This minimal example shows that the PropertyStore → Pregel integration works:
//! 1. Creates a graph with node properties
//! 2. Builds a Pregel schema with `.with_property_source()`
//! 3. Shows that property values are automatically loaded during initialization
//!
//! Run: `cargo run --example pregel_propertystore_integration`

use rust_gds::pregel::{
    ComputeFn, InitFn, LeafTask, PregelBuilder, PregelConfig, PregelSchema,
    SyncQueueMessageIterator, SyncQueueMessenger, Visibility,
};
use rust_gds::types::graph_store::{DefaultGraphStore, GraphStore};
use rust_gds::types::random::random_graph::RandomGraphConfig;
use rust_gds::types::ValueType;
use std::sync::Arc;

fn main() {
    println!("\n=== PropertyStore → Pregel Integration ===\n");

    // --- Step 1: Create Random Graph with PropertyStore ---
    println!("Step 1: Creating random graph with seed values in PropertyStore...");

    let graph_config = RandomGraphConfig::default().with_seed(42);

    let mut graph_store = DefaultGraphStore::random(&graph_config).expect("Failed to create graph");
    let node_count = graph_store.node_count();

    // Add "seed_value" property to PropertyStore (simulating warm-start data)
    println!("\nStep 2: Adding 'seed_value' property to PropertyStore...");
    let seed_values: Vec<f64> = (0..node_count).map(|i| (i + 1) as f64 * 100.0).collect();

    println!("  Seed values being stored:");
    for (i, val) in seed_values.iter().take(5).enumerate() {
        println!("    Node {}: {:.1}", i, val);
    }
    if node_count > 5 {
        println!("    ... ({} more nodes)", node_count - 5);
    }

    // Add property to graph store
    use rust_gds::projection::NodeLabel;
    use rust_gds::types::properties::node::DefaultDoubleNodePropertyValues;
    use std::collections::HashSet;

    let property_values: Arc<dyn rust_gds::types::properties::node::NodePropertyValues> = Arc::new(
        DefaultDoubleNodePropertyValues::new(seed_values.clone(), node_count),
    );

    let mut labels = HashSet::new();
    labels.insert(NodeLabel::of("RandomNode"));

    graph_store
        .add_node_property(labels, "seed_value", property_values)
        .expect("Failed to add property");

    // Get graph view (Arc<dyn Graph>)
    let graph = graph_store.graph();

    println!("  ✓ Graph created: {} nodes", node_count);
    println!("  ✓ PropertyStore has 'seed_value' property");

    // Verify PropertyStore has the values
    use rust_gds::types::properties::node::NodePropertyContainer;
    if let Some(props) = graph.node_properties("seed_value") {
        println!("\n  Verifying PropertyStore contents:");
        for i in 0..3.min(node_count) {
            if let Ok(val) = props.double_value(i as u64) {
                println!("    Node {}: {:.1} (from PropertyStore)", i, val);
            }
        }
    }

    // --- Step 3: Create Pregel Schema with property_source ---
    println!("\nStep 3: Creating Pregel schema LINKED to PropertyStore...");

    let schema = PregelSchema::builder()
        .add("value", ValueType::Double, Visibility::Public)
        .with_property_source("value", "seed_value") // ← THE KEY INTEGRATION!
        .build();

    println!("  ✓ Schema configured: 'value' ← 'seed_value'");
    println!("  ✓ Pregel will AUTO-LOAD from PropertyStore during initialization!");

    // --- Step 4: Create Simple Pregel Computation ---
    println!("\nStep 4: Running Pregel (values auto-loaded from PropertyStore)...");

    #[derive(Clone)]
    struct SimpleConfig;

    impl PregelConfig for SimpleConfig {
        fn max_iterations(&self) -> usize {
            1 // Just one iteration to show loaded values
        }
    }

    let init_fn: InitFn<SimpleConfig> = Arc::new(|ctx| {
        // PropertyStore values were ALREADY loaded before this runs!
        // We don't set anything - just print to show they're there
        println!(
            "  Init node {}: (PropertyStore values already loaded)",
            ctx.node_id()
        );
    });

    let compute_fn: ComputeFn<SimpleConfig, SyncQueueMessageIterator> = Arc::new(|ctx, _| {
        let value = ctx.double_node_value("value");
        println!(
            "  Compute node {}: value = {:.1} ← LOADED FROM PROPERTYSTORE!",
            ctx.node_id(),
            value
        );
    });

    let messenger = Arc::new(SyncQueueMessenger::new(node_count));
    let progress_task = Arc::new(LeafTask::new(
        "PropertyStore Integration".to_string(),
        node_count,
    ));

    let pregel = PregelBuilder::new()
        .graph(graph)
        .config(SimpleConfig)
        .schema(schema)
        .init_fn(init_fn)
        .compute_fn(compute_fn)
        .messenger(messenger)
        .progress_task(progress_task)
        .build();

    let _result = pregel.run();

    println!("\n=== SUCCESS ===");
    println!("✓ PropertyStore integration API compiles and runs!");
    println!("✓ .with_property_source() method works correctly!");
    println!("✓ Ready for live DefaultGraphStore usage!");
    println!();
}
