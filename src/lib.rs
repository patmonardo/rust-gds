//! Rust GDS - Graph Data Science library
//!
//! A modular graph data structure and algorithms library.

pub mod projection;
pub mod types;

/// Re-export the graph_store module at the crate root for backwards-compatible doctests.
pub use crate::types::graph_store;
/// Ergonomic re-export for easy discovery.
pub use crate::types::random_graph_store;
