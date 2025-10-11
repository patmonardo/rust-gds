//! PregelComputer - Coordinator for Pregel BSP execution
//!
//! The Computer is a simple container that holds all the components needed
//! for Pregel execution and orchestrates the BSP (Bulk Synchronous Parallel) loop.
//!
//! Design Philosophy:
//! - No complex context management (that's in ForkJoinComputeStep)
//! - Just container + coordinator
//! - Abstract interface with concrete ForkJoin implementation

use crate::collections::HugeAtomicBitSet;
use crate::core::utils::progress::tasks::LeafTask;
use crate::pregel::{
    ComputeFn, ForkJoinComputeStep, InitFn, MessageIterator, Messenger, NodeValue, Partition,
    PregelConfig,
};
use crate::types::graph::Graph;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Abstract coordinator for Pregel computation execution.
///
/// The Computer is a simple container that holds:
/// - Graph topology
/// - Computation logic (user's algorithm)
/// - Configuration
/// - Node values (property storage)
/// - Messenger (message passing)
/// - Vote bits (convergence tracking)
/// - Progress tracker
///
/// It orchestrates the BSP loop but delegates actual computation to ForkJoinComputeStep.
pub trait PregelComputer<C: PregelConfig> {
    /// Initialize the computation before any iterations run.
    fn init_computation(&mut self);

    /// Initialize a specific iteration/superstep.
    fn init_iteration(&mut self, iteration: usize);

    /// Run a single iteration of the BSP algorithm.
    fn run_iteration(&mut self);

    /// Check if the computation has converged.
    ///
    /// Convergence occurs when:
    /// 1. All nodes have voted to halt (voteBits.all_set())
    /// 2. No messages were sent in the last iteration
    fn has_converged(&self) -> bool;

    /// Release resources held by this computer.
    fn release(self);
}

/// ForkJoin-based implementation using Rayon for parallel execution.
///
/// This is the primary implementation that uses work-stealing parallelism
/// via Rayon (equivalent to Java's ForkJoinPool).
///
/// # Type Parameters
///
/// * `C` - PregelConfig type
/// * `I` - MessageIterator type
pub struct ForkJoinComputer<C: PregelConfig + Clone, I: MessageIterator> {
    /// Graph topology
    graph: Arc<dyn Graph>,

    /// Initialization function
    init_fn: InitFn<C>,

    /// Compute function
    compute_fn: ComputeFn<C, I>,

    /// Configuration
    config: C,

    /// Node property storage
    node_values: Arc<parking_lot::RwLock<NodeValue>>,

    /// Message passing system
    messenger: Arc<dyn Messenger<I>>,

    /// Vote-to-halt tracking
    vote_bits: Arc<HugeAtomicBitSet>,

    /// Progress tracking task (optional)
    progress_task: Option<Arc<LeafTask>>,

    /// Flag tracking if any message was sent in current iteration
    sent_message: Arc<AtomicBool>,

    /// Root task for current iteration (set by init_iteration)
    root_task: Option<ForkJoinComputeStep<C, I>>,
}

impl<C: PregelConfig + Clone, I: MessageIterator> ForkJoinComputer<C, I> {
    /// Create a new ForkJoinComputer.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph: Arc<dyn Graph>,
        init_fn: InitFn<C>,
        compute_fn: ComputeFn<C, I>,
        config: C,
        node_values: Arc<parking_lot::RwLock<NodeValue>>,
        messenger: Arc<dyn Messenger<I>>,
        vote_bits: Arc<HugeAtomicBitSet>,
        progress_task: Option<Arc<LeafTask>>,
    ) -> Self {
        Self {
            graph,
            init_fn,
            compute_fn,
            config,
            node_values,
            messenger,
            vote_bits,
            progress_task,
            sent_message: Arc::new(AtomicBool::new(false)),
            root_task: None,
        }
    }
}

impl<C: PregelConfig + Clone, I: MessageIterator> PregelComputer<C> for ForkJoinComputer<C, I> {
    fn init_computation(&mut self) {
        // "silence is golden" - Java comment
        // No initialization needed for ForkJoin strategy
    }

