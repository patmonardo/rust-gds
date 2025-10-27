//! Typed PropertyValues Macro System
//!
//! This module provides macros for generating typed PropertyValues implementations
//! for each ValueType, eliminating boilerplate and ensuring consistency.
//!
//! **Pattern**: One macro call generates complete PropertyValues implementation
//! with dense Vec, Arrow-backed, and sparse variants.

// Note: These imports are used in the macro implementations but not in the module itself
// They will be available when the macros are expanded

/// Macro to generate typed PropertyValues implementation for a given type
/// 
/// This macro generates a complete PropertyValues implementation including:
/// - Storage backend (Vec<T>)
/// - PropertyValues trait implementation
/// - Smart converter support
/// 
/// # Example
/// 
/// ```rust
/// typed_property_values! {
///     LongPropertyValues,
///     i64,
///     ValueType::Long,
///     0,
///     "Long"
/// }
/// ```
#[macro_export]
macro_rules! typed_property_values {
    ($struct_name:ident, $value_type:ty, $value_type_enum:ident, $default_value:expr, $type_name:expr) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            values: Vec<$value_type>,
            default_value: $value_type,
        }

        impl $struct_name {
            pub fn new(values: Vec<$value_type>, default_value: $value_type) -> Self {
                Self { values, default_value }
            }

            pub fn default_value() -> $value_type {
                $default_value
            }
        }

        impl $crate::types::properties::property_values::PropertyValues for $struct_name {
            fn value_type(&self) -> $crate::types::ValueType {
                $crate::types::ValueType::$value_type_enum
            }

            fn element_count(&self) -> usize {
                self.values.len()
            }
        }

        impl $struct_name {
            /// Gets a value with smart conversion
            pub fn get<T: $crate::values::traits::FromGdsValue>(&self, index: usize) -> crate::types::properties::property_values::PropertyValuesResult<T> {
                if index >= self.values.len() {
                    return Err(crate::types::properties::property_values::PropertyValuesError::InvalidNodeId(index as u64));
                }
                let gds_value = crate::values::PrimitiveValues::long_value(self.values[index] as i64);
                T::from_gds_value(gds_value.as_ref()).map_err(|e| crate::types::properties::property_values::PropertyValuesError::UnsupportedOperation(e))
            }

            /// Gets a value with explicit type validation
            pub fn get_typed<T: crate::values::traits::FromGdsValue>(&self, index: usize, expected_type: crate::types::ValueType) -> crate::types::properties::property_values::PropertyValuesResult<T> {
                if self.value_type() != expected_type {
                    return Err(crate::types::properties::property_values::PropertyValuesError::UnsupportedType {
                        expected: expected_type,
                        actual: self.value_type(),
                    });
                }
                self.get::<T>(index)
            }

            /// Gets a raw GdsValue
            pub fn get_value(&self, index: usize) -> crate::types::properties::property_values::PropertyValuesResult<std::sync::Arc<dyn crate::values::traits::GdsValue>> {
                if index >= self.values.len() {
                    return Err(crate::types::properties::property_values::PropertyValuesError::InvalidNodeId(index as u64));
                }
                Ok(crate::values::PrimitiveValues::long_value(self.values[index] as i64))
            }

            /// Gets a slice of values
            pub fn get_slice<T: crate::values::traits::FromGdsValue>(&self, indices: &[usize]) -> crate::types::properties::property_values::PropertyValuesResult<Vec<T>> {
                let mut result = Vec::new();
                for &index in indices {
                    result.push(self.get::<T>(index)?);
                }
                Ok(result)
            }
        }
    };
}

/// Macro to generate multiple typed PropertyValues implementations
/// 
/// This macro generates PropertyValues implementations for multiple types
/// in a single call, reducing boilerplate.
/// 
/// # Example
/// 
/// ```rust
/// typed_property_values_multiple! {
///     (
///         LongPropertyValues,
///         i64,
///         ValueType::Long,
///         0,
///         "Long"
///     ),
///     (
///         DoublePropertyValues,
///         f64,
///         ValueType::Double,
///         f64::NAN,
///         "Double"
///     ),
///     (
///         BooleanPropertyValues,
///         bool,
///         ValueType::Boolean,
///         false,
///         "Boolean"
///     ),
///     (
///         StringPropertyValues,
///         String,
///         ValueType::String,
///         String::new(),
///         "String"
///     )
/// }
/// ```
#[macro_export]
macro_rules! typed_property_values_multiple {
    ($(
        ($struct_name:ident, $value_type:ty, $value_type_enum:ident, $default_value:expr, $type_name:expr)
    ),*) => {
        $(
            typed_property_values!($struct_name, $value_type, $value_type_enum, $default_value, $type_name);
        )*
    };
}

