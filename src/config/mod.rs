//! Configuration system for rust-gds
//!
//! This module provides a type-safe, builder-based configuration system
//! for graph construction, algorithms, and I/O operations.
//!
//! # Architecture
//!
//! - **Base types**: Core configuration traits and types
//! - **Specific configs**: Algorithm, graph, and I/O configurations
//! - **Builders**: Type-safe configuration construction
//! - **Validation**: Compile-time and runtime validation
//! - **Serialization**: Optional YAML/JSON support via serde
//!
//! # Design Principles
//!
//! 1. Type safety over dynamic dispatch
//! 2. Sensible defaults via `Default` trait
//! 3. Builder pattern for complex configurations
//! 4. Validation at construction time
//! 5. Zero-cost abstractions

pub mod base_types;
pub mod algo_config;
pub mod graph_config;
pub mod graphstore_config;
pub mod io_config;
pub mod validation;

#[cfg(feature = "config-files")]
pub mod loader;

// Re-export core types for convenience
pub use base_types::*;
pub use algo_config::*;
pub use graph_config::*;
pub use graphstore_config::*;
pub use io_config::*;
pub use validation::*;

#[cfg(feature = "config-files")]
pub use loader::*;
