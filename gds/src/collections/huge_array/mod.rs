//! HugeArray - Paged arrays supporting billions of elements
//!
//! This module provides array types that exceed JavaScript/standard array limitations
//! by using a paged memory architecture. Arrays are split into pages of manageable size,
//! with automatic selection between single-page and multi-page implementations.
//!
//! ## Architecture
//!
//! ```text
//! Logical Array: [0][1][2][3][4][5][6][7][8][9]...
//! Physical Pages:
//!   Page 0: [0,1,2,3]
//!   Page 1: [4,5,6,7]
//!   Page 2: [8,9,...]
//! ```
//!
//! ## Design Philosophy
//!
//! - **Fixed size**: Arrays don't grow/shrink after creation
//! - **Dense storage**: Optimized for dense data (use sparse variants for sparse data)
//! - **Zero defaults**: All elements initialize to 0/0.0/null
//! - **Explicit lifecycle**: Manual `release()` for memory management

pub mod huge_double_array;
pub mod huge_int_array;
pub mod huge_long_array;
pub mod huge_object_array;

pub use huge_double_array::HugeDoubleArray;
pub use huge_int_array::HugeIntArray;
pub use huge_long_array::HugeLongArray;
pub use huge_object_array::HugeObjectArray;
