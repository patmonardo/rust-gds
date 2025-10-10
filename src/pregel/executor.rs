//! Pregel - Main executor for BSP graph computation
//!
//! The Pregel struct ties together all components and runs the
//! Bulk Synchronous Parallel (BSP) loop.

use crate::collections::HugeAtomicBitSet;
use crate::pregel::{
    projection::PropertyProjection, ComputeFn, DefaultValue, ForkJoinComputer, InitFn,
    MasterComputeContext, Messenger, NodeValue, PregelComputer, PregelConfig, PregelResult,
    PregelSchema, ProgressTracker,
};
use crate::types::graph::Graph;
use crate::types::properties::node::NodePropertyContainer;
use std::sync::Arc;

/// Main executor for Pregel computations.
///
/// Coordinates the Bulk Synchronous Parallel (BSP) loop:
/// 1. Initialize computation
/// 2. For each iteration:
///    - Initialize iteration
///    - Run compute step (parallel)
///    - Run master compute (convergence check)
///    - Check if converged
/// 3. Return results
///
/// # Example
///
/// ```ignore
/// use rust_gds::pregel::Pregel;
///
/// let pregel = Pregel::create(
///     graph,
///     config,
///     init_fn,
///     compute_fn,
///     progress_tracker,
/// )?;
///
/// let result = pregel.run()?;
///
/// if result.did_converge {
///     println!("Converged after {} iterations", result.ran_iterations);
/// }
/// ```
pub struct Pregel<C: PregelConfig + Clone, I: crate::pregel::MessageIterator> {
    /// Configuration for this computation
    config: C,

    /// The graph to compute on
    graph: Arc<dyn Graph>,

    /// Node property values (results)
    node_values: Arc<parking_lot::RwLock<NodeValue>>,

    /// Message passing system (held for lifecycle management)
    #[allow(dead_code)]
    messenger: Arc<dyn Messenger<I>>,

    /// The computer that executes iterations
    computer: ForkJoinComputer<C, I>,

    /// Progress tracking
    progress_tracker: Arc<ProgressTracker>,
}

impl<C: PregelConfig + Clone, I: crate::pregel::MessageIterator> Pregel<C, I> {
    /// Create a new Pregel executor.
    ///
    /// This is the main entry point for running Pregel computations.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph to compute on
    /// * `config` - Algorithm configuration
    /// * `schema` - Node property schema
    /// * `init_fn` - Initialization function (called once per node)
    /// * `compute_fn` - Compute function (called each iteration)
    /// * `messenger` - Message passing system
    /// * `progress_tracker` - Progress tracking
    ///
    /// # Returns
    ///
    /// A configured Pregel executor ready to run.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph: Arc<dyn Graph>,
        config: C,
        schema: PregelSchema,
        init_fn: InitFn<C>,
        compute_fn: ComputeFn<C, I>,
        messenger: Arc<dyn Messenger<I>>,
        progress_tracker: Arc<ProgressTracker>,
    ) -> Self {
        // Create node value storage based on schema
        let node_values = Arc::new(parking_lot::RwLock::new(NodeValue::of(
            &schema,
            graph.node_count() as u64,
            config.concurrency(),
        )));

        // Initialize node values from PropertyStore (if property_source is set)
        Self::initialize_from_property_store(&graph, &schema, &node_values);

        // Create vote bits for convergence tracking
        let vote_bits = Arc::new(HugeAtomicBitSet::new(graph.node_count()));

        // Create the computer
        let computer = ForkJoinComputer::new(
            Arc::clone(&graph),
            init_fn,
            compute_fn,
            config.clone(),
            Arc::clone(&node_values),
            Arc::clone(&messenger),
            Arc::clone(&vote_bits),
            Arc::clone(&progress_tracker),
        );

        Self {
            config,
            graph,
            node_values,
            messenger,
            computer,
            progress_tracker,
        }
    }

    /// Initialize node values from PropertyStore based on schema mappings.
    ///
    /// For each property in the schema that has a `property_source` set, this method
    /// attempts to load values from the PropertyStore and convert them to Pregel's
    /// DefaultValue format using the PropertyProjection trait.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph (must implement NodePropertyContainer)
    /// * `schema` - The Pregel schema with property_source mappings
    /// * `node_values` - The NodeValue storage to populate
    fn initialize_from_property_store(
        graph: &Arc<dyn Graph>,
        schema: &PregelSchema,
        node_values: &Arc<parking_lot::RwLock<NodeValue>>,
    ) {
        for element in schema.elements() {
            // Skip if no property_source is set
            let Some(source_key) = &element.property_source else {
                continue;
            };

            // Try to get property values from PropertyStore
            let Some(property_values) = graph.node_properties(source_key) else {
                // Property not found in store - silently continue (will use defaults)
                continue;
            };

            // Convert and populate values for all nodes
            let mut node_values_guard = node_values.write();
            for node_id in 0..graph.node_count() {
                // Use PropertyProjection to convert PropertyStore → Pregel DefaultValue
                if let Some(value) =
                    DefaultValue::from_property(property_values.as_ref(), node_id as u64)
                {
                    // Store the value in NodeValue based on type
                    match value {
                        DefaultValue::Long(v) => {
                            node_values_guard.set_long(&element.property_key, node_id, v);
                        }
                        DefaultValue::Double(v) => {
                            node_values_guard.set(&element.property_key, node_id, v);
                        }
                        DefaultValue::LongArray(v) => {
                            node_values_guard.set_long_array(&element.property_key, node_id, v);
                        }
                        DefaultValue::DoubleArray(v) => {
                            node_values_guard.set_double_array(&element.property_key, node_id, v);
                        }
                    }
                }
            }
        }
    }

    /// Run the Pregel computation.
    ///
    /// Executes the BSP loop until convergence or max iterations reached.
    ///
    /// # Returns
    ///
    /// `PregelResult` containing computed node values and execution metadata.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = pregel.run()?;
    /// println!("Ran {} iterations", result.ran_iterations);
    /// println!("Converged: {}", result.did_converge);
    /// ```
    pub fn run(mut self) -> PregelResult {
        let mut did_converge = false;

        // Initialize computation
        self.computer.init_computation();

        // Track progress
        self.progress_tracker.begin_task();

        let mut iteration = 0;
        for iter in 0..self.config.max_iterations() {
            iteration = iter;

            // Log iteration progress
            self.progress_tracker
                .log_progress(iteration, &format!("Starting iteration {}", iteration));

            // Initialize iteration in computer
            // (Messenger init is handled by computer/compute_step)
            self.computer.init_iteration(iteration);

            // Run the compute step (parallel execution)
            self.computer.run_iteration();

            // Run master compute step (convergence check)
            let master_converged = self.run_master_compute(iteration);

            // Check convergence
            did_converge = master_converged || self.computer.has_converged();

            if did_converge {
                self.progress_tracker.log_progress(iteration, "Converged!");
                break;
            }
        }

        self.progress_tracker.end_task();

        // Release resources
        self.computer.release();

        // Return results - unwrap Arc<RwLock<NodeValue>> to get NodeValue
        let node_values = Arc::try_unwrap(self.node_values)
            .map(|lock| lock.into_inner())
            .unwrap_or_else(|_arc| NodeValue::stub()); // Fallback if still shared

        PregelResult::new(node_values, iteration, did_converge)
    }

    /// Run the master compute step for convergence checking.
    ///
    /// The master compute runs in a single thread after each superstep
    /// and can signal early termination.
    fn run_master_compute(&self, iteration: usize) -> bool {
        // Create master compute context
        let context = MasterComputeContext::new(
            self.config.clone(),
            Arc::clone(&self.graph),
            iteration,
            Arc::clone(&self.node_values),
            Arc::clone(&self.progress_tracker),
        );

        // For now, always return false (don't terminate early)
        // In full implementation, this would call computation.master_compute(context)
        let _ = context; // Suppress unused warning
        false
    }
}

