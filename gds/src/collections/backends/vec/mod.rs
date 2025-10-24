//! Vec Collections: Enhanced Standard Library Vectors
//!
//! Provides enhanced Vec implementations with Collections API,
//! including aggregation methods, nullability support, and
//! Arrow compatibility.

use crate::collections::traits::Collections;
use std::iter::Sum;

// Enhanced Vec implementation
pub mod enhanced_vec;
pub use enhanced_vec::EnhancedVec;

// Generated Vec types (by macro)
pub mod vec_int;
pub mod vec_long;
pub mod vec_double;
pub mod vec_float;
pub mod vec_byte;
pub mod vec_short;
pub mod vec_boolean;
pub mod vec_char;

// Re-export generated types
pub use vec_int::VecInt;
pub use vec_long::VecLong;
pub use vec_double::VecDouble;
pub use vec_float::VecFloat;
pub use vec_byte::VecByte;
pub use vec_short::VecShort;
pub use vec_boolean::VecBoolean;
pub use vec_char::VecChar;

// Vec-specific utilities
pub mod utils {
    // Vec-specific helper functions
}

// Vec-specific macros
pub use crate::vec_collections;
