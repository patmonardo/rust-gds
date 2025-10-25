//! Monadic Property: Simple Collections First Property
//!
//! A standalone property implementation that works directly with Collections,
//! independent of graph/node/relationship complexity.
//!
//! This is a proof-of-concept demonstrating the Collections First architecture.

use crate::types::properties::Property;
use crate::types::properties::PropertyValues;
use crate::types::schema::PropertySchema;
use crate::types::DefaultValue;
use crate::types::PropertyState;
use std::sync::Arc;

/// Monadic property: Simple Collections First implementation
///
/// Works with ANY PropertyValues implementation, not specific to node/relationship/graph.
#[derive(Debug, Clone)]
pub struct MonadicProperty {
    values: Arc<dyn PropertyValues>,
    schema: PropertySchema,
}

impl MonadicProperty {
    /// Construct a property using the default `PropertyState::Persistent`.
    pub fn of(key: impl Into<String>, values: Arc<dyn PropertyValues>) -> Self {
        Self::with_state(key, PropertyState::Persistent, values)
    }

    /// Construct a property with an explicit property state.
    pub fn with_state(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn PropertyValues>,
    ) -> Self {
        let key_str = key.into();
        let value_type = values.value_type();
        let default_value = DefaultValue::of(value_type);
        Self::with_schema(
            PropertySchema::new(key_str, value_type, default_value, state),
            values,
        )
    }

    /// Construct a property with a provided default value in addition to the state.
    pub fn with_default(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn PropertyValues>,
        default_value: DefaultValue,
    ) -> Self {
        let key_str = key.into();
        let value_type = values.value_type();
        Self::with_schema(
            PropertySchema::new(key_str, value_type, default_value, state),
            values,
        )
    }

    /// Construct a property from an existing schema, reusing the provided values.
    pub fn with_schema(schema: PropertySchema, values: Arc<dyn PropertyValues>) -> Self {
        Self { values, schema }
    }

    /// Returns the property values as a shared trait object.
    pub fn values(&self) -> &dyn PropertyValues {
        self.values.as_ref()
    }

    /// Returns a cloned `Arc` handle to the underlying property values.
    pub fn values_arc(&self) -> Arc<dyn PropertyValues> {
        Arc::clone(&self.values)
    }

    /// Returns the property schema.
    pub fn property_schema(&self) -> &PropertySchema {
        &self.schema
    }

    /// Convenience accessor for the property key.
    pub fn key(&self) -> &str {
        self.schema.key()
    }
}

impl Property for MonadicProperty {
    fn values(&self) -> Arc<dyn PropertyValues> {
        Arc::clone(&self.values) as Arc<dyn PropertyValues>
    }

    fn schema(&self) -> &PropertySchema {
        &self.schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::property_values::{MonadicLongPropertyValues, MonadicDoublePropertyValues};
    use crate::collections::backends::vec::{VecLong, VecDouble};
    use crate::types::ValueType;

    #[test]
    fn monadic_property_creation() {
        // Create a simple property using Collections First
        let vec_long = VecLong::from(vec![1, 2, 3]);
        let values = MonadicLongPropertyValues::new(vec_long, 0);
        let values_arc: Arc<dyn PropertyValues> = Arc::new(values);
        
        let property = MonadicProperty::of("age", values_arc.clone());

        assert_eq!(property.key(), "age");
        assert_eq!(
            property.property_schema().state(),
            PropertyState::Persistent
        );
        assert_eq!(property.property_schema().value_type(), ValueType::Long);
        assert!(Arc::ptr_eq(&property.values_arc(), &values_arc));
    }

    #[test]
    fn monadic_property_with_state() {
        let vec_long = VecLong::from(vec![10, 20]);
        let values = MonadicLongPropertyValues::new(vec_long, 0);
        let values_arc: Arc<dyn PropertyValues> = Arc::new(values);
        
        let property = MonadicProperty::with_state("rank", PropertyState::Persistent, values_arc);

        assert_eq!(property.key(), "rank");
        assert_eq!(
            property.property_schema().state(),
            PropertyState::Persistent
        );
    }

    #[test]
    fn monadic_property_with_explicit_default() {
        let vec_double = VecDouble::from(vec![5.0, 6.0]);
        let values = MonadicDoublePropertyValues::new(vec_double, 0.0);
        let values_arc: Arc<dyn PropertyValues> = Arc::new(values);
        
        let default_value = DefaultValue::double(0.0);
        let property = MonadicProperty::with_default(
            "score",
            PropertyState::Persistent,
            values_arc,
            default_value.clone(),
        );

        assert_eq!(property.key(), "score");
        assert_eq!(property.property_schema().default_value(), &default_value);
    }

    #[test]
    fn monadic_property_values_access() {
        let vec_long = VecLong::from(vec![1, 2, 3]);
        let values = MonadicLongPropertyValues::new(vec_long, 0);
        let values_arc: Arc<dyn PropertyValues> = Arc::new(values);
        
        let property = MonadicProperty::of("age", values_arc);

        // Test trait object access
        let values_ref = property.values();
        assert_eq!(values_ref.element_count(), 3);

        // Test Arc access
        let values_arc = property.values_arc();
        assert_eq!(values_arc.element_count(), 3);
    }
}
