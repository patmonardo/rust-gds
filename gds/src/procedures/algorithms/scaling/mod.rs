//! Scaling primitives for algorithm property normalization
//!
//! This module provides a Rust port of the Neo4j GDS scaling stack
//! (scalar scalers and a factory/registry) for use across algorithms.

pub mod traits;
pub mod properties;
pub mod factory;
pub mod none;
pub mod l1norm;
pub mod l2norm;
pub mod center;
pub mod max;
pub mod minmax;
pub mod mean;
pub mod stdscore;
pub mod log;


