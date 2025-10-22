//! Bridges Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.bridges.Bridges`
//!
//! This module finds all bridges (cut edges) in an undirected graph.

pub mod spec;
pub mod storage;
pub mod computation;
pub mod integration_tests;

pub use spec::{BridgesAlgorithmSpec, BridgesConfig, BridgesResult};
pub use storage::BridgesStorageRuntime;
pub use computation::BridgesComputationRuntime;
