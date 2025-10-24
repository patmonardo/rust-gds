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
                        // Try to parse as number first
                        if let Ok(i) = s.parse::<i64>() {
                            Some(Self::long_value(i))
                        } else if let Ok(f) = s.parse::<f64>() {
                            Some(Self::floating_point_value(f))
                        } else if s == "true" || s == "false" {
                            Some(Self::boolean_value(s == "true"))
                        } else {
                            // Treat as string
                            Some(Self::string_value(s.clone()))
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

            pub fn string_value(value: String) -> Arc<dyn GdsValue> {
                Arc::new(DefaultStringValue(value))
            }

            pub fn boolean_value(value: bool) -> Arc<dyn GdsValue> {
                Arc::new(DefaultBooleanValue(value))
            }

            pub fn binary_value(data: Vec<u8>, mime_type: Option<String>) -> Arc<dyn GdsValue> {
                Arc::new(DefaultBinaryValue::new(data, mime_type))
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
