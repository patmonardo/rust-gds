//! # Procedure Facades - Idiomatic Rust API Layer
//!
//! This module provides user-facing facades for all translated graph algorithms.
//! Each facade exposes a builder pattern with fluent configuration and multiple
//! execution modes (stream, stats, mutate, write).
//!
//! ## Design Philosophy
//!
//! - **User-First**: Designed for Rust users, not Java translations
//! - **Type-Safe**: Strong typing for all configurations and results
//! - **Fluent Builders**: Chainable configuration methods
//! - **Multiple Modes**: Each algorithm supports all relevant execution modes
//! - **Documented**: Every facade documents algorithm parameters and edge cases
//!
//! ## Architecture
//!
//! ```text
//! User Code
//!     ↓
//! Facade Layer (this module)
//!     ├─ Centrality facades (PageRank, DegreeCentrality, etc.)
//!     ├─ Community facades (Louvain, LabelPropagation, etc.)
//!     ├─ Path facades (Dijkstra, BFS, DFS, etc.)
//!     └─ Other facades (TriangleCount, K-Core, etc.)
//!     ↓
//! Algorithm Spec (procedures/*/spec.rs)
//!     ↓
//! Computation Runtime (procedures/*/computation.rs)
//!     ↓
//! Storage Runtime (procedures/*/storage.rs)
//!     ↓
//! GraphStore
//! ```
//!
//! ## Usage Pattern
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph = Graph::default();
//! // Centrality algorithms use builder pattern
//! let results: Vec<_> = graph
//!     .pagerank()
//!     .iterations(20)
//!     .damping_factor(0.85)
//!     .stream()?
//!     .collect();
//!
//! // Community algorithms use direct methods
//! let stats = graph.degree_centrality().stats()?;
//!
//! // All support multiple execution modes
//! graph.pagerank()
//!     .stream()?      // Get individual results as iterator
//!     .stats()?       // Get aggregated statistics
//!     .mutate("pr")?  // Store results as node property
//!     .write("pr")?   // Persist results to storage
//! ```

pub mod traits;
pub mod builder_base;

// Facade implementations by algorithm family
pub mod centrality;
pub mod community;
pub mod pathfinding;
pub mod utilities;

// Re-export commonly used types
pub use builder_base::{ExecutionContext, MutationResult, WriteResult};
pub use traits::{AlgorithmRunner, StreamResults, StatsResults, MutateResults, WriteResults};

