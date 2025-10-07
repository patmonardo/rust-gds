//! Concrete implementations of projection types.
//!
//! This module contains the concrete implementations of projections
//! for nodes and relationships in graph data science operations.

pub mod node_projection;
pub mod relationship_projection;

pub use node_projection::{
    NodeProjection, NodeProjectionBuilder, NodeProjections, NodeProjectionsBuilder,
};
pub use relationship_projection::{
    RelationshipProjection, RelationshipProjectionBuilder, RelationshipProjections,
    RelationshipProjectionsBuilder,
};
