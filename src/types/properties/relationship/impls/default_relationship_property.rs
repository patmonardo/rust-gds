use crate::types::properties::property::Property;
use crate::types::properties::relationship::relationship_property_values::RelationshipPropertyValues;
use crate::types::property::PropertyState;
use crate::types::schema::{DefaultValue, PropertySchema};
use std::sync::Arc;

/// Concrete relationship property implementation that mirrors the structure of
/// node and graph property defaults. It stores the shared property values
/// alongside the computed schema metadata.
#[derive(Debug, Clone)]
pub struct DefaultRelationshipProperty {
    values: Arc<dyn RelationshipPropertyValues>,
    schema: PropertySchema,
}

impl DefaultRelationshipProperty {
    /// Construct a relationship property using the default `PropertyState::Normal`.
    pub fn of(key: impl Into<String>, values: Arc<dyn RelationshipPropertyValues>) -> Self {
        Self::with_state(key, PropertyState::Normal, values)
    }

    /// Construct a property with an explicit property state.
    pub fn with_state(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn RelationshipPropertyValues>,
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
        values: Arc<dyn RelationshipPropertyValues>,
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
    pub fn with_schema(
        schema: PropertySchema,
        values: Arc<dyn RelationshipPropertyValues>,
    ) -> Self {
        Self { values, schema }
    }

    /// Returns the property values as a shared trait object.
    pub fn values(&self) -> &dyn RelationshipPropertyValues {
        self.values.as_ref()
    }

    /// Returns a cloned `Arc` handle to the underlying property values.
    pub fn values_arc(&self) -> Arc<dyn RelationshipPropertyValues> {
        Arc::clone(&self.values)
    }

    /// Returns the relationship property schema.
    pub fn property_schema(&self) -> &PropertySchema {
        &self.schema
    }

    /// Convenience accessor for the property key.
    pub fn key(&self) -> &str {
        self.schema.key()
    }
}

impl Property for DefaultRelationshipProperty {
    fn schema(&self) -> &PropertySchema {
        &self.schema
    }

    fn values(&self) -> Arc<dyn crate::types::properties::property_values::PropertyValues> {
        Arc::clone(&self.values)
            as Arc<dyn crate::types::properties::property_values::PropertyValues>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::property_values::PropertyValues;
    use crate::types::properties::relationship::impls::values::DefaultRelationshipPropertyValues;

    #[test]
    fn default_relationship_property_creation() {
        let values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::new(vec![1.0, 2.5, 3.7], 0.0, 3),
        );
        let property = DefaultRelationshipProperty::of("weight", values.clone());

        assert_eq!(property.key(), "weight");
        assert_eq!(property.property_schema().state(), PropertyState::Normal);
        assert_eq!(property.property_schema().value_type(), values.value_type());
        assert!(Arc::ptr_eq(&property.values_arc(), &values));
    }

    #[test]
    fn relationship_property_with_state() {
        let values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::new(vec![10.0, 20.0], 0.0, 2),
        );
        let property =
            DefaultRelationshipProperty::with_state("distance", PropertyState::Deleted, values);

        assert_eq!(property.key(), "distance");
        assert_eq!(property.property_schema().state(), PropertyState::Deleted);
    }

    #[test]
    fn relationship_property_with_explicit_default() {
        let values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::new(vec![5.5, 6.6], 1.0, 2),
        );
        let default_value = DefaultValue::Double(1.0);
        let property = DefaultRelationshipProperty::with_default(
            "score",
            PropertyState::Normal,
            values,
            default_value.clone(),
        );

        assert_eq!(property.key(), "score");
        assert_eq!(property.property_schema().default_value(), &default_value);
    }

    #[test]
    fn relationship_property_values_access() {
        let values: Arc<dyn RelationshipPropertyValues> = Arc::new(
            DefaultRelationshipPropertyValues::new(vec![1.0, 2.0, 3.0], 0.0, 3),
        );
        let property = DefaultRelationshipProperty::of("weight", values);

        // Test trait object access
        let values_ref = property.values();
        assert_eq!(values_ref.element_count(), 3);

        // Test Arc access
        let values_arc = property.values_arc();
        assert_eq!(values_arc.element_count(), 3);
    }
}
