use crate::types::property::ValueType;
use serde::{Deserialize, Serialize};

/// Default value for a property when no value is present.
/// Mirrors the TypeScript DefaultValue abstraction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DefaultValue {
    /// Null/absent value
    Null,
    /// Long integer value
    Long(i64),
    /// Double floating point value
    Double(f64),
    /// String value
    String(String),
    /// Boolean value
    Bool(bool),
}

impl DefaultValue {
    /// Creates a default value appropriate for the given value type.
    pub fn of(value_type: ValueType) -> Self {
        match value_type {
            ValueType::Long => DefaultValue::Long(0),
            ValueType::Double => DefaultValue::Double(0.0),
            ValueType::String => DefaultValue::String(String::new()),
            ValueType::Boolean => DefaultValue::Bool(false),
            ValueType::DoubleArray | ValueType::FloatArray | ValueType::LongArray => {
                DefaultValue::Null
            }
        }
    }

    /// Creates a null default value.
    pub fn null() -> Self {
        DefaultValue::Null
    }

    /// Returns the long value, or 0 if not a long.
    pub fn long_value(&self) -> i64 {
        match self {
            DefaultValue::Long(v) => *v,
            _ => 0,
        }
    }

    /// Checks if this default value equals another.
    pub fn equals(&self, other: &DefaultValue) -> bool {
        self == other
    }
}

impl std::fmt::Display for DefaultValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefaultValue::Null => write!(f, "null"),
            DefaultValue::Long(v) => write!(f, "{}", v),
            DefaultValue::Double(v) => write!(f, "{}", v),
            DefaultValue::String(v) => write!(f, "\"{}\"", v),
            DefaultValue::Bool(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_value_of() {
        assert_eq!(DefaultValue::of(ValueType::Long), DefaultValue::Long(0));
        assert_eq!(
            DefaultValue::of(ValueType::Double),
            DefaultValue::Double(0.0)
        );
        assert_eq!(
            DefaultValue::of(ValueType::String),
            DefaultValue::String(String::new())
        );
        assert_eq!(
            DefaultValue::of(ValueType::Boolean),
            DefaultValue::Bool(false)
        );
    }

    #[test]
    fn test_equals() {
        let dv1 = DefaultValue::Long(42);
        let dv2 = DefaultValue::Long(42);
        let dv3 = DefaultValue::Long(0);

        assert!(dv1.equals(&dv2));
        assert!(!dv1.equals(&dv3));
    }
}
