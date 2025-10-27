//! Dyadic schema types: The Schema of Self-Assertion
//!
//! The second schema is the schema of Self-Assertion (Self-Certification).
//! Dyadic represents the claim: ground ↔ condition
//! - left: The ground
//! - right: The condition
//! 
//! This is the moment of self-assertion in Logic as Science:
//! The schema asserts the relationship (ground claims condition)

use crate::types::schema::hyper::monadic::MonadicSchema;
use crate::types::schema::HyperSchemaLevel;

/// DyadicSchemaId: identifier for dyadic schema entries
pub type DyadicSchemaId = String;

/// Reflection: Relational Essence (Dyad without Center Mark)
/// Pure relational structure without a center mark.
/// This is the Dyadic pattern - two elements in relation
/// but without a mediating third element.
#[derive(Clone, Debug)]
pub struct DyadicReflection {
    pub left: MonadicSchema,
    pub right: MonadicSchema,
}

/// DyadicSchema: schema for binary relationships
#[derive(Clone, Debug)]
pub struct DyadicSchema {
    level: HyperSchemaLevel,
    key: String,
    reflection: DyadicReflection,
    value_type: crate::types::ValueType,
}

impl DyadicSchema {
    /// Create a new dyadic schema
    pub fn new(
        key: impl Into<String>, 
        value_type: crate::types::ValueType,
        left: MonadicSchema,
        right: MonadicSchema,
    ) -> Self {
        Self {
            level: HyperSchemaLevel::Dyadic,
            key: key.into(),
            reflection: DyadicReflection { left, right },
            value_type,
        }
    }

    /// Get the reflection (left and right)
    pub fn reflection(&self) -> &DyadicReflection {
        &self.reflection
    }
}

impl crate::types::schema::HyperSchema for DyadicSchema {
    fn level(&self) -> HyperSchemaLevel {
        HyperSchemaLevel::Dyadic
    }

    fn value_type(&self) -> crate::types::ValueType {
        self.value_type
    }

    fn key(&self) -> &str {
        &self.key
    }
}

