//! Monadic schema implementation

use super::types::MonadicSchema;
use super::traits::MonadicHyperSchema;
use crate::types::schema::HyperSchema;
use crate::types::ValueType;

/// DefaultMonadicSchema: concrete implementation
#[derive(Clone, Debug)]
pub struct DefaultMonadicSchema {
    inner: MonadicSchema,
}

impl DefaultMonadicSchema {
    /// Create a new default monadic schema
    pub fn new(key: impl Into<String>, value_type: ValueType) -> Self {
        Self {
            inner: MonadicSchema::new(key, value_type),
        }
    }

    /// Get the inner schema
    pub fn inner(&self) -> &MonadicSchema {
        &self.inner
    }
}

impl HyperSchema for DefaultMonadicSchema {
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

impl MonadicHyperSchema for DefaultMonadicSchema {
    fn schema(&self) -> MonadicSchema {
        self.inner.clone()
    }
}

