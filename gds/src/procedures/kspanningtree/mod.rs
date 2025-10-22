//! K-Spanning Tree Module
//!
//! Implements k-spanning tree decomposition by computing an MST then
//! progressively cutting the k weakest edges.

pub mod spec;
pub mod storage;
pub mod computation;

#[cfg(test)]
mod integration_tests;

pub use spec::{KSpanningTreeConfig, KSpanningTreeResult, KSpanningTreeAlgorithmSpec};
pub use storage::KSpanningTreeStorageRuntime;
pub use computation::KSpanningTreeComputationRuntime;
