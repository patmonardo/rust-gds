//! High-performance collection data structures for graph analytics
//!
//! This module provides specialized collection utilities optimized for large-scale
//! graph processing operations.

pub mod array_util;
pub mod bit_set;
pub mod cursor;
pub mod huge_array;
pub mod huge_atomic_array;
pub mod huge_sparse_array;
pub mod huge_sparse_list;
pub mod indirect_comparator;
pub mod long_multiset;
pub mod page_util;
pub mod primitive;

// Re-export commonly used types
pub use array_util::ArrayUtil;
pub use bit_set::BitSet;
pub use cursor::{
    init_cursor, init_cursor_range, HugeCursor, HugeCursorSupport, PagedCursor, SinglePageCursor,
};
pub use huge_array::{HugeDoubleArray, HugeIntArray, HugeLongArray, HugeObjectArray};
pub use huge_atomic_array::{HugeAtomicDoubleArray, HugeAtomicLongArray};
// Re-exported from core::utils::paged for backward compatibility
pub use crate::core::utils::paged::{HugeAtomicBitSet, HugeAtomicGrowingBitSet};
pub use huge_sparse_array::{
    HugeSparseDoubleArray, HugeSparseDoubleArrayArray, HugeSparseDoubleArrayArrayBuilder,
    HugeSparseDoubleArrayBuilder, HugeSparseLongArray, HugeSparseLongArrayArray,
    HugeSparseLongArrayArrayBuilder, HugeSparseLongArrayBuilder,
};
pub use huge_sparse_list::{
    HugeSparseDoubleArrayList, HugeSparseDoubleList, HugeSparseLongArrayArrayList,
    HugeSparseLongArrayList, HugeSparseLongList, LongByteArrayArrayConsumer, LongByteArrayConsumer,
    LongDoubleArrayArrayConsumer, LongDoubleArrayConsumer, LongDoubleConsumer,
    LongFloatArrayConsumer, LongIntConsumer, LongLongArrayArrayConsumer, LongLongArrayConsumer,
    LongLongConsumer,
};
pub use indirect_comparator::IndirectComparator;
pub use long_multiset::LongMultiSet;
pub use page_util::PageUtil;

// Re-export primitive iterator types
pub use primitive::{
    empty, of, range, single, LongPredicate, PrimitiveLongBaseIterator, PrimitiveLongIterable,
    PrimitiveLongIterator,
};
