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
pub use crate::config::{Partitioning, PregelConfig, PregelRuntimeConfig};
pub use computation::{BasePregelComputation, PregelComputation};
pub use compute_step::{ComputeFn, ForkJoinComputeStep, InitFn};
pub use computer::{ForkJoinComputer, PregelComputer, PregelComputerBuilder};
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

// Re-export progress tracking from core
pub use crate::core::utils::progress::tasks::{LeafTask, Task};
pub use crate::core::utils::progress::ProgressLogger;
