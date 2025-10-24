// Load submodules
pub mod array_equals;

// Re-export for flat API
pub use array_equals::*;

// Generate all Default* types here using the mega macro
// This generates: DefaultLongValue, DefaultFloatingPointValue,
// DefaultLongArray, DefaultIntLongArray, DefaultShortLongArray,
// DefaultByteLongArray, DefaultDoubleArray, DefaultFloatArray
use crate::generate_primitive_values;
generate_primitive_values!();
