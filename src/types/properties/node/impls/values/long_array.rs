use crate::{node_long_array_property_values_impl, property_values_impl};
use crate::types::properties::node::node_property_values::{
    LongArrayNodePropertyValues, NodePropertyValues,
};
use crate::types::properties::property_values::{
    PropertyValues, PropertyValuesError, PropertyValuesResult,
};
use crate::types::property::ValueType;
use crate::types::property_value::PropertyValue;

/// Default implementation for long array node property values.
/// 
/// Storage: Vec<Option<Vec<i64>>> - suitable for integer sequence properties.
/// Future: Can be replaced with Arrow2 ListArray<Int64Array> for columnar storage,
/// useful for storing sequences, paths, or multi-valued integer attributes.
#[derive(Debug, Clone)]
pub struct DefaultLongArrayNodePropertyValues {
    values: Vec<Option<Vec<i64>>>,
    node_count: usize,
    dimension: Option<usize>,
}

impl DefaultLongArrayNodePropertyValues {
    pub fn new(values: Vec<Option<Vec<i64>>>, node_count: usize) -> Self {
        let dimension = values.first().and_then(|v| v.as_ref().map(|arr| arr.len()));
        DefaultLongArrayNodePropertyValues {
            values,
            node_count,
            dimension,
        }
    }
}

// Generate PropertyValues trait implementation for arrays
property_values_impl!(
    DefaultLongArrayNodePropertyValues,
    LongArray,
    Vec<i64>,
    PropertyValue::LongArray,
    array
);

// Generate complete NodePropertyValues trait implementation with all conversions and errors
node_long_array_property_values_impl!(DefaultLongArrayNodePropertyValues);

// Specialized unchecked accessor for LongArray values
impl LongArrayNodePropertyValues for DefaultLongArrayNodePropertyValues {
    fn long_array_value_unchecked(&self, node_id: u64) -> Option<Vec<i64>> {
        self.values[node_id as usize].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::property::ValueType;
    use crate::types::properties::property_values::PropertyValues;

    #[test]
    fn test_long_array_node_property_values() {
        let values = DefaultLongArrayNodePropertyValues::new(
            vec![Some(vec![1, 2, 3]), Some(vec![4, 5]), None],
            3,
        );

        assert_eq!(values.value_type(), ValueType::LongArray);
        assert_eq!(values.node_count(), 3);
        assert_eq!(values.long_array_value(0).unwrap(), vec![1, 2, 3]);
        assert_eq!(values.dimension(), Some(3));
        assert!(values.has_value(0));
        assert!(!values.has_value(2));
    }

    #[test]
    fn test_variable_length_arrays() {
        let values = DefaultLongArrayNodePropertyValues::new(
            vec![Some(vec![1]), Some(vec![1, 2, 3, 4])],
            2,
        );
        // dimension is inferred from first element
        assert_eq!(values.dimension(), Some(1));
        assert_eq!(values.long_array_value(1).unwrap().len(), 4);
    }

    #[test]
    fn test_unsupported_float_access() {
        let values = DefaultLongArrayNodePropertyValues::new(
            vec![Some(vec![1, 2])],
            1,
        );
        assert!(values.double_array_value(0).is_err());
        assert!(values.float_array_value(0).is_err());
    }
}
