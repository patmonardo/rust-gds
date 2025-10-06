use crate::types::properties::node::node_property_values::{
    DoubleArrayNodePropertyValues, NodePropertyValues,
};
use crate::types::properties::property_values::{
    PropertyValues, PropertyValuesError, PropertyValuesResult,
};
use crate::types::value_type::ValueType;
use crate::types::property_value::PropertyValue;
use crate::{node_double_array_property_values_impl, property_values_impl};

/// Default implementation for double array node property values.
///
/// Storage: Vec<Option<Vec<f64>>> - suitable for pure in-memory graphs with optional embeddings.
/// Future: Can be replaced with Arrow2 ListArray<Float64Array> for columnar storage,
/// enabling efficient slicing and zero-copy operations on embeddings.
#[derive(Debug, Clone)]
pub struct DefaultDoubleArrayNodePropertyValues {
    values: Vec<Option<Vec<f64>>>,
    node_count: usize,
    dimension: Option<usize>,
}

impl DefaultDoubleArrayNodePropertyValues {
    pub fn new(values: Vec<Option<Vec<f64>>>, node_count: usize) -> Self {
        let dimension = values.first().and_then(|v| v.as_ref().map(|arr| arr.len()));
        DefaultDoubleArrayNodePropertyValues {
            values,
            node_count,
            dimension,
        }
    }
}

// Generate PropertyValues trait implementation for arrays
property_values_impl!(
    DefaultDoubleArrayNodePropertyValues,
    DoubleArray,
    Vec<f64>,
    PropertyValue::DoubleArray,
    array
);

// Generate complete NodePropertyValues trait implementation with all conversions and errors
node_double_array_property_values_impl!(DefaultDoubleArrayNodePropertyValues);

// Specialized unchecked accessor for DoubleArray values
impl DoubleArrayNodePropertyValues for DefaultDoubleArrayNodePropertyValues {
    fn double_array_value_unchecked(&self, node_id: u64) -> Option<Vec<f64>> {
        self.values[node_id as usize].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::property_values::PropertyValues;
    use crate::types::value_type::ValueType;

    #[test]
    fn test_double_array_node_property_values() {
        let values = DefaultDoubleArrayNodePropertyValues::new(
            vec![Some(vec![1.0, 2.0, 3.0]), Some(vec![4.0, 5.0, 6.0]), None],
            3,
        );

        assert_eq!(values.value_type(), ValueType::DoubleArray);
        assert_eq!(values.node_count(), 3);
        assert_eq!(values.double_array_value(0).unwrap(), vec![1.0, 2.0, 3.0]);
        assert_eq!(values.dimension(), Some(3));
        assert!(values.has_value(0));
        assert!(!values.has_value(2));
    }

    #[test]
    fn test_double_array_to_float_array_conversion() {
        let values = DefaultDoubleArrayNodePropertyValues::new(vec![Some(vec![1.0, 2.0])], 1);
        let float_arr = values.float_array_value(0).unwrap();
        assert_eq!(float_arr, vec![1.0f32, 2.0f32]);
    }

    #[test]
    fn test_unsupported_scalar_access() {
        let values = DefaultDoubleArrayNodePropertyValues::new(vec![Some(vec![1.0, 2.0])], 1);
        assert!(values.double_value(0).is_err());
        assert!(values.long_value(0).is_err());
    }
}
