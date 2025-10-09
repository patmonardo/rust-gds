//! Pregel context types for vertex computation
//!
//! Provides the API that vertex programs use to interact with the Pregel framework.
//!
//! # Context Hierarchy
//!
//! ```text
//! PregelContext (base: config, logging, graph stats)
//! └── NodeCentricContext (node-specific: node ID, degree, neighbors, set values)
//!     ├── InitContext (initialization phase)
//!     └── ComputeContext (compute phase: messages, voting)
//!
//! MasterComputeContext (global coordination between supersteps)
//! ```

mod compute_context;
mod init_context;
mod master_compute_context;
mod node_centric_context;
mod pregel_context;

pub use compute_context::ComputeContext;
pub use init_context::InitContext;
pub use master_compute_context::MasterComputeContext;
pub use node_centric_context::{BidirectionalNodeCentricContext, NodeCentricContext};
pub use pregel_context::PregelContext;
