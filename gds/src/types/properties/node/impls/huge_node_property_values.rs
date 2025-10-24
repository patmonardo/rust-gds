//! HugeArray-backed Node PropertyValues implementations
//!
//! These implementations use HugeArrays as storage backends instead of Vec<T>,
//! providing support for billion-scale property storage with paged memory architecture.

use crate::collections::backends::huge::{HugeLongArray, HugeDoubleArray, HugeObjectArray};
use crate::types::properties::node::{
    LongNodePropertyValues, DoubleNodePropertyValues, LongArrayNodePropertyValues, DoubleArrayNodePropertyValues, NodePropertyValues
};
use crate::types::properties::{PropertyValues, PropertyValuesError, PropertyValuesResult};
use crate::types::ValueType;

/// Node property values backed by HugeLongArray for billion-scale storage.
///
/// Provides the same interface as Vec-backed implementations but uses
/// paged memory architecture for massive datasets.
pub struct HugeLongNodePropertyValues {
    values: HugeLongArray,
    default_value: i64,
    element_count: usize,
}

impl std::fmt::Debug for HugeLongNodePropertyValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HugeLongNodePropertyValues")
            .field("default_value", &self.default_value)
            .field("element_count", &self.element_count)
            .field("values_size", &self.values.size())
            .finish()
    }
}

impl Clone for HugeLongNodePropertyValues {
    fn clone(&self) -> Self {
        // Note: This creates a copy of the HugeArray, which may be expensive
        // In practice, you might want to use Arc<HugeLongArray> for sharing
        let mut values_copy = HugeLongArray::new(self.values.size());
        // Copy values (this is expensive for large arrays)
        for i in 0..self.values.size() {
            values_copy.set(i, self.values.get(i));
        }
        Self {
            values: values_copy,
            default_value: self.default_value,
            element_count: self.element_count,
        }
    }
}

impl HugeLongNodePropertyValues {
    /// Creates a new HugeLongNodePropertyValues from a HugeLongArray.
    pub fn new(values: HugeLongArray, default_value: i64, element_count: usize) -> Self {
        Self {
            values,
            default_value,
            element_count,
        }
    }

    /// Creates a new instance with default value 0.
    pub fn with_default(values: HugeLongArray, element_count: usize) -> Self {
        Self::new(values, 0, element_count)
    }

    /// Returns a reference to the underlying HugeLongArray.
    pub fn values(&self) -> &HugeLongArray {
        &self.values
    }

    /// Returns the default value for missing properties.
    pub fn default_value(&self) -> i64 {
        self.default_value
    }
}

// Implement PropertyValues trait
impl PropertyValues for HugeLongNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }

    fn element_count(&self) -> usize {
        self.element_count
    }
}

// Implement NodePropertyValues trait
impl NodePropertyValues for HugeLongNodePropertyValues {
    fn double_value(&self, node_id: u64) -> PropertyValuesResult<f64> {
        Ok(self.long_value(node_id)? as f64)
    }

    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        if (node_id as usize) < self.element_count {
            Ok(self.values.get(node_id as usize))
        } else {
            Err(PropertyValuesError::InvalidNodeId(node_id))
        }
    }

    fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        Err(PropertyValuesError::UnsupportedOperation(
            "Long properties don't support double array access".to_string()
        ))
    }

    fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(PropertyValuesError::UnsupportedOperation(
            "Long properties don't support float array access".to_string()
        ))
    }

    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(PropertyValuesError::UnsupportedOperation(
            "Long properties don't support long array access".to_string()
        ))
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Ok(Box::new(self.long_value(node_id)?))
    }

    fn dimension(&self) -> Option<usize> {
        Some(1) // Scalar values have dimension 1
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        // Find maximum value by iterating through the array
        let mut max_val = None;
        for i in 0..self.element_count {
            let val = self.values.get(i);
            max_val = Some(max_val.map_or(val, |max: i64| max.max(val)));
        }
        max_val
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        self.get_max_long_property_value().map(|v| v as f64)
    }

    fn has_value(&self, node_id: u64) -> bool {
        (node_id as usize) < self.element_count
    }
}

// Implement LongNodePropertyValues trait
impl LongNodePropertyValues for HugeLongNodePropertyValues {
    fn long_value_unchecked(&self, node_id: u64) -> i64 {
        self.values.get(node_id as usize)
    }
}

/// Node property values backed by HugeDoubleArray for billion-scale storage.
pub struct HugeDoubleNodePropertyValues {
    values: HugeDoubleArray,
    default_value: f64,
    element_count: usize,
}

impl std::fmt::Debug for HugeDoubleNodePropertyValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HugeDoubleNodePropertyValues")
            .field("default_value", &self.default_value)
            .field("element_count", &self.element_count)
            .field("values_size", &self.values.size())
            .finish()
    }
}

