/// Huge sparse lists with memory-efficient paged storage and mutability.
///
/// This module provides mutable sparse list implementations that only allocate
/// pages where values actually exist, using HashMap-based page tracking.
///
/// Unlike sparse arrays (which build once), sparse lists are mutable after creation.
pub mod huge_sparse_consumers;
pub mod huge_sparse_double_array_list;
pub mod huge_sparse_double_list;
pub mod huge_sparse_long_array_array_list;
pub mod huge_sparse_long_array_list;
pub mod huge_sparse_long_list;

pub use huge_sparse_consumers::{
    LongByteArrayArrayConsumer, LongByteArrayConsumer, LongDoubleArrayArrayConsumer,
    LongDoubleArrayConsumer, LongDoubleConsumer, LongFloatArrayConsumer, LongIntConsumer,
    LongLongArrayArrayConsumer, LongLongArrayConsumer, LongLongConsumer,
};
pub use huge_sparse_double_array_list::HugeSparseDoubleArrayList;
pub use huge_sparse_double_list::HugeSparseDoubleList;
pub use huge_sparse_long_array_array_list::HugeSparseLongArrayArrayList;
pub use huge_sparse_long_array_list::HugeSparseLongArrayList;
pub use huge_sparse_long_list::HugeSparseLongList;
