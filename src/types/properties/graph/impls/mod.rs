// pub mod default_graph_property; // No longer needed - using Property<Box<dyn GraphPropertyValues>> type alias
pub mod default_graph_property_store;

// Legacy consolidated module - kept for backward compatibility
// New code should use the modular values/* structure
pub mod default_graph_property_values;

// Modular value type implementations - each ValueType in its own file
pub mod values;

pub use default_graph_property_store::{
    DefaultGraphPropertyStore, DefaultGraphPropertyStoreBuilder,
};

// Re-export all value types from the modular structure
pub use values::{
    DefaultDoubleArrayGraphPropertyValues, DefaultDoubleGraphPropertyValues,
    DefaultFloatArrayGraphPropertyValues, DefaultLongArrayGraphPropertyValues,
    DefaultLongGraphPropertyValues,
};
