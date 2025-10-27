//! PropertyValues Implementation Macros
//!
//! Generates typed storage backends for each ValueType with proper
//! element counting and type conversion policies.
//!
//! **Pattern**: Every PropertyValues implementation is a Smart Converter
//! that accepts all type queries and either returns exact type (zero-cost),
//! converts compatible types (i64→f64), or throws error if incompatible.

use crate::types::ValueType;
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
#[macro_export]
macro_rules! property_values_impl {
    // Base variant: For node properties with node_count field
    ($struct_name:ident, $value_type:ident) => {
        impl PropertyValues for $struct_name {
            fn value_type(&self) -> ValueType {
                ValueType::$value_type
            }

            fn element_count(&self) -> usize {
                self.node_count
            }
        }
    };
    // Relationship variant: Uses element_count field instead of node_count
    ($struct_name:ident, $value_type:ident, relationship) => {
        impl PropertyValues for $struct_name {
            fn value_type(&self) -> ValueType {
                ValueType::$value_type
            }

            fn element_count(&self) -> usize {
                self.element_count
            }
        }
    };
    // Array variant: For node array properties (same as base, using node_count)
    ($struct_name:ident, $value_type:ident, array) => {
        impl PropertyValues for $struct_name {
            fn value_type(&self) -> ValueType {
                ValueType::$value_type
            }

            fn element_count(&self) -> usize {
                self.node_count
            }
        }
    };
    // Graph variant: element_count based on values.len()
    ($struct_name:ident, $value_type:ident, graph) => {
        impl PropertyValues for $struct_name {
            fn value_type(&self) -> ValueType {
                ValueType::$value_type
            }

            fn element_count(&self) -> usize {
                self.values.len()
            }
        }
    };
    // Graph array variant: element_count based on values.len()
    ($struct_name:ident, $value_type:ident, graph_array) => {
        impl PropertyValues for $struct_name {
            fn value_type(&self) -> ValueType {
                ValueType::$value_type
            }

            fn element_count(&self) -> usize {
                self.values.len()
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

// ============================================================================
// Universal PropertyValues Trait Implementation Macros (Collections-backed)
// ============================================================================
//
// These macros generate trait implementations for property values that are
// backed by the UniversalAdapter + Collections system, enabling backend
// flexibility (Vec, Huge, Arrow) at runtime.

/// Implements PropertyValues trait for UniversalAdapter-backed types.
///
/// This macro generates the base PropertyValues trait implementation for
/// any property value type that wraps UniversalPropertyValues<T, C>.
///
/// # Parameters
///
/// - `$struct_name`: The generic struct type (e.g., `DefaultLongNodePropertyValues<C>`)
/// - `$value_type`: The ValueType enum variant (e.g., `Long`)
/// - `$count_field`: The field name for element count (e.g., `node_count`, `element_count`)
/// - `$element_type`: The Rust type of the elements (e.g., `i64`, `f64`)
#[macro_export]
macro_rules! impl_property_values_universal {
    ($struct_name:ty, $value_type:ident, $count_field:ident, $element_type:ty) => {
        impl<C> $crate::types::properties::PropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<$element_type> 
                + $crate::collections::traits::PropertyValuesAdapter<$element_type> 
                + Send + Sync + std::fmt::Debug,
        {
            fn value_type(&self) -> $crate::types::ValueType {
                $crate::types::ValueType::$value_type
            }

            fn element_count(&self) -> usize {
                self.$count_field
            }
        }
    };
}

/// Implements NodePropertyValues trait for UniversalAdapter-backed node properties.
///
/// This macro generates the NodePropertyValues trait implementation, providing
/// default implementations for all accessor methods and the node_count field.
///
/// # Parameters
///
/// - `$struct_name`: The generic struct type
/// - `$element_type`: The Rust type of the elements
#[macro_export]
macro_rules! impl_node_property_values_universal {
    ($struct_name:ty, $element_type:ty) => {
        impl<C> $crate::types::properties::node::NodePropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<$element_type> 
                + $crate::collections::traits::PropertyValuesAdapter<$element_type> 
                + Send + Sync + std::fmt::Debug,
        {
            fn node_count(&self) -> usize {
                self.node_count
            }

            fn get_max_long_property_value(&self) -> Option<i64> {
                // This will be overridden by typed implementations for integral types
                None
            }

            fn get_max_double_property_value(&self) -> Option<f64> {
                // This will be overridden by typed implementations for floating-point types
                None
            }

            fn has_value(&self, node_id: u64) -> bool {
                node_id < self.node_count as u64 
                    && !self.universal.collection().is_null(node_id as usize)
            }

            fn double_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<f64> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::Double,
                ))
            }

            fn long_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<i64> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::Long,
                ))
            }

            fn double_array_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<f64>> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::DoubleArray,
                ))
            }

            fn float_array_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<f32>> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::FloatArray,
                ))
            }

            fn long_array_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<i64>> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::LongArray,
                ))
            }

            fn get_object(&self, node_id: u64) -> $crate::types::properties::PropertyValuesResult<Box<dyn std::any::Any>> {
                use $crate::collections::traits::Collections;
                self.universal
                    .collection()
                    .get(node_id as usize)
                    .map(|v| Box::new(v) as Box<dyn std::any::Any>)
                    .ok_or_else(|| $crate::types::properties::PropertyValuesError::InvalidNodeId(node_id))
            }

            fn dimension(&self) -> Option<usize> {
                // Default: scalar values have dimension 1
                Some(1)
            }
        }
    };
}