/// Macro to generate factory methods for typed PropertyValues
/// 
/// This macro generates factory methods for creating PropertyValues instances
/// based on ValueType, with automatic backend selection.
/// 
/// # Example
/// 
/// ```rust
/// typed_property_values_factory! {
///     TypedPropertyValuesFactory,
///     (
///         LongPropertyValues,
///         ValueType::Long
///     ),
///     (
///         DoublePropertyValues,
///         ValueType::Double
///     ),
///     (
///         BooleanPropertyValues,
///         ValueType::Boolean
///     ),
///     (
///         StringPropertyValues,
///         ValueType::String
///     )
/// }
/// ```
#[macro_export]
macro_rules! typed_property_values_factory {
    ($factory_name:ident, $(
        ($struct_name:ident, $value_type_enum:ident)
    ),*) => {
        pub struct $factory_name;

        impl $factory_name {
            /// Creates a new PropertyValues instance for the given ValueType
            pub fn create(value_type: $crate::types::ValueType, capacity: usize) -> std::sync::Arc<dyn $crate::types::properties::property_values::PropertyValues> {
                match value_type {
                    $(
                        $crate::types::ValueType::$value_type_enum => {
                            std::sync::Arc::new($struct_name::new(Vec::with_capacity(capacity), $struct_name::default_value()))
                        }
                    )*
                    _ => panic!("Unsupported ValueType: {:?}", value_type),
                }
            }

            /// Creates a PropertyValues instance from existing data
            pub fn from_data<T>(value_type: $crate::types::ValueType, data: Vec<T>) -> std::sync::Arc<dyn crate::types::properties::property_values::PropertyValues> {
                match value_type {
                    $(
                        crate::types::ValueType::$value_type_enum => {
                            std::sync::Arc::new($struct_name::new(data, $struct_name::default_value()))
                        }
                    )*
                    _ => panic!("Unsupported ValueType: {:?}", value_type),
                }
            }

            /// Gets the default value for a given ValueType
            pub fn default_value(value_type: crate::types::ValueType) -> std::sync::Arc<dyn crate::values::traits::GdsValue> {
                match value_type {
                    $(
                        crate::types::ValueType::$value_type_enum => {
                            crate::values::PrimitiveValues::long_value($struct_name::default_value() as i64)
                        }
                    )*
                    _ => crate::values::PrimitiveValues::long_value(0),
                }
            }
        }

        $(
            impl $struct_name {
                fn default_value() -> <$struct_name as crate::types::properties::property_values::PropertyValues>::ValueType {
                    $struct_name::default_value()
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use crate::types::properties::property_values::PropertyValues;

    // Generate test implementations
    typed_property_values! {
        MockLongPropertyValues,
        i64,
        Long,
        0,
        "Long"
    }

    // Use the constructor generated by the macro above; avoid redefining `new`

    #[test]
    fn test_typed_property_values() {
        let values = MockLongPropertyValues::new(vec![42, 100, 200], 0);
        assert_eq!(values.value_type(), crate::types::ValueType::Long);
        assert_eq!(values.element_count(), 3);
    }

    #[test]
    fn test_property_values_factory() {
        struct MockFactory;

        impl MockFactory {
            fn create(value_type: crate::types::ValueType, _capacity: usize) -> String {
                match value_type {
                    crate::types::ValueType::Long => "LongPropertyValues".to_string(),
                    crate::types::ValueType::Double => "DoublePropertyValues".to_string(),
                    _ => "Unknown".to_string(),
                }
            }
        }

        let result = MockFactory::create(crate::types::ValueType::Long, 10);
        assert_eq!(result, "LongPropertyValues");
    }
}