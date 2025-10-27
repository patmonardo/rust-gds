//! Cursor Accessors: Typed Property Getters
//!
//! This module provides macros for generating typed cursor accessors that
//! support the smart converter pattern with zero-copy views and efficient
//! type dispatch.
//!
//! **Pattern**: Generate typed getters for each ValueType variant with
//! automatic type conversion and validation.

// Note: These imports are used in the macro implementations but not in the module itself
// They will be available when the macros are expanded

/// Macro to generate typed cursor accessors for a given cursor type
/// 
/// This macro generates typed getter methods for each ValueType variant,
/// providing efficient type dispatch and automatic conversions.
/// 
/// # Example
/// 
/// ```rust
/// cursor_accessors! {
///     MyCursor => {
///         get_long() -> i64;
///         get_double() -> f64;
///         get_boolean() -> bool;
///         get_string() -> String;
///     }
/// }
/// ```
#[macro_export]
macro_rules! cursor_accessors {
    ($cursor_type:ty => {
        $(
            $method_name:ident() -> $return_type:ty;
        )*
    }) => {
        impl $cursor_type {
            $(
                /// Typed getter with automatic type conversion
                pub fn $method_name(&self) -> Result<$return_type, String> {
                    // This would be implemented based on the specific cursor type
                    // For now, we'll use a placeholder implementation
                    Err("Not yet implemented".to_string())
                }
            )*
        }
    };
}

/// Macro to generate smart converter implementations for cursor types
/// 
/// This macro generates the necessary implementations for cursor types
/// that need to support the smart converter pattern with GdsValue.
#[macro_export]
macro_rules! impl_cursor_smart_converter {
    ($cursor_type:ty, $value_field:ident) => {
        impl $crate::projection::codegen::property::smart_converter::RelationshipCursorExt for $cursor_type {
            fn get<T: $crate::values::traits::FromGdsValue>(&self) -> Result<T, String> {
                let value = self.$value_field;
                let gds_value = $crate::values::PrimitiveValues::long_value(value);
                T::from_gds_value(gds_value.as_ref())
            }
            
            fn get_typed<T: $crate::values::traits::FromGdsValue>(&self, expected_type: crate::types::ValueType) -> Result<T, String> {
                let value = self.$value_field;
                let gds_value = crate::values::PrimitiveValues::long_value(value);
                if gds_value.value_type() != expected_type {
                    return Err(format!("Type mismatch: expected {:?}, got {:?}", expected_type, gds_value.value_type()));
                }
                T::from_gds_value(gds_value.as_ref())
            }
            
            fn get_value(&self) -> Result<std::sync::Arc<dyn crate::values::traits::GdsValue>, String> {
                let value = self.$value_field;
                Ok(crate::values::PrimitiveValues::long_value(value))
            }
            
            fn get_value_type(&self) -> Result<crate::types::ValueType, String> {
                Ok(crate::types::ValueType::Long)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    
    use crate::values::PrimitiveValues;
    use crate::types::ValueType;

    #[test]
    fn test_gds_value_creation() {
        let long_val = PrimitiveValues::long_value(42);
        assert_eq!(long_val.value_type(), ValueType::Long);
        
        let double_val = PrimitiveValues::floating_point_value(3.14);
        assert_eq!(double_val.value_type(), ValueType::Double);
        
        let string_val = PrimitiveValues::string_value("hello".to_string());
        assert_eq!(string_val.value_type(), ValueType::String);
        
        let bool_val = PrimitiveValues::boolean_value(true);
        assert_eq!(bool_val.value_type(), ValueType::Boolean);
    }

    #[test]
    fn test_smart_conversion() {
        let long_val = PrimitiveValues::long_value(42);
        
        // Test conversion to different types
        let as_i64: i64 = crate::values::traits::FromGdsValue::from_gds_value(long_val.as_ref()).unwrap();
        assert_eq!(as_i64, 42);
        
        let as_f64: f64 = crate::values::traits::FromGdsValue::from_gds_value(long_val.as_ref()).unwrap();
        assert_eq!(as_f64, 42.0);
        
        let as_bool: bool = crate::values::traits::FromGdsValue::from_gds_value(long_val.as_ref()).unwrap();
        assert!(as_bool);
        
        let as_string: String = crate::values::traits::FromGdsValue::from_gds_value(long_val.as_ref()).unwrap();
        assert_eq!(as_string, "42");
    }
}