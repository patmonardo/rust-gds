pub mod abstract_projections;
pub mod element_projection;
/// Abstract traits and types for graph element projections.
///
/// This module provides the foundational abstractions for projecting
/// graph elements (nodes and relationships) with property mappings.
pub mod property_mapping;

pub use abstract_projections::{AbstractProjections, Projections, ProjectionsBuilder};
pub use element_projection::{
    ElementProjection, InlineProperties, InlinePropertiesBuilder, PropertyMappings,
    PropertyMappingsBuilder,
};
pub use property_mapping::{Aggregation, PropertyMapping, PropertyMappingBuilder};
