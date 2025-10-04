//! Curated prelude for the `types` module.
//!
//! The prelude re-exports a small, stable set of traits and defaults that
//! downstream consumers can rely on. Keep this module intentionally small.

// Re-export a conservative set of graph and graph_store types
pub use crate::types::graph::{DefaultGraph, Degrees, Graph, GraphExt};
pub use crate::types::graph_store::{DefaultGraphStore, GraphStore, GraphStoreAdapter};

// Schema & property essentials
pub use crate::types::property::{Property, ValueType};
pub use crate::types::schema::{
    GraphSchema, NodeLabel, NodeSchema, PropertySchema, RelationshipType,
};

// Property traits
pub use crate::types::properties::{
    GraphPropertyValues, NodePropertyValues, PropertyValues, RelationshipPropertyValues,
};

// Random utilities
pub use crate::types::random::{RandomGraphConfig, RandomGraphResult, RandomRelationshipConfig};

// Re-export core helpers
pub use crate::types::concurrency::Concurrency;

// Keep additions conservative: expand on demand.
