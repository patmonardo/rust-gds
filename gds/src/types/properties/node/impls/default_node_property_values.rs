//! Default Node Property Values: Universal Collections-Backed Implementations
//!
//! This module generates node property value adapters using the ValueType table
//! and the universal adapter system. All adapters are generic over Collections
//! backends (Vec, Huge, Arrow), enabling runtime backend selection.

use crate::types::properties::PropertyValues;

// Import the macros from the crate root
use crate::{generate_all_node_adapters, generate_all_node_array_adapters};

// Generate all node property adapters from the ValueType table
// This expands to adapters for: Byte, Short, Int, Long, Float, Double, Boolean
generate_all_node_adapters!();

// Generate node array property adapters
// This expands to adapters for: ByteArray, ShortArray, IntArray, LongArray, FloatArray, DoubleArray, BooleanArray
generate_all_node_array_adapters!();

// Note: The generated types are generic over Collections backend C:
// - DefaultLongNodePropertyValues<C>
// - DefaultDoubleNodePropertyValues<C>
// - DefaultFloatNodePropertyValues<C>
// - DefaultIntNodePropertyValues<C>
// - DefaultShortNodePropertyValues<C>
// - DefaultByteNodePropertyValues<C>
// - DefaultBooleanNodePropertyValues<C>
// - DefaultLongArrayNodePropertyValues<C>
// - DefaultDoubleArrayNodePropertyValues<C>
// - DefaultFloatArrayNodePropertyValues<C>
// - DefaultIntArrayNodePropertyValues<C>
// - DefaultShortArrayNodePropertyValues<C>
// - DefaultByteArrayNodePropertyValues<C>
// - DefaultBooleanArrayNodePropertyValues<C>

// For backwards compatibility, create type aliases using Vec backends
// These can be used where the old non-generic types were expected

// Re-export the generated types with their full generic signatures
pub use self::{
    DefaultLongNodePropertyValues as GenericLongNodePropertyValues,
    DefaultDoubleNodePropertyValues as GenericDoubleNodePropertyValues,
};

// Note: The actual Default*NodePropertyValues types are now generic.
// Code that needs concrete types should specify the backend explicitly,
// e.g., DefaultLongNodePropertyValues::<VecLong>::from_collection(...)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::backends::vec::VecLong;
    use crate::types::ValueType;
    use crate::types::properties::node::{LongNodePropertyValues, NodePropertyValues};
    use crate::types::properties::PropertyValues;

    #[test]
    fn test_long_node_property_values_with_vec_backend() {
        // Create a VecLong backend with test data
        let backend = VecLong::from(vec![1i64, 2, 3, 4, 5]);
        
        // Create the adapter
        let values = DefaultLongNodePropertyValues::from_collection(backend, 5);
        
        // Test basic properties via PropertyValues trait
        assert_eq!(values.value_type(), ValueType::Long);
        assert_eq!(values.node_count(), 5);
        
        // Test value access via LongNodePropertyValues trait
        assert_eq!(values.long_value(0).unwrap(), 1);
        assert_eq!(values.long_value(4).unwrap(), 5);
        assert_eq!(values.long_value_unchecked(2), 3);
        
        // Test type conversion
        assert_eq!(values.double_value(2).unwrap(), 3.0);
        
        // Test dimension
        assert_eq!(values.dimension(), Some(1));
        
        // Test has_value
        assert!(values.has_value(0));
        assert!(!values.has_value(10));
    }

    #[test]
    fn test_double_node_property_values_with_vec_backend() {
        use crate::collections::backends::vec::VecDouble;
        use crate::types::properties::node::DoubleNodePropertyValues;
        
        let backend = VecDouble::from(vec![1.5, 2.5, 3.5]);
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, 3);
        
        assert_eq!(values.value_type(), ValueType::Double);
        assert_eq!(values.node_count(), 3);
        
        assert_eq!(values.double_value(1).unwrap(), 2.5);
        assert_eq!(values.double_value_unchecked(2), 3.5);
        
        // Test type conversion to long
        assert_eq!(values.long_value(0).unwrap(), 1);
    }
}
