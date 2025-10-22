//! Weakly Connected Components (WCC) Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.wcc.Wcc`
//!
//! Finds all weakly connected components in an undirected graph.

pub mod spec;
pub mod storage;
pub mod computation;
pub mod integration_tests;

pub use spec::{WccAlgorithmSpec, WccConfig, WccResult};
pub use storage::WccStorageRuntime;
pub use computation::WccComputationRuntime;
