use super::traits::{
    DoubleArrayNodePropertyValues, DoubleNodePropertyValues, FloatArrayNodePropertyValues,
    LongArrayNodePropertyValues, LongNodePropertyValues, NodePropertyValues,
};
use crate::types::properties::property_values::{
    PropertyValues, PropertyValuesError, PropertyValuesResult,
};
use crate::types::property::ValueType;

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

impl PropertyValues for DefaultLongNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }

    fn element_count(&self) -> usize {
        self.node_count
    }
}

impl NodePropertyValues for DefaultLongNodePropertyValues {
    fn double_value(&self, node_id: u64) -> PropertyValuesResult<f64> {
        Ok(self.long_value(node_id)? as f64)
    }

    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        self.values
            .get(node_id as usize)
            .copied()
            .ok_or(PropertyValuesError::InvalidNodeId(node_id))
    }

    fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::DoubleArray,
        ))
    }

    fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::FloatArray,
        ))
    }

    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::LongArray,
        ))
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Ok(Box::new(self.long_value(node_id)?))
    }

    fn dimension(&self) -> Option<usize> {
        Some(1)
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        self.values.iter().max().copied()
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        self.get_max_long_property_value().map(|v| v as f64)
    }

    fn has_value(&self, node_id: u64) -> bool {
        (node_id as usize) < self.values.len()
    }
}

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

impl PropertyValues for DefaultDoubleNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Double
    }

    fn element_count(&self) -> usize {
        self.node_count
    }
}

impl NodePropertyValues for DefaultDoubleNodePropertyValues {
    fn double_value(&self, node_id: u64) -> PropertyValuesResult<f64> {
        self.values
            .get(node_id as usize)
            .copied()
            .ok_or(PropertyValuesError::InvalidNodeId(node_id))
    }

    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        Ok(self.double_value(node_id)? as i64)
    }

    fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::DoubleArray,
        ))
    }

    fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::FloatArray,
        ))
    }

    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::LongArray,
        ))
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Ok(Box::new(self.double_value(node_id)?))
    }

    fn dimension(&self) -> Option<usize> {
        Some(1)
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        self.get_max_double_property_value().map(|v| v as i64)
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        self.values
            .iter()
            .copied()
            .fold(None, |max, v| Some(max.map_or(v, |m| f64::max(m, v))))
    }

    fn has_value(&self, node_id: u64) -> bool {
        (node_id as usize) < self.values.len()
    }
}

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

impl PropertyValues for DefaultDoubleArrayNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::DoubleArray
    }

    fn element_count(&self) -> usize {
        self.node_count
    }
}

impl NodePropertyValues for DefaultDoubleArrayNodePropertyValues {
    fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::Double,
        ))
    }

    fn long_value(&self, _node_id: u64) -> PropertyValuesResult<i64> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::Long,
        ))
    }

    fn double_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        self.values
            .get(node_id as usize)
            .and_then(|v| v.clone())
            .ok_or(PropertyValuesError::InvalidNodeId(node_id))
    }

    fn float_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Ok(self
            .double_array_value(node_id)?
            .iter()
            .map(|&v| v as f32)
            .collect())
    }

    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::LongArray,
        ))
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Ok(Box::new(self.double_array_value(node_id)?))
    }

    fn dimension(&self) -> Option<usize> {
        self.dimension
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        None
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        None
    }

    fn has_value(&self, node_id: u64) -> bool {
        self.values
            .get(node_id as usize)
            .and_then(|v| v.as_ref())
            .is_some()
    }
}

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

impl PropertyValues for DefaultFloatArrayNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::FloatArray
    }

    fn element_count(&self) -> usize {
        self.node_count
    }
}

impl NodePropertyValues for DefaultFloatArrayNodePropertyValues {
    fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::Double,
        ))
    }

    fn long_value(&self, _node_id: u64) -> PropertyValuesResult<i64> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::Long,
        ))
    }

    fn double_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        Ok(self
            .float_array_value(node_id)?
            .iter()
            .map(|&v| v as f64)
            .collect())
    }

    fn float_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        self.values
            .get(node_id as usize)
            .and_then(|v| v.clone())
            .ok_or(PropertyValuesError::InvalidNodeId(node_id))
    }

    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::LongArray,
        ))
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Ok(Box::new(self.float_array_value(node_id)?))
    }

    fn dimension(&self) -> Option<usize> {
        self.dimension
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        None
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        None
    }

    fn has_value(&self, node_id: u64) -> bool {
        self.values
            .get(node_id as usize)
            .and_then(|v| v.as_ref())
            .is_some()
    }
}

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

impl PropertyValues for DefaultLongArrayNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::LongArray
    }

    fn element_count(&self) -> usize {
        self.node_count
    }
}

impl NodePropertyValues for DefaultLongArrayNodePropertyValues {
    fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::Double,
        ))
    }

    fn long_value(&self, _node_id: u64) -> PropertyValuesResult<i64> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::Long,
        ))
    }

    fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::DoubleArray,
        ))
    }

    fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(PropertyValuesError::unsupported_type(
            self.value_type(),
            ValueType::FloatArray,
        ))
    }

    fn long_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        self.values
            .get(node_id as usize)
            .and_then(|v| v.clone())
            .ok_or(PropertyValuesError::InvalidNodeId(node_id))
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Ok(Box::new(self.long_array_value(node_id)?))
    }

    fn dimension(&self) -> Option<usize> {
        self.dimension
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        None
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        None
    }

    fn has_value(&self, node_id: u64) -> bool {
        self.values
            .get(node_id as usize)
            .and_then(|v| v.as_ref())
            .is_some()
    }
}

impl LongArrayNodePropertyValues for DefaultLongArrayNodePropertyValues {
    fn long_array_value_unchecked(&self, node_id: u64) -> Option<Vec<i64>> {
        self.values[node_id as usize].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::property::ValueType;

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
