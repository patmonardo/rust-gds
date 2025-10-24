/// Mega Macro: Generate all Default* implementations at once
///
/// This generates the complete set of primitive value implementations
/// in a single macro invocation, reducing boilerplate dramatically.
#[macro_export]
macro_rules! generate_primitive_values {
    () => {
        use std::sync::Arc;
        use $crate::values::traits::*;

        // Import the other macros
        use $crate::gds_value_scalar;
        use $crate::gds_value_binary;
        use $crate::gds_value_array_direct;
        use $crate::gds_value_array_convert;

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

        // Scalar string value
        gds_value_scalar!(
            DefaultStringValue,
            String,
            String,
            StringValue,
            string_value
        );

        // Scalar boolean value
        gds_value_scalar!(
            DefaultBooleanValue,
            bool,
            Boolean,
            BooleanValue,
            boolean_value
        );

        // Scalar binary value with MIME
        gds_value_binary!(DefaultBinaryValue);

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