/// Builder for creating Pregel instances with a fluent API.
pub struct PregelBuilder<C: PregelConfig, I: crate::pregel::MessageIterator> {
    graph: Option<Arc<dyn Graph>>,
    config: Option<C>,
    schema: Option<PregelSchema>,
    init_fn: Option<InitFn<C>>,
    compute_fn: Option<ComputeFn<C, I>>,
    messenger: Option<Arc<dyn Messenger<I>>>,
    progress_tracker: Option<Arc<ProgressTracker>>,
}

impl<C: PregelConfig + Clone, I: crate::pregel::MessageIterator> PregelBuilder<C, I> {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            graph: None,
            config: None,
            schema: None,
            init_fn: None,
            compute_fn: None,
            messenger: None,
            progress_tracker: None,
        }
    }

    /// Set the graph.
    pub fn graph(mut self, graph: Arc<dyn Graph>) -> Self {
        self.graph = Some(graph);
        self
    }

    /// Set the configuration.
    pub fn config(mut self, config: C) -> Self {
        self.config = Some(config);
        self
    }

    /// Set the schema.
    pub fn schema(mut self, schema: PregelSchema) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Set the init function.
    pub fn init_fn(mut self, init_fn: InitFn<C>) -> Self {
        self.init_fn = Some(init_fn);
        self
    }

    /// Set the compute function.
    pub fn compute_fn(mut self, compute_fn: ComputeFn<C, I>) -> Self {
        self.compute_fn = Some(compute_fn);
        self
    }

    /// Set the messenger.
    pub fn messenger(mut self, messenger: Arc<dyn Messenger<I>>) -> Self {
        self.messenger = Some(messenger);
        self
    }

    /// Set the progress tracker.
    pub fn progress_tracker(mut self, progress_tracker: Arc<ProgressTracker>) -> Self {
        self.progress_tracker = Some(progress_tracker);
        self
    }

    /// Build the Pregel executor.
    ///
    /// # Panics
    ///
    /// Panics if any required field is missing.
    pub fn build(self) -> Pregel<C, I> {
        Pregel::new(
            self.graph.expect("graph is required"),
            self.config.expect("config is required"),
            self.schema.expect("schema is required"),
            self.init_fn.expect("init_fn is required"),
            self.compute_fn.expect("compute_fn is required"),
            self.messenger.expect("messenger is required"),
            self.progress_tracker.expect("progress_tracker is required"),
        )
    }
}

impl<C: PregelConfig + Clone, I: crate::pregel::MessageIterator> Default for PregelBuilder<C, I> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add integration tests once we have:
    // - Mock graph implementation
    // - Mock messenger implementation
    // - Simple test algorithm (e.g., node count)
}
