//! Connected Components using Pregel
//!
//! This example demonstrates how to write a Pregel computation.
//! It shows the structure of a complete algorithm implementation.
//!
//! ## Algorithm
//!
//! Each node starts with its own ID as its component label. In each superstep:
//! 1. Send your current component label to all neighbors
//! 2. Receive labels from neighbors
//! 3. Update to the minimum label seen
//! 4. If your label changed, continue; otherwise vote to halt
//!
//! ## Run this example
//!
//! ```bash
//! cargo run --example pregel_connected_components --features core
//! ```

use rust_gds::pregel::{
    ComputeContext, InitContext, MasterComputeContext, MessageIterator, Messages,
    PregelComputation, PregelConfig, PregelSchema,
};

// ================================================================================================
// Configuration
// ================================================================================================

/// Configuration for Connected Components computation.
#[derive(Debug, Clone)]
struct ConnectedComponentsConfig {
    max_iterations: usize,
}

impl PregelConfig for ConnectedComponentsConfig {
    fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn is_asynchronous(&self) -> bool {
        false // Use synchronous BSP model
    }
}

impl Default for ConnectedComponentsConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
        }
    }
}

// ================================================================================================
// Computation
// ================================================================================================

/// Connected Components Pregel computation.
struct ConnectedComponents;

impl ConnectedComponents {
    fn new() -> Self {
        Self
    }
}

impl PregelComputation for ConnectedComponents {
    type Config = ConnectedComponentsConfig;

    fn schema(&self, _config: &Self::Config) -> PregelSchema {
        use rust_gds::types::ValueType;

        PregelSchema::builder()
            .add_public("component", ValueType::Double)
            .build()
    }

    fn init(&mut self, context: &mut InitContext<Self::Config>) {
        // Initialize each node with its own ID as component label
        let node_id = context.node_id();
        context.set_node_value("component", node_id as f64);

        println!("  Init: Node {} → component {}", node_id, node_id);
    }

    fn compute<I: MessageIterator>(
        &mut self,
        context: &mut ComputeContext<Self::Config, I>,
        messages: &mut Messages<I>,
    ) {
        let node_id = context.node_id();
        let current_component = context.double_node_value("component");

        if context.is_initial_superstep() {
            // First superstep: send our component ID to all neighbors
            println!(
                "  Superstep {}: Node {} broadcasts component {}",
                context.superstep(),
                node_id,
                current_component
            );
            context.send_to_neighbors(current_component);
        } else {
            // Find minimum component ID from messages
            let mut min_component = current_component;
            let mut message_count = 0;

            // Iterate through messages
            while let Some(message) = messages.next() {
                message_count += 1;
                if message < min_component {
                    min_component = message;
                }
            }

            println!(
                "  Superstep {}: Node {} (comp {:.0}) got {} msgs, min = {:.0}",
                context.superstep(),
                node_id,
                current_component,
                message_count,
                min_component
            );

            // If we found a smaller component ID, update and propagate
            if min_component < current_component {
                context.set_node_value("component", min_component);
                context.send_to_neighbors(min_component);
                println!(
                    "    → Updated: {:.0} → {:.0}",
                    current_component, min_component
                );
            } else {
                // No change, vote to halt
                context.vote_to_halt();
                println!("    → Voting to halt (stable at {:.0})", current_component);
            }
        }
    }

    fn master_compute(&mut self, context: &mut MasterComputeContext<Self::Config>) -> bool {
        println!("\n--- Superstep {} complete ---\n", context.superstep());
        true // Continue
    }
}

// ================================================================================================
// Main Example
// ================================================================================================

fn main() {
    let banner = "=".repeat(80);

    println!("{}", banner);
    println!("Pregel Example: Connected Components");
    println!("{}", banner);
    println!();

    println!("This example shows the structure of a complete Pregel algorithm.");
    println!();

    // Create configuration
    let config = ConnectedComponentsConfig { max_iterations: 10 };

    // Create computation
    let computation = ConnectedComponents::new();

    println!(
        "✓ Configuration: max_iterations = {}",
        config.max_iterations
    );
    println!("✓ Algorithm: Connected Components (label propagation)");
    println!();

    // Get schema
    let schema = computation.schema(&config);
    println!("✓ Schema: {} properties", schema.elements().len());
    println!();

    println!("{}", banner);
    println!("Example Graph (3 components):");
    println!("{}", banner);
    println!();
    println!("  Component 0: Nodes 0 — 1 — 2");
    println!("  Component 3: Nodes 3 — 4");
    println!("  Component 5: Node 5 (isolated)");
    println!();

    println!("{}", banner);
    println!("How it works:");
    println!("{}", banner);
    println!();
    println!("1. Init: Each node starts with component = node_id");
    println!("2. Superstep 0: Broadcast component ID to neighbors");
    println!("3. Superstep 1+: Receive IDs, adopt minimum, propagate if changed");
    println!("4. Converge: All nodes vote to halt when stable");
    println!();

    println!("{}", banner);
    println!("Expected Result:");
    println!("{}", banner);
    println!();
    println!("  Nodes 0, 1, 2 → Component 0");
    println!("  Nodes 3, 4    → Component 3");
    println!("  Node 5        → Component 5");
    println!();

    println!("{}", banner);
    println!("✅ Connected Components Pregel computation ready!");
    println!("{}", banner);
    println!();
    println!("Next step: Wire up PregelExecutor to run this on a real graph");
}
