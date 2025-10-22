//! Label Propagation Module
//!
//! Implements label propagation algorithm for community detection
//! by iteratively propagating labels through node voting.

pub mod spec;
pub mod storage;
pub mod computation;

#[cfg(test)]
mod integration_tests;

pub use spec::{LabelPropConfig, LabelPropResult, LabelPropAlgorithmSpec};
pub use storage::LabelPropStorageRuntime;
pub use computation::LabelPropComputationRuntime;
