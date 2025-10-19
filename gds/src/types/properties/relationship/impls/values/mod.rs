/// Individual value type implementations for relationship properties.
/// Each file contains a focused implementation of a specific ValueType.
///
/// Relationship properties are currently simpler than node/graph properties:
/// - Currently only Double (f64) values are supported (for relationship weights)
/// - Uses indexed access like node properties
/// - Includes default_value support for sparse properties
/// - Future: Can expand to Long, arrays, etc. as needed
mod double;

pub use double::DefaultRelationshipPropertyValues;