impl Clone for HugeDoubleNodePropertyValues {
    fn clone(&self) -> Self {
        let mut values_copy = HugeDoubleArray::new(self.values.size());
        for i in 0..self.values.size() {
            values_copy.set(i, self.values.get(i));
        }
        Self {
            values: values_copy,
            default_value: self.default_value,
            element_count: self.element_count,
        }
    }
}

impl HugeDoubleNodePropertyValues {
    /// Creates a new HugeDoubleNodePropertyValues from a HugeDoubleArray.
    pub fn new(values: HugeDoubleArray, default_value: f64, element_count: usize) -> Self {
        Self {
            values,
            default_value,
            element_count,
        }
    }

    /// Creates a new instance with default value 0.0.
    pub fn with_default(values: HugeDoubleArray, element_count: usize) -> Self {
        Self::new(values, 0.0, element_count)
    }

    /// Returns a reference to the underlying HugeDoubleArray.
    pub fn values(&self) -> &HugeDoubleArray {
        &self.values
    }

    /// Returns the default value for missing properties.
    pub fn default_value(&self) -> f64 {
        self.default_value
    }
}

// Implement PropertyValues trait
impl PropertyValues for HugeDoubleNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Double
    }

    fn element_count(&self) -> usize {
        self.element_count
    }
}

// Implement NodePropertyValues trait
impl NodePropertyValues for HugeDoubleNodePropertyValues {
    fn double_value(&self, node_id: u64) -> PropertyValuesResult<f64> {
        if (node_id as usize) < self.element_count {
            Ok(self.values.get(node_id as usize))
        } else {
            Err(PropertyValuesError::InvalidNodeId(node_id))
        }
    }

    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64> {
        Ok(self.double_value(node_id)? as i64)
    }

    fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        Err(PropertyValuesError::UnsupportedOperation(
            "Double properties don't support double array access".to_string()
        ))
    }

    fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(PropertyValuesError::UnsupportedOperation(
            "Double properties don't support float array access".to_string()
        ))
    }

    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(PropertyValuesError::UnsupportedOperation(
            "Double properties don't support long array access".to_string()
        ))
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Ok(Box::new(self.double_value(node_id)?))
    }

    fn dimension(&self) -> Option<usize> {
        Some(1) // Scalar values have dimension 1
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        self.get_max_double_property_value().map(|v| v as i64)
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        // Find maximum value by iterating through the array
        let mut max_val = None;
        for i in 0..self.element_count {
            let val = self.values.get(i);
            max_val = Some(max_val.map_or(val, |max: f64| max.max(val)));
        }
        max_val
    }

    fn has_value(&self, node_id: u64) -> bool {
        (node_id as usize) < self.element_count
    }
}

// Implement DoubleNodePropertyValues trait
impl DoubleNodePropertyValues for HugeDoubleNodePropertyValues {
    fn double_value_unchecked(&self, node_id: u64) -> f64 {
        self.values.get(node_id as usize)
    }
}

/// Node property values backed by HugeObjectArray<Vec<i64>> for billion-scale array storage.
pub struct HugeLongArrayNodePropertyValues {
    values: HugeObjectArray<Vec<i64>>,
    element_count: usize,
}

impl std::fmt::Debug for HugeLongArrayNodePropertyValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HugeLongArrayNodePropertyValues")
            .field("element_count", &self.element_count)
            .field("values_size", &self.values.size())
            .finish()
    }
}

impl Clone for HugeLongArrayNodePropertyValues {
    fn clone(&self) -> Self {
        let mut values_copy = HugeObjectArray::new(self.values.size());
        for i in 0..self.values.size() {
            let array = self.values.get(i);
            values_copy.set(i, array.clone());
        }
        Self {
            values: values_copy,
            element_count: self.element_count,
        }
    }
}

impl HugeLongArrayNodePropertyValues {
    /// Creates a new HugeLongArrayNodePropertyValues from a HugeObjectArray<Vec<i64>>.
    pub fn new(values: HugeObjectArray<Vec<i64>>, element_count: usize) -> Self {
        Self {
            values,
            element_count,
        }
    }

    /// Returns a reference to the underlying HugeObjectArray.
    pub fn values(&self) -> &HugeObjectArray<Vec<i64>> {
        &self.values
    }
}

// Implement PropertyValues trait
impl PropertyValues for HugeLongArrayNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::LongArray
    }

    fn element_count(&self) -> usize {
        self.element_count
    }
}

