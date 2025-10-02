use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::types::value_type::ValueType;
use crate::values::primitive::traits::{GdsValue, IntegralValue};

/// Scalar long value — renamed to explicit "Default" prefix.
#[derive(Clone, Debug)]
pub struct DefaultLongValue(pub i64);

impl DefaultLongValue {
    pub fn new(v: i64) -> Self {
        Self(v)
    }

    /// Java-style 32-bit hash (lower32 ^ upper32)
    pub fn hash_code(&self) -> i32 {
        let v = self.0 as i128;
        let lower = (v & 0xffffffff) as i64;
        let upper = ((v >> 32) & 0xffffffff) as i64;
        (lower ^ upper) as i32
    }

    /// Provide an object-like representation (JSON) for callers.
    pub fn as_object(&self) -> JsonValue {
        JsonValue::from(self.0)
    }
}

impl IntegralValue for DefaultLongValue {
    fn long_value(&self) -> i64 {
        self.0
    }
}

impl GdsValue for DefaultLongValue {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }

    // Many modules expect `as_object()` on GdsValue; if your trait uses a different
    // method name (e.g. `as_json()`), adjust the trait or add a forwarding method.
    fn as_object(&self) -> JsonValue {
        self.as_object()
    }
}
