//! K1-Coloring Graph Coloring Algorithm
pub mod spec;
pub mod storage;
pub mod computation;
pub mod integration_tests;

pub use spec::{K1ColoringAlgorithmSpec, K1ColoringConfig, K1ColoringResult};
pub use storage::K1ColoringStorageRuntime;
pub use computation::K1ColoringComputationRuntime;
