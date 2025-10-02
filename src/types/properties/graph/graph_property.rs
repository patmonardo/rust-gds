use super::graph_property_values::GraphPropertyValues;
use crate::types::properties::property::Property;
use crate::types::property::PropertyState;
use crate::types::schema::{DefaultValue, PropertySchema};

/// Represents a property at the graph level.
/// This specializes the general Property trait for graph properties.
pub type GraphProperty = Property<Box<dyn GraphPropertyValues>>;

impl GraphProperty {
    /// Creates a new GraphProperty with the given key and values.
    pub fn of(key: impl Into<String>, values: Box<dyn GraphPropertyValues>) -> Self {
        let value_type = values.value_type();
        let default_value = DefaultValue::of(value_type);
        let schema = PropertySchema::new(key, value_type, default_value, PropertyState::Normal);
        Property::new(values, schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::graph::DefaultLongGraphPropertyValues;
    use crate::types::properties::property::PropertyTrait;

    #[test]
    fn test_graph_property_creation() {
        let values = DefaultLongGraphPropertyValues::singleton(100);
        let prop = GraphProperty::of("node_count", Box::new(values));

        assert_eq!(prop.key(), "node_count");
        assert_eq!(prop.property_state(), PropertyState::Normal);
    }
}
