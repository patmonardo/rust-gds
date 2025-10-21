//! **Breadth-First Search (BFS) Algorithm**
//!
//! **Translation Source**: `org.neo4j.gds.traversal.BFS`
//!
//! This module implements the Breadth-First Search algorithm for graph traversal,
//! providing efficient exploration of nodes level by level from a source node.

pub mod spec;
pub mod storage;
pub mod computation;
pub mod integration_tests;

// Re-export main types
pub use spec::{BFSAlgorithmSpec, BfsConfig, BfsResult};
pub use storage::BfsStorageRuntime;
pub use computation::BfsComputationRuntime;
