//! Monadic PropertyValues: Direct Collections Integration
//!
//! Simple PropertyValues implementations that work directly with Collections,
//! demonstrating the Collections First architecture.
//!
//! This module uses macros to generate PropertyValues types for all primitive
//! types, arrays, and maps - proving Collections can be the universal backend.

use crate::collections::traits::Collections;
use crate::types::properties::PropertyValues;
use crate::types::ValueType;
use std::fmt::Debug;

// Import the macro
use super::macros::monadic_property_values;

// ============================================================================
// PRIMITIVES: 9 basic types
// ============================================================================

monadic_property_values!(MonadicLongPropertyValues => i64, ValueType::Long);
monadic_property_values!(MonadicDoublePropertyValues => f64, ValueType::Double);
monadic_property_values!(MonadicIntPropertyValues => i32, ValueType::Int);
monadic_property_values!(MonadicFloatPropertyValues => f32, ValueType::Float);
monadic_property_values!(MonadicShortPropertyValues => i16, ValueType::Short);
monadic_property_values!(MonadicBytePropertyValues => i8, ValueType::Byte);
monadic_property_values!(MonadicBooleanPropertyValues => bool, ValueType::Boolean);
monadic_property_values!(MonadicCharPropertyValues => char, ValueType::Char);
monadic_property_values!(MonadicStringPropertyValues => String, ValueType::String);

// ============================================================================
// ARRAYS: Collections of primitives
// ============================================================================

monadic_property_values!(MonadicLongArrayPropertyValues => Vec<i64>, ValueType::LongArray);
monadic_property_values!(MonadicDoubleArrayPropertyValues => Vec<f64>, ValueType::DoubleArray);
monadic_property_values!(MonadicIntArrayPropertyValues => Vec<i32>, ValueType::IntArray);
monadic_property_values!(MonadicFloatArrayPropertyValues => Vec<f32>, ValueType::FloatArray);
monadic_property_values!(MonadicShortArrayPropertyValues => Vec<i16>, ValueType::ShortArray);
monadic_property_values!(MonadicByteArrayPropertyValues => Vec<i8>, ValueType::ByteArray);
monadic_property_values!(MonadicBooleanArrayPropertyValues => Vec<bool>, ValueType::BooleanArray);
monadic_property_values!(MonadicCharArrayPropertyValues => Vec<char>, ValueType::CharArray);
monadic_property_values!(MonadicStringArrayPropertyValues => Vec<String>, ValueType::StringArray);

