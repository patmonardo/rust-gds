//! Computation Lifecycle Demo
//!
//! Demonstrates the full lifecycle of registering a ComputationDescriptor,
//! registering a Computer factory, instantiating a Computer, and running
//! the init → step → finalize cycle.
//!
//! This is the end-to-end example (Option C) showing how macro-generated
//! code will interact with the computation runtime infrastructure.

use std::sync::Arc;

use rust_gds::projection::computation_descriptor::{
    get_computation_descriptor, register_computation_descriptor,
};
use rust_gds::projection::pipeline_descriptor::PipelineDescriptor;
use rust_gds::projection::{
    instantiate_computer_from_descriptor, register_computer_factory, ComputationDescriptor,
    ComputationPattern, ComputationSpecies, ComputeContext, ComputeError, ComputeStep, Computer,
    Messages,
};
use rust_gds::types::graph::{Graph, IdMap};
use rust_gds::types::graph_store::DefaultGraphStore;

/// Example ComputeStep: simple PageRank-like iteration
struct PageRankStep {
    iteration: std::sync::atomic::AtomicUsize,
    max_iterations: usize,
}

impl PageRankStep {
    fn new(max_iterations: usize) -> Self {
        Self {
            iteration: std::sync::atomic::AtomicUsize::new(0),
            max_iterations,
        }
    }
}

impl ComputeStep for PageRankStep {
    fn compute(
        &self,
        ctx: &mut ComputeContext<'_>,
        messages: &Messages,
    ) -> Result<bool, ComputeError> {
        let iter = self
            .iteration
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        println!(
            "  [Step {}] Processing {} nodes, {} messages",
            iter, ctx.node_count, messages.payload_count
        );

        // Continue if we haven't reached max iterations
        let should_continue = iter < self.max_iterations;

        if !should_continue {
            println!("  [Step {}] Converged after {} iterations", iter, iter);
        }

        Ok(should_continue)
    }
}

/// Example Computer: PageRank computation
struct PageRankComputer {
    descriptor_id: u32,
    step: Box<dyn ComputeStep>,
    node_values: Vec<f64>,
}

impl PageRankComputer {
    fn new(descriptor_id: u32, max_iterations: usize) -> Self {
        Self {
            descriptor_id,
            step: Box::new(PageRankStep::new(max_iterations)),
            node_values: Vec::new(),
        }
    }
}

impl Computer for PageRankComputer {
    fn init(&mut self, ctx: &mut ComputeContext<'_>) -> Result<(), ComputeError> {
        println!("[Init] Initializing PageRank for {} nodes", ctx.node_count);

        // Allocate node values (initial rank = 1.0)
        self.node_values = vec![1.0; ctx.node_count as usize];

        println!("[Init] Allocated {} node values", self.node_values.len());
        Ok(())
    }

    fn step(&mut self, ctx: &mut ComputeContext<'_>) -> Result<bool, ComputeError> {
        // Delegate to step implementation
        self.step.compute(ctx, &Messages::empty())
    }

    fn finalize(&mut self, _ctx: &mut ComputeContext<'_>) -> Result<(), ComputeError> {
        println!(
            "[Finalize] Writing back {} node values",
            self.node_values.len()
        );

        // In real implementation: materialize_to_property_store
        // For demo: just show summary stats
        let sum: f64 = self.node_values.iter().sum();
        let avg = sum / self.node_values.len() as f64;

        println!("[Finalize] Final stats: sum={:.4}, avg={:.4}", sum, avg);
        println!(
            "[Finalize] Computation descriptor: {:?}",
            get_computation_descriptor(self.descriptor_id)
        );

        Ok(())
    }
}

/// Factory function that macro-generated code would provide
fn pagerank_factory(descriptor_id: u32) -> Result<Box<dyn Computer>, ComputeError> {
    Ok(Box::new(PageRankComputer::new(descriptor_id, 5)))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Computation Lifecycle Demo ===\n");

    // Step 1: Register ComputationDescriptor (what eval! macro would do)
    println!("Step 1: Register ComputationDescriptor");
    let descriptor = ComputationDescriptor::new(
        1,
        "page_rank_bsp",
        ComputationSpecies::Bsp,
        ComputationPattern::VertexCentric,
    )
    .with_description("PageRank using BSP computation pattern");

    let registered = register_computation_descriptor(descriptor.clone());
    println!("  Registered: {} (id={})\n", registered, descriptor.id);

    // Step 2: Register Computer factory (what eval! macro would do)
    println!("Step 2: Register Computer factory");
    let factory_registered = register_computer_factory(descriptor.id, pagerank_factory);
    println!("  Factory registered: {}\n", factory_registered);

    // Step 3: Create graph (what user code provides)
    println!("Step 3: Create graph");
    use rust_gds::types::random::RandomGraphConfig;
    let config = RandomGraphConfig::default().with_seed(42);
    let graph_store = DefaultGraphStore::random(&config).expect("graph creation failed");
    println!(
        "  Created random graph: {} nodes, {} relationships\n",
        graph_store.graph().node_count(),
        graph_store.graph().relationship_count()
    );

    // Step 4: Instantiate Computer from descriptor
    println!("Step 4: Instantiate Computer from descriptor");
    let mut computer = instantiate_computer_from_descriptor(descriptor.id)?;
    println!("  Computer instantiated\n");

    // Step 5: Run computation lifecycle
    println!("Step 5: Run computation lifecycle\n");

    let graph = graph_store.graph();
    let graph_arc: Arc<dyn Graph> = graph.clone();
    // Build a minimal PipelineDescriptor for this demo and pass it into the context.
    // PipelineDescriptor::new(name) and with_computation_flow(...) exist for ergonomic construction.
    let pipeline = PipelineDescriptor::new("demo_pipeline");
    let mut ctx = ComputeContext::new(&graph_arc, &pipeline, &descriptor);

    // Init
    computer.init(&mut ctx)?;
    println!();

    // Run steps until convergence
    let mut step_count = 0;
    loop {
        step_count += 1;
        let should_continue = computer.step(&mut ctx)?;

        if !should_continue {
            println!("  Converged after {} steps\n", step_count);
            break;
        }

        // Safety: prevent infinite loop
        if step_count > 100 {
            println!("  Max steps reached\n");
            break;
        }
    }

    // Finalize
    computer.finalize(&mut ctx)?;

    println!("\n=== Demo Complete ===");

    Ok(())
}
