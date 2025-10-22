//! K-Core Decomposition
pub mod spec;
pub mod storage;
pub mod computation;
#[cfg(test)]
mod integration_tests;

pub use computation::KCoreDecompositionRuntime;
pub use spec::{KCoreConfig, KCoreResult, KCoreAlgorithmSpec};
pub use storage::KCoreStorageRuntime;
