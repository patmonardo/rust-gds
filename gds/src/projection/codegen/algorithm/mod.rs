//! Algorithm Code Generation
//!
//! This module contains macros and utilities for generating algorithm infrastructure:
//! - `define_algorithm!` - Generate complete algorithm specifications
//! - `algorithm_config!` - Generate configuration structs + builders
//! - Test utilities for algorithm generation
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gds::projection::codegen::algorithm::*;
//!
//! // Define a complete algorithm
//! define_algorithm! {
//!     name: "pagerank",
//!     category: Centrality,
//!     config: { ... },
//!     result: PageRankResult { ... },
//!     projection_hint: Dense,
//!     modes: [Stream, Stats],
//!     execute: |graph_store, config, context| { ... }
//! }
//! ```

// Import the macros
#[macro_use]
mod define_algorithm;
#[macro_use]
mod test_algorithm;
#[macro_use]
mod focused_macros;

// Re-export the main macros
pub use crate::define_algorithm;
pub use crate::define_algorithm_spec;
pub use crate::define_storage_runtime;
pub use crate::define_computation_runtime;
