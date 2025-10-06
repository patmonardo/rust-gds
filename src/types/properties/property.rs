use super::property_values::PropertyValues;
use crate::types::property::PropertyState;
use crate::types::schema::{DefaultValue, PropertySchema};
use std::sync::Arc;

/// Canonical trait representing a property (no "Trait" suffix).
pub trait Property: Send + Sync {
    fn schema(&self) -> &PropertySchema;
    fn values(&self) -> Arc<dyn PropertyValues>;
}

// Concrete header+body property used by stores (non-generic, holds Arc dyn).
#[derive(Clone, Debug)]
pub struct DefaultProperty {
    pub schema: PropertySchema,
    pub values: Arc<dyn PropertyValues>,
}

impl DefaultProperty {
    pub fn new(schema: PropertySchema, values: Arc<dyn PropertyValues>) -> Self {
        Self { schema, values }
    }

    pub fn of(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn PropertyValues>,
    ) -> Self {
        let vt = values.value_type();
        let default_value = DefaultValue::of(vt);
        let schema = PropertySchema::new(key.into(), vt, default_value, state);
        Self::new(schema, values)
    }

    pub fn with_default(
        key: impl Into<String>,
        state: PropertyState,
        values: Arc<dyn PropertyValues>,
        default_value: DefaultValue,
    ) -> Self {
        let vt = values.value_type();
        let schema = PropertySchema::new(key.into(), vt, default_value, state);
        Self::new(schema, values)
    }

    // Convenience accessors
    pub fn key(&self) -> &str {
        self.schema.key()
    }

    pub fn value_type(&self) -> crate::types::property::ValueType {
        self.schema.value_type()
    }

    pub fn property_state(&self) -> PropertyState {
        self.schema.state()
    }

    pub fn default_value(&self) -> &DefaultValue {
        self.schema.default_value()
    }
}

impl Property for DefaultProperty {
    fn schema(&self) -> &PropertySchema {
        &self.schema
    }
    fn values(&self) -> Arc<dyn PropertyValues> {
        Arc::clone(&self.values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::properties::node::DefaultLongNodePropertyValues;
    use crate::types::property::ValueType;

    #[test]
    fn test_property_creation() {
        let values = DefaultLongNodePropertyValues::new(vec![1, 2, 3], 3);
        let schema = PropertySchema::new(
            "test_prop",
            ValueType::Long,
            DefaultValue::Long(0),
            PropertyState::Normal,
        );

        let prop = DefaultProperty::new(schema, Arc::new(values));
        assert_eq!(prop.key(), "test_prop");
        assert_eq!(prop.value_type(), ValueType::Long);
        assert_eq!(prop.property_state(), PropertyState::Normal);
    }
}
