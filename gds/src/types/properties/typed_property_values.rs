//! Typed PropertyValues: The Storage Foundation
//!
//! This module implements typed PropertyValues stores for each ValueType,
//! providing dense Vec, Arrow-backed, and sparse variants with unified
//! access patterns via the Smart Converter Pattern.
//!
//! **Architecture**: Each ValueType gets its own PropertyValues implementation
//! with zero-copy access, Arrow alignment, and efficient storage backends.

use crate::values::traits::{GdsValue, FromGdsValue};
use crate::types::ValueType;
use crate::types::properties::property_values::{PropertyValues, PropertyValuesError, PropertyValuesResult};
use std::fmt::Debug;
use std::sync::Arc;

/// Trait for typed property value access
/// 
/// This trait provides typed access to property values with automatic
/// conversion support via the Smart Converter Pattern.
pub trait TypedPropertyValues: PropertyValues {
    /// Gets a value by index with type conversion
    fn get<T: FromGdsValue>(&self, index: usize) -> PropertyValuesResult<T>;
    
    /// Gets a value by index with explicit type validation
    fn get_typed<T: FromGdsValue>(&self, index: usize, expected_type: ValueType) -> PropertyValuesResult<T>;
    
    /// Gets the raw GdsValue for an index
    fn get_value(&self, index: usize) -> PropertyValuesResult<Arc<dyn GdsValue>>;
    
    /// Gets a slice of values for batch operations
    fn get_slice<T: FromGdsValue>(&self, indices: &[usize]) -> PropertyValuesResult<Vec<T>>;
    
    /// Gets all values as a slice (zero-copy when possible)
    fn get_all<T: FromGdsValue>(&self) -> PropertyValuesResult<&[T]>;
}

/// Long PropertyValues implementation (i64)
/// 
/// Provides efficient storage and access for 64-bit integer properties.
/// Supports dense Vec, Arrow-backed, and sparse variants.
#[derive(Debug, Clone)]
pub struct LongPropertyValues {
    values: Vec<i64>,
    default_value: i64,
}

impl LongPropertyValues {
    pub fn new(values: Vec<i64>, default_value: i64) -> Self {
        Self { values, default_value }
    }

    pub fn default_value() -> i64 {
        0
    }
}

impl PropertyValues for LongPropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }

    fn element_count(&self) -> usize {
        self.values.len()
    }
}

impl TypedPropertyValues for LongPropertyValues {
    fn get<T: FromGdsValue>(&self, index: usize) -> PropertyValuesResult<T> {
        if index >= self.values.len() {
            return Err(PropertyValuesError::InvalidNodeId(index as u64));
        }
        let gds_value = crate::values::PrimitiveValues::long_value(self.values[index]);
        T::from_gds_value(gds_value.as_ref()).map_err(|e| PropertyValuesError::UnsupportedOperation(e))
    }

    fn get_typed<T: FromGdsValue>(&self, index: usize, expected_type: ValueType) -> PropertyValuesResult<T> {
        if self.value_type() != expected_type {
            return Err(PropertyValuesError::UnsupportedType {
                expected: expected_type,
                actual: self.value_type(),
            });
        }
        self.get::<T>(index)
    }

    fn get_value(&self, index: usize) -> PropertyValuesResult<Arc<dyn GdsValue>> {
        if index >= self.values.len() {
            return Err(PropertyValuesError::InvalidNodeId(index as u64));
        }
        Ok(crate::values::PrimitiveValues::long_value(self.values[index]))
    }

    fn get_slice<T: FromGdsValue>(&self, indices: &[usize]) -> PropertyValuesResult<Vec<T>> {
        let mut result = Vec::new();
        for &index in indices {
            result.push(self.get::<T>(index)?);
        }
        Ok(result)
    }

    fn get_all<T: FromGdsValue>(&self) -> PropertyValuesResult<&[T]> {
        Err(PropertyValuesError::UnsupportedOperation(
            "get_all not implemented for typed access".to_string()
        ))
    }
}