// ============================================================================
// TESTS: Verify generated types work with Vec and Huge backends
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::backends::vec::{VecLong, VecDouble, VecInt, VecFloat};
    use crate::collections::backends::huge::{HugeLongArray, HugeDoubleArray};

    // ========================================================================
    // Primitive Tests - Vec Backend
    // ========================================================================

    #[test]
    fn monadic_long_property_values_with_vec() {
        let vec_long = VecLong::from(vec![1, 2, 3, 4, 5]);
        let property_values = MonadicLongPropertyValues::new(vec_long, 0);

        assert_eq!(property_values.value_type(), ValueType::Long);
        assert_eq!(property_values.element_count(), 5);
    }

    #[test]
    fn monadic_double_property_values_with_vec() {
        let vec_double = VecDouble::from(vec![1.5, 2.5, 3.5]);
        let property_values = MonadicDoublePropertyValues::new(vec_double, 0.0);

        assert_eq!(property_values.value_type(), ValueType::Double);
        assert_eq!(property_values.element_count(), 3);
    }

    #[test]
    fn monadic_int_property_values_with_vec() {
        let vec_int = VecInt::from(vec![10, 20, 30]);
        let property_values = MonadicIntPropertyValues::new(vec_int, 0);

        assert_eq!(property_values.value_type(), ValueType::Int);
        assert_eq!(property_values.element_count(), 3);
    }

    #[test]
    fn monadic_float_property_values_with_vec() {
        let vec_float = VecFloat::from(vec![1.0, 2.0, 3.0]);
        let property_values = MonadicFloatPropertyValues::new(vec_float, 0.0);

        assert_eq!(property_values.value_type(), ValueType::Float);
        assert_eq!(property_values.element_count(), 3);
    }

    // ========================================================================
    // Primitive Tests - Huge Backend
    // ========================================================================

    #[test]
    fn monadic_long_property_values_with_huge() {
        let mut huge = HugeLongArray::new(3);
        huge.set(0, 10);
        huge.set(1, 20);
        huge.set(2, 30);
        
        let property_values = MonadicLongPropertyValues::new(huge, 0);

        assert_eq!(property_values.value_type(), ValueType::Long);
        assert_eq!(property_values.element_count(), 3);
        assert_eq!(property_values.values().get(0), 10);
        assert_eq!(property_values.values().get(1), 20);
        assert_eq!(property_values.values().get(2), 30);
    }

    #[test]
    fn monadic_double_property_values_with_huge() {
        let mut huge = HugeDoubleArray::new(2);
        huge.set(0, 1.5);
        huge.set(1, 2.5);
        
        let property_values = MonadicDoublePropertyValues::new(huge, 0.0);

        assert_eq!(property_values.value_type(), ValueType::Double);
        assert_eq!(property_values.element_count(), 2);
        assert_eq!(property_values.values().get(0), 1.5);
        assert_eq!(property_values.values().get(1), 2.5);
    }

    // TODO: Uncomment when HugeIntArray implements Collections<i32>
    // #[test]
    // fn monadic_int_property_values_with_huge() {
    //     let mut huge = HugeIntArray::new(4);
    //     huge.set(0, 100);
    //     huge.set(1, 200);
    //     huge.set(2, 300);
    //     huge.set(3, 400);
    //     
    //     let property_values = MonadicIntPropertyValues::new(huge, 0);
    //
    //     assert_eq!(property_values.value_type(), ValueType::Int);
    //     assert_eq!(property_values.element_count(), 4);
    //     assert_eq!(property_values.values().get(0), 100);
    //     assert_eq!(property_values.values().get(3), 400);
    // }

    // TODO: Uncomment when HugeFloatArray implements Collections<f32>
    // #[test]
    // fn monadic_float_property_values_with_huge() {
    //     let mut huge = HugeFloatArray::new(2);
    //     huge.set(0, 10.5);
    //     huge.set(1, 20.5);
    //     
    //     let property_values = MonadicFloatPropertyValues::new(huge, 0.0);
    //
    //     assert_eq!(property_values.value_type(), ValueType::Float);
    //     assert_eq!(property_values.element_count(), 2);
    //     assert_eq!(property_values.values().get(0), 10.5);
    //     assert_eq!(property_values.values().get(1), 20.5);
    // }

    // ========================================================================
    // Array Tests - TODO: Need to implement Collections for HugeObjectArray
    // ========================================================================

    // TODO: Uncomment when HugeObjectArray implements Collections<Vec<T>>
    //
    // #[test]
    // fn monadic_long_array_property_values_with_huge_object() {
    //     use crate::collections::backends::huge::HugeObjectArray;
    //     
    //     let mut huge = HugeObjectArray::new(2);
    //     huge.set(0, vec![1i64, 2, 3]);
    //     huge.set(1, vec![4i64, 5, 6]);
    //     
    //     let property_values = MonadicLongArrayPropertyValues::new(huge, vec![]);
    //
    //     assert_eq!(property_values.value_type(), ValueType::LongArray);
    //     assert_eq!(property_values.element_count(), 2);
    // }
    //
    // #[test]
    // fn monadic_double_array_property_values_with_huge_object() {
    //     use crate::collections::backends::huge::HugeObjectArray;
    //     
    //     let mut huge = HugeObjectArray::new(2);
    //     huge.set(0, vec![1.5, 2.5]);
    //     huge.set(1, vec![3.5, 4.5, 5.5]);
    //     
    //     let property_values = MonadicDoubleArrayPropertyValues::new(huge, vec![]);
    //
    //     assert_eq!(property_values.value_type(), ValueType::DoubleArray);
    //     assert_eq!(property_values.element_count(), 2);
    // }
}
