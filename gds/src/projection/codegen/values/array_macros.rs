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
            fn value_type(&self) -> $crate::types::ValueType {
                $crate::types::ValueType::$value_type
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
            fn value_type(&self) -> $crate::types::ValueType {
                $crate::types::ValueType::$value_type
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
