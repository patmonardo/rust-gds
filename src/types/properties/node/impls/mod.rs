// pub mod default_node_property; // No longer needed - using Property<Box<dyn NodePropertyValues>> type alias
pub mod default_node_property_store;

// Legacy consolidated module - kept for backward compatibility
// New code should use the modular values/* structure
pub mod default_node_property_values;

// Modular value type implementations - each ValueType in its own file
pub mod values;

pub use default_node_property_store::{
    DefaultNodePropertyStore, DefaultNodePropertyStoreBuilder,
};

// Re-export all value types from the modular structure
pub use values::{
    DefaultDoubleArrayNodePropertyValues, DefaultDoubleNodePropertyValues,
    DefaultFloatArrayNodePropertyValues, DefaultLongArrayNodePropertyValues,
    DefaultLongNodePropertyValues,
};
