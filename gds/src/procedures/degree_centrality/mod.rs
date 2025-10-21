//! Degree Centrality Algorithm
//!
//! This module implements the Degree Centrality algorithm, which measures
//! the number of connections (edges) for each node in the graph.
//!
//! ## Architecture
//!
//! Following the Five-Fold Brahmachakra design:
//! - **spec.rs** - AlgorithmSpec implementation (Species)
//! - **storage.rs** - Storage Runtime (Gross pole - GraphStore access)
//! - **computation.rs** - Computation Runtime (Subtle pole - degree scores)
//!
//! ## Algorithm
//!
//! Degree Centrality is one of the simplest centrality measures:
//! 1. For each node, count its degree (number of edges)
//! 2. Optionally normalize by maximum degree
//! 3. Return degree scores for all nodes
//!
//! **Complexity**: O(V + E) - linear in nodes and edges
//! **Use Case**: Identify highly connected nodes (hubs)

pub mod spec;
pub mod storage;
pub mod computation;
pub mod integration_tests;

// Re-export the main types
pub use spec::{
    DEGREE_CENTRALITYAlgorithmSpec,
    DegreeCentralityConfig,
    DegreeCentralityResult,
};
pub use storage::DegreeCentralityStorageRuntime;
pub use computation::DegreeCentralityComputationRuntime;
