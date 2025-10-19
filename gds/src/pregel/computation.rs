//! PregelComputation trait - User-defined vertex programs
//!
//! This module defines the main trait that algorithm implementers use to express
//! their graph algorithms in the Pregel framework.

use super::context::{ComputeContext, InitContext, MasterComputeContext};
use super::messages::{MessageIterator, MessageReducer, Messages};
use super::{PregelRuntimeConfig, PregelSchema};

/// Main trait to express user-defined logic using the Pregel framework.
///
/// An algorithm is expressed using a node-centric view. A node can receive messages from
/// other nodes, change its state and send messages to other nodes in each iteration (superstep).
///
/// # Lifecycle
///
/// 1. **init()** - Called once per vertex before the first superstep (optional)
/// 2. **compute()** - Called each superstep for active vertices
///
/// # Message Passing
///
/// Since Pregel computation is stateless, nodes can only communicate via messages.
/// In each superstep, a node:
/// - Receives messages sent in the previous superstep
/// - Updates its own state based on those messages
/// - Sends messages to neighbors or any node (if ID is known)
/// - Can vote to halt (stop receiving compute calls)
///
/// # Example: PageRank
///
/// ```ignore
/// use rust_gds::pregel::{PregelComputation, ComputeContext, Messages};
///
/// struct PageRank {
///     damping_factor: f64,
/// }
///
/// impl PregelComputation for PageRank {
///     type Config = PageRankConfig;
///     
///     fn init(&mut self, context: &mut InitContext<Self::Config>) {
///         let initial_value = 1.0 / context.node_count() as f64;
///         context.set_node_value(initial_value);
///     }
///     
///     fn compute(&mut self, context: &mut ComputeContext<Self::Config>, messages: Messages) {
///         let sum: f64 = messages.iter().sum();
///         let new_rank = (1.0 - self.damping_factor) + self.damping_factor * sum;
///         
///         context.set_node_value(new_rank);
///         
///         // Send rank / degree to all neighbors
///         let degree = context.degree();
///         if degree > 0 {
///             context.send_to_neighbors(new_rank / degree as f64);
///         }
///     }
/// }
/// ```
///
/// # References
///
/// - [Pregel Paper](https://kowshik.github.io/JPregel/pregel_paper.pdf)
/// - Java GDS: `org.neo4j.gds.beta.pregel.PregelComputation`
pub trait PregelComputation {
    /// Configuration type for this computation.
    ///
    /// This associated type allows each Pregel algorithm to define its own
    /// configuration parameters (e.g., damping factor for PageRank, tolerance
    /// for convergence, etc.).
    type Config: PregelRuntimeConfig;

    /// Returns the schema for this computation (required).
    ///
    /// See `BasePregelComputation::schema()` for full documentation.
    fn schema(&self, config: &Self::Config) -> PregelSchema;