/// Implements typed NodePropertyValues methods for specific value types.
///
/// This macro generates the type-specific accessor methods (e.g., long_value, double_value)
/// for the given ValueType, using the Collections interface for data access.
///
/// # Scalar Variants
///
/// - IntegralScalar: Implements long_value, provides long→double conversion
/// - FloatingPointScalar: Implements double_value, provides double→long conversion
/// - OtherScalar (Boolean): Implements custom accessor
#[macro_export]
macro_rules! impl_typed_node_property_values_universal {
    // Integral Scalar types (Byte, Short, Int, Long)
    ($struct_name:ty, $value_type:ident, IntegralScalar, $element_type:ty) => {
        // Implement NodePropertyValues with proper long_value/double_value
        impl<C> $crate::types::properties::node::NodePropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<$element_type> 
                + $crate::collections::traits::PropertyValuesAdapter<$element_type> 
                + Send + Sync + std::fmt::Debug,
        {
            fn long_value(&self, node_id: u64) -> $crate::types::properties::PropertyValuesResult<i64> {
                use $crate::collections::traits::Collections;
                self.universal
                    .collection()
                    .get(node_id as usize)
                    .map(|v| v as i64)
                    .ok_or_else(|| $crate::types::properties::PropertyValuesError::InvalidNodeId(node_id))
            }
            
            fn double_value(&self, node_id: u64) -> $crate::types::properties::PropertyValuesResult<f64> {
                Ok(self.long_value(node_id)? as f64)
            }
            
            fn has_value(&self, node_id: u64) -> bool {
                node_id < self.node_count as u64 
                    && !self.universal.collection().is_null(node_id as usize)
            }
            
            fn double_array_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<f64>> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::DoubleArray,
                ))
            }

            fn float_array_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<f32>> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::FloatArray,
                ))
            }

            fn long_array_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<i64>> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::LongArray,
                ))
            }

            fn get_object(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<Box<dyn std::any::Any>> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::String,
                ))
            }
            
            fn dimension(&self) -> Option<usize> {
                Some(1) // Scalar values have dimension 1
            }
            
            fn get_max_long_property_value(&self) -> Option<i64> {
                None // Would require scanning all values
            }
            
            fn get_max_double_property_value(&self) -> Option<f64> {
                None // Would require scanning all values
            }
        }
        
        // Implement the specific Long trait
        impl<C> $crate::types::properties::node::LongNodePropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<$element_type> 
                + $crate::collections::traits::PropertyValuesAdapter<$element_type> 
                + Send + Sync + std::fmt::Debug,
        {
            fn long_value_unchecked(&self, node_id: u64) -> i64 {
                use $crate::collections::traits::Collections;
                self.universal
                    .collection()
                    .get(node_id as usize)
                    .map(|v| v as i64)
                    .unwrap_or_else(|| self.universal.default_value() as i64)
            }
        }
    };

    // FloatingPoint Scalar types (Float, Double)
    ($struct_name:ty, $value_type:ident, FloatingPointScalar, $element_type:ty) => {
        // Implement NodePropertyValues with proper double_value/long_value
        impl<C> $crate::types::properties::node::NodePropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<$element_type> 
                + $crate::collections::traits::PropertyValuesAdapter<$element_type> 
                + Send + Sync + std::fmt::Debug,
        {
            fn double_value(&self, node_id: u64) -> $crate::types::properties::PropertyValuesResult<f64> {
                use $crate::collections::traits::Collections;
                self.universal
                    .collection()
                    .get(node_id as usize)
                    .map(|v| v as f64)
                    .ok_or_else(|| $crate::types::properties::PropertyValuesError::InvalidNodeId(node_id))
            }
            
            fn long_value(&self, node_id: u64) -> $crate::types::properties::PropertyValuesResult<i64> {
                Ok(self.double_value(node_id)? as i64)
            }
            
            fn has_value(&self, node_id: u64) -> bool {
                node_id < self.node_count as u64 
                    && !self.universal.collection().is_null(node_id as usize)
            }
            
            fn double_array_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<f64>> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::DoubleArray,
                ))
            }

            fn float_array_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<f32>> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::FloatArray,
                ))
            }

            fn long_array_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<i64>> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::LongArray,
                ))
            }

            fn get_object(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<Box<dyn std::any::Any>> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::String,
                ))
            }
            
            fn dimension(&self) -> Option<usize> {
                Some(1) // Scalar values have dimension 1
            }
            
            fn get_max_long_property_value(&self) -> Option<i64> {
                None // Would require scanning all values
            }
            
            fn get_max_double_property_value(&self) -> Option<f64> {
                None // Would require scanning all values
            }
        }
        
        // Implement the specific Double trait
        impl<C> $crate::types::properties::node::DoubleNodePropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<$element_type> 
                + $crate::collections::traits::PropertyValuesAdapter<$element_type> 
                + Send + Sync + std::fmt::Debug,
        {
            fn double_value_unchecked(&self, node_id: u64) -> f64 {
                use $crate::collections::traits::Collections;
                self.universal
                    .collection()
                    .get(node_id as usize)
                    .map(|v| v as f64)
                    .unwrap_or_else(|| self.universal.default_value() as f64)
            }
        }
    };

    // Integral Array types (LongArray, IntArray, etc.)
    ($struct_name:ty, $value_type:ident, IntegralArray, $element_type:ty) => {
        // Implement NodePropertyValues with proper array accessors
        impl<C> $crate::types::properties::node::NodePropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<$element_type> 
                + $crate::collections::traits::PropertyValuesAdapter<$element_type> 
                + Send + Sync + std::fmt::Debug,
        {
            fn long_array_value(&self, node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<i64>> {
                use $crate::collections::traits::Collections;
                // For arrays: get() returns Option<Option<Vec<T>>>
                // Convert element type to i64
                match self.universal.collection().get(node_id as usize) {
                    Some(Some(vec)) => Ok(vec.into_iter().map(|v| v as i64).collect()),
                    _ => Err($crate::types::properties::PropertyValuesError::InvalidNodeId(node_id))
                }
            }
            
            fn long_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<i64> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::Long,
                ))
            }
            
            fn double_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<f64> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::Double,
                ))
            }
            
            fn has_value(&self, node_id: u64) -> bool {
                use $crate::collections::traits::Collections;
                node_id < self.node_count as u64 
                    && self.universal.collection().get(node_id as usize).flatten().is_some()
            }
            
            fn double_array_value(&self, node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<f64>> {
                // Convert i64 array to f64 array
                self.long_array_value(node_id)
                    .map(|arr| arr.into_iter().map(|v| v as f64).collect())
            }

            fn float_array_value(&self, node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<f32>> {
                // Convert i64 array to f32 array
                self.long_array_value(node_id)
                    .map(|arr| arr.into_iter().map(|v| v as f32).collect())
            }

            fn get_object(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<Box<dyn std::any::Any>> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::String,
                ))
            }
            
            fn dimension(&self) -> Option<usize> {
                // Calculate dimension by checking first non-null array
                use $crate::collections::traits::Collections;
                for i in 0..self.node_count {
                    if let Some(Some(arr)) = self.universal.collection().get(i) {
                        return Some(arr.len());
                    }
                }
                None
            }
            
            fn get_max_long_property_value(&self) -> Option<i64> {
                None // Would require scanning all arrays
            }
            
            fn get_max_double_property_value(&self) -> Option<f64> {
                None // Would require scanning all arrays
            }
        }
        
        // Implement the specific LongArray trait
        impl<C> $crate::types::properties::node::LongArrayNodePropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<$element_type> 
                + $crate::collections::traits::PropertyValuesAdapter<$element_type> 
                + Send + Sync + std::fmt::Debug,
        {
            fn long_array_value_unchecked(&self, node_id: u64) -> Option<Vec<i64>> {
                use $crate::collections::traits::Collections;
                // For arrays: get() returns Option<Option<Vec<T>>>
                // Convert element type to i64
                self.universal
                    .collection()
                    .get(node_id as usize)
                    .and_then(|opt_vec| opt_vec)
                    .map(|vec| vec.into_iter().map(|v| v as i64).collect())
            }
        }
    };

    // FloatingPoint Array types (DoubleArray, FloatArray)
    ($struct_name:ty, $value_type:ident, FloatingPointArray, $element_type:ty) => {
        // Implement NodePropertyValues with proper array accessors
        impl<C> $crate::types::properties::node::NodePropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<$element_type> 
                + $crate::collections::traits::PropertyValuesAdapter<$element_type> 
                + Send + Sync + std::fmt::Debug,
        {
            fn double_array_value(&self, node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<f64>> {
                use $crate::collections::traits::Collections;
                // For arrays: get() returns Option<Option<Vec<T>>>
                // Convert f32/f64 to f64 and unwrap
                match self.universal.collection().get(node_id as usize) {
                    Some(Some(vec)) => Ok(vec.into_iter().map(|v| v as f64).collect()),
                    _ => Err($crate::types::properties::PropertyValuesError::InvalidNodeId(node_id))
                }
            }
            
            fn float_array_value(&self, node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<f32>> {
                use $crate::collections::traits::Collections;
                // For arrays: get() returns Option<Option<Vec<T>>>
                // Convert f32/f64 to f32 and unwrap
                match self.universal.collection().get(node_id as usize) {
                    Some(Some(vec)) => Ok(vec.into_iter().map(|v| v as f32).collect()),
                    _ => Err($crate::types::properties::PropertyValuesError::InvalidNodeId(node_id))
                }
            }
            
            fn long_array_value(&self, node_id: u64) -> $crate::types::properties::PropertyValuesResult<Vec<i64>> {
                // Convert f64/f32 array to i64 array
                self.double_array_value(node_id)
                    .map(|arr| arr.into_iter().map(|v| v as i64).collect())
            }
            
            fn long_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<i64> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::Long,
                ))
            }
            
            fn double_value(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<f64> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::Double,
                ))
            }
            
            fn has_value(&self, node_id: u64) -> bool {
                use $crate::collections::traits::Collections;
                node_id < self.node_count as u64 
                    && self.universal.collection().get(node_id as usize).flatten().is_some()
            }

            fn get_object(&self, _node_id: u64) -> $crate::types::properties::PropertyValuesResult<Box<dyn std::any::Any>> {
                Err($crate::types::properties::PropertyValuesError::unsupported_type(
                    self.value_type(),
                    $crate::types::ValueType::String,
                ))
            }
            
            fn dimension(&self) -> Option<usize> {
                // Calculate dimension by checking first non-null array
                use $crate::collections::traits::Collections;
                for i in 0..self.node_count {
                    if let Some(Some(arr)) = self.universal.collection().get(i) {
                        return Some(arr.len());
                    }
                }
                None
            }
            
            fn get_max_long_property_value(&self) -> Option<i64> {
                None
            }
            
            fn get_max_double_property_value(&self) -> Option<f64> {
                None
            }
        }
        
        // Implement the specific DoubleArray trait
        impl<C> $crate::types::properties::node::DoubleArrayNodePropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<$element_type> 
                + $crate::collections::traits::PropertyValuesAdapter<$element_type> 
                + Send + Sync + std::fmt::Debug,
        {
            fn double_array_value_unchecked(&self, node_id: u64) -> Option<Vec<f64>> {
                use $crate::collections::traits::Collections;
                // For arrays: get() returns Option<Option<Vec<T>>>
                self.universal
                    .collection()
                    .get(node_id as usize)
                    .and_then(|opt_vec| opt_vec)
                    .map(|arr| arr.into_iter().map(|v| v as f64).collect())
            }
        }
        
        // Also implement FloatArray trait for completeness
        impl<C> $crate::types::properties::node::FloatArrayNodePropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<$element_type> 
                + $crate::collections::traits::PropertyValuesAdapter<$element_type> 
                + Send + Sync + std::fmt::Debug,
        {
            fn float_array_value_unchecked(&self, node_id: u64) -> Option<Vec<f32>> {
                use $crate::collections::traits::Collections;
                // For arrays: get() returns Option<Option<Vec<T>>>
                self.universal
                    .collection()
                    .get(node_id as usize)
                    .and_then(|opt_vec| opt_vec)
                    .map(|arr| arr.into_iter().map(|v| v as f32).collect())
            }
        }
    };

    // Other Array types (BooleanArray, etc.) - placeholder
    ($struct_name:ty, $value_type:ident, $category:ident, $element_type:ty) => {
        // Placeholder for other array types (BooleanArray, CharArray, StringArray)
        // These will be implemented when needed
    };
}

/// Implement RelationshipPropertyValues trait for Universal adapters
/// 
/// This macro generates the RelationshipPropertyValues trait implementation.
/// Relationships are simpler than nodes - they only have double_value, long_value accessors.
#[macro_export]
macro_rules! impl_relationship_property_values_universal {
    // For numeric types (i8, i16, i32, i64, i128, f32, f64)
    ($struct_name:ty, i8) => {
        $crate::impl_relationship_property_values_universal!(@impl $struct_name, i8, |v| v as f64, |v| v as f64);
    };
    ($struct_name:ty, i16) => {
        $crate::impl_relationship_property_values_universal!(@impl $struct_name, i16, |v| v as f64, |v| v as f64);
    };
    ($struct_name:ty, i32) => {
        $crate::impl_relationship_property_values_universal!(@impl $struct_name, i32, |v| v as f64, |v| v as f64);
    };
    ($struct_name:ty, i64) => {
        $crate::impl_relationship_property_values_universal!(@impl $struct_name, i64, |v| v as f64, |v| v as f64);
    };
    ($struct_name:ty, i128) => {
        $crate::impl_relationship_property_values_universal!(@impl $struct_name, i128, |v| v as f64, |v| v as f64);
    };
    ($struct_name:ty, f32) => {
        $crate::impl_relationship_property_values_universal!(@impl $struct_name, f32, |v| v as f64, |v| v as f64);
    };
    ($struct_name:ty, f64) => {
        $crate::impl_relationship_property_values_universal!(@impl $struct_name, f64, |v| v, |v| v);
    };
    // For boolean - convert to 1.0/0.0
    ($struct_name:ty, bool) => {
        $crate::impl_relationship_property_values_universal!(@impl $struct_name, bool, |v| if v { 1.0 } else { 0.0 }, |v| if v { 1.0 } else { 0.0 });
    };
    // For char - convert via unicode value
    ($struct_name:ty, char) => {
        $crate::impl_relationship_property_values_universal!(@impl $struct_name, char, |v| v as u32 as f64, |v| v as u32 as f64);
    };
    // For String - default to 0
    ($struct_name:ty, String) => {
        $crate::impl_relationship_property_values_universal!(@impl $struct_name, String, |_v| 0.0f64, |_v| 0.0f64);
    };
    
    // Internal implementation
    (@impl $struct_name:ty, $element_type:ty, $to_double:expr, $to_default:expr) => {
        impl<C> $crate::types::properties::relationship::RelationshipPropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<$element_type> 
                + $crate::collections::traits::PropertyValuesAdapter<$element_type> 
                + Send + Sync + std::fmt::Debug,
        {
            fn double_value(&self, rel_index: u64) -> $crate::types::properties::PropertyValuesResult<f64> {
                use $crate::collections::traits::Collections;
                let convert = $to_double;
                self.universal
                    .collection()
                    .get(rel_index as usize)
                    .map(convert)
                    .ok_or_else(|| $crate::types::properties::PropertyValuesError::InvalidNodeId(rel_index))
            }

            fn long_value(&self, rel_index: u64) -> $crate::types::properties::PropertyValuesResult<i64> {
                Ok(self.double_value(rel_index)? as i64)
            }

            fn get_object(&self, rel_index: u64) -> $crate::types::properties::PropertyValuesResult<Box<dyn std::any::Any>> {
                Ok(Box::new(self.double_value(rel_index)?))
            }

            fn default_value(&self) -> f64 {
                let convert = $to_default;
                convert(self.universal.default_value())
            }

            fn has_value(&self, rel_index: u64) -> bool {
                rel_index < self.element_count as u64 
                    && !self.universal.collection().is_null(rel_index as usize)
            }
        }
    };
}

