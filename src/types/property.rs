use polars::prelude::{DataType, Series};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Basic set of value types - derived from Polars DataType when needed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueType {
    String,
    Long,
    Double,
    Boolean,
    DoubleArray,
    FloatArray,
    LongArray,
}

impl From<&DataType> for ValueType {
    fn from(dt: &DataType) -> Self {
        match dt {
            DataType::Utf8 => ValueType::String,
            DataType::Int64 => ValueType::Long,
            DataType::Float64 => ValueType::Double,
            DataType::Boolean => ValueType::Boolean,
            // Add array type mappings as needed
            _ => ValueType::Double, // Default fallback
        }
    }
}

/// Property lifecycle/state marker.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyState {
    Normal,
    Deleted,
}

/// Schema describing a property.
#[derive(Clone, Debug)]
pub struct PropertySchema {
    key: String,
    value_type: ValueType,
    state: PropertyState,
}

impl PropertySchema {
    pub fn new(key: impl Into<String>, value_type: ValueType, state: PropertyState) -> Self {
        Self {
            key: key.into(),
            value_type,
            state,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value_type(&self) -> ValueType {
        self.value_type
    }

    pub fn state(&self) -> PropertyState {
        self.state.clone()
    }
}

/// Property trait: exposes a Polars Series as the values container, plus schema.
pub trait Property: Debug + Send + Sync {
    /// Return a reference to the Series that backs this property.
    fn values(&self) -> &Series;

    /// Return the schema for this property.
    fn property_schema(&self) -> &PropertySchema;

    /// Convenience helpers delegating to schema.
    fn key(&self) -> &str {
        self.property_schema().key()
    }

    fn value_type(&self) -> ValueType {
        self.property_schema().value_type()
    }

    fn property_state(&self) -> PropertyState {
        self.property_schema().state()
    }
}

/// A simple concrete property holding an owned Series and schema.
/// Useful for development and for mapping TS SimpleProperty -> Rust.
#[derive(Debug, Clone)]
pub struct SimpleProperty {
    pub values: Series,
    pub schema: PropertySchema,
}

impl SimpleProperty {
    pub fn new(values: Series, schema: PropertySchema) -> Self {
        Self { values, schema }
    }
}

impl Property for SimpleProperty {
    fn values(&self) -> &Series {
        &self.values
    }

    fn property_schema(&self) -> &PropertySchema {
        &self.schema
    }
}