// Implement NodePropertyValues trait
impl NodePropertyValues for HugeLongArrayNodePropertyValues {
    fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
        Err(PropertyValuesError::UnsupportedOperation(
            "LongArray properties don't support double value access".to_string()
        ))
    }

    fn long_value(&self, _node_id: u64) -> PropertyValuesResult<i64> {
        Err(PropertyValuesError::UnsupportedOperation(
            "LongArray properties don't support long value access".to_string()
        ))
    }

    fn double_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        Err(PropertyValuesError::UnsupportedOperation(
            "LongArray properties don't support double array access".to_string()
        ))
    }

    fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(PropertyValuesError::UnsupportedOperation(
            "LongArray properties don't support float array access".to_string()
        ))
    }

    fn long_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        if (node_id as usize) < self.element_count {
            Ok(self.values.get(node_id as usize).clone())
        } else {
            Err(PropertyValuesError::InvalidNodeId(node_id))
        }
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        if (node_id as usize) < self.element_count {
            Ok(Box::new(self.values.get(node_id as usize).clone()))
        } else {
            Err(PropertyValuesError::InvalidNodeId(node_id))
        }
    }

    fn dimension(&self) -> Option<usize> {
        None // Variable dimension for arrays
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        // Find maximum value by iterating through all arrays
        let mut max_val = None;
        for i in 0..self.element_count {
            let array = self.values.get(i);
            for &val in array {
                max_val = Some(max_val.map_or(val, |max: i64| max.max(val)));
            }
        }
        max_val
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        // Find maximum value by iterating through all arrays
        let mut max_val = None;
        for i in 0..self.element_count {
            let array = self.values.get(i);
            for &val in array {
                let val_f64 = val as f64;
                max_val = Some(max_val.map_or(val_f64, |max: f64| max.max(val_f64)));
            }
        }
        max_val
    }

    fn has_value(&self, node_id: u64) -> bool {
        (node_id as usize) < self.element_count
    }
}

// Implement LongArrayNodePropertyValues trait
impl LongArrayNodePropertyValues for HugeLongArrayNodePropertyValues {
    fn long_array_value_unchecked(&self, node_id: u64) -> Option<Vec<i64>> {
        Some(self.values.get(node_id as usize).clone())
    }
}

/// Node property values backed by HugeObjectArray<Vec<f64>> for billion-scale array storage.
pub struct HugeDoubleArrayNodePropertyValues {
    values: HugeObjectArray<Vec<f64>>,
    element_count: usize,
}

impl std::fmt::Debug for HugeDoubleArrayNodePropertyValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HugeDoubleArrayNodePropertyValues")
            .field("element_count", &self.element_count)
            .field("values_size", &self.values.size())
            .finish()
    }
}

impl Clone for HugeDoubleArrayNodePropertyValues {
    fn clone(&self) -> Self {
        let mut values_copy = HugeObjectArray::new(self.values.size());
        for i in 0..self.values.size() {
            let array = self.values.get(i);
            values_copy.set(i, array.clone());
        }
        Self {
            values: values_copy,
            element_count: self.element_count,
        }
    }
}

impl HugeDoubleArrayNodePropertyValues {
    /// Creates a new HugeDoubleArrayNodePropertyValues from a HugeObjectArray<Vec<f64>>.
    pub fn new(values: HugeObjectArray<Vec<f64>>, element_count: usize) -> Self {
        Self {
            values,
            element_count,
        }
    }

    /// Returns a reference to the underlying HugeObjectArray.
    pub fn values(&self) -> &HugeObjectArray<Vec<f64>> {
        &self.values
    }
}

// Implement PropertyValues trait
impl PropertyValues for HugeDoubleArrayNodePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::DoubleArray
    }

    fn element_count(&self) -> usize {
        self.element_count
    }
}

// Implement NodePropertyValues trait
impl NodePropertyValues for HugeDoubleArrayNodePropertyValues {
    fn double_value(&self, _node_id: u64) -> PropertyValuesResult<f64> {
        Err(PropertyValuesError::UnsupportedOperation(
            "DoubleArray properties don't support double value access".to_string()
        ))
    }

    fn long_value(&self, _node_id: u64) -> PropertyValuesResult<i64> {
        Err(PropertyValuesError::UnsupportedOperation(
            "DoubleArray properties don't support long value access".to_string()
        ))
    }

    fn double_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f64>> {
        if (node_id as usize) < self.element_count {
            Ok(self.values.get(node_id as usize).clone())
        } else {
            Err(PropertyValuesError::InvalidNodeId(node_id))
        }
    }

    fn float_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<f32>> {
        Err(PropertyValuesError::UnsupportedOperation(
            "DoubleArray properties don't support float array access".to_string()
        ))
    }

