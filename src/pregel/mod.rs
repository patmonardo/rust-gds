//! Pregel - Bulk Synchronous Parallel (BSP) graph computation framework
//!
//! This module implements a Pregel-style computation model for large-scale graph algorithms.
//! Pregel is a vertex-centric programming model where computation proceeds in synchronized
//! supersteps, with vertices exchanging messages between steps.
//!
//! # Architecture
//!
//! - **Vertex Programs**: User-defined computation logic per vertex
//! - **Message Passing**: Vertices communicate via typed messages
//! - **Supersteps**: Synchronized computation phases with automatic barriers
//! - **Aggregators**: Global state accumulation across vertices
//! - **Master Compute**: Global coordination between supersteps
//!
//! # Example (Basic Structure)
//!
//! ```ignore
//! use rust_gds::pregel::{PregelComputation, PregelContext};
//!
//! struct PageRankComputation {
//!     damping_factor: f64,
//! }
//!
//! impl PregelComputation for PageRankComputation {
//!     type Config = PageRankConfig;
//!     
//!     fn compute(&mut self, context: &mut ComputeContext<Self::Config>, messages: Messages) {
//!         if context.is_initial_superstep() {
//!             let initial_value = 1.0 / context.node_count() as f64;
//!             context.set_node_value(initial_value);
//!         }
//!         
//!         let sum: f64 = messages.iter().sum();
//!         let new_value = (1.0 - self.damping_factor) + self.damping_factor * sum;
//!         context.set_node_value(new_value);
//!         
//!         // Send messages to neighbors
//!         let out_degree = context.degree();
//!         if out_degree > 0 {
//!             let message = new_value / out_degree as f64;
//!             context.send_to_neighbors(message);
//!         }
//!     }
//! }
//! ```

// Core traits and configuration
mod computation;
mod compute_step;
mod computer;
mod config;
pub mod context;
mod executor;
mod messages;
mod messengers;
mod node_value;
pub mod projection; // Optional PropertyStore ↔ Pregel bridges
mod queues;
mod reducers;
mod result;
mod schema;

// Re-exports from core (Partition now lives in core/utils)
pub use crate::core::utils::partition::Partition;

// Re-exports from this module
pub use computation::{BasePregelComputation, PregelComputation};
pub use compute_step::{ComputeFn, ForkJoinComputeStep, InitFn};
pub use computer::{ForkJoinComputer, PregelComputer, PregelComputerBuilder};
pub use config::{Partitioning, PregelConfig};
pub use context::{ComputeContext, InitContext, MasterComputeContext, NodeCentricContext};
pub use executor::{Pregel, PregelBuilder};
pub use messages::{
    empty_messages, EmptyMessageIterator, EmptyMessages, MessageIterator, MessageReducer, Messages,
    Messenger,
};
pub use messengers::{
    AsyncQueueMessageIterator, AsyncQueueMessenger, ReducingMessageIterator, ReducingMessenger,
    SyncQueueMessageIterator, SyncQueueMessenger,
};
pub use node_value::NodeValue;
pub use projection::{default_value_to_gds, materialize_pregel_values, PropertyProjection};
pub use queues::{AsyncDoubleQueues, AsyncQueueIterator, SyncDoubleQueues, SyncQueueIterator};
pub use reducers::{CountReducer, MaxReducer, MinReducer, Reducer, SumReducer};
pub use result::PregelResult;
pub use schema::{DefaultValue, Element, PregelSchema, PregelSchemaBuilder, Visibility};

// Mock ProgressTracker until we implement the full Java GDS Task Progress Tracking system
//
// TODO: Replace with full implementation that includes:
// - Task hierarchy and nesting
// - Progress percentage tracking
// - Time estimation
// - Logging integration
// - Concurrent task updates
// - Task cancellation
//
// See: org.neo4j.gds.core.utils.progress.tasks.ProgressTracker (Java GDS)

/// Mock progress tracker for Pregel computation.
///
/// This is a placeholder until we implement the full Java GDS ProgressTracker,
/// which provides comprehensive task progress tracking with:
/// - Hierarchical task structure
/// - Progress percentage and ETA
/// - Logging and monitoring hooks
/// - Concurrent-safe updates
///
/// # Design Note
///
/// The Java GDS ProgressTracker is a sophisticated system for tracking
/// long-running graph algorithm progress. We'll implement it properly
/// when needed, but for now this mock lets us focus on Pregel logic.
#[derive(Debug, Clone)]
pub struct ProgressTracker {
    task_name: String,
    enabled: bool,
}

impl ProgressTracker {
    /// Creates a new progress tracker for the given task.
    pub fn new(task_name: impl Into<String>) -> Self {
        Self {
            task_name: task_name.into(),
            enabled: true,
        }
    }

    /// Creates a disabled progress tracker (no-op).
    pub fn disabled() -> Self {
        Self {
            task_name: String::new(),
            enabled: false,
        }
    }

    /// Begins tracking progress for this task.
    pub fn begin_task(&self) {
        if self.enabled {
            println!("[ProgressTracker] Starting task: {}", self.task_name);
        }
    }

    /// Logs progress for the current superstep.
    pub fn log_progress(&self, superstep: usize, message: &str) {
        if self.enabled {
            println!("[ProgressTracker] Superstep {}: {}", superstep, message);
        }
    }

    /// Marks the task as complete.
    pub fn end_task(&self) {
        if self.enabled {
            println!("[ProgressTracker] Completed task: {}", self.task_name);
        }
    }

    /// Updates progress for a subtask (e.g., within a superstep).
    pub fn log_subtask(&self, subtask: &str, progress: f64) {
        if self.enabled {
            println!(
                "[ProgressTracker] {} - {:.1}% complete",
                subtask,
                progress * 100.0
            );
        }
    }

    /// Returns whether progress tracking is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self::disabled()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_tracker_new() {
        let tracker = ProgressTracker::new("PageRank");
        assert!(tracker.is_enabled());
    }

    #[test]
    fn test_progress_tracker_disabled() {
        let tracker = ProgressTracker::disabled();
        assert!(!tracker.is_enabled());
    }

    #[test]
    fn test_progress_tracker_lifecycle() {
        let tracker = ProgressTracker::new("TestAlgorithm");
        tracker.begin_task();
        tracker.log_progress(0, "Initializing");
        tracker.log_progress(1, "Computing");
        tracker.log_subtask("Message passing", 0.5);
        tracker.end_task();
        // Just verify it doesn't panic
    }

    #[test]
    fn test_progress_tracker_default() {
        let tracker = ProgressTracker::default();
        assert!(!tracker.is_enabled());
    }
}
