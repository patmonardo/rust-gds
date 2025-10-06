use crate::types::properties::node::node_property_values::{
    FloatArrayNodePropertyValues, NodePropertyValues,
};
use crate::types::properties::property_values::{
    PropertyValues, PropertyValuesError, PropertyValuesResult,
};
use crate::types::property::ValueType;
use crate::types::property_value::PropertyValue;
use crate::{node_float_array_property_values_impl, property_values_impl};

/// Default implementation for float array node property values.
///
/// Storage: Vec<Option<Vec<f32>>> - suitable for memory-efficient embeddings.
/// Future: Can be replaced with Arrow2 ListArray<Float32Array> for columnar storage,
/// particularly useful for ML embeddings where f32 precision is sufficient.
#[derive(Debug, Clone)]
pub struct DefaultFloatArrayNodePropertyValues {
    values: Vec<Option<Vec<f32>>>,
    node_count: usize,
    dimension: Option<usize>,
}

impl DefaultFloatArrayNodePropertyValues {
    pub fn new(values: Vec<Option<Vec<f32>>>, node_count: usize) -> Self {
        let dimension = values.first().and_then(|v| v.as_ref().map(|arr| arr.len()));
        DefaultFloatArrayNodePropertyValues {
            values,
            node_count,
            dimension,
        }
    }
}

// Generate PropertyValues trait implementation for arrays
property_values_impl!(
    DefaultFloatArrayNodePropertyValues,
    FloatArray,
    Vec<f32>,
    PropertyValue::FloatArray,
    array
);

// Generate complete NodePropertyValues trait implementation with all conversions and errors
node_float_array_property_values_impl!(DefaultFloatArrayNodePropertyValues);

// Specialized unchecked accessor for FloatArray values
impl FloatArrayNodePropertyValues for DefaultFloatArrayNodePropertyValues {
    fn float_array_value_unchecked(&self, node_id: u64) -> Option<Vec<f32>> {
        self.values[node_id as usize].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::property_values::PropertyValues;
    use crate::types::property::ValueType;

    #[test]
    fn test_float_array_node_property_values() {
        let values =
            DefaultFloatArrayNodePropertyValues::new(vec![Some(vec![1.0, 2.0, 3.0]), None], 2);

        assert_eq!(values.value_type(), ValueType::FloatArray);
        assert_eq!(values.node_count(), 2);
        assert_eq!(values.float_array_value(0).unwrap(), vec![1.0, 2.0, 3.0]);
        assert_eq!(values.dimension(), Some(3));
        assert!(values.has_value(0));
        assert!(!values.has_value(1));
    }

    #[test]
    fn test_float_array_to_double_array_conversion() {
        let values = DefaultFloatArrayNodePropertyValues::new(vec![Some(vec![1.5f32, 2.5f32])], 1);
        let double_arr = values.double_array_value(0).unwrap();
        assert_eq!(double_arr, vec![1.5f64, 2.5f64]);
    }
}
