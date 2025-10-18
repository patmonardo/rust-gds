//! Algorithm implementations
//!
//! This module contains concrete algorithm implementations.
//! Each algorithm lives in a submodule and implements the `AlgorithmSpec` trait
//! from `src/projection/eval/procedure/algorithm_spec.rs`.
//!
//! ## Module Structure
//!
//! - `sum` - Sum aggregation algorithm
//! - `pagerank` - PageRank centrality algorithm

pub mod pagerank;
pub mod sum;

// Re-export public types for algorithm consumers
pub use pagerank::{PageRankAlgorithmSpec, PageRankComputationResult};
pub use sum::{SumAlgorithmSpec, SumConfig};