    /// Initialize node values before the first superstep.
    ///
    /// This method is called once per node in the beginning of the first superstep
    /// and allows initializing node values based on:
    /// - Node properties from the input graph
    /// - Algorithm configuration
    /// - Static initial values
    ///
    /// # Default Implementation
    ///
    /// The default implementation does nothing. Override this method if your
    /// algorithm needs custom initialization logic.
    ///
    /// # Arguments
    ///
    /// * `context` - Provides access to node properties and configuration
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn init(&mut self, context: &mut InitContext<Self::Config>) {
    ///     // Initialize all nodes to 1.0
    ///     context.set_node_value(1.0);
    ///     
    ///     // Or use a node property as initial value
    ///     if let Some(initial) = context.node_property("seed_value") {
    ///         context.set_node_value(initial);
    ///     }
    /// }
    /// ```
    fn init(&mut self, _context: &mut InitContext<Self::Config>) {
        // Default: no initialization
    }

    /// Compute function called for each active node in every superstep.
    ///
    /// This is the core of the Pregel algorithm. It's called individually for each
    /// node in every superstep as long as:
    /// - The node receives messages, OR
    /// - The node has not voted to halt yet
    ///
    /// # Message-Passing Model
    ///
    /// Since Pregel computation is stateless, nodes communicate only via messages:
    /// - **Input**: Messages sent to this node in the previous superstep
    /// - **Output**: Messages sent to other nodes via `context.send_to()`
    ///
    /// # Voting to Halt
    ///
    /// A node can vote to halt by calling `context.vote_to_halt()`. Once halted:
    /// - The node won't receive further `compute()` calls
    /// - UNLESS it receives a message (which reactivates it)
    ///
    /// The computation terminates when all nodes have voted to halt AND no
    /// messages are in flight.
    ///
    /// # Arguments
    ///
    /// * `context` - API for interacting with the framework (send messages, get/set values, etc.)
    /// * `messages` - Iterator over messages received in the previous superstep
    ///
    /// # Example: Connected Components
    ///
    /// ```ignore
    /// fn compute<I: MessageIterator>(
    ///     &mut self,
    ///     context: &mut ComputeContext<Self::Config>,
    ///     messages: &mut Messages<I>
    /// ) {
    ///     let current_component = context.node_value();
    ///     
    ///     // Find minimum component ID from messages
    ///     let min_component = messages
    ///         .map(|msg| msg as i64)
    ///         .min()
    ///         .unwrap_or(current_component);
    ///     
    ///     // If we found a smaller component, propagate it
    ///     if min_component < current_component {
    ///         context.set_node_value(min_component);
    ///         context.send_to_neighbors(min_component as f64);
    ///     } else {
    ///         // No change, vote to halt
    ///         context.vote_to_halt();
    ///     }
    /// }
    /// ```
    fn compute<I: MessageIterator>(
        &mut self,
        context: &mut ComputeContext<Self::Config, I>,
        messages: &mut Messages<I>,
    );

    /// Master compute step (optional, default returns false).
    ///
    /// See `BasePregelComputation::master_compute()` for full documentation.
    fn master_compute(&mut self, _context: &mut MasterComputeContext<Self::Config>) -> bool {
        false
    }

    /// Optional message reducer (default returns None).
    ///
    /// See `BasePregelComputation::reducer()` for full documentation.
    fn reducer(&self) -> Option<Box<dyn MessageReducer<f64>>> {
        None
    }

    /// Apply relationship weight (default returns unchanged value).
    ///
    /// See `BasePregelComputation::apply_relationship_weight()` for full documentation.
    fn apply_relationship_weight(&self, node_value: f64, _relationship_weight: f64) -> f64 {
        node_value
    }

    /// Cleanup resources (default does nothing).
    ///
    /// See `BasePregelComputation::close()` for full documentation.
    fn close(&mut self) {
        // Default: no cleanup
    }
}

/// Base trait for all Pregel computations.
///
/// This trait defines the common functionality that all Pregel algorithm implementations
/// must provide, including schema definition, optional master compute, message reduction,
/// and cleanup.
///
/// All types that implement `PregelComputation` automatically implement this trait.
///
/// # Required Methods
///
/// - `schema()` - Define the node property schema
///
/// # Optional Methods
///
/// - `master_compute()` - Global coordination after each superstep
/// - `reducer()` - Combine messages to reduce memory/compute
/// - `apply_relationship_weight()` - Apply edge weights to messages
/// - `close()` - Cleanup resources when computation finishes
pub trait BasePregelComputation {
    /// Configuration type for this computation.
    type Config: PregelRuntimeConfig;

    /// Returns the schema for this computation, defining which properties will be stored for each node.
    ///
    /// The schema describes the node property layout. A node property can be composed of
    /// multiple primitive values (long, double) as well as arrays of those. Each part of
    /// that composite schema is named by a unique key.
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn schema(&self, config: &Self::Config) -> PregelSchema {
    ///     PregelSchema::builder()
    ///         .add("rank", ValueType::Double, Visibility::Public)
    ///         .add("temp", ValueType::Long, Visibility::Private)
    ///         .build()
    /// }
    /// ```
    fn schema(&self, config: &Self::Config) -> PregelSchema;

    /// The master compute method is called exactly once after every superstep.
    ///
    /// It runs in a single thread and provides global coordination capabilities:
    /// - Check convergence criteria
    /// - Gather global statistics
    /// - Signal early termination
    /// - Log progress
    ///
    /// # Returns
    ///
    /// `true` if the computation converged and should stop, `false` to continue
    ///
    /// # Default Implementation
    ///
    /// The default implementation always returns `false` (never terminates early).
    fn master_compute(&mut self, _context: &mut MasterComputeContext<Self::Config>) -> bool {
        false // Don't terminate
    }

    /// Returns an optional message reducer that combines messages sent to the same target.
    ///
    /// Based on the reduce function, multiple messages are condensed into a single one.
    /// Use cases include computing the sum, count, minimum, or maximum of messages.
    ///
    /// Specifying a reducer can significantly reduce memory consumption and runtime
    /// of the computation.
    ///
    /// # Returns
    ///
    /// `Some(reducer)` if message reduction is desired, `None` otherwise
    fn reducer(&self) -> Option<Box<dyn MessageReducer<f64>>> {
        None
    }

    /// Apply relationship weight to a message value.
    ///
    /// If the input graph is weighted (relationships have a property), this method
    /// can be overridden to apply that weight on a message before it is read by
    /// the receiving node.
    ///
    /// If the input graph has no relationship properties (is unweighted), this
    /// method is skipped.
    ///
    /// # Arguments
    ///
    /// * `node_value` - The message value to be weighted
    /// * `relationship_weight` - The weight of the relationship/edge
    ///
    /// # Returns
    ///
    /// The weighted message value
    ///
    /// # Default Implementation
    ///
    /// Returns `node_value` unchanged (no weighting applied).
    fn apply_relationship_weight(&self, node_value: f64, _relationship_weight: f64) -> f64 {
        node_value
    }

    /// Cleanup method called at the very end of the computation.
    ///
    /// This is called after the end result has been produced and no more work is being done.
    /// Implement this method to close any resources that the computation opened,
    /// for example ThreadLocal storage or file handles.
    ///
    /// # Default Implementation
    ///
    /// Does nothing.
    fn close(&mut self) {
        // No resources to clean up by default
    }
}

// Blanket implementation: PregelComputation extends BasePregelComputation
impl<T: PregelComputation> BasePregelComputation for T {
    type Config = T::Config;

    fn schema(&self, config: &Self::Config) -> PregelSchema {
        self.schema(config)
    }

    fn master_compute(&mut self, context: &mut MasterComputeContext<Self::Config>) -> bool {
        self.master_compute(context)
    }

    fn reducer(&self) -> Option<Box<dyn MessageReducer<f64>>> {
        self.reducer()
    }

    fn apply_relationship_weight(&self, node_value: f64, relationship_weight: f64) -> f64 {
        self.apply_relationship_weight(node_value, relationship_weight)
    }

    fn close(&mut self) {
        self.close()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple test computation
    struct TestComputation {
        init_called: bool,
        compute_called: bool,
    }

    impl PregelComputation for TestComputation {
        type Config = crate::config::PregelConfig;

        fn schema(&self, _config: &Self::Config) -> PregelSchema {
            use crate::pregel::Visibility;
            use crate::types::ValueType;
            PregelSchema::builder()
                .add("test_value", ValueType::Long, Visibility::Public)
                .build()
        }

        fn init(&mut self, _context: &mut InitContext<Self::Config>) {
            self.init_called = true;
        }

        fn compute<I: MessageIterator>(
            &mut self,
            _context: &mut ComputeContext<Self::Config, I>,
            _messages: &mut Messages<I>,
        ) {
            self.compute_called = true;
        }
    }

    #[test]
    fn test_pregel_computation_trait() {
        // Just verify the trait compiles and can be implemented
        let comp = TestComputation {
            init_called: false,
            compute_called: false,
        };

        // We'd need actual contexts to call these, but we're just testing the API compiles
        assert!(!comp.init_called);
        assert!(!comp.compute_called);
    }

    #[test]
    fn test_base_pregel_computation_blanket_impl() {
        // Verify that implementing PregelComputation automatically gives BasePregelComputation
        fn is_base_pregel<T: BasePregelComputation>(_: &T) {}

        let comp = TestComputation {
            init_called: false,
            compute_called: false,
        };

        is_base_pregel(&comp); // Should compile
    }
}
