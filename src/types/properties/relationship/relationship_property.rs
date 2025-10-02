use super::relationship_property_values::RelationshipPropertyValues;
use crate::types::property::PropertyState;
use crate::types::schema::{
    Aggregation, DefaultValue, PropertySchemaTrait, RelationshipPropertySchema,
};

/// Represents a property associated with relationships in a graph.
/// Provides access to property values and schema information.
#[derive(Debug)]
pub struct RelationshipProperty {
    values: Box<dyn RelationshipPropertyValues>,
    schema: RelationshipPropertySchema,
}

impl RelationshipProperty {
    /// Creates a new RelationshipProperty with the given parameters.
    pub fn of(
        key: impl Into<String>,
        state: PropertyState,
        values: Box<dyn RelationshipPropertyValues>,
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
        RelationshipProperty { values, schema }
    }

    /// Creates a RelationshipProperty with default aggregation (NONE).
    pub fn with_default_aggregation(
        key: impl Into<String>,
        state: PropertyState,
        values: Box<dyn RelationshipPropertyValues>,
    ) -> Self {
        let value_type = values.value_type();
        let default_value = DefaultValue::of(value_type);
        Self::of(key, state, values, default_value, Aggregation::None)
    }

    /// Returns the property values.
    pub fn values(&self) -> &dyn RelationshipPropertyValues {
        self.values.as_ref()
    }

    /// Returns the schema for this relationship property.
    pub fn property_schema(&self) -> &RelationshipPropertySchema {
        &self.schema
    }

    /// Returns the property key.
    pub fn key(&self) -> &str {
        self.schema.key()
    }

    /// Returns the aggregation strategy for this property.
    pub fn aggregation(&self) -> Aggregation {
        self.schema.aggregation()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::relationship::DefaultRelationshipPropertyValues;

    #[test]
    fn test_relationship_property_creation() {
        let values = DefaultRelationshipPropertyValues::new(vec![1.0, 2.0, 3.0], 0.0, 3);
        let prop = RelationshipProperty::with_default_aggregation(
            "weight",
            PropertyState::Normal,
            Box::new(values),
        );

        assert_eq!(prop.key(), "weight");
        assert_eq!(prop.aggregation(), Aggregation::None);
    }
}
