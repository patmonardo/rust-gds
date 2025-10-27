//! PropertyValues trait and error types
//!
//! **Note**: Implementation macros have been moved to `projection::codegen::property`
//! for better organization and aesthetic barrel imports.
//!
//! Use: `use gds::projection::codegen::property_values_impl!;`

use crate::types::ValueType;
use thiserror::Error;

/// Base trait for all property value containers.
/// Provides access to the value type and common utilities.
///
/// This mirrors the TypeScript PropertyValues interface.
pub trait PropertyValues: Send + Sync + std::fmt::Debug {
    /// Returns the value type of the property values.
    fn value_type(&self) -> ValueType;

    /// Returns the number of elements with property values.
    /// For node properties, this is the node count.
    /// For graph properties, this is the value count.
    fn element_count(&self) -> usize;
}

/// Error type for property value operations.
#[derive(Error, Debug, Clone)]
pub enum PropertyValuesError {
    #[error("Tried to retrieve a value of type {expected:?} from properties of type {actual:?}")]
    UnsupportedType {
        actual: ValueType,
        expected: ValueType,
    },

    #[error("Operation not supported: {0}")]
    UnsupportedOperation(String),

    #[error("Invalid node ID: {0}")]
    InvalidNodeId(u64),

    #[error("Value not found for ID: {0}")]
    ValueNotFound(u64),
}

impl PropertyValuesError {
    /// Creates an error for unsupported type operations.
    pub fn unsupported_type(actual: ValueType, expected: ValueType) -> Self {
        PropertyValuesError::UnsupportedType { actual, expected }
    }

    /// Creates an error for unsupported operations.
    pub fn unsupported_operation(message: impl Into<String>) -> Self {
        PropertyValuesError::UnsupportedOperation(message.into())
    }
}

pub type PropertyValuesResult<T> = Result<T, PropertyValuesError>;

// Implement PropertyValues for Box<dyn PropertyValues> to allow trait objects
impl PropertyValues for Box<dyn PropertyValues> {
    fn value_type(&self) -> ValueType {
        (**self).value_type()
    }

    fn element_count(&self) -> usize {
        (**self).element_count()
    }
}

// Re-export macros from codegen for backward compatibility

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = PropertyValuesError::unsupported_type(ValueType::Long, ValueType::Double);
        assert!(err.to_string().contains("Long"));
        assert!(err.to_string().contains("Double"));

        let err = PropertyValuesError::unsupported_operation("test operation");
        assert!(err.to_string().contains("test operation"));
    }
}