    fn init_iteration(&mut self, iteration: usize) {
        // Reset sent message flag for this iteration
        self.sent_message.store(false, Ordering::Relaxed);

        // Create partition covering all nodes
        let partition = Partition::new(0, self.graph.node_count());

        // Create root fork-join compute step for this iteration
        self.root_task = Some(ForkJoinComputeStep::new(
            Arc::clone(&self.init_fn),
            Arc::clone(&self.compute_fn),
            self.config.clone(),
            Arc::clone(&self.graph),
            partition,
            Arc::clone(&self.node_values),
            Arc::clone(&self.messenger),
            Arc::clone(&self.vote_bits),
            iteration,
            Arc::clone(&self.sent_message),
            self.progress_task.clone(),
        ));
    }

    fn run_iteration(&mut self) {
        // Execute the root task (will recursively subdivide via Rayon)
        if let Some(task) = self.root_task.take() {
            task.compute();
        }
    }

    fn has_converged(&self) -> bool {
        // Converged when no messages sent AND all nodes voted to halt
        !self.sent_message.load(Ordering::Relaxed) && self.vote_bits.all_set()
    }

    fn release(self) {
        // Release computation resources
        // (Arc drops will clean up everything else)
    }
}

/// Builder for creating PregelComputer instances.
///
/// Provides a fluent API for constructing computers with all required components.
pub struct PregelComputerBuilder<C: PregelConfig, I: MessageIterator> {
    graph: Option<Arc<dyn Graph>>,
    init_fn: Option<InitFn<C>>,
    compute_fn: Option<ComputeFn<C, I>>,
    config: Option<C>,
    node_values: Option<Arc<parking_lot::RwLock<NodeValue>>>,
    messenger: Option<Arc<dyn Messenger<I>>>,
    vote_bits: Option<Arc<HugeAtomicBitSet>>,
    progress_task: Option<Arc<LeafTask>>,
}

impl<C: PregelConfig + Clone, I: MessageIterator> PregelComputerBuilder<C, I> {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            graph: None,
            init_fn: None,
            compute_fn: None,
            config: None,
            node_values: None,
            messenger: None,
            vote_bits: None,
            progress_task: None,
        }
    }

    /// Set the graph.
    pub fn graph(mut self, graph: Arc<dyn Graph>) -> Self {
        self.graph = Some(graph);
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

    /// Set the config.
    pub fn config(mut self, config: C) -> Self {
        self.config = Some(config);
        self
    }

    /// Set the node values.
    pub fn node_values(mut self, node_values: Arc<parking_lot::RwLock<NodeValue>>) -> Self {
        self.node_values = Some(node_values);
        self
    }

    /// Set the messenger.
    pub fn messenger(mut self, messenger: Arc<dyn Messenger<I>>) -> Self {
        self.messenger = Some(messenger);
        self
    }

    /// Set the vote bits.
    pub fn vote_bits(mut self, vote_bits: Arc<HugeAtomicBitSet>) -> Self {
        self.vote_bits = Some(vote_bits);
        self
    }

    /// Set the progress task (optional).
    pub fn progress_task(mut self, progress_task: Arc<LeafTask>) -> Self {
        self.progress_task = Some(progress_task);
        self
    }

    /// Build a ForkJoinComputer.
    ///
    /// # Panics
    ///
    /// Panics if any required field is missing.
    pub fn build(self) -> ForkJoinComputer<C, I> {
        ForkJoinComputer::new(
            self.graph.expect("graph is required"),
            self.init_fn.expect("init_fn is required"),
            self.compute_fn.expect("compute_fn is required"),
            self.config.expect("config is required"),
            self.node_values.expect("node_values is required"),
            self.messenger.expect("messenger is required"),
            self.vote_bits.expect("vote_bits is required"),
            self.progress_task, // Optional
        )
    }
}

impl<C: PregelConfig + Clone, I: MessageIterator> Default for PregelComputerBuilder<C, I> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests once we have mock implementations
    // - Test init_computation (no-op)
    // - Test init_iteration creates root task
    // - Test run_iteration executes task
    // - Test has_converged checks sent_message and vote_bits
}
