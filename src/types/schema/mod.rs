// Schema module for Rust GDS
//
// This module provides schema definitions for graph elements (nodes, relationships, and graphs).
// It mirrors the TypeScript GDS schema API structure with Rust idioms.

pub mod direction;
pub mod graph_schema;
pub mod node_schema;
pub mod property_schema;
pub mod relationship_schema;

pub use crate::types::ValueType;
pub use direction::*;
pub use graph_schema::*;
pub use node_schema::*;
pub use property_schema::*;
pub use relationship_schema::*;

pub use crate::projection::{NodeLabel, RelationshipType};

/// Error type for schema operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum SchemaError {
    #[error("Property '{key}' has conflicting value types: {left:?} vs {right:?}")]
    PropertyTypeConflict {
        key: String,
        left: ValueType,
        right: ValueType,
    },

    #[error("Cannot union entries with different identifiers: {left} and {right}")]
    IdentifierMismatch { left: String, right: String },

    #[error("Conflicting directionality for relationship type: {relationship_type}")]
    DirectionalityConflict { relationship_type: String },

    #[error("Invalid operation: {message}")]
    InvalidOperation { message: String },
}

pub type SchemaResult<T> = Result<T, SchemaError>;

use serde::{Deserialize, Serialize};

/// Aggregation strategy for relationship properties.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Aggregation {
    None,
    Min,
    Max,
    Sum,
    Count,
    Single,
    Default,
}

impl Aggregation {
    /// Resolves DEFAULT aggregation to a concrete aggregation strategy.
    pub fn resolve(agg: Aggregation) -> Aggregation {
        match agg {
            Aggregation::Default => Aggregation::None,
            other => other,
        }
    }

    pub fn equals(a: Aggregation, b: Aggregation) -> bool {
        a == b
    }

    pub fn hash_code(agg: Aggregation) -> i32 {
        agg as i32
    }
}

impl std::fmt::Display for Aggregation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Aggregation::None => write!(f, "NONE"),
            Aggregation::Min => write!(f, "MIN"),
            Aggregation::Max => write!(f, "MAX"),
            Aggregation::Sum => write!(f, "SUM"),
            Aggregation::Count => write!(f, "COUNT"),
            Aggregation::Single => write!(f, "SINGLE"),
            Aggregation::Default => write!(f, "DEFAULT"),
        }
    }
}
