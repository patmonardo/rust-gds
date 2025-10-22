//! HITS Algorithm (Hyperlink-Induced Topic Search)
//!
//! This module implements the HITS algorithm for discovering hub and authority nodes
//! in directed graphs.
//!
//! ## What is HITS?
//!
//! HITS is an iterative algorithm that assigns two scores to each node:
//! - **Authority**: How authoritative is this node (many high-hub nodes point to it)
//! - **Hub**: How good is this node at pointing to authorities (points to many authoritative nodes)
//!
//! ## Algorithm Overview
//!
//! HITS operates in phases:
//! 1. **INIT**: Initialize hub and authority values to 1
//! 2. **CALCULATE_AUTHS**: Authority = sum of incoming hub values
//! 3. **NORMALIZE_AUTHS**: Normalize authority values by L2 norm
//! 4. **CALCULATE_HUBS**: Hub = sum of outgoing authority values
//! 5. **NORMALIZE_HUBS**: Normalize hub values by L2 norm
//! 6. Repeat steps 2-5 until convergence
//!
//! ## Architecture
//!
//! Following the Five-Fold Brahmachakra design:
//! - **spec.rs** - AlgorithmSpec implementation (Species)
//! - **storage.rs** - Storage Runtime (Gross pole - GraphStore access)
//! - **computation.rs** - Computation Runtime (Subtle pole - hub/authority scores)

pub mod spec;
pub mod storage;
pub mod computation;
pub mod integration_tests;

// Re-export main types
pub use spec::{
    HITSAlgorithmSpec,
    HitsConfig,
    HitsResult,
};
pub use storage::HitsStorageRuntime;
pub use computation::HitsComputationRuntime;
