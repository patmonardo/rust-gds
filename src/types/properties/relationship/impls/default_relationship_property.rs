use crate::types::properties::relationship::relationship_property_values::RelationshipPropertyValues;
use crate::types::property::PropertyState;
use crate::types::schema::{
    Aggregation, DefaultValue, PropertySchemaTrait, RelationshipPropertySchema,
};
use std::sync::Arc;

/// Concrete relationship property implementation aligned with the TypeScript
/// `DefaultRelationshipProperty` primitive. It stores the underlying property
/// values alongside their schema metadata and exposes the high-level accessors
/// used throughout the graph store.
#[derive(Debug, Clone)]
pub struct DefaultRelationshipProperty {
    values: Arc<dyn RelationshipPropertyValues>,
    schema: RelationshipPropertySchema,
}

impl DefaultRelationshipProperty {
    /// Build a relationship property with a fully specified schema.
    pub fn of(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn RelationshipPropertyValues>,
        default_value: DefaultValue,
        aggregation: Aggregation,
    ) -> Self {
        let value_type = values.value_type();
        let schema = RelationshipPropertySchema::with_aggregation(
            key,
            value_type,
            default_value,
            state,
            aggregation,
        );
        Self { values, schema }
    }

    /// Construct a property using the default aggregation strategy (`NONE`).
    pub fn with_default_aggregation(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn RelationshipPropertyValues>,
    ) -> Self {
        let value_type = values.value_type();
        let default_value = DefaultValue::of(value_type);
        Self::of(key, state, values, default_value, Aggregation::None)
    }

    /// Returns the property values.
    pub fn values(&self) -> &dyn RelationshipPropertyValues {
        self.values.as_ref()
    }

    /// Returns a clone of the underlying values handle for scenarios that need to
    /// share the trait object across graph views.
    pub fn values_arc(&self) -> Arc<dyn RelationshipPropertyValues> {
        Arc::clone(&self.values)
    }

    /// Returns the relationship property schema.
    pub fn property_schema(&self) -> &RelationshipPropertySchema {
        &self.schema
    }

    /// Convenience accessor for the property key.
    pub fn key(&self) -> &str {
        self.schema.key()
    }

    /// Convenience accessor for the aggregation strategy.
    pub fn aggregation(&self) -> Aggregation {
        self.schema.aggregation()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::relationship::DefaultRelationshipPropertyValues;

    #[test]
    fn default_relationship_property_creation() {
        let values = Arc::new(DefaultRelationshipPropertyValues::new(
            vec![1.0, 2.0, 3.0],
            0.0,
            3,
        ));

        let property = DefaultRelationshipProperty::with_default_aggregation(
            "weight",
            PropertyState::Normal,
            values,
        );

        assert_eq!(property.key(), "weight");
        assert_eq!(property.aggregation(), Aggregation::None);
        assert_eq!(property.property_schema().state(), PropertyState::Normal);
    }
}
