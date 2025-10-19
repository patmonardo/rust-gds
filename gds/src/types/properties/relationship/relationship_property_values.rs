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
