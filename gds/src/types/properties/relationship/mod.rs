// Relationship properties module
//
// Provides property storage for relationship properties.
// Relationship properties are values associated with edges in the graph.

pub mod impls;
pub mod relationship_properties;
pub mod relationship_property;
pub mod relationship_property_store;
pub mod relationship_property_values;
pub mod traits;

// Re-export commonly used items from traits
pub use impls::*;
pub use relationship_properties::*;
pub use relationship_property::*;
pub use relationship_property_store::*;
pub use relationship_property_values::*;
pub use traits::*;
