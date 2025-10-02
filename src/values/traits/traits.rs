use crate::types::value_type::ValueType;
use once_cell::sync::Lazy;
use serde_json::Value as JsonValue;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Core GDS value trait (maps TS GdsValue)
pub trait GdsValue: Send + Sync {
    fn value_type(&self) -> ValueType;
    fn as_object(&self) -> JsonValue;

    /// Default equality: compare object representations
    fn equals(&self, other: &dyn GdsValue) -> bool {
        self.as_object() == other.as_object()
    }

    /// Hash code derived from object JSON string (stable enough for defaults)
    fn hash_code(&self) -> u64 {
        let s = self.as_object().to_string();
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

    fn to_string(&self) -> String {
        self.as_object().to_string()
    }
}

/// No-value singleton (maps TS GdsNoValue)
#[derive(Debug)]
pub struct GdsNoValue;

impl GdsNoValue {
    fn new() -> Self {
        GdsNoValue
    }
}

/// Global static reference for NO_VALUE
pub static NO_VALUE_SINGLETON: Lazy<GdsNoValue> = Lazy::new(|| GdsNoValue::new());

/// Convenience accessor
pub fn no_value() -> &'static GdsNoValue {
    &*NO_VALUE_SINGLETON
}

impl GdsValue for GdsNoValue {
    fn value_type(&self) -> ValueType {
        ValueType::Unknown
    }
    fn as_object(&self) -> JsonValue {
        JsonValue::Null
    }
    fn equals(&self, other: &dyn GdsValue) -> bool {
        matches!(other.as_object(), JsonValue::Null)
    }
    fn hash_code(&self) -> u64 {
        0
    }
    fn to_string(&self) -> String {
        "NO_VALUE".into()
    }
}

impl fmt::Display for GdsNoValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NO_VALUE")
    }
}

/// Base array trait (maps TS Array)
pub trait ArrayTrait: GdsValue {
    fn length(&self) -> usize;

    fn equals_bytes(&self, _other: &[u8]) -> bool {
        false
    }
    fn equals_shorts(&self, _other: &[i16]) -> bool {
        false
    }
    fn equals_ints(&self, _other: &[i32]) -> bool {
        false
    }
    fn equals_longs(&self, _other: &[i64]) -> bool {
        false
    }
    fn equals_floats(&self, _other: &[f32]) -> bool {
        false
    }
    fn equals_doubles(&self, _other: &[f64]) -> bool {
        false
    }

    fn to_string(&self) -> String {
        self.as_object().to_string()
    }
}

/// IntegralArray extends ArrayTrait (maps TS IntegralArray)
pub trait IntegralArray: ArrayTrait {
    fn long_value(&self, idx: usize) -> i64;
    fn long_array_value(&self) -> Vec<i64>;
}

/// FloatingPointArray extends ArrayTrait
pub trait FloatingPointArray: ArrayTrait {
    fn double_value(&self, idx: usize) -> f64;
    fn double_array_value(&self) -> Vec<f64>;
}

/// LongArray interface (maps TS LongArray)
pub trait LongArrayTrait: IntegralArray {
    fn type_name(&self) -> ValueType {
        ValueType::LongArray
    }
}

/// FloatArray interface (maps TS FloatArray)
pub trait FloatArrayTrait: FloatingPointArray {
    fn type_name(&self) -> ValueType {
        ValueType::FloatArray
    }
}

/// DoubleArray interface (maps TS DoubleArray)
pub trait DoubleArrayTrait: FloatingPointArray {
    fn type_name(&self) -> ValueType {
        ValueType::DoubleArray
    }
}

/// Scalar integral value (maps TS IntegralValue)
pub trait IntegralValue: GdsValue {
    fn long_value(&self) -> i64;
    fn as_object_default(&self) -> JsonValue {
        JsonValue::from(self.long_value())
    }
}

/// Scalar floating value (maps TS FloatingPointValue)
pub trait FloatingPointValue: GdsValue {
    fn double_value(&self) -> f64;
    fn as_object_default(&self) -> JsonValue {
        JsonValue::from(self.double_value())
    }
}