    fn long_array_value(&self, _node_id: u64) -> PropertyValuesResult<Vec<i64>> {
        Err(PropertyValuesError::UnsupportedOperation(
            "DoubleArray properties don't support long array access".to_string()
        ))
    }

    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        if (node_id as usize) < self.element_count {
            Ok(Box::new(self.values.get(node_id as usize).clone()))
        } else {
            Err(PropertyValuesError::InvalidNodeId(node_id))
        }
    }

    fn dimension(&self) -> Option<usize> {
        None // Variable dimension for arrays
    }

    fn get_max_long_property_value(&self) -> Option<i64> {
        // Find maximum value by iterating through all arrays
        let mut max_val = None;
        for i in 0..self.element_count {
            let array = self.values.get(i);
            for &val in array {
                let val_i64 = val as i64;
                max_val = Some(max_val.map_or(val_i64, |max: i64| max.max(val_i64)));
            }
        }
        max_val
    }

    fn get_max_double_property_value(&self) -> Option<f64> {
        // Find maximum value by iterating through all arrays
        let mut max_val = None;
        for i in 0..self.element_count {
            let array = self.values.get(i);
            for &val in array {
                max_val = Some(max_val.map_or(val, |max: f64| max.max(val)));
            }
        }
        max_val
    }

    fn has_value(&self, node_id: u64) -> bool {
        (node_id as usize) < self.element_count
    }
}

// Implement DoubleArrayNodePropertyValues trait
impl DoubleArrayNodePropertyValues for HugeDoubleArrayNodePropertyValues {
    fn double_array_value_unchecked(&self, node_id: u64) -> Option<Vec<f64>> {
        Some(self.values.get(node_id as usize).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::backends::huge::{HugeLongArray, HugeDoubleArray, HugeObjectArray};

    #[test]
    fn test_huge_long_node_property_values() {
        let values = HugeLongArray::new(100);
        let props = HugeLongNodePropertyValues::with_default(values, 100);
        
        assert_eq!(props.value_type(), ValueType::Long);
        assert_eq!(props.element_count(), 100);
        assert_eq!(props.default_value(), 0);
    }

    #[test]
    fn test_huge_double_node_property_values() {
        let values = HugeDoubleArray::new(100);
        let props = HugeDoubleNodePropertyValues::with_default(values, 100);
        
        assert_eq!(props.value_type(), ValueType::Double);
        assert_eq!(props.element_count(), 100);
        assert_eq!(props.default_value(), 0.0);
    }

    #[test]
    fn test_huge_long_property_access() {
        let mut values = HugeLongArray::new(10);
        values.set(5, 42);
        let props = HugeLongNodePropertyValues::new(values, 0, 10);
        
        assert_eq!(props.long_value(5).unwrap(), 42);
        assert_eq!(props.double_value(5).unwrap(), 42.0);
        assert!(props.has_value(5));
        assert!(!props.has_value(15));
    }

    #[test]
    fn test_huge_double_property_access() {
        let mut values = HugeDoubleArray::new(10);
        values.set(3, 3.14);
        let props = HugeDoubleNodePropertyValues::new(values, 0.0, 10);
        
        assert_eq!(props.double_value(3).unwrap(), 3.14);
        assert_eq!(props.long_value(3).unwrap(), 3);
        assert!(props.has_value(3));
        assert!(!props.has_value(20));
    }

    #[test]
    fn test_huge_long_array_node_property_values() {
        let values = HugeObjectArray::new(100);
        let props = HugeLongArrayNodePropertyValues::new(values, 100);
        
        assert_eq!(props.value_type(), ValueType::LongArray);
        assert_eq!(props.element_count(), 100);
    }

    #[test]
    fn test_huge_double_array_node_property_values() {
        let values = HugeObjectArray::new(100);
        let props = HugeDoubleArrayNodePropertyValues::new(values, 100);
        
        assert_eq!(props.value_type(), ValueType::DoubleArray);
        assert_eq!(props.element_count(), 100);
    }

    #[test]
    fn test_huge_long_array_property_access() {
        let mut values = HugeObjectArray::new(10);
        values.set(5, vec![1, 2, 3]);
        let props = HugeLongArrayNodePropertyValues::new(values, 10);
        
        assert_eq!(props.long_array_value(5).unwrap(), vec![1, 2, 3]);
        assert_eq!(props.long_array_value_unchecked(5), Some(vec![1, 2, 3]));
        assert!(props.has_value(5));
        assert!(!props.has_value(15));
    }

    #[test]
    fn test_huge_double_array_property_access() {
        let mut values = HugeObjectArray::new(10);
        values.set(3, vec![1.1, 2.2, 3.3]);
        let props = HugeDoubleArrayNodePropertyValues::new(values, 10);
        
        assert_eq!(props.double_array_value(3).unwrap(), vec![1.1, 2.2, 3.3]);
        assert_eq!(props.double_array_value_unchecked(3), Some(vec![1.1, 2.2, 3.3]));
        assert!(props.has_value(3));
        assert!(!props.has_value(20));
    }
}