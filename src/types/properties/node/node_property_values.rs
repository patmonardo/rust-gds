use crate::types::properties::property_values::{PropertyValues, PropertyValuesResult};
use crate::types::property::ValueType;

/// Interface for accessing property values for nodes in a graph.
/// Provides methods for retrieving values of different types for specific nodes.
///
/// This mirrors the TypeScript NodePropertyValues interface while aligning with the
/// relationship property layout. Concrete implementations live under the `impls`
/// module, leaving this definition focused on trait contracts.
pub trait NodePropertyValues: PropertyValues + std::fmt::Debug + Send + Sync {
    /// Returns the double value for the given node.
    fn double_value(&self, node_id: u64) -> PropertyValuesResult<f64>;

    /// Returns the long value for the given node.
    fn long_value(&self, node_id: u64) -> PropertyValuesResult<i64>;

    /// Returns the double array value for the given node.
    fn double_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f64>>;

    /// Returns the float array value for the given node.
    fn float_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<f32>>;

    /// Returns the long array value for the given node.
    fn long_array_value(&self, node_id: u64) -> PropertyValuesResult<Vec<i64>>;

    /// Returns the object value for the given node (generic accessor).
    fn get_object(&self, node_id: u64) -> PropertyValuesResult<Box<dyn std::any::Any>>;

    /// Returns the number of nodes that have property values.
    fn node_count(&self) -> usize {
        self.element_count()
    }

    /// The dimension of the properties.
    /// For scalar values, this is 1.
    /// For arrays, this is the length of the array.
    fn dimension(&self) -> Option<usize>;

    /// Gets the maximum long value contained in the mapping.
    fn get_max_long_property_value(&self) -> Option<i64>;

    /// Gets the maximum double value contained in the mapping.
    fn get_max_double_property_value(&self) -> Option<f64>;

    /// Returns whether the node has a value.
    fn has_value(&self, node_id: u64) -> bool;
}

// ========== Specialized traits for typed node property values ==========

/// Node property values that are scalar longs (64-bit integers).
pub trait LongNodePropertyValues: NodePropertyValues {
    /// Returns the long value for the given node ID.
    fn long_value_unchecked(&self, node_id: u64) -> i64;
}

/// Node property values that are scalar doubles (64-bit floats).
pub trait DoubleNodePropertyValues: NodePropertyValues {
    /// Returns the double value for the given node ID.
    fn double_value_unchecked(&self, node_id: u64) -> f64;
}

/// Node property values that are arrays of doubles.
pub trait DoubleArrayNodePropertyValues: NodePropertyValues {
    /// Returns the double array value for the given node ID.
    fn double_array_value_unchecked(&self, node_id: u64) -> Option<Vec<f64>>;
}

/// Node property values that are arrays of floats.
pub trait FloatArrayNodePropertyValues: NodePropertyValues {
    /// Returns the float array value for the given node ID.
    fn float_array_value_unchecked(&self, node_id: u64) -> Option<Vec<f32>>;
}

/// Node property values that are arrays of longs.
pub trait LongArrayNodePropertyValues: NodePropertyValues {
    /// Returns the long array value for the given node ID.
    fn long_array_value_unchecked(&self, node_id: u64) -> Option<Vec<i64>>;
}

// Implement PropertyValues for Box<dyn NodePropertyValues> to allow trait objects
impl PropertyValues for Box<dyn NodePropertyValues> {
    fn value_type(&self) -> ValueType {
        (**self).value_type()
    }

    fn element_count(&self) -> usize {
        (**self).element_count()
    }
}

// Implement PropertyValues for Arc<dyn NodePropertyValues> to allow trait objects
impl PropertyValues for std::sync::Arc<dyn NodePropertyValues> {
    fn value_type(&self) -> ValueType {
        (**self).value_type()
    }

    fn element_count(&self) -> usize {
        (**self).element_count()
    }
}
