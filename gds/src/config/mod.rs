//! Configuration system for rust-gds
//!
//! This module provides a type-safe, builder-based configuration system
//! for graph construction, algorithms, and I/O operations.

pub mod algo_config;
pub mod base_types;
pub mod graph_config;
pub mod graphstore_config;
pub mod io_config;
pub mod model_config;
pub mod morph_config;
pub mod pregel_config;
pub mod validation;

// Re-export core types for convenience
pub use crate::core::utils::partition::Partitioning;
pub use algo_config::*;
pub use base_types::*;
pub use graph_config::*;
pub use graphstore_config::*;
pub use io_config::*;
pub use model_config::*;
pub use morph_config::*;
pub use pregel_config::*;
pub use validation::*;
