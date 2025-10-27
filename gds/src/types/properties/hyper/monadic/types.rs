//! Monadic types: small wrapper around DefaultProperty for convenience

use crate::types::properties::property::DefaultProperty;
use crate::types::properties::property_values::PropertyValues;
use crate::types::property_state::PropertyState;
use std::sync::Arc;

/// MonadicProperty: thin wrapper around DefaultProperty that provides a
/// convenient `of(key, values)` constructor used by examples/tests.
#[derive(Clone, Debug)]
pub struct MonadicProperty(pub DefaultProperty);

impl MonadicProperty {
    /// Create a monadic property with a default Persistent state.
    pub fn of(key: impl Into<String>, values: Arc<dyn PropertyValues>) -> Self {
        MonadicProperty(DefaultProperty::of(key, PropertyState::Persistent, values))
    }

    pub fn new_inner(inner: DefaultProperty) -> Self {
        MonadicProperty(inner)
    }

    pub fn inner(&self) -> &DefaultProperty {
        &self.0
    }
}

impl crate::types::properties::property::Property for MonadicProperty {
    fn schema(&self) -> &crate::types::schema::PropertySchema {
        self.0.schema()
    }

    fn values(&self) -> Arc<dyn PropertyValues> {
        self.0.values()
    }
}
