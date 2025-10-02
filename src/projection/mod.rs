pub mod impls;
pub mod node_label;
pub mod relationship_type;
/// Projection-related types for graph data science.
///
/// This module contains types used for projecting and identifying
/// nodes and relationships in graphs.
pub mod traits;

pub use node_label::NodeLabel;
pub use relationship_type::RelationshipType;

// Re-export trait types
pub use traits::{
    AbstractProjections, Aggregation, ElementProjection, InlineProperties, Projections,
    ProjectionsBuilder, PropertyMapping, PropertyMappingBuilder, PropertyMappings,
    PropertyMappingsBuilder,
};
