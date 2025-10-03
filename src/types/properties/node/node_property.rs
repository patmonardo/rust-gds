use super::node_property_values::NodePropertyValues;
use crate::types::properties::property::Property;
use crate::types::property::PropertyState;
use crate::types::schema::{DefaultValue, PropertySchema};

/// Represents a property of a node in the graph.
/// Specializes the Property trait for node-specific property values.
pub type NodeProperty = Property<Box<dyn NodePropertyValues>>;

impl NodeProperty {
    /// Creates a new NodeProperty with the given key, state, and values.
    pub fn of(
        key: impl Into<String>,
        state: PropertyState,
        values: Box<dyn NodePropertyValues>,
    ) -> Self {
        let value_type = values.value_type();
        let default_value = DefaultValue::of(value_type);
        let schema = PropertySchema::new(key, value_type, default_value, state);
        Property::new(values, schema)
    }

    /// Creates a new NodeProperty with the given key, state, values, and default value.
    pub fn with_default(
        key: impl Into<String>,
        state: PropertyState,
        values: Box<dyn NodePropertyValues>,
        default_value: DefaultValue,
    ) -> Self {
        let value_type = values.value_type();
        let schema = PropertySchema::new(key, value_type, default_value, state);
        Property::new(values, schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::node::DefaultLongNodePropertyValues;
    use crate::types::properties::property::PropertyTrait;

    #[test]
    fn test_node_property_creation() {
        let values = DefaultLongNodePropertyValues::new(vec![1, 2, 3], 3);
        let prop = NodeProperty::of("age", PropertyState::Normal, Box::new(values));

        assert_eq!(prop.key(), "age");
        assert_eq!(prop.property_state(), PropertyState::Normal);
    }
}
