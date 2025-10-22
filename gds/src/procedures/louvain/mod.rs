//! Louvain Community Detection
pub mod spec;
pub mod storage;
pub mod computation;
pub mod integration_tests;

pub use spec::{LouvainAlgorithmSpec, LouvainConfig, LouvainResult};
pub use storage::LouvainStorageRuntime;
pub use computation::LouvainComputationRuntime;
