use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::types::default_value::DefaultValue;
use crate::types::node_property_values::NodePropertyValues;
use crate::types::value_type::ValueType;

/// Trait marker for a GDS value wrapper (scalar or array)
pub trait GdsValue: Send + Sync {
    fn value_type(&self) -> ValueType;
    fn as_json(&self) -> JsonValue;
}

/// No-value singleton
#[derive(Debug, Clone)]
pub struct NoValue;
impl NoValue {
    pub const fn instance() -> Self {
        NoValue
    }
}
impl GdsValue for NoValue {
    fn value_type(&self) -> ValueType {
        ValueType::Unknown
    }
    fn as_json(&self) -> JsonValue {
        JsonValue::Null
    }
}

/// Example LongValue wrapper
#[derive(Debug, Clone)]
pub struct LongValue(pub i64);
impl GdsValue for LongValue {
    fn value_type(&self) -> ValueType {
        ValueType::Long
    }
    fn as_json(&self) -> JsonValue {
        JsonValue::from(self.0)
    }
}

/// Example DoubleArray wrapper (owns Vec<f64> or can wrap Arc<Vec<f64>>)
#[derive(Debug, Clone)]
pub struct DoubleArrayValue(pub Arc<Vec<f64>>);
impl GdsValue for DoubleArrayValue {
    fn value_type(&self) -> ValueType {
        ValueType::DoubleArray
    }
    fn as_json(&self) -> JsonValue {
        JsonValue::from(self.0.as_ref().clone())
    }
}

/// High-level factory similar to PrimitiveValues.of/create
pub struct PrimitiveValues;
impl PrimitiveValues {
    /// of -> Option<Arc<dyn GdsValue>> : non-panicking, returns None if not convertible
    pub fn of(input: &JsonValue) -> Option<Arc<dyn GdsValue>> {
        use JsonValue::*;
        match input {
            Null => Some(Arc::new(NoValue::instance())),
            Number(n) => {
                if let Some(i) = n.as_i64() {
                    Some(Arc::new(LongValue(i)))
                } else if let Some(f) = n.as_f64() {
                    // decide float vs double semantics; here treat all non-integer as double
                    let v = vec![*f];
                    Some(Arc::new(DoubleArrayValue(Arc::new(v))))
                } else {
                    None
                }
            }
            String(s) => {
                // try parse as number, fallback to string wrapper if you implement one
                if let Ok(i) = s.parse::<i64>() {
                    Some(Arc::new(LongValue(i)))
                } else if let Ok(f) = s.parse::<f64>() {
                    Some(Arc::new(DoubleArrayValue(Arc::new(vec![f]))))
                } else {
                    None
                }
            }
            Array(arr) => {
                if arr.is_empty() {
                    // convention: empty -> empty long array
                    Some(Arc::new(DoubleArrayValue(Arc::new(Vec::new()))))
                } else if arr.iter().all(|v| v.is_number()) {
                    // choose integer vs float
                    if arr.iter().all(|v| v.as_i64().is_some()) {
                        let mut out = Vec::with_capacity(arr.len());
                        for v in arr {
                            out.push(v.as_i64().unwrap());
                        }
                        // wrap as LongArray (not implemented here)
                        // placeholder: return None to indicate need for concrete impl
                        None
                    } else {
                        let mut out = Vec::with_capacity(arr.len());
                        for v in arr {
                            out.push(v.as_f64().unwrap());
                        }
                        Some(Arc::new(DoubleArrayValue(Arc::new(out))))
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// create -> Result: fails with error if not convertible (mirrors TS create)
    pub fn create(input: &JsonValue) -> Result<Arc<dyn GdsValue>, String> {
        Self::of(input).ok_or_else(|| format!("Unsupported value: {}", input))
    }
}
