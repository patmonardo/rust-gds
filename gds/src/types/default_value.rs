use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt;

use crate::types::ValueType;

/// Small set of fallback constants mirroring the TS implementation.
pub const INTEGER_DEFAULT_FALLBACK: i32 = -2_147_483_648;
pub const LONG_DEFAULT_FALLBACK: i64 = 0;
pub const FLOAT_DEFAULT_FALLBACK: f32 = f32::NAN;
pub const DOUBLE_DEFAULT_FALLBACK: f64 = f64::NAN;

/// Error type for conversions/validation.
#[derive(thiserror::Error, Debug)]
pub enum DefaultValueError {
    #[error("invalid type for conversion: expected {expected}, got {got}")]
    InvalidType { expected: &'static str, got: String },

    #[error("conversion overflow/unsafe: {0}")]
    UnsafeConversion(String),

    #[error("parse error: {0}")]
    ParseError(String),
}

/// DefaultValue: holds an optional JSON value and a user-defined flag.
/// Uses serde_json::Value for flexible representation (strings, numbers, arrays).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultValue {
    value: Option<JsonValue>,
    is_user_defined: bool,
}

impl DefaultValue {
    pub fn create(value: Option<JsonValue>, is_user_defined: bool) -> Self {
        Self {
            value,
            is_user_defined,
        }
    }

    pub fn user_defined(value: Option<JsonValue>) -> Self {
        Self::create(value, true)
    }

    pub fn system_default(value: Option<JsonValue>) -> Self {
        Self::create(value, false)
    }

    pub fn is_user_defined(&self) -> bool {
        self.is_user_defined
    }

    pub fn is_null_value(&self) -> bool {
        self.value.is_none()
    }

    pub fn get_object(&self) -> Option<&JsonValue> {
        self.value.as_ref()
    }

