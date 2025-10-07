/// Generate a GdsValue implementation for an array type with direct storage (no conversion).
///
/// # Parameters
/// - `$name`: The struct name (e.g., DefaultLongArray)
/// - `$storage_ty`: The storage type (e.g., i64)
/// - `$value_type`: The ValueType variant (e.g., LongArray)
/// - `$array_trait`: The array trait to implement (e.g., IntegralArray)
/// - `$accessor_method`: The method name for single element access (e.g., long_value)
/// - `$array_method`: The method name for array access (e.g., long_array_value)
///
#[macro_export]
macro_rules! gds_value_array_direct {
    ($name:ident, $storage_ty:ty, $value_type:ident, $array_trait:ident, $accessor_method:ident, $array_method:ident) => {
        #[derive(Clone)]
        pub struct $name {
            data: Arc<Vec<$storage_ty>>,
        }

        impl $name {
            pub fn new(data: Vec<$storage_ty>) -> Self {
                Self {
                    data: Arc::new(data),
                }
            }
        }

        impl $array_trait for $name {
            fn $accessor_method(&self, idx: usize) -> $storage_ty {
                self.data[idx]
            }
            fn $array_method(&self) -> Vec<$storage_ty> {
                (*self.data).clone()
            }
        }

        impl $crate::values::traits::Array for $name {
            fn length(&self) -> usize {
                self.data.len()
            }
        }

        impl $crate::values::traits::GdsValue for $name {
            fn value_type(&self) -> $crate::types::value_type::ValueType {
                $crate::types::value_type::ValueType::$value_type
            }
            fn as_object(&self) -> serde_json::Value {
                serde_json::Value::from(self.$array_method())
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}

/// Generate a GdsValue implementation for an array type WITH type conversion.
///
/// # Parameters
/// - `$name`: The struct name (e.g., DefaultIntLongArray)
/// - `$storage_ty`: The storage type (e.g., i32)
/// - `$accessor_ty`: The accessor type (e.g., i64)
/// - `$value_type`: The ValueType variant (e.g., LongArray)
/// - `$array_trait`: The array trait to implement (e.g., IntegralArray)
/// - `$accessor_method`: The method name for single element access (e.g., long_value)
/// - `$array_method`: The method name for array access (e.g., long_array_value)
///
#[macro_export]
macro_rules! gds_value_array_convert {
    ($name:ident, $storage_ty:ty, $accessor_ty:ty, $value_type:ident, $array_trait:ident, $accessor_method:ident, $array_method:ident) => {
        #[derive(Clone)]
        pub struct $name {
            data: Arc<Vec<$storage_ty>>,
        }

        impl $name {
            pub fn new(data: Vec<$storage_ty>) -> Self {
                Self {
                    data: Arc::new(data),
                }
            }
        }

        impl $array_trait for $name {
            fn $accessor_method(&self, idx: usize) -> $accessor_ty {
                self.data[idx] as $accessor_ty
            }
            fn $array_method(&self) -> Vec<$accessor_ty> {
                self.data.iter().map(|v| *v as $accessor_ty).collect()
            }
        }

        impl $crate::values::traits::Array for $name {
            fn length(&self) -> usize {
                self.data.len()
            }
        }

        impl $crate::values::traits::GdsValue for $name {
            fn value_type(&self) -> $crate::types::value_type::ValueType {
                $crate::types::value_type::ValueType::$value_type
            }
            fn as_object(&self) -> serde_json::Value {
                serde_json::Value::from(self.$array_method())
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}

/// Generate a GdsValue implementation for a scalar value type.
///
/// # Parameters
/// - `$name`: The struct name (e.g., DefaultLongValue)
/// - `$storage_ty`: The storage type (e.g., i64)
/// - `$value_type`: The ValueType variant (e.g., Long)
/// - `$scalar_trait`: The scalar trait to implement (e.g., IntegralValue)
/// - `$accessor_method`: The method name for value access (e.g., long_value)
#[macro_export]
macro_rules! gds_value_scalar {
    ($name:ident, $storage_ty:ty, $value_type:ident, $scalar_trait:ident, $accessor_method:ident) => {
        #[derive(Clone)]
        pub struct $name(pub $storage_ty);

        impl $scalar_trait for $name {
            fn $accessor_method(&self) -> $storage_ty {
                self.0
            }
        }

        impl $crate::values::traits::GdsValue for $name {
            fn value_type(&self) -> $crate::types::value_type::ValueType {
                $crate::types::value_type::ValueType::$value_type
            }
            fn as_object(&self) -> serde_json::Value {
                serde_json::Value::from(self.0)
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}

/// Mega Macro: Generate all Default* implementations at once
///
/// This generates the complete set of primitive value implementations
/// in a single macro invocation, reducing boilerplate dramatically.
#[macro_export]
macro_rules! generate_primitive_values {
    () => {
        use std::sync::Arc;
        use $crate::values::traits::*;

        // Scalar integral value
        gds_value_scalar!(DefaultLongValue, i64, Long, IntegralValue, long_value);

        // Scalar floating point value
        gds_value_scalar!(
            DefaultFloatingPointValue,
            f64,
            Double,
            FloatingPointValue,
            double_value
        );

        // Long arrays (direct storage)
        gds_value_array_direct!(
            DefaultLongArray,
            i64,
            LongArray,
            IntegralArray,
            long_value,
            long_array_value
        );

        // Long arrays (with conversion)
        gds_value_array_convert!(
            DefaultIntLongArray,
            i32,
            i64,
            LongArray,
            IntegralArray,
            long_value,
            long_array_value
        );
        gds_value_array_convert!(
            DefaultShortLongArray,
            i16,
            i64,
            LongArray,
            IntegralArray,
            long_value,
            long_array_value
        );
        gds_value_array_convert!(
            DefaultByteLongArray,
            u8,
            i64,
            LongArray,
            IntegralArray,
            long_value,
            long_array_value
        );

        // Double arrays (direct storage)
        gds_value_array_direct!(
            DefaultDoubleArray,
            f64,
            DoubleArray,
            FloatingPointArray,
            double_value,
            double_array_value
        );

        // Float arrays (with conversion)
        gds_value_array_convert!(
            DefaultFloatArray,
            f32,
            f64,
            FloatArray,
            FloatingPointArray,
            double_value,
            double_array_value
        );

        // Special case: ByteLongArray needs extra method
        impl DefaultByteLongArray {
            pub fn as_bytes(&self) -> &[u8] {
                &self.data
            }
        }
    };
}

/// Generate PrimitiveValues factory methods
///
/// This macro generates the factory methods for the PrimitiveValues struct,
/// including create(), of(), and all the specific factory methods.
#[macro_export]
macro_rules! generate_primitive_values_factory {
    () => {
        impl PrimitiveValues {
            /// Create a GdsValue from any supported Rust type
            /// Panics if the value cannot be converted
            pub fn create(input: &JsonValue) -> Result<Arc<dyn GdsValue>, String> {
                Self::of(input).ok_or_else(|| format!("Unsupported value: {}", input))
            }

            /// Try to create a GdsValue from any supported Rust type
            /// Returns None if the value cannot be converted (non-panicking)
            pub fn of(input: &JsonValue) -> Option<Arc<dyn GdsValue>> {
                use JsonValue::*;
                match input {
                    Null => Some(Arc::new(GdsNoValue)),
                    Number(n) => {
                        if let Some(i) = n.as_i64() {
                            Some(Self::long_value(i))
                        } else if let Some(f) = n.as_f64() {
                            Some(Self::floating_point_value(f))
                        } else {
                            None
                        }
                    }
                    String(s) => {
                        // Try to parse as number
                        if let Ok(i) = s.parse::<i64>() {
                            Some(Self::long_value(i))
                        } else if let Ok(f) = s.parse::<f64>() {
                            Some(Self::floating_point_value(f))
                        } else {
                            // TODO: String values not yet implemented
                            None
                        }
                    }
                    Array(arr) => Self::array_value(arr),
                    _ => None,
                }
            }

            /// Handle array conversion
            fn array_value(arr: &[JsonValue]) -> Option<Arc<dyn GdsValue>> {
                if arr.is_empty() {
                    // Empty array -> empty long array by convention
                    return Some(Self::long_array(Vec::new()));
                }

                // Check for null elements
                if arr.iter().any(|v| v.is_null()) {
                    return None; // Null elements not allowed
                }

                if arr.iter().all(|v| v.is_number()) {
                    // Numeric array - choose integer or float based on content
                    if arr.iter().all(|v| v.as_i64().is_some()) {
                        // All integers -> LongArray
                        let values: Vec<i64> = arr.iter().map(|v| v.as_i64().unwrap()).collect();
                        Some(Self::long_array(values))
                    } else {
                        // Has floats -> DoubleArray
                        let values: Vec<f64> = arr.iter().map(|v| v.as_f64().unwrap()).collect();
                        Some(Self::double_array(values))
                    }
                } else {
                    // Mixed types or non-numeric - not yet supported
                    None
                }
            }

            // Factory methods for each type
            pub fn long_value(value: i64) -> Arc<dyn GdsValue> {
                Arc::new(DefaultLongValue(value))
            }

            pub fn floating_point_value(value: f64) -> Arc<dyn GdsValue> {
                Arc::new(DefaultFloatingPointValue(value))
            }

            pub fn double_array(data: Vec<f64>) -> Arc<dyn GdsValue> {
                Arc::new(DefaultDoubleArray::new(data))
            }

            pub fn float_array(data: Vec<f32>) -> Arc<dyn GdsValue> {
                Arc::new(DefaultFloatArray::new(data))
            }

            pub fn long_array(data: Vec<i64>) -> Arc<dyn GdsValue> {
                Arc::new(DefaultLongArray::new(data))
            }

            pub fn int_array(data: Vec<i32>) -> Arc<dyn GdsValue> {
                Arc::new(DefaultIntLongArray::new(data))
            }

            pub fn short_array(data: Vec<i16>) -> Arc<dyn GdsValue> {
                Arc::new(DefaultShortLongArray::new(data))
            }

            pub fn byte_array(data: Vec<u8>) -> Arc<dyn GdsValue> {
                Arc::new(DefaultByteLongArray::new(data))
            }

            // Constants
            pub const NO_VALUE: &'static GdsNoValue = &GdsNoValue;

            pub fn empty_long_array() -> Arc<dyn GdsValue> {
                Self::long_array(Vec::new())
            }
        }
    };
}
