//! Betweenness Centrality Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.betweenness.BetweennessCentrality`
//!
//! Measures node importance based on how often a node lies on shortest paths.
//! Uses two-phase algorithm: forward BFS + backward dependency propagation.

pub mod spec;
pub mod storage;
pub mod computation;
pub mod integration_tests;

pub use spec::{BetweennessCentralityAlgorithmSpec, BetweennessCentralityConfig, BetweennessCentralityResult};
pub use storage::BetweennessCentralityStorageRuntime;
pub use computation::BetweennessCentralityComputationRuntime;
