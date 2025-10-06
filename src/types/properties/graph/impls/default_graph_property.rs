use crate::types::properties::graph::graph_property_values::GraphPropertyValues;
use crate::types::properties::property::Property;
use crate::types::property_state::PropertyState;
use crate::types::schema::{DefaultValue, PropertySchema};
use std::sync::Arc;

/// Concrete graph property implementation that mirrors the structure of the
/// relationship and node property defaults. It stores the shared property
/// values alongside the computed schema metadata.
#[derive(Debug, Clone)]
pub struct DefaultGraphProperty {
    values: Arc<dyn GraphPropertyValues>,
    schema: PropertySchema,
}

impl DefaultGraphProperty {
    /// Construct a graph property using the default `PropertyState::Normal`.
    pub fn of(key: impl Into<String>, values: Arc<dyn GraphPropertyValues>) -> Self {
        Self::with_state(key, PropertyState::Normal, values)
    }

    /// Construct a property with an explicit property state.
    pub fn with_state(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn GraphPropertyValues>,
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
        values: Arc<dyn GraphPropertyValues>,
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
    pub fn with_schema(schema: PropertySchema, values: Arc<dyn GraphPropertyValues>) -> Self {
        Self { values, schema }
    }

    /// Returns the property values as a shared trait object.
    pub fn values(&self) -> &dyn GraphPropertyValues {
        self.values.as_ref()
    }

    /// Returns a cloned `Arc` handle to the underlying property values.
    pub fn values_arc(&self) -> Arc<dyn GraphPropertyValues> {
        Arc::clone(&self.values)
    }

    /// Returns the graph property schema.
    pub fn property_schema(&self) -> &PropertySchema {
        &self.schema
    }

    /// Convenience accessor for the property key.
    pub fn key(&self) -> &str {
        self.schema.key()
    }
}

impl Property for DefaultGraphProperty {
    type Values = Arc<dyn GraphPropertyValues>;

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
    use crate::types::properties::graph::impls::default_graph_property_values::DefaultLongGraphPropertyValues;
    use crate::types::properties::property_values::PropertyValues;

    #[test]
    fn default_graph_property_creation() {
        let values: Arc<dyn GraphPropertyValues> =
            Arc::new(DefaultLongGraphPropertyValues::singleton(42));
        let property = DefaultGraphProperty::of("node_count", values.clone());

        assert_eq!(property.key(), "node_count");
        assert_eq!(property.property_schema().state(), PropertyState::Normal);
        assert_eq!(property.property_schema().value_type(), values.value_type());
        assert!(Arc::ptr_eq(&property.values_arc(), &values));
    }
}
