//! Default Graph Property Values: Universal Collections-Backed Implementations
//!
//! This module generates graph property value adapters using the ValueType table
//! and the universal adapter system. All adapters are generic over Collections
//! backends (Vec, Huge, Arrow), enabling runtime backend selection.


// Import the macros from the crate root
use crate::{generate_all_graph_adapters, generate_all_graph_array_adapters};

// Generate all graph adapters from the ValueType table
// This expands to adapters for: Byte, Short, Int, Long, Float, Double, Boolean
generate_all_graph_adapters!();

// Generate all graph array adapters
// This expands to: ByteArray, ShortArray, IntArray, LongArray, FloatArray, DoubleArray, etc.
generate_all_graph_array_adapters!();

// Note: The generated types are generic over Collections backend C:
// Scalars:
// - DefaultLongGraphPropertyValues<C>
// - DefaultDoubleGraphPropertyValues<C>
// - DefaultFloatGraphPropertyValues<C>
// - DefaultIntGraphPropertyValues<C>
// - DefaultShortGraphPropertyValues<C>
// - DefaultByteGraphPropertyValues<C>
// - DefaultBooleanGraphPropertyValues<C>
//
// Arrays:
// - DefaultLongArrayGraphPropertyValues<C>
// - DefaultDoubleArrayGraphPropertyValues<C>
// - DefaultFloatArrayGraphPropertyValues<C>
// - etc.

// For backwards compatibility, provide convenient FromIterator implementations
// for the common generic types with Vec backends
use crate::collections::backends::vec::{VecDouble, VecLong};

impl FromIterator<i64> for DefaultLongGraphPropertyValues<VecLong> {
    fn from_iter<T: IntoIterator<Item = i64>>(iter: T) -> Self {
        let vec: Vec<i64> = iter.into_iter().collect();
        let backend = VecLong::from(vec);
        Self::from_collection(backend)
    }
}

impl FromIterator<f64> for DefaultDoubleGraphPropertyValues<VecDouble> {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        let vec: Vec<f64> = iter.into_iter().collect();
        let backend = VecDouble::from(vec);
        Self::from_collection(backend)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::backends::vec::{VecDouble, VecLong};
    use crate::types::ValueType;
    use crate::types::properties::PropertyValues;
    use crate::types::properties::graph::GraphPropertyValues;

    #[test]
    fn test_long_graph_property_values() {
        let values: DefaultLongGraphPropertyValues<VecLong> = [1, 2, 3].into_iter().collect();
        assert_eq!(values.value_type(), ValueType::Long);
        assert_eq!(values.element_count(), 3);
        let collected: Vec<i64> = values.long_values().collect();
        assert_eq!(collected, vec![1, 2, 3]);
        let as_doubles: Vec<f64> = values.double_values().collect();
        assert_eq!(as_doubles, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_double_graph_property_values() {
        let values: DefaultDoubleGraphPropertyValues<VecDouble> = [1.5, 2.5, 3.5].into_iter().collect();
        assert_eq!(values.value_type(), ValueType::Double);
        assert_eq!(values.element_count(), 3);
        let collected: Vec<f64> = values.double_values().collect();
        assert_eq!(collected, vec![1.5, 2.5, 3.5]);
    }
}

