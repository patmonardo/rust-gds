use crate::types::properties::node::node_property_values::{
    DoubleArrayNodePropertyValues, DoubleNodePropertyValues, FloatArrayNodePropertyValues,
    LongArrayNodePropertyValues, LongNodePropertyValues, NodePropertyValues,
};
use crate::types::properties::property_values::{
    PropertyValues, PropertyValuesError, PropertyValuesResult,
};
use crate::types::value_type::ValueType;
use crate::types::property_value::PropertyValue;
use crate::{
    node_double_array_property_values_impl, node_double_property_values_impl,
    node_float_array_property_values_impl, node_long_array_property_values_impl,
    node_long_property_values_impl, property_values_impl,
};

/// Default implementation for long node property values.
#[derive(Debug, Clone)]
pub struct DefaultLongNodePropertyValues {
    values: Vec<i64>,
    node_count: usize,
}

impl DefaultLongNodePropertyValues {
    pub fn new(values: Vec<i64>, node_count: usize) -> Self {
        DefaultLongNodePropertyValues { values, node_count }
    }
}

property_values_impl!(
    DefaultLongNodePropertyValues,
    Long,
    i64,
    PropertyValue::Long
);

node_long_property_values_impl!(DefaultLongNodePropertyValues);

impl LongNodePropertyValues for DefaultLongNodePropertyValues {
    fn long_value_unchecked(&self, node_id: u64) -> i64 {
        self.values[node_id as usize]
    }
}

/// Default implementation for double node property values.
#[derive(Debug, Clone)]
pub struct DefaultDoubleNodePropertyValues {
    values: Vec<f64>,
    node_count: usize,
}

impl DefaultDoubleNodePropertyValues {
    pub fn new(values: Vec<f64>, node_count: usize) -> Self {
        DefaultDoubleNodePropertyValues { values, node_count }
    }
}

property_values_impl!(
    DefaultDoubleNodePropertyValues,
    Double,
    f64,
    PropertyValue::Double
);

node_double_property_values_impl!(DefaultDoubleNodePropertyValues);

impl DoubleNodePropertyValues for DefaultDoubleNodePropertyValues {
    fn double_value_unchecked(&self, node_id: u64) -> f64 {
        self.values[node_id as usize]
    }
}

/// Default implementation for double array node property values.
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

property_values_impl!(
    DefaultDoubleArrayNodePropertyValues,
    DoubleArray,
    Vec<f64>,
    PropertyValue::DoubleArray,
    array
);

node_double_array_property_values_impl!(DefaultDoubleArrayNodePropertyValues);

impl DoubleArrayNodePropertyValues for DefaultDoubleArrayNodePropertyValues {
    fn double_array_value_unchecked(&self, node_id: u64) -> Option<Vec<f64>> {
        self.values[node_id as usize].clone()
    }
}

/// Default implementation for float array node property values.
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

property_values_impl!(
    DefaultFloatArrayNodePropertyValues,
    FloatArray,
    Vec<f32>,
    PropertyValue::FloatArray,
    array
);

node_float_array_property_values_impl!(DefaultFloatArrayNodePropertyValues);

impl FloatArrayNodePropertyValues for DefaultFloatArrayNodePropertyValues {
    fn float_array_value_unchecked(&self, node_id: u64) -> Option<Vec<f32>> {
        self.values[node_id as usize].clone()
    }
}

/// Default implementation for long array node property values.
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

property_values_impl!(
    DefaultLongArrayNodePropertyValues,
    LongArray,
    Vec<i64>,
    PropertyValue::LongArray,
    array
);

node_long_array_property_values_impl!(DefaultLongArrayNodePropertyValues);

impl LongArrayNodePropertyValues for DefaultLongArrayNodePropertyValues {
    fn long_array_value_unchecked(&self, node_id: u64) -> Option<Vec<i64>> {
        self.values[node_id as usize].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::value_type::ValueType;

    #[test]
    fn test_long_node_property_values() {
        let values = DefaultLongNodePropertyValues::new(vec![1, 2, 3, 4, 5], 5);
        assert_eq!(values.value_type(), ValueType::Long);
        assert_eq!(values.node_count(), 5);
        assert_eq!(values.long_value(0).unwrap(), 1);
        assert_eq!(values.long_value(4).unwrap(), 5);
        assert_eq!(values.double_value(2).unwrap(), 3.0);
        assert_eq!(values.dimension(), Some(1));
        assert_eq!(values.get_max_long_property_value(), Some(5));
        assert!(values.has_value(0));
        assert!(!values.has_value(10));
    }

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
}
