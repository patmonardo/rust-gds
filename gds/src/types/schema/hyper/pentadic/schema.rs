//! Pentadic schema implementation

use super::types::PentadicSchema;
use super::traits::PentadicHyperSchema;
use crate::types::schema::HyperSchema;
use crate::types::ValueType;

/// DefaultPentadicSchema: concrete implementation
#[derive(Clone, Debug)]
pub struct DefaultPentadicSchema {
    inner: PentadicSchema,
}

impl DefaultPentadicSchema {
    /// Create a new default pentadic schema
    pub fn new(
        key: impl Into<String>,
        value_type: ValueType,
        primary_triad: crate::types::schema::hyper::triadic::TriadicSchema,
        secondary_triad: crate::types::schema::hyper::triadic::TriadicSchema,
        confidence: Option<f32>,
    ) -> Self {
        Self {
            inner: PentadicSchema::new(key, value_type, primary_triad, secondary_triad, confidence),
        }
    }

    /// Get the inner schema
    pub fn inner(&self) -> &PentadicSchema {
        &self.inner
    }
}

impl HyperSchema for DefaultPentadicSchema {
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

impl PentadicHyperSchema for DefaultPentadicSchema {
    fn schema(&self) -> PentadicSchema {
        self.inner.clone()
    }
}