/// Implement GraphPropertyValues trait for Universal adapters
/// 
/// Graph properties return iterators over values rather than indexed access.
/// This macro generates the GraphPropertyValues trait implementation with proper
/// type conversion for numeric types and empty iterators for unsupported conversions.
#[macro_export]
macro_rules! impl_graph_property_values_universal {
    // For integral scalars (i8, i16, i32, i64, i128)
    ($struct_name:ty, i8) => {
        $crate::impl_graph_property_values_universal!(@impl_scalar $struct_name, i8);
    };
    ($struct_name:ty, i16) => {
        $crate::impl_graph_property_values_universal!(@impl_scalar $struct_name, i16);
    };
    ($struct_name:ty, i32) => {
        $crate::impl_graph_property_values_universal!(@impl_scalar $struct_name, i32);
    };
    ($struct_name:ty, i64) => {
        $crate::impl_graph_property_values_universal!(@impl_scalar $struct_name, i64);
    };
    ($struct_name:ty, i128) => {
        $crate::impl_graph_property_values_universal!(@impl_scalar $struct_name, i128);
    };
    // For floating point scalars
    ($struct_name:ty, f32) => {
        $crate::impl_graph_property_values_universal!(@impl_scalar $struct_name, f32);
    };
    ($struct_name:ty, f64) => {
        $crate::impl_graph_property_values_universal!(@impl_scalar $struct_name, f64);
    };
    // For boolean - convert to 1.0/0.0
    ($struct_name:ty, bool) => {
        $crate::impl_graph_property_values_universal!(@impl_scalar $struct_name, bool);
    };
    // For char - convert via unicode value
    ($struct_name:ty, char) => {
        $crate::impl_graph_property_values_universal!(@impl_scalar $struct_name, char);
    };
    // For String - special handling
    ($struct_name:ty, String) => {
        $crate::impl_graph_property_values_universal!(@impl_scalar $struct_name, String);
    };
    
    // For integral arrays
    ($struct_name:ty, Option<Vec<i8>>) => {
        $crate::impl_graph_property_values_universal!(@impl_integral_array $struct_name, i8);
    };
    ($struct_name:ty, Option<Vec<i16>>) => {
        $crate::impl_graph_property_values_universal!(@impl_integral_array $struct_name, i16);
    };
    ($struct_name:ty, Option<Vec<i32>>) => {
        $crate::impl_graph_property_values_universal!(@impl_integral_array $struct_name, i32);
    };
    ($struct_name:ty, Option<Vec<i64>>) => {
        $crate::impl_graph_property_values_universal!(@impl_integral_array $struct_name, i64);
    };
    ($struct_name:ty, Option<Vec<i128>>) => {
        $crate::impl_graph_property_values_universal!(@impl_integral_array $struct_name, i128);
    };
    // For floating point arrays
    ($struct_name:ty, Option<Vec<f32>>) => {
        $crate::impl_graph_property_values_universal!(@impl_float_array $struct_name, f32);
    };
    ($struct_name:ty, Option<Vec<f64>>) => {
        $crate::impl_graph_property_values_universal!(@impl_float_array $struct_name, f64);
    };
    // For boolean arrays
    ($struct_name:ty, Option<Vec<bool>>) => {
        $crate::impl_graph_property_values_universal!(@impl_bool_array $struct_name);
    };
    // For char arrays
    ($struct_name:ty, Option<Vec<char>>) => {
        $crate::impl_graph_property_values_universal!(@impl_char_array $struct_name);
    };
    // For String arrays  
    ($struct_name:ty, Option<Vec<String>>) => {
        $crate::impl_graph_property_values_universal!(@impl_string_array $struct_name);
    };
    
    // Internal: numeric scalar implementation (i8-i128, f32, f64) - works with `as` cast
    (@impl_scalar $struct_name:ty, $element_type:ty) => {
        impl<C> $crate::types::properties::graph::GraphPropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<$element_type> 
                + $crate::collections::traits::PropertyValuesAdapter<$element_type> 
                + Send + Sync + std::fmt::Debug,
        {
            fn double_values(&self) -> Box<dyn Iterator<Item = f64> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).map(|v| v as f64)
                }))
            }

            fn long_values(&self) -> Box<dyn Iterator<Item = i64> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).map(|v| v as i64)
                }))
            }

            fn double_array_values(&self) -> Box<dyn Iterator<Item = Vec<f64>> + '_> {
                Box::new(std::iter::empty())
            }

            fn float_array_values(&self) -> Box<dyn Iterator<Item = Vec<f32>> + '_> {
                Box::new(std::iter::empty())
            }

            fn long_array_values(&self) -> Box<dyn Iterator<Item = Vec<i64>> + '_> {
                Box::new(std::iter::empty())
            }

            fn objects(&self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).map(|v| Box::new(v) as Box<dyn std::any::Any>)
                }))
            }
        }
    };
    
    // Internal: integral array implementation (i8, i16, i32, i64, i128)
    (@impl_integral_array $struct_name:ty, $inner_type:ty) => {
        impl<C> $crate::types::properties::graph::GraphPropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<Option<Vec<$inner_type>>> 
                + $crate::collections::traits::PropertyValuesAdapter<Option<Vec<$inner_type>>> 
                + Send + Sync + std::fmt::Debug,
        {
            fn double_values(&self) -> Box<dyn Iterator<Item = f64> + '_> {
                Box::new(std::iter::empty())
            }

            fn long_values(&self) -> Box<dyn Iterator<Item = i64> + '_> {
                Box::new(std::iter::empty())
            }

            fn double_array_values(&self) -> Box<dyn Iterator<Item = Vec<f64>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).flatten().map(|arr| {
                        arr.into_iter().map(|v| v as f64).collect()
                    })
                }))
            }

            fn float_array_values(&self) -> Box<dyn Iterator<Item = Vec<f32>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).flatten().map(|arr| {
                        arr.into_iter().map(|v| v as f32).collect()
                    })
                }))
            }

            fn long_array_values(&self) -> Box<dyn Iterator<Item = Vec<i64>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).flatten().map(|arr| {
                        arr.into_iter().map(|v| v as i64).collect()
                    })
                }))
            }

            fn objects(&self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).map(|v| Box::new(v) as Box<dyn std::any::Any>)
                }))
            }
        }
    };
    
    // Internal: floating point array implementation (f32, f64)
    (@impl_float_array $struct_name:ty, $inner_type:ty) => {
        impl<C> $crate::types::properties::graph::GraphPropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<Option<Vec<$inner_type>>> 
                + $crate::collections::traits::PropertyValuesAdapter<Option<Vec<$inner_type>>> 
                + Send + Sync + std::fmt::Debug,
        {
            fn double_values(&self) -> Box<dyn Iterator<Item = f64> + '_> {
                Box::new(std::iter::empty())
            }

            fn long_values(&self) -> Box<dyn Iterator<Item = i64> + '_> {
                Box::new(std::iter::empty())
            }

            fn double_array_values(&self) -> Box<dyn Iterator<Item = Vec<f64>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).flatten().map(|arr| {
                        arr.into_iter().map(|v| v as f64).collect()
                    })
                }))
            }

            fn float_array_values(&self) -> Box<dyn Iterator<Item = Vec<f32>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).flatten().map(|arr| {
                        arr.into_iter().map(|v| v as f32).collect()
                    })
                }))
            }

            fn long_array_values(&self) -> Box<dyn Iterator<Item = Vec<i64>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).flatten().map(|arr| {
                        arr.into_iter().map(|v| v as i64).collect()
                    })
                }))
            }

            fn objects(&self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).map(|v| Box::new(v) as Box<dyn std::any::Any>)
                }))
            }
        }
    };
    
    // Internal: boolean array implementation
    (@impl_bool_array $struct_name:ty) => {
        impl<C> $crate::types::properties::graph::GraphPropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<Option<Vec<bool>>> 
                + $crate::collections::traits::PropertyValuesAdapter<Option<Vec<bool>>> 
                + Send + Sync + std::fmt::Debug,
        {
            fn double_values(&self) -> Box<dyn Iterator<Item = f64> + '_> {
                Box::new(std::iter::empty())
            }

            fn long_values(&self) -> Box<dyn Iterator<Item = i64> + '_> {
                Box::new(std::iter::empty())
            }

            fn double_array_values(&self) -> Box<dyn Iterator<Item = Vec<f64>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).flatten().map(|arr| {
                        arr.into_iter().map(|v| if v { 1.0 } else { 0.0 }).collect()
                    })
                }))
            }

            fn float_array_values(&self) -> Box<dyn Iterator<Item = Vec<f32>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).flatten().map(|arr| {
                        arr.into_iter().map(|v| if v { 1.0 } else { 0.0 }).collect()
                    })
                }))
            }

            fn long_array_values(&self) -> Box<dyn Iterator<Item = Vec<i64>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).flatten().map(|arr| {
                        arr.into_iter().map(|v| if v { 1 } else { 0 }).collect()
                    })
                }))
            }

            fn objects(&self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).map(|v| Box::new(v) as Box<dyn std::any::Any>)
                }))
            }
        }
    };
    
    // Internal: char array implementation
    (@impl_char_array $struct_name:ty) => {
        impl<C> $crate::types::properties::graph::GraphPropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<Option<Vec<char>>> 
                + $crate::collections::traits::PropertyValuesAdapter<Option<Vec<char>>> 
                + Send + Sync + std::fmt::Debug,
        {
            fn double_values(&self) -> Box<dyn Iterator<Item = f64> + '_> {
                Box::new(std::iter::empty())
            }

            fn long_values(&self) -> Box<dyn Iterator<Item = i64> + '_> {
                Box::new(std::iter::empty())
            }

            fn double_array_values(&self) -> Box<dyn Iterator<Item = Vec<f64>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).flatten().map(|arr| {
                        arr.into_iter().map(|v| v as u32 as f64).collect()
                    })
                }))
            }

            fn float_array_values(&self) -> Box<dyn Iterator<Item = Vec<f32>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).flatten().map(|arr| {
                        arr.into_iter().map(|v| v as u32 as f32).collect()
                    })
                }))
            }

            fn long_array_values(&self) -> Box<dyn Iterator<Item = Vec<i64>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).flatten().map(|arr| {
                        arr.into_iter().map(|v| v as u32 as i64).collect()
                    })
                }))
            }

            fn objects(&self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).map(|v| Box::new(v) as Box<dyn std::any::Any>)
                }))
            }
        }
    };
    
    // Internal: String array implementation
    (@impl_string_array $struct_name:ty) => {
        impl<C> $crate::types::properties::graph::GraphPropertyValues for $struct_name
        where
            C: $crate::collections::traits::Collections<Option<Vec<String>>> 
                + $crate::collections::traits::PropertyValuesAdapter<Option<Vec<String>>> 
                + Send + Sync + std::fmt::Debug,
        {
            fn double_values(&self) -> Box<dyn Iterator<Item = f64> + '_> {
                Box::new(std::iter::empty())
            }

            fn long_values(&self) -> Box<dyn Iterator<Item = i64> + '_> {
                Box::new(std::iter::empty())
            }

            fn double_array_values(&self) -> Box<dyn Iterator<Item = Vec<f64>> + '_> {
                // String arrays can't be converted to numeric arrays - return empty
                Box::new(std::iter::empty())
            }

            fn float_array_values(&self) -> Box<dyn Iterator<Item = Vec<f32>> + '_> {
                Box::new(std::iter::empty())
            }

            fn long_array_values(&self) -> Box<dyn Iterator<Item = Vec<i64>> + '_> {
                Box::new(std::iter::empty())
            }

            fn objects(&self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>> + '_> {
                use $crate::collections::traits::Collections;
                Box::new((0..self.universal.collection().len()).filter_map(move |i| {
                    self.universal.collection().get(i).map(|v| Box::new(v) as Box<dyn std::any::Any>)
                }))
            }
        }
    };
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
