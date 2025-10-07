pub mod abstract_projections;
pub mod element_projection;
/// Abstract traits and types for graph element projections.
///
/// This module provides the foundational abstractions for projecting
/// graph elements (nodes and relationships) with property mappings.
pub mod property_mapping;

pub use abstract_projections::*;
pub use element_projection::*;
pub use property_mapping::*;
