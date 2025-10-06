use crate::property_values_impl;
use crate::types::properties::property_values::{
    PropertyValues, PropertyValuesError, PropertyValuesResult,
};
use crate::types::properties::relationship::relationship_property_values::RelationshipPropertyValues;
use crate::types::property_value::PropertyValue;
use crate::types::value_type::ValueType;

/// Default implementation for relationship property values storing `f64`
/// entries with an optional default value.
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
}

property_values_impl!(
    DefaultRelationshipPropertyValues,
    Double,
    f64,
    PropertyValue::Double,
    relationship
);

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
    fn default_relationship_property_values_behavior() {
        let values = DefaultRelationshipPropertyValues::new(vec![1.0, 2.5, 3.7], 0.0, 3);

        assert_eq!(values.value_type(), ValueType::Double);
        assert_eq!(values.relationship_count(), 3);
        assert_eq!(values.double_value(1).unwrap(), 2.5);
        assert_eq!(values.default_value(), 0.0);
        assert!(values.has_value(0));
        assert!(!values.has_value(10));
    }
}
