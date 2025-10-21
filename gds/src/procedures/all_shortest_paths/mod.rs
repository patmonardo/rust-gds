//! All Shortest Paths Algorithm
//!
//! This module implements the All Shortest Paths algorithm for computing shortest paths
//! between all pairs of nodes in a graph.
//!
//! **Translation Source**: `org.neo4j.gds.allshortestpaths.*`
//! **Algorithm Types**: 
//! - Unweighted: Multi-Source BFS (MSBFS)
//! - Weighted: Multi-Source Dijkstra with Priority Queue
//!
//! **Key Features**:
//! - Multi-source parallelization
//! - Streaming results to avoid O(V²) memory usage
//! - Weighted/unweighted graph support
//! - Termination support

pub mod spec;
pub mod storage;
pub mod computation;
pub mod integration_tests;

// Re-exports for public API
pub use spec::{ALL_SHORTEST_PATHSAlgorithmSpec, AllShortestPathsConfig, AllShortestPathsResult};
pub use storage::{AllShortestPathsStorageRuntime, AlgorithmType};
pub use computation::AllShortestPathsComputationRuntime;
pub use storage::ShortestPathResult;
