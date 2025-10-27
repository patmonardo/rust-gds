//! Tetradic schema implementation

use super::types::TetradicSchema;
use super::traits::TetradicHyperSchema;
use crate::types::schema::HyperSchema;
use crate::types::ValueType;

/// DefaultTetradicSchema: concrete implementation
#[derive(Clone, Debug)]
pub struct DefaultTetradicSchema {
    inner: TetradicSchema,
}

impl DefaultTetradicSchema {
    /// Create a new default tetradic schema
    pub fn new(
        key: impl Into<String>,
        value_type: ValueType,
        left_dyad: crate::types::schema::hyper::dyadic::DyadicSchema,
        right_dyad: crate::types::schema::hyper::dyadic::DyadicSchema,
        pipeline_metadata: Option<String>,
    ) -> Self {
        Self {
            inner: TetradicSchema::new(key, value_type, left_dyad, right_dyad, pipeline_metadata),
        }
    }

    /// Get the inner schema
    pub fn inner(&self) -> &TetradicSchema {
        &self.inner
    }
}

impl HyperSchema for DefaultTetradicSchema {
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

impl TetradicHyperSchema for DefaultTetradicSchema {
    fn schema(&self) -> TetradicSchema {
        self.inner.clone()
    }
}

