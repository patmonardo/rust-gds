use crate::types::properties::node::node_property_values::NodePropertyValues;
use crate::types::properties::property::PropertyTrait;
use crate::types::property::PropertyState;
use crate::types::schema::{DefaultValue, PropertySchema};
use std::sync::Arc;

/// Default concrete implementation of a node property.
#[derive(Debug, Clone)]
pub struct DefaultNodeProperty {
    values: Arc<dyn NodePropertyValues>,
    schema: PropertySchema,
}

impl DefaultNodeProperty {
    /// Construct a node property using the default `PropertyState::Normal`.
    pub fn of(key: impl Into<String>, values: Arc<dyn NodePropertyValues>) -> Self {
        Self::with_state(key, PropertyState::Normal, values)
    }

    /// Construct a property with an explicit property state.
    pub fn with_state(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn NodePropertyValues>,
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
        values: Arc<dyn NodePropertyValues>,
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
    pub fn with_schema(schema: PropertySchema, values: Arc<dyn NodePropertyValues>) -> Self {
        Self { values, schema }
    }

    /// Returns the property values as a shared trait object.
    pub fn values(&self) -> &dyn NodePropertyValues {
        self.values.as_ref()
    }

    /// Returns a cloned `Arc` handle to the underlying property values.
    pub fn values_arc(&self) -> Arc<dyn NodePropertyValues> {
        Arc::clone(&self.values)
    }

    /// Returns the node property schema.
    pub fn property_schema(&self) -> &PropertySchema {
        &self.schema
    }

    /// Convenience accessor for the property key.
    pub fn key(&self) -> &str {
        self.schema.key()
    }
}

impl PropertyTrait for DefaultNodeProperty {
    type Values = Arc<dyn NodePropertyValues>;

    fn values(&self) -> &Self::Values {
        &self.values
    }

    fn property_schema(&self) -> &PropertySchema {
        &self.schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::node::DefaultLongNodePropertyValues;
    use crate::types::properties::property_values::PropertyValues;

    #[test]
    fn default_node_property_creation() {
        let values: Arc<dyn NodePropertyValues> =
            Arc::new(DefaultLongNodePropertyValues::new(vec![1, 2, 3], 3));
        let property = DefaultNodeProperty::of("age", values.clone());

        assert_eq!(property.key(), "age");
        assert_eq!(property.property_schema().state(), PropertyState::Normal);
        assert_eq!(property.property_schema().value_type(), values.value_type());
        assert!(Arc::ptr_eq(&property.values_arc(), &values));
    }

    #[test]
    fn node_property_with_state() {
        let values: Arc<dyn NodePropertyValues> =
            Arc::new(DefaultLongNodePropertyValues::new(vec![10, 20], 2));
        let property = DefaultNodeProperty::with_state("rank", PropertyState::Deleted, values);

        assert_eq!(property.key(), "rank");
        assert_eq!(property.property_schema().state(), PropertyState::Deleted);
    }

    #[test]
    fn node_property_with_explicit_default() {
        let values: Arc<dyn NodePropertyValues> =
            Arc::new(DefaultLongNodePropertyValues::new(vec![5, 6], 2));
        let default_value = DefaultValue::Long(0);
        let property = DefaultNodeProperty::with_default(
            "score",
            PropertyState::Normal,
            values,
            default_value.clone(),
        );

        assert_eq!(property.key(), "score");
        assert_eq!(property.property_schema().default_value(), &default_value);
    }

    #[test]
    fn node_property_values_access() {
        let values: Arc<dyn NodePropertyValues> =
            Arc::new(DefaultLongNodePropertyValues::new(vec![1, 2, 3], 3));
        let property = DefaultNodeProperty::of("age", values);

        // Test trait object access
        let values_ref = property.values();
        assert_eq!(values_ref.element_count(), 3);

        // Test Arc access
        let values_arc = property.values_arc();
        assert_eq!(values_arc.element_count(), 3);
    }
}
