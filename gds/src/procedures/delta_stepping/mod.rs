//! Delta Stepping Algorithm Module
//!
//! **Translation Source**: `org.neo4j.gds.paths.delta.DeltaStepping`
//!
//! This module implements the Delta Stepping algorithm for parallel shortest path
//! computation using a sophisticated binning strategy for efficient frontier management.
//! Delta Stepping is particularly effective for graphs with varying edge weights.

pub mod spec;
pub mod storage;
pub mod computation;
pub mod integration_tests;

// Re-export main types for easy access
pub use spec::{DELTA_STEPPINGAlgorithmSpec, DeltaSteppingConfig, DeltaSteppingResult};
pub use storage::DeltaSteppingStorageRuntime;
pub use computation::DeltaSteppingComputationRuntime;
