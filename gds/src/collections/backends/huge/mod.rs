//! Huge Collections: Paged Arrays for Billions of Elements
//!
//! Provides paged array implementations that can handle massive datasets
//! by splitting data across multiple pages with automatic single-page
//! vs multi-page selection.

// Declare modules
pub mod huge_int_array;
pub mod huge_long_array;
pub mod huge_double_array;
pub mod huge_float_array;
pub mod huge_byte_array;
pub mod huge_short_array;
pub mod huge_boolean_array;
pub mod huge_char_array;
pub mod huge_object_array;
pub mod huge_atomic_array;

// Re-export existing HugeArray types (backward compatibility)
pub use huge_int_array::HugeIntArray;
pub use huge_long_array::HugeLongArray;
pub use huge_double_array::HugeDoubleArray;
pub use huge_float_array::HugeFloatArray;
pub use huge_byte_array::HugeByteArray;
pub use huge_short_array::HugeShortArray;
pub use huge_boolean_array::HugeBooleanArray;
pub use huge_char_array::HugeCharArray;
pub use huge_object_array::HugeObjectArray;

// Re-export atomic array types
pub use huge_atomic_array::*;

// Collections impls live in per-type files via `huge_collections!` macro.

// Huge-specific utilities
pub mod utils {
    pub use crate::collections::utils::PageUtil;
    pub use crate::collections::utils::ArrayUtil;
}

// Huge-specific macros
pub use crate::huge_collections;