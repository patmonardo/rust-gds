//! ValueType Master Table: Canonical ValueType → Rust Type Mapping
//!
//! This module defines the authoritative mapping from ValueType enum variants
//! to their corresponding Rust types, categories, and default values.
//!
//! The ValueType table is the single source of truth for all property value
//! generation, ensuring consistency across Node, Relationship, and Graph
//! property implementations.

/// The canonical ValueType → Rust Type mapping table.
///
/// This macro defines the complete taxonomy of all supported ValueTypes
/// and their corresponding Rust types, categorized by kind.
///
/// # Format
///
/// Each entry has the format:
/// ```text
/// $callback!(ValueType, RustType, Category, DefaultValue);
/// ```
///
/// # Categories
///
/// - `IntegralScalar`: Signed integer primitives (i8, i16, i32, i64)
/// - `FloatingPointScalar`: Floating point primitives (f32, f64)
/// - `OtherScalar`: Other scalar types (bool, char, String)
/// - `IntegralArray`: Arrays of signed integers
/// - `FloatingPointArray`: Arrays of floating point numbers
/// - `OtherArray`: Arrays of other types
///
/// # Usage
///
/// ```rust
/// // Define a callback macro that receives table entries
/// macro_rules! my_callback {
///     ($vt:ident, $rt:ty, $cat:ident, $dv:expr) => {
///         // Process each entry...
///     };
/// }
///
/// // Invoke the table with your callback
/// value_type_table!(my_callback);
/// ```
#[macro_export]
macro_rules! value_type_table {
    ($callback:ident) => {
        // ===== 5 Integral Scalars =====
        $callback!(Byte, i8, IntegralScalar, 0i8);
        $callback!(Short, i16, IntegralScalar, 0i16);
        $callback!(Int, i32, IntegralScalar, 0i32);
        $callback!(Long, i64, IntegralScalar, 0i64);
        $callback!(BigInt, i128, IntegralScalar, 0i128);
        
        // ===== 2 FloatingPoint Scalars =====
        $callback!(Float, f32, FloatingPointScalar, 0.0f32);
        $callback!(Double, f64, FloatingPointScalar, 0.0f64);
        
        // ===== 3 Other Scalars =====
        $callback!(Boolean, bool, OtherScalar, false);
        $callback!(Char, char, OtherScalar, '\0');
        $callback!(String, String, OtherScalar, String::new());
        
        // ===== 5 Integral Arrays =====
        $callback!(ByteArray, Option<Vec<i8>>, IntegralArray, None);
        $callback!(ShortArray, Option<Vec<i16>>, IntegralArray, None);
        $callback!(IntArray, Option<Vec<i32>>, IntegralArray, None);
        $callback!(LongArray, Option<Vec<i64>>, IntegralArray, None);
        $callback!(BigIntArray, Option<Vec<i128>>, IntegralArray, None);
        
        // ===== 2 FloatingPoint Arrays =====
        $callback!(FloatArray, Option<Vec<f32>>, FloatingPointArray, None);
        $callback!(DoubleArray, Option<Vec<f64>>, FloatingPointArray, None);
        
        // ===== 3 Other Arrays =====
        $callback!(BooleanArray, Option<Vec<bool>>, OtherArray, None);
        $callback!(CharArray, Option<Vec<char>>, OtherArray, None);
        $callback!(StringArray, Option<Vec<String>>, OtherArray, None);
    };
}


#[cfg(test)]
mod tests {
    /// Test that the value_type_table macro expands correctly
    #[test]
    fn test_value_type_table_expansion() {
        // Test that the macro expands without errors
        // by defining a simple callback that counts entries
        macro_rules! count_test {
            ($vt:ident, $rt:ty, $cat:ident, $dv:expr) => {
                const _: () = ();
            };
        }
        
        // Invoke the table - if it compiles, the test passes
        crate::value_type_table!(count_test);
    }
}

