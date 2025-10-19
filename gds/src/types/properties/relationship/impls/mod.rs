pub mod default_relationship_cursor;
pub mod default_relationship_property;
pub mod default_relationship_property_store;

// Legacy consolidated module - kept for backward compatibility
// New code should use the modular values/* structure
pub mod default_relationship_property_values;

// Modular value type implementations - each ValueType in its own file
pub mod values;

pub use default_relationship_cursor::*;
pub use default_relationship_property::DefaultRelationshipProperty;
pub use default_relationship_property_store::*;

// Re-export all value types from the modular structure
pub use values::DefaultRelationshipPropertyValues;
