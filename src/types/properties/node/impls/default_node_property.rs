use crate::types::properties::node::node_property::NodeProperty;
use crate::types::properties::node::node_property_values::NodePropertyValues;
use crate::types::properties::property::PropertyTrait;
use crate::types::property::PropertyState;
use crate::types::schema::{DefaultValue, PropertySchema};

/// Default concrete implementation of a node property.
#[derive(Debug)]
pub struct DefaultNodeProperty {
    values: Box<dyn NodePropertyValues>,
    schema: PropertySchema,
}

impl DefaultNodeProperty {
    /// Creates a new node property with an auto-computed default value.
    pub fn of(
        key: impl Into<String>,
        state: PropertyState,
        values: Box<dyn NodePropertyValues>,
    ) -> Self {
        let value_type = values.value_type();
        let default_value = DefaultValue::of(value_type);
        let schema = PropertySchema::new(key, value_type, default_value, state);
        Self { values, schema }
    }

    /// Creates a node property with an explicit default value.
    pub fn with_default(
        key: impl Into<String>,
        state: PropertyState,
        values: Box<dyn NodePropertyValues>,
        default_value: DefaultValue,
    ) -> Self {
        let value_type = values.value_type();
        let schema = PropertySchema::new(key, value_type, default_value, state);
        Self { values, schema }
    }

    /// Borrow the raw boxed values (internal helper if needed).
    pub fn values_box(&self) -> &Box<dyn NodePropertyValues> {
        &self.values
    }
}

impl NodeProperty for DefaultNodeProperty {
    fn key(&self) -> &str {
        self.schema.key()
    }

    fn property_state(&self) -> PropertyState {
        self.schema.state()
    }

    fn property_schema(&self) -> &PropertySchema {
        &self.schema
    }

    fn values(&self) -> &dyn NodePropertyValues {
        self.values.as_ref()
    }
}

impl PropertyTrait for DefaultNodeProperty {
    fn key(&self) -> &str {
        self.schema.key()
    }

    fn property_state(&self) -> PropertyState {
        self.schema.state()
    }

    fn property_schema(&self) -> &PropertySchema {
        &self.schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::node::DefaultLongNodePropertyValues;

    #[test]
    fn create_default_node_property() {
        let values = Box::new(DefaultLongNodePropertyValues::new(vec![1, 2, 3], 3));
        let prop = DefaultNodeProperty::of("age", PropertyState::Normal, values);
        assert_eq!(prop.key(), "age");
        assert_eq!(prop.property_state(), PropertyState::Normal);
    }

    #[test]
    fn create_with_explicit_default() {
        let values = Box::new(DefaultLongNodePropertyValues::new(vec![10, 20], 2));
        let default_value = DefaultValue::of(values.value_type());
        let prop = DefaultNodeProperty::with_default(
            "score",
            PropertyState::Normal,
            values,
            default_value,
        );
        assert_eq!(prop.key(), "score");
    }
}