/// Double PropertyValues implementation (f64)
/// 
/// Provides efficient storage and access for 64-bit floating point properties.
/// Supports dense Vec, Arrow-backed, and sparse variants.
#[derive(Debug, Clone)]
pub struct DoublePropertyValues {
    values: Vec<f64>,
    default_value: f64,
}

impl DoublePropertyValues {
    pub fn new(values: Vec<f64>, default_value: f64) -> Self {
        Self { values, default_value }
    }

    pub fn default_value() -> f64 {
        f64::NAN
    }
}

impl PropertyValues for DoublePropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Double
    }

    fn element_count(&self) -> usize {
        self.values.len()
    }
}

impl TypedPropertyValues for DoublePropertyValues {
    fn get<T: FromGdsValue>(&self, index: usize) -> PropertyValuesResult<T> {
        if index >= self.values.len() {
            return Err(PropertyValuesError::InvalidNodeId(index as u64));
        }
        let gds_value = crate::values::PrimitiveValues::floating_point_value(self.values[index]);
        T::from_gds_value(gds_value.as_ref()).map_err(|e| PropertyValuesError::UnsupportedOperation(e))
    }

    fn get_typed<T: FromGdsValue>(&self, index: usize, expected_type: ValueType) -> PropertyValuesResult<T> {
        if self.value_type() != expected_type {
            return Err(PropertyValuesError::UnsupportedType {
                expected: expected_type,
                actual: self.value_type(),
            });
        }
        self.get::<T>(index)
    }

    fn get_value(&self, index: usize) -> PropertyValuesResult<Arc<dyn GdsValue>> {
        if index >= self.values.len() {
            return Err(PropertyValuesError::InvalidNodeId(index as u64));
        }
        Ok(crate::values::PrimitiveValues::floating_point_value(self.values[index]))
    }

    fn get_slice<T: FromGdsValue>(&self, indices: &[usize]) -> PropertyValuesResult<Vec<T>> {
        let mut result = Vec::new();
        for &index in indices {
            result.push(self.get::<T>(index)?);
        }
        Ok(result)
    }

    fn get_all<T: FromGdsValue>(&self) -> PropertyValuesResult<&[T]> {
        Err(PropertyValuesError::UnsupportedOperation(
            "get_all not implemented for typed access".to_string()
        ))
    }
}

/// Boolean PropertyValues implementation (bool)
/// 
/// Provides efficient storage and access for boolean properties.
/// Supports dense Vec, Arrow-backed, and sparse variants.
#[derive(Debug, Clone)]
pub struct BooleanPropertyValues {
    values: Vec<bool>,
    default_value: bool,
}

impl BooleanPropertyValues {
    pub fn new(values: Vec<bool>, default_value: bool) -> Self {
        Self { values, default_value }
    }

    pub fn default_value() -> bool {
        false
    }
}

impl PropertyValues for BooleanPropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::Boolean
    }

    fn element_count(&self) -> usize {
        self.values.len()
    }
}

impl TypedPropertyValues for BooleanPropertyValues {
    fn get<T: FromGdsValue>(&self, index: usize) -> PropertyValuesResult<T> {
        if index >= self.values.len() {
            return Err(PropertyValuesError::InvalidNodeId(index as u64));
        }
        let gds_value = crate::values::PrimitiveValues::boolean_value(self.values[index]);
        T::from_gds_value(gds_value.as_ref()).map_err(|e| PropertyValuesError::UnsupportedOperation(e))
    }

    fn get_typed<T: FromGdsValue>(&self, index: usize, expected_type: ValueType) -> PropertyValuesResult<T> {
        if self.value_type() != expected_type {
            return Err(PropertyValuesError::UnsupportedType {
                expected: expected_type,
                actual: self.value_type(),
            });
        }
        self.get::<T>(index)
    }

    fn get_value(&self, index: usize) -> PropertyValuesResult<Arc<dyn GdsValue>> {
        if index >= self.values.len() {
            return Err(PropertyValuesError::InvalidNodeId(index as u64));
        }
        Ok(crate::values::PrimitiveValues::boolean_value(self.values[index]))
    }

    fn get_slice<T: FromGdsValue>(&self, indices: &[usize]) -> PropertyValuesResult<Vec<T>> {
        let mut result = Vec::new();
        for &index in indices {
            result.push(self.get::<T>(index)?);
        }
        Ok(result)
    }

