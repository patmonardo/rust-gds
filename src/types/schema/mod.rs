// Schema module for Rust GDS
//
// This module provides schema definitions for graph elements (nodes, relationships, and graphs).
// It mirrors the TypeScript GDS schema API structure with Rust idioms.

pub mod default_value;
pub mod direction;
pub mod element_identifier;
pub mod graph_schema;
pub mod node_schema;
pub mod property_schema;
pub mod relationship_schema;

// Re-export core types for convenience
pub use default_value::DefaultValue;
pub use direction::Direction;
pub use element_identifier::{ElementIdentifier, NodeLabel, RelationshipType};
pub use graph_schema::{GraphSchema, MutableGraphSchema};
pub use node_schema::{MutableNodeSchema, MutableNodeSchemaEntry, NodeSchema, NodeSchemaEntry};
pub use property_schema::{
    Aggregation, PropertySchema, PropertySchemaTrait, RelationshipPropertySchema,
};
pub use relationship_schema::{
    MutableRelationshipSchema, MutableRelationshipSchemaEntry, RelationshipSchema,
    RelationshipSchemaEntry,
};

use crate::types::property::ValueType;

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
