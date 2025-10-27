//! Triadic schema implementation

use super::types::{TriadicSchema, CenterMark};
use super::traits::TriadicHyperSchema;
use crate::types::schema::HyperSchema;
use crate::types::ValueType;

/// DefaultTriadicSchema: concrete implementation
#[derive(Clone, Debug)]
pub struct DefaultTriadicSchema {
    inner: TriadicSchema,
}

impl DefaultTriadicSchema {
    /// Create a new default triadic schema
    pub fn new(
        key: impl Into<String>,
        value_type: ValueType,
        being: crate::types::schema::hyper::monadic::MonadicSchema,
        essence: crate::types::schema::hyper::monadic::MonadicSchema,
        concept: crate::types::schema::hyper::monadic::MonadicSchema,
        center_mark: CenterMark,
    ) -> Self {
        Self {
            inner: TriadicSchema::new(key, value_type, being, essence, concept, center_mark),
        }
    }

    /// Get the inner schema
    pub fn inner(&self) -> &TriadicSchema {
        &self.inner
    }
}

impl HyperSchema for DefaultTriadicSchema {
    fn level(&self) -> crate::types::schema::HyperSchemaLevel {
        self.inner.level()
    }

    fn value_type(&self) -> ValueType {
        self.inner.value_type()
    }

    fn key(&self) -> &str {
        self.inner.key()
    }
}

impl TriadicHyperSchema for DefaultTriadicSchema {
    fn schema(&self) -> TriadicSchema {
        self.inner.clone()
    }
}