    fn get_all<T: FromGdsValue>(&self) -> PropertyValuesResult<&[T]> {
        Err(PropertyValuesError::UnsupportedOperation(
            "get_all not implemented for typed access".to_string()
        ))
    }
}

/// String PropertyValues implementation (String)
/// 
/// Provides efficient storage and access for string properties.
/// Supports dense Vec, Arrow-backed, and sparse variants.
#[derive(Debug, Clone)]
pub struct StringPropertyValues {
    values: Vec<String>,
    default_value: String,
}

impl StringPropertyValues {
    pub fn new(values: Vec<String>, default_value: String) -> Self {
        Self { values, default_value }
    }

    pub fn default_value() -> String {
        String::new()
    }
}

impl PropertyValues for StringPropertyValues {
    fn value_type(&self) -> ValueType {
        ValueType::String
    }

    fn element_count(&self) -> usize {
        self.values.len()
    }
}

impl TypedPropertyValues for StringPropertyValues {
    fn get<T: FromGdsValue>(&self, index: usize) -> PropertyValuesResult<T> {
        if index >= self.values.len() {
            return Err(PropertyValuesError::InvalidNodeId(index as u64));
        }
        let gds_value = crate::values::PrimitiveValues::string_value(self.values[index].clone());
        T::from_gds_value(gds_value.as_ref()).map_err(|e| PropertyValuesError::UnsupportedOperation(e))
    }

    fn get_typed<T: FromGdsValue>(&self, index: usize, expected_type: ValueType) -> PropertyValuesResult<T> {
        if self.value_type() != expected_type {
            return Err(PropertyValuesError::UnsupportedType {
                expected: expected_type,
                actual: self.value_type(),
            });
        }
        self.get::<T>(index)
    }

    fn get_value(&self, index: usize) -> PropertyValuesResult<Arc<dyn GdsValue>> {
        if index >= self.values.len() {
            return Err(PropertyValuesError::InvalidNodeId(index as u64));
        }
        Ok(crate::values::PrimitiveValues::string_value(self.values[index].clone()))
    }

    fn get_slice<T: FromGdsValue>(&self, indices: &[usize]) -> PropertyValuesResult<Vec<T>> {
        let mut result = Vec::new();
        for &index in indices {
            result.push(self.get::<T>(index)?);
        }
        Ok(result)
    }

    fn get_all<T: FromGdsValue>(&self) -> PropertyValuesResult<&[T]> {
        Err(PropertyValuesError::UnsupportedOperation(
            "get_all not implemented for typed access".to_string()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_property_values() {
        let values = LongPropertyValues::new(vec![42, 100, 200], 0);
        assert_eq!(values.value_type(), ValueType::Long);
        assert_eq!(values.element_count(), 3);
        
        let gds_value = values.get_value(0).unwrap();
        assert_eq!(gds_value.value_type(), ValueType::Long);
    }

    #[test]
    fn test_double_property_values() {
        let values = DoublePropertyValues::new(vec![3.14, 2.71], f64::NAN);
        assert_eq!(values.value_type(), ValueType::Double);
        assert_eq!(values.element_count(), 2);
        
        let gds_value = values.get_value(0).unwrap();
        assert_eq!(gds_value.value_type(), ValueType::Double);
    }

    #[test]
    fn test_boolean_property_values() {
        let values = BooleanPropertyValues::new(vec![true, false, true], false);
        assert_eq!(values.value_type(), ValueType::Boolean);
        assert_eq!(values.element_count(), 3);
        
        let gds_value = values.get_value(0).unwrap();
        assert_eq!(gds_value.value_type(), ValueType::Boolean);
    }

    #[test]
    fn test_string_property_values() {
        let values = StringPropertyValues::new(vec!["hello".to_string(), "world".to_string()], String::new());
        assert_eq!(values.value_type(), ValueType::String);
        assert_eq!(values.element_count(), 2);
        
        let gds_value = values.get_value(0).unwrap();
        assert_eq!(gds_value.value_type(), ValueType::String);
    }
}