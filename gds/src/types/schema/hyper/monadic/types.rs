//! Monadic schema types: universal building block schema

use crate::types::schema::HyperSchema;
use crate::types::schema::HyperSchemaLevel;

/// MonadicSchemaId: identifier for monadic schema entries
pub type MonadicSchemaId = String;

/// MonadicSchema: thin wrapper around base HyperSchema
#[derive(Clone, Debug)]
pub struct MonadicSchema {
    level: HyperSchemaLevel,
    key: String,
    value_type: crate::types::ValueType,
}

impl MonadicSchema {
    /// Create a new monadic schema
    pub fn new(key: impl Into<String>, value_type: crate::types::ValueType) -> Self {
        Self {
            level: HyperSchemaLevel::Monadic,
            key: key.into(),
            value_type,
        }
    }

    /// Get the inner key
    pub fn inner_key(&self) -> &str {
        &self.key
    }

    /// Get the level
    pub fn level(&self) -> HyperSchemaLevel {
        self.level
    }
}

impl HyperSchema for MonadicSchema {
    fn level(&self) -> HyperSchemaLevel {
        HyperSchemaLevel::Monadic
    }

    fn value_type(&self) -> crate::types::ValueType {
        self.value_type
    }

    fn key(&self) -> &str {
        &self.key
    }
}

