use crate::types::property_value::PropertyValue;
use crate::types::value_type::ValueType;
use thiserror::Error;

/// Macro to generate PropertyValues implementations for different value types.
/// This eliminates duplication across Node, Relationship, and Graph property value implementations.
///
/// # Variants
///
/// - Base variant: For node properties with `Option<Vec<T>>`
/// - `relationship`: For relationship properties with direct `Vec<T>` and `element_count`
/// - `array`: For node array properties with `Option<Vec<Option<Vec<T>>>>`
/// - `graph`: For graph properties with direct `Vec<T>`
/// - `graph_array`: For graph array properties with `Vec<Vec<T>>`
///
/// # Examples
///
/// See the modular value type implementations in:
/// - `src/types/properties/node/impls/values/`
/// - `src/types/properties/graph/impls/values/`
/// - `src/types/properties/relationship/impls/values/`
#[macro_export]
macro_rules! property_values_impl {
    ($struct_name:ident, $value_type:ident, $rust_type:ty, $property_value_variant:expr) => {
        impl PropertyValues for $struct_name {
            fn value_type(&self) -> ValueType {
                ValueType::$value_type
            }

            fn element_count(&self) -> usize {
                self.node_count
            }

            fn get_property_value(&self, index: usize) -> Option<PropertyValue> {
                self.values.get(index).map(|&v| $property_value_variant(v))
            }
        }
    };
    ($struct_name:ident, $value_type:ident, $rust_type:ty, $property_value_variant:expr, relationship) => {
        impl PropertyValues for $struct_name {
            fn value_type(&self) -> ValueType {
                ValueType::$value_type
            }

            fn element_count(&self) -> usize {
                self.element_count
            }

            fn get_property_value(&self, index: usize) -> Option<PropertyValue> {
                self.values.get(index).map(|&v| $property_value_variant(v))
            }
        }
    };
    ($struct_name:ident, $value_type:ident, $rust_type:ty, $property_value_variant:expr, array) => {
        impl PropertyValues for $struct_name {
            fn value_type(&self) -> ValueType {
                ValueType::$value_type
            }

            fn element_count(&self) -> usize {
                self.node_count
            }

            fn get_property_value(&self, index: usize) -> Option<PropertyValue> {
                self.values
                    .get(index)
                    .and_then(|v| v.as_ref().map(|arr| $property_value_variant(arr.clone())))
            }
        }
    };
    ($struct_name:ident, $value_type:ident, $rust_type:ty, $property_value_variant:expr, graph) => {
        impl PropertyValues for $struct_name {
            fn value_type(&self) -> ValueType {
                ValueType::$value_type
            }

            fn element_count(&self) -> usize {
                self.values.len()
            }

            fn get_property_value(&self, index: usize) -> Option<PropertyValue> {
                self.values.get(index).map(|&v| $property_value_variant(v))
            }
        }
    };
    ($struct_name:ident, $value_type:ident, $rust_type:ty, $property_value_variant:expr, graph_array) => {
        impl PropertyValues for $struct_name {
            fn value_type(&self) -> ValueType {
                ValueType::$value_type
            }

            fn element_count(&self) -> usize {
                self.values.len()
            }

            fn get_property_value(&self, index: usize) -> Option<PropertyValue> {
                self.values
                    .get(index)
                    .map(|v| $property_value_variant(v.clone()))
            }
        }
    };
}

/// Macro to generate complete NodePropertyValues implementation for scalar Long type.
/// Generates all accessor methods, type conversions, and error cases.
#[macro_export]
macro_rules! node_long_property_values_impl {
    ($struct_name:ident) => {
        impl NodePropertyValues for $struct_name {
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
    };
}

/// Macro to generate complete NodePropertyValues implementation for scalar Double type.
#[macro_export]
macro_rules! node_double_property_values_impl {
    ($struct_name:ident) => {
        impl NodePropertyValues for $struct_name {
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
    };
}

/// Macro to generate complete NodePropertyValues implementation for DoubleArray type.
#[macro_export]
macro_rules! node_double_array_property_values_impl {
    ($struct_name:ident) => {
        impl NodePropertyValues for $struct_name {
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
    };
}

/// Macro to generate complete NodePropertyValues implementation for FloatArray type.
#[macro_export]
macro_rules! node_float_array_property_values_impl {
    ($struct_name:ident) => {
        impl NodePropertyValues for $struct_name {
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
    };
}

/// Macro to generate complete NodePropertyValues implementation for LongArray type.
#[macro_export]
macro_rules! node_long_array_property_values_impl {
    ($struct_name:ident) => {
        impl NodePropertyValues for $struct_name {
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
    };
}

/// Error type for property value operations.
#[derive(Error, Debug, Clone)]
pub enum PropertyValuesError {
    #[error("Tried to retrieve a value of type {expected:?} from properties of type {actual:?}")]
    UnsupportedType {
        actual: ValueType,
        expected: ValueType,
    },

    #[error("Operation not supported: {0}")]
    UnsupportedOperation(String),

    #[error("Invalid node ID: {0}")]
    InvalidNodeId(u64),

    #[error("Value not found for ID: {0}")]
    ValueNotFound(u64),
}

/// Base trait for all property value containers.
/// Provides access to the value type and common utilities.
///
/// This mirrors the TypeScript PropertyValues interface.
pub trait PropertyValues: Send + Sync + std::fmt::Debug {
    /// Returns the value type of the property values.
    fn value_type(&self) -> ValueType;

    /// Returns the number of elements with property values.
    /// For node properties, this is the node count.
    /// For graph properties, this is the value count.
    fn element_count(&self) -> usize;

    /// Unified accessor that returns a PropertyValue enum variant for the given index.
    /// This provides a single interface for accessing all property value types.
    fn get_property_value(&self, index: usize) -> Option<PropertyValue>;
}

impl PropertyValuesError {
    /// Creates an error for unsupported type operations.
    pub fn unsupported_type(actual: ValueType, expected: ValueType) -> Self {
        PropertyValuesError::UnsupportedType { actual, expected }
    }

    /// Creates an error for unsupported operations.
    pub fn unsupported_operation(message: impl Into<String>) -> Self {
        PropertyValuesError::UnsupportedOperation(message.into())
    }
}

pub type PropertyValuesResult<T> = Result<T, PropertyValuesError>;

// Implement PropertyValues for Box<dyn PropertyValues> to allow trait objects
impl PropertyValues for Box<dyn PropertyValues> {
    fn value_type(&self) -> ValueType {
        (**self).value_type()
    }

    fn element_count(&self) -> usize {
        (**self).element_count()
    }

    fn get_property_value(&self, index: usize) -> Option<PropertyValue> {
        (**self).get_property_value(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = PropertyValuesError::unsupported_type(ValueType::Long, ValueType::Double);
        assert!(err.to_string().contains("Long"));
        assert!(err.to_string().contains("Double"));

        let err = PropertyValuesError::unsupported_operation("test operation");
        assert!(err.to_string().contains("test operation"));
    }
}
