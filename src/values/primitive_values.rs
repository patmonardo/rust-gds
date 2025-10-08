//! PrimitiveValues - The Mega Macro Factory
//!
//! This is the runtime type system for GDSL (Graph Data Science Language).
//! Like Zod for TypeScript, this provides runtime validation, transformation,
//! and type-safe extraction of graph property values.
//!
//! The factory provides:
//! - Runtime parsing/validation (of(), create())
//! - Type inference (GdsValue trait hierarchy)
//! - Transformation (widening conversions i32→i64, f32→f64)
//! - Data extraction (IntegralValue::long_value(), FloatingPointValue::double_value())
//! - Default value support
//! - Error handling for validation failures

use crate::values::*;
use serde_json::Value as JsonValue;
use std::sync::Arc;

/// PrimitiveValues factory - the entry point for the Second Macro System
///
/// This factory creates GdsValue instances from various input types,
/// providing runtime type validation and transformation.
pub struct PrimitiveValues;

// Generate all factory methods using the mega macro
// This macro generates the impl block with create(), of(), long_value(), etc.
generate_primitive_values_factory!();

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_null_value() {
        let val = PrimitiveValues::of(&json!(null)).unwrap();
        assert_eq!(val.value_type(), crate::types::ValueType::Unknown);
    }

    #[test]
    fn test_long_value() {
        let val = PrimitiveValues::of(&json!(42)).unwrap();
        let integral = val.as_any().downcast_ref::<DefaultLongValue>().unwrap();
        assert_eq!(integral.long_value(), 42);
    }

    #[test]
    fn test_double_value() {
        let val = PrimitiveValues::of(&json!(3.14)).unwrap();
        let fp = val
            .as_any()
            .downcast_ref::<DefaultFloatingPointValue>()
            .unwrap();
        assert!((fp.double_value() - 3.14).abs() < 0.001);
    }

    #[test]
    fn test_long_array() {
        let val = PrimitiveValues::of(&json!([1, 2, 3])).unwrap();
        let arr = val.as_any().downcast_ref::<DefaultLongArray>().unwrap();
        assert_eq!(arr.length(), 3);
        assert_eq!(arr.long_value(0), 1);
        assert_eq!(arr.long_value(2), 3);
    }

    #[test]
    fn test_double_array() {
        let val = PrimitiveValues::of(&json!([1.1, 2.2, 3.3])).unwrap();
        let arr = val.as_any().downcast_ref::<DefaultDoubleArray>().unwrap();
        assert_eq!(arr.length(), 3);
        assert!((arr.double_value(1) - 2.2).abs() < 0.001);
    }

    #[test]
    fn test_empty_array() {
        let val = PrimitiveValues::of(&json!([])).unwrap();
        let arr = val.as_any().downcast_ref::<DefaultLongArray>().unwrap();
        assert_eq!(arr.length(), 0);
    }

    #[test]
    fn test_string_parse_int() {
        let val = PrimitiveValues::of(&json!("123")).unwrap();
        let integral = val.as_any().downcast_ref::<DefaultLongValue>().unwrap();
        assert_eq!(integral.long_value(), 123);
    }

    #[test]
    fn test_string_parse_float() {
        let val = PrimitiveValues::of(&json!("3.14")).unwrap();
        let fp = val
            .as_any()
            .downcast_ref::<DefaultFloatingPointValue>()
            .unwrap();
        assert!((fp.double_value() - 3.14).abs() < 0.001);
    }

    #[test]
    fn test_null_in_array_returns_none() {
        let val = PrimitiveValues::of(&json!([1, null, 3]));
        assert!(val.is_none());
    }

    #[test]
    fn test_unsupported_type_returns_none() {
        let val = PrimitiveValues::of(&json!({"key": "value"}));
        assert!(val.is_none());
    }

    #[test]
    fn test_create_panics_on_invalid() {
        let result = PrimitiveValues::create(&json!({"key": "value"}));
        assert!(result.is_err());
    }
}
