//! Dyadic schema implementation

use super::types::DyadicSchema;
use super::traits::DyadicHyperSchema;
use crate::types::schema::HyperSchema;
use crate::types::ValueType;

/// DefaultDyadicSchema: concrete implementation
#[derive(Clone, Debug)]
pub struct DefaultDyadicSchema {
    inner: DyadicSchema,
}

impl DefaultDyadicSchema {
    /// Create a new default dyadic schema
    pub fn new(
        key: impl Into<String>,
        value_type: ValueType,
        left: crate::types::schema::hyper::monadic::MonadicSchema,
        right: crate::types::schema::hyper::monadic::MonadicSchema,
    ) -> Self {
        Self {
            inner: DyadicSchema::new(key, value_type, left, right),
        }
    }

    /// Get the inner schema
    pub fn inner(&self) -> &DyadicSchema {
        &self.inner
    }
}

impl HyperSchema for DefaultDyadicSchema {
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

impl DyadicHyperSchema for DefaultDyadicSchema {
    fn schema(&self) -> DyadicSchema {
        self.inner.clone()
    }
}

