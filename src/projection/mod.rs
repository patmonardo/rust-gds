pub mod impls;
pub mod node_label;
pub mod orientation;
pub mod relationship_type;
/// Projection-related types for graph data science.
///
/// This module contains types used for projecting and identifying
/// nodes and relationships in graphs.
pub mod traits;

pub use node_label::NodeLabel;
pub use orientation::Orientation;
pub use relationship_type::RelationshipType;

// Re-export trait types
pub use traits::{
    AbstractProjections, Aggregation, ElementProjection, InlineProperties, Projections,
    ProjectionsBuilder, PropertyMapping, PropertyMappingBuilder, PropertyMappings,
    PropertyMappingsBuilder,
};

// Re-export implementation types
pub use impls::{
    NodeProjection, NodeProjectionBuilder, NodeProjections, NodeProjectionsBuilder,
    RelationshipProjection, RelationshipProjectionBuilder, RelationshipProjections,
    RelationshipProjectionsBuilder,
};
