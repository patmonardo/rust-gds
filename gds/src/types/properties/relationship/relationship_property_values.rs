use crate::types::properties::{PropertyValues, PropertyValuesResult};
use crate::types::ValueType;

/// Represents properties of relationships in a graph.
/// Provides access to relationship property values and metadata.
///
/// Concrete implementations live under the `impls` module.
pub trait RelationshipPropertyValues: PropertyValues + std::fmt::Debug + Send + Sync {
    /// Returns the double value for the given relationship index.
    fn double_value(&self, rel_index: u64) -> PropertyValuesResult<f64>;

    /// Returns the long value for the given relationship index.
    fn long_value(&self, rel_index: u64) -> PropertyValuesResult<i64>;

    /// Returns the object value for the given relationship index.
    fn get_object(&self, rel_index: u64) -> PropertyValuesResult<Box<dyn std::any::Any>>;

    /// Returns the number of relationship elements with properties.
    fn relationship_count(&self) -> usize {
        self.element_count()
    }

    /// Returns the default property value used when a relationship has no property.
    fn default_value(&self) -> f64;

    /// Returns whether the relationship has a value.
    fn has_value(&self, rel_index: u64) -> bool;
}

// ========== Specialized traits for typed relationship property values ==========

/// Relationship property values that are scalar longs (64-bit integers).
pub trait LongRelationshipPropertyValues: RelationshipPropertyValues {
    /// Returns the long value for the given relationship index.
    fn long_value_unchecked(&self, rel_index: u64) -> i64;
}

/// Relationship property values that are scalar doubles (64-bit floats).
pub trait DoubleRelationshipPropertyValues: RelationshipPropertyValues {
    /// Returns the double value for the given relationship index.
    fn double_value_unchecked(&self, rel_index: u64) -> f64;
}

/// Relationship property values that are arrays of doubles.
pub trait DoubleArrayRelationshipPropertyValues: RelationshipPropertyValues {
    /// Returns the double array value for the given relationship index.
    fn double_array_value_unchecked(&self, rel_index: u64) -> Option<Vec<f64>>;
}

/// Relationship property values that are arrays of floats.
pub trait FloatArrayRelationshipPropertyValues: RelationshipPropertyValues {
    /// Returns the float array value for the given relationship index.
    fn float_array_value_unchecked(&self, rel_index: u64) -> Option<Vec<f32>>;
}

/// Relationship property values that are arrays of longs.
pub trait LongArrayRelationshipPropertyValues: RelationshipPropertyValues {
    /// Returns the long array value for the given relationship index.
    fn long_array_value_unchecked(&self, rel_index: u64) -> Option<Vec<i64>>;
}

// Implement PropertyValues for Box<dyn RelationshipPropertyValues> to allow trait objects
impl PropertyValues for Box<dyn RelationshipPropertyValues> {
    fn value_type(&self) -> ValueType {
        (**self).value_type()
    }

    fn element_count(&self) -> usize {
        (**self).element_count()
    }
}

// Implement PropertyValues for Arc<dyn RelationshipPropertyValues> to allow trait objects
impl PropertyValues for std::sync::Arc<dyn RelationshipPropertyValues> {
    fn value_type(&self) -> ValueType {
        (**self).value_type()
    }

    fn element_count(&self) -> usize {
        (**self).element_count()
    }
}
