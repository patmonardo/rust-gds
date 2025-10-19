/// Huge sparse arrays with memory-efficient paged storage.
///
/// This module provides sparse array implementations that only allocate
/// pages where values actually exist, using HashMap-based page tracking.
///
/// Perfect for sparse node/relationship mappings in massive graphs.
pub mod huge_sparse_double_array;
pub mod huge_sparse_double_array_array;
pub mod huge_sparse_long_array;
pub mod huge_sparse_long_array_array;

pub use huge_sparse_double_array::{HugeSparseDoubleArray, HugeSparseDoubleArrayBuilder};
pub use huge_sparse_double_array_array::{
    HugeSparseDoubleArrayArray, HugeSparseDoubleArrayArrayBuilder,
};
pub use huge_sparse_long_array::{HugeSparseLongArray, HugeSparseLongArrayBuilder};
pub use huge_sparse_long_array_array::{HugeSparseLongArrayArray, HugeSparseLongArrayArrayBuilder};
