use crate::property_values_impl;
use crate::types::properties::property_values::{
    PropertyValues, PropertyValuesError, PropertyValuesResult,
};
use crate::types::properties::relationship::relationship_property_values::RelationshipPropertyValues;
use crate::types::property::ValueType;
use crate::types::property_value::PropertyValue;

/// Default implementation for relationship property values storing `f64` entries.
///
/// Storage: Vec<f64> - suitable for pure in-memory relationship weights/properties.
/// Future: Can be replaced with Arrow2 Float64Array for columnar storage.
///
/// Relationship properties differ from node properties in several ways:
/// - Include a `default_value` for sparse property support
/// - Currently focused on Double (f64) for relationship weights
/// - Uses indexed access like node properties (rel_index)
/// - Simpler than nodes (no Option wrapping, fewer type variants)
#[derive(Debug, Clone)]
pub struct DefaultRelationshipPropertyValues {
    values: Vec<f64>,
    default_value: f64,
    element_count: usize,
}

impl DefaultRelationshipPropertyValues {
    pub fn new(values: Vec<f64>, default_value: f64, element_count: usize) -> Self {
        Self {
            values,
            default_value,
            element_count,
        }
    }

    pub fn with_default(values: Vec<f64>, element_count: usize) -> Self {
        Self::new(values, 0.0, element_count)
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn default_value(&self) -> f64 {
        self.default_value
    }
}

// Generate PropertyValues trait implementation
property_values_impl!(
    DefaultRelationshipPropertyValues,
    Double,
    f64,
    PropertyValue::Double,
    relationship
);

// Manual RelationshipPropertyValues implementation
impl RelationshipPropertyValues for DefaultRelationshipPropertyValues {
    fn double_value(&self, rel_index: u64) -> PropertyValuesResult<f64> {
        self.values
            .get(rel_index as usize)
            .copied()
            .ok_or(PropertyValuesError::InvalidNodeId(rel_index))
    }

    fn long_value(&self, rel_index: u64) -> PropertyValuesResult<i64> {
        Ok(self.double_value(rel_index)? as i64)
    }

    fn get_object(&self, rel_index: u64) -> PropertyValuesResult<Box<dyn std::any::Any>> {
        Ok(Box::new(self.double_value(rel_index)?))
    }

    fn default_value(&self) -> f64 {
        self.default_value
    }

    fn has_value(&self, rel_index: u64) -> bool {
        (rel_index as usize) < self.values.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_property_values_basic() {
        let values = DefaultRelationshipPropertyValues::new(vec![1.0, 2.5, 3.7], 0.0, 3);

        assert_eq!(values.value_type(), ValueType::Double);
        assert_eq!(values.relationship_count(), 3);
        assert_eq!(values.double_value(1).unwrap(), 2.5);
        assert_eq!(values.default_value(), 0.0);
        assert!(values.has_value(0));
        assert!(!values.has_value(10));
    }

    #[test]
    fn test_double_to_long_conversion() {
        let values = DefaultRelationshipPropertyValues::new(vec![1.9, 2.1, 3.7], 0.0, 3);
        assert_eq!(values.long_value(0).unwrap(), 1);
        assert_eq!(values.long_value(1).unwrap(), 2);
        assert_eq!(values.long_value(2).unwrap(), 3);
    }

    #[test]
    fn test_with_default_constructor() {
        let values = DefaultRelationshipPropertyValues::with_default(vec![1.0, 2.0], 2);
        assert_eq!(values.default_value(), 0.0);
        assert_eq!(values.element_count(), 2);
    }

    #[test]
    fn test_custom_default_value() {
        let values = DefaultRelationshipPropertyValues::new(vec![1.0, 2.0], 5.0, 2);
        assert_eq!(values.default_value(), 5.0);
    }

    #[test]
    fn test_sparse_access() {
        let values = DefaultRelationshipPropertyValues::new(vec![1.0, 2.0, 3.0], 0.0, 10);
        // Has values for indices 0-2
        assert!(values.has_value(0));
        assert!(values.has_value(2));
        // No values for indices 3-9
        assert!(!values.has_value(3));
        assert!(!values.has_value(9));
    }

    #[test]
    fn test_out_of_bounds_access() {
        let values = DefaultRelationshipPropertyValues::new(vec![1.0, 2.0], 0.0, 2);
        assert!(values.double_value(10).is_err());
    }

    #[test]
    fn test_get_object() {
        let values = DefaultRelationshipPropertyValues::new(vec![42.0], 0.0, 1);
        let obj = values.get_object(0).unwrap();
        let double_val = obj.downcast_ref::<f64>().unwrap();
        assert_eq!(*double_val, 42.0);
    }

    #[test]
    fn test_unified_property_value_accessor() {
        let values = DefaultRelationshipPropertyValues::new(vec![1.5, 2.5, 3.5], 0.0, 3);

        let val0 = values.get_property_value(0);
        assert!(matches!(val0, Some(PropertyValue::Double(v)) if v == 1.5));

        let val1 = values.get_property_value(1);
        assert!(matches!(val1, Some(PropertyValue::Double(v)) if v == 2.5));

        let val_oob = values.get_property_value(10);
        assert!(val_oob.is_none());
    }
}
