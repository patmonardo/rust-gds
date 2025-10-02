use super::property_values::PropertyValues;
use crate::types::property::{PropertyState, ValueType};
use crate::types::schema::{DefaultValue, PropertySchema};

/// Trait representing a property in a graph element (node, relationship, or graph).
/// Provides access to property values and schema information.
///
/// This mirrors the TypeScript Property interface.
pub trait PropertyTrait: Send + Sync {
    type Values: PropertyValues;

    /// Returns the property values.
    fn values(&self) -> &Self::Values;

    /// Returns the schema for this property.
    fn property_schema(&self) -> &PropertySchema;

    /// Returns the property key.
    /// Convenience method that delegates to the property schema.
    fn key(&self) -> &str {
        self.property_schema().key()
    }

    /// Returns the value type of this property.
    /// Convenience method that delegates to the property schema.
    fn value_type(&self) -> ValueType {
        self.property_schema().value_type()
    }

    /// Returns the default value for this property.
    /// Convenience method that delegates to the property schema.
    fn default_value(&self) -> &DefaultValue {
        self.property_schema().default_value()
    }

    /// Returns the state of this property.
    /// Convenience method that delegates to the property schema.
    fn property_state(&self) -> PropertyState {
        self.property_schema().state()
    }
}

/// Generic property implementation that can hold any PropertyValues type.
/// This is the default implementation used throughout the system.
#[derive(Debug, Clone)]
pub struct Property<V: PropertyValues> {
    values: V,
    schema: PropertySchema,
}

impl<V: PropertyValues> Property<V> {
    /// Creates a new property with the given values and schema.
    pub fn new(values: V, schema: PropertySchema) -> Self {
        Property { values, schema }
    }
}
impl<V: PropertyValues> PropertyTrait for Property<V> {
    type Values = V;

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

    #[test]
    fn test_property_creation() {
        let values = DefaultLongNodePropertyValues::new(vec![1, 2, 3], 3);
        let schema = PropertySchema::new(
            "test_prop",
            ValueType::Long,
            DefaultValue::Long(0),
            PropertyState::Normal,
        );

        let prop = Property::new(values, schema);
        assert_eq!(prop.key(), "test_prop");
        assert_eq!(prop.value_type(), ValueType::Long);
        assert_eq!(prop.property_state(), PropertyState::Normal);
    }
}
