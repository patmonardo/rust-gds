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
