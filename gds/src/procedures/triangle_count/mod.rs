//! Triangle Count Algorithm
//!
//! Counts triangles in the graph using efficient edge-based enumeration.

pub mod spec;
pub mod storage;
pub mod computation;
pub mod integration_tests;

pub use spec::{TriangleCountAlgorithmSpec, TriangleCountConfig, TriangleCountResult};
pub use storage::TriangleCountStorageRuntime;
pub use computation::TriangleCountComputationRuntime;
