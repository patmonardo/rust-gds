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
                self.0.clone()
            }
        }

        impl $crate::values::traits::GdsValue for $name {
            fn value_type(&self) -> $crate::types::ValueType {
                $crate::types::ValueType::$value_type
            }
            fn as_object(&self) -> serde_json::Value {
                serde_json::Value::from(self.0.clone())
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}