    pub fn long_value(&self) -> Result<i64, DefaultValueError> {
        match &self.value {
            None => Ok(LONG_DEFAULT_FALLBACK),
            Some(v) => match v {
                JsonValue::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        Ok(i)
                    } else if let Some(f) = n.as_f64() {
                        exact_double_to_long(f).map_err(DefaultValueError::UnsafeConversion)
                    } else {
                        Err(DefaultValueError::InvalidType {
                            expected: "Long",
                            got: type_name(v),
                        })
                    }
                }
                JsonValue::String(s) => s
                    .parse::<i64>()
                    .map_err(|e| DefaultValueError::ParseError(e.to_string())),
                _ => Err(DefaultValueError::InvalidType {
                    expected: "Long",
                    got: type_name(v),
                }),
            },
        }
    }

    pub fn double_value(&self) -> Result<f64, DefaultValueError> {
        match &self.value {
            None => Ok(DOUBLE_DEFAULT_FALLBACK),
            Some(v) => match v {
                JsonValue::Number(n) => n.as_f64().ok_or_else(|| DefaultValueError::InvalidType {
                    expected: "Double",
                    got: type_name(v),
                }),
                JsonValue::String(s) => s
                    .parse::<f64>()
                    .map_err(|e| DefaultValueError::ParseError(e.to_string())),
                _ => Err(DefaultValueError::InvalidType {
                    expected: "Double",
                    got: type_name(v),
                }),
            },
        }
    }

    pub fn float_value(&self) -> Result<f32, DefaultValueError> {
        let d = self.double_value()?;
        not_overflowing_double_to_float(d).map_err(DefaultValueError::UnsafeConversion)
    }

    pub fn boolean_value(&self) -> Result<bool, DefaultValueError> {
        match &self.value {
            None => Ok(false),
            Some(JsonValue::Bool(b)) => Ok(*b),
            Some(v) => Err(DefaultValueError::InvalidType {
                expected: "Boolean",
                got: type_name(v),
            }),
        }
    }

    pub fn string_value(&self) -> Option<String> {
        self.value.as_ref().map(|v| match v {
            JsonValue::String(s) => s.clone(),
            other => other.to_string(),
        })
    }

    pub fn double_array_value(&self) -> Result<Option<Vec<f64>>, DefaultValueError> {
        match &self.value {
            None => Ok(None),
            Some(JsonValue::Array(arr)) => {
                let mut out = Vec::with_capacity(arr.len());
                for v in arr {
                    match v {
                        JsonValue::Number(n) => {
                            let f = n.as_f64().ok_or_else(|| DefaultValueError::InvalidType {
                                expected: "double[] element",
                                got: type_name(v),
                            })?;
                            out.push(exact_long_to_double_if_int(n, f)?);
                        }
                        JsonValue::String(s) => {
                            let parsed = s
                                .parse::<f64>()
                                .map_err(|e| DefaultValueError::ParseError(e.to_string()))?;
                            out.push(parsed);
                        }
                        _ => {
                            return Err(DefaultValueError::InvalidType {
                                expected: "double[]",
                                got: type_name(v),
                            })
                        }
                    }
                }
                Ok(Some(out))
            }
            Some(other) => Err(DefaultValueError::InvalidType {
                expected: "double[]",
                got: type_name(other),
            }),
        }
    }

    pub fn float_array_value(&self) -> Result<Option<Vec<f32>>, DefaultValueError> {
        match self.double_array_value()? {
            None => Ok(None),
            Some(vec) => {
                let mut out: Vec<f32> = Vec::with_capacity(vec.len());
                for d in vec {
                    out.push(
                        not_overflowing_double_to_float(d)
                            .map_err(DefaultValueError::UnsafeConversion)?,
                    );
                }
                Ok(Some(out))
            }
        }
    }

    pub fn long_array_value(&self) -> Result<Option<Vec<i64>>, DefaultValueError> {
        match &self.value {
            None => Ok(None),
            Some(JsonValue::Array(arr)) => {
                let mut out = Vec::with_capacity(arr.len());
                for v in arr {
                    match v {
                        JsonValue::Number(n) => {
                            if let Some(i) = n.as_i64() {
                                out.push(i);
                            } else if let Some(f) = n.as_f64() {
                                out.push(
                                    exact_double_to_long(f)
                                        .map_err(DefaultValueError::UnsafeConversion)?,
                                );
                            } else {
                                return Err(DefaultValueError::InvalidType {
                                    expected: "long[] element",
                                    got: type_name(v),
                                });
                            }
                        }
                        JsonValue::String(s) => {
                            let parsed = s
                                .parse::<i64>()
                                .map_err(|e| DefaultValueError::ParseError(e.to_string()))?;
                            out.push(parsed);
                        }
                        _ => {
                            return Err(DefaultValueError::InvalidType {
                                expected: "long[]",
                                got: type_name(v),
                            })
                        }
                    }
                }
                Ok(Some(out))
            }
            Some(other) => Err(DefaultValueError::InvalidType {
                expected: "long[]",
                got: type_name(other),
            }),
        }
    }

    pub fn equals(&self, other: &DefaultValue) -> bool {
        self.value == other.value
    }

    pub fn validate_for_type(&self, expected: ValueType) -> Result<(), DefaultValueError> {
        match expected {
            ValueType::Long => {
                self.long_value()?;
                Ok(())
            }
            ValueType::Double => {
                // If the underlying JSON is an integer number, treat it as not-a-Double
                if let Some(JsonValue::Number(n)) = &self.value {
                    if n.is_i64() {
                        return Err(DefaultValueError::InvalidType {
                            expected: "Double",
                            got: "integer".into(),
                        });
                    }
                }
                self.double_value()?;
                Ok(())
            }
            ValueType::DoubleArray => {
                self.double_array_value()?;
                Ok(())
            }
            ValueType::FloatArray => {
                self.float_array_value()?;
                Ok(())
            }
            ValueType::LongArray => {
                self.long_array_value()?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Creates a system default for a given ValueType.
    /// Mirrors the TS `DefaultValue.of(null, type, false)` behavior.
    pub fn of(value_type: ValueType) -> Self {
        use serde_json::Number;

        let json_val = match value_type {
            ValueType::Long => Some(JsonValue::Number(Number::from(LONG_DEFAULT_FALLBACK))),
            ValueType::Double => {
                // NaN cannot be represented in JSON Number, so use null
                None
            }
            ValueType::Float => None, // NaN
            ValueType::Int => Some(JsonValue::Number(Number::from(INTEGER_DEFAULT_FALLBACK))),
            ValueType::Boolean => Some(JsonValue::Bool(false)),
            ValueType::String => Some(JsonValue::String(String::new())),
            ValueType::LongArray => None,
            ValueType::DoubleArray => None,
            ValueType::FloatArray => None,
            ValueType::IntArray => None,
            ValueType::StringArray => None,
            ValueType::BooleanArray => None,
            _ => None,
        };

        Self::system_default(json_val)
    }

    /// Convenience factory methods matching TS forLong(), forDouble(), etc.
    pub fn for_long() -> Self {
        Self::system_default(Some(JsonValue::Number(serde_json::Number::from(
            LONG_DEFAULT_FALLBACK,
        ))))
    }

    pub fn for_double() -> Self {
        Self::system_default(None) // NaN cannot be JSON
    }

    pub fn for_float() -> Self {
        Self::system_default(None) // NaN cannot be JSON
    }

    pub fn for_int() -> Self {
        Self::system_default(Some(JsonValue::Number(serde_json::Number::from(
            INTEGER_DEFAULT_FALLBACK,
        ))))
    }

    pub fn for_long_array() -> Self {
        Self::system_default(None)
    }

    pub fn for_double_array() -> Self {
        Self::system_default(None)
    }

    pub fn for_float_array() -> Self {
        Self::system_default(None)
    }

    // Ergonomic constructors for common types
    pub fn long(value: i64) -> Self {
        Self::user_defined(Some(JsonValue::Number(serde_json::Number::from(value))))
    }

    pub fn int(value: i32) -> Self {
        Self::user_defined(Some(JsonValue::Number(serde_json::Number::from(value))))
    }

    pub fn double(value: f64) -> Self {
        Self::user_defined(serde_json::Number::from_f64(value).map(JsonValue::Number))
    }

    pub fn float(value: f32) -> Self {
        Self::user_defined(serde_json::Number::from_f64(value as f64).map(JsonValue::Number))
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self::user_defined(Some(JsonValue::String(value.into())))
    }

    pub fn boolean(value: bool) -> Self {
        Self::user_defined(Some(JsonValue::Bool(value)))
    }

    pub fn null() -> Self {
        Self::user_defined(Some(JsonValue::Null))
    }

    pub fn none() -> Self {
        Self::user_defined(None)
    }
}

pub fn type_name(v: &JsonValue) -> String {
    match v {
        JsonValue::Null => "null".into(),
        JsonValue::Bool(_) => "boolean".into(),
        JsonValue::Number(n) => {
            if n.is_i64() {
                "number (integer)".into()
            } else {
                "number (float)".into()
            }
        }
        JsonValue::String(_) => "string".into(),
        JsonValue::Array(_) => "array".into(),
        JsonValue::Object(_) => "object".into(),
    }
}

/// Helpers that mimic the TS/Java ValueConversion behaviors.
pub fn exact_double_to_long(d: f64) -> Result<i64, String> {
    if d.fract() == 0.0 {
        let as_i = d as i64;
        // check if conversion preserved exact value
        if (as_i as f64) == d {
            Ok(as_i)
        } else {
            Err(format!("Cannot safely convert {} into a long value", d))
        }
    } else {
        Err(format!("Cannot safely convert {} into a long value", d))
    }
}

pub fn exact_long_to_double_if_int(
    n: &serde_json::Number,
    fallback: f64,
) -> Result<f64, DefaultValueError> {
    if n.is_i64() {
        Ok(n.as_i64().unwrap() as f64)
    } else if n.is_f64() {
        Ok(fallback)
    } else {
        Err(DefaultValueError::InvalidType {
            expected: "double[] element",
            got: "number".into(),
        })
    }
}

pub fn exact_long_to_double(l: i64) -> Result<f64, String> {
    // roughly check 53-bit safe range (use same logic as TS)
    let limit = (1u128 << 53) as i128;
    let l_i128 = l as i128;
    if l_i128 >= -limit && l_i128 <= limit {
        Ok(l as f64)
    } else {
        Err(format!("Cannot safely convert {} into an double value", l))
    }
}

pub fn exact_long_to_float(l: i64) -> Result<f32, String> {
    let limit = 1i128 << 24;
    let l_i128 = l as i128;
    if l_i128 >= -limit && l_i128 <= limit {
        Ok(l as f32)
    } else {
        Err(format!("Cannot safely convert {} into a float value", l))
    }
}

pub fn not_overflowing_double_to_float(d: f64) -> Result<f32, String> {
    const FLOAT_MAX_VALUE: f64 = 3.4028234663852886e38_f64;
    if d.is_nan() {
        Ok(f32::NAN)
    } else if !(-FLOAT_MAX_VALUE..=FLOAT_MAX_VALUE).contains(&d) {
        Err(format!("Cannot safely convert {} into a float value", d))
    } else {
        Ok(d as f32)
    }
}

/// Convenience global defaults (mirrors TS static DEFAULT and fallbacks)
pub static DEFAULT: Lazy<DefaultValue> = Lazy::new(|| DefaultValue::create(None, false));

impl fmt::Display for DefaultValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DefaultValue({})",
            match &self.value {
                Some(v) => v.to_string(),
                None => "null".into(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_defaults() {
        assert_eq!(
            DefaultValue::for_long().long_value().unwrap(),
            LONG_DEFAULT_FALLBACK
        );
        assert!(DefaultValue::for_double().double_value().unwrap().is_nan());
        assert!(DefaultValue::for_float().float_value().unwrap().is_nan());
        assert_eq!(
            DefaultValue::for_int().long_value().unwrap(),
            INTEGER_DEFAULT_FALLBACK as i64
        );
    }

    #[test]
    fn test_user_defined_values() {
        assert_eq!(DefaultValue::long(42).long_value().unwrap(), 42);
        assert_eq!(DefaultValue::double(2.5).double_value().unwrap(), 2.5);
        assert_eq!(DefaultValue::int(100).long_value().unwrap(), 100);
        assert_eq!(
            DefaultValue::string("hello").string_value().unwrap(),
            "hello"
        );
        assert!(DefaultValue::boolean(true).boolean_value().unwrap());
    }

    #[test]
    fn test_of_factory() {
        let long_default = DefaultValue::of(ValueType::Long);
        assert_eq!(long_default.long_value().unwrap(), LONG_DEFAULT_FALLBACK);
        assert!(!long_default.is_user_defined());

        let double_default = DefaultValue::of(ValueType::Double);
        assert!(double_default.double_value().unwrap().is_nan());

        let string_default = DefaultValue::of(ValueType::String);
        assert_eq!(string_default.string_value().unwrap(), "");
    }

    #[test]
    fn test_array_values() {
        let long_arr = DefaultValue::user_defined(Some(serde_json::json!([1, 2, 3])));
        assert_eq!(long_arr.long_array_value().unwrap(), Some(vec![1, 2, 3]));

        let double_arr = DefaultValue::user_defined(Some(serde_json::json!([1.5, 2.5])));
        assert_eq!(
            double_arr.double_array_value().unwrap(),
            Some(vec![1.5, 2.5])
        );
    }

    #[test]
    fn test_validation() {
        let long_val = DefaultValue::long(42);
        assert!(long_val.validate_for_type(ValueType::Long).is_ok());
        assert!(long_val.validate_for_type(ValueType::Double).is_err());

        let double_val = DefaultValue::double(2.5);
        assert!(double_val.validate_for_type(ValueType::Double).is_ok());
    }

    #[test]
    fn test_null_handling() {
        let null_val = DefaultValue::null();
        assert!(null_val.get_object().is_some());

        let none_val = DefaultValue::none();
        assert!(none_val.is_null_value());
        assert_eq!(none_val.long_value().unwrap(), LONG_DEFAULT_FALLBACK);
    }
}
