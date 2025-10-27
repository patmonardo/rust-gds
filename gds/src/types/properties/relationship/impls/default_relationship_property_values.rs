//! Default Relationship Property Values: Universal Collections-Backed Implementations
//!
//! This module generates relationship property value adapters using the ValueType table
//! and the universal adapter system. All adapters are generic over Collections
//! backends (Vec, Huge, Arrow), enabling runtime backend selection.

use crate::types::properties::relationship::RelationshipPropertyValues;
use crate::types::properties::PropertyValues;

// Import the macros from the crate root
use crate::generate_all_relationship_adapters;

// Generate all relationship property adapters from the ValueType table
// This expands to adapters for: Byte, Short, Int, Long, BigInt, Float, Double, Boolean, Char, String
generate_all_relationship_adapters!();

// TODO: Relationship array adapters require additional trait implementations
// (DoubleArrayRelationshipPropertyValues, FloatArrayRelationshipPropertyValues, etc.)
// Deferred until array accessor methods are fully designed.
// use crate::generate_all_relationship_array_adapters;
// generate_all_relationship_array_adapters!();

// Note: The generated types are generic over Collections backend C:
// - DefaultLongRelationshipPropertyValues<C>
// - DefaultDoubleRelationshipPropertyValues<C>
// - DefaultFloatRelationshipPropertyValues<C>
// - DefaultIntRelationshipPropertyValues<C>
// - DefaultShortRelationshipPropertyValues<C>
// - DefaultByteRelationshipPropertyValues<C>
// - DefaultBooleanRelationshipPropertyValues<C>

// For backwards compatibility, create a type alias for the most common case (Double with Vec)
use crate::collections::backends::vec::VecDouble;
pub type DefaultRelationshipPropertyValues = DefaultDoubleRelationshipPropertyValues<VecDouble>;

// Provide backwards-compatible constructors
impl DefaultRelationshipPropertyValues {
    pub fn with_values(values: Vec<f64>, _default_value: f64, element_count: usize) -> Self {
        let backend = VecDouble::from(values);
        Self::from_collection(backend, element_count)
    }

    pub fn with_default(values: Vec<f64>, element_count: usize) -> Self {
        Self::with_values(values, 0.0, element_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ValueType;

    #[test]
    fn default_relationship_property_values_behavior() {
        let values = DefaultRelationshipPropertyValues::with_values(vec![1.0, 2.5, 3.7], 0.0, 3);

        assert_eq!(values.value_type(), ValueType::Double);
        assert_eq!(values.relationship_count(), 3);
        assert_eq!(values.double_value(1).unwrap(), 2.5);
        assert_eq!(values.default_value(), 0.0);
        assert!(values.has_value(0));
        assert!(!values.has_value(10));
    }
}

