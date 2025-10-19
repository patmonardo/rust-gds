/// Functional interfaces for sparse list iteration callbacks.
///
/// These type aliases define closure signatures for the `for_all()` method
/// on various sparse list types. They enable type-safe, functional-style
/// iteration over sparse data structures.
/// Consumer for (index, value) pairs from HugeSparseLongList.
///
/// # Example
/// ```
/// use gds::collections::HugeSparseLongList;
///
/// let list = HugeSparseLongList::of(0);
/// list.set(10, 42);
/// list.set(100, 99);
///
/// list.for_all(|index, value| {
///     println!("Index {} has value {}", index, value);
/// });
/// ```
pub type LongLongConsumer = fn(usize, i64);

/// Consumer for (index, value) pairs from HugeSparseDoubleList.
///
/// # Example
/// ```
/// use gds::collections::HugeSparseDoubleList;
///
/// let list = HugeSparseDoubleList::of(0.0);
/// list.set(10, 3.14);
///
/// list.for_all(|index, value| {
///     println!("Index {} has value {}", index, value);
/// });
/// ```
pub type LongDoubleConsumer = fn(usize, f64);

/// Consumer for (index, value) pairs from HugeSparseIntList.
///
/// # Example
/// ```
/// // let list = HugeSparseIntList::of(0);
/// // list.for_all(|index, value| {
/// //     println!("Index {} has value {}", index, value);
/// // });
/// ```
pub type LongIntConsumer = fn(usize, i32);

/// Consumer for (index, array) pairs from HugeSparseLongArrayList.
///
/// # Example
/// ```
/// use gds::collections::HugeSparseLongArrayList;
///
/// let list = HugeSparseLongArrayList::of(vec![]);
/// list.set(10, vec![1, 2, 3]);
///
/// list.for_all(|index, array| {
///     println!("Index {} has array of length {}", index, array.len());
/// });
/// ```
pub type LongLongArrayConsumer = fn(usize, &Vec<i64>);

/// Consumer for (index, array) pairs from HugeSparseDoubleArrayList.
///
/// # Example
/// ```
/// use gds::collections::HugeSparseDoubleArrayList;
///
/// let list = HugeSparseDoubleArrayList::of(vec![]);
/// list.set(10, vec![3.14, 2.71]);
///
/// list.for_all(|index, array| {
///     println!("Index {} has array of length {}", index, array.len());
/// });
/// ```
pub type LongDoubleArrayConsumer = fn(usize, &Vec<f64>);

/// Consumer for (index, array) pairs from HugeSparseFloatArrayList.
///
/// Note: In Rust, we use f32 for float precision (TypeScript uses number/f64 for all floats).
pub type LongFloatArrayConsumer = fn(usize, &Vec<f32>);

/// Consumer for (index, array) pairs from HugeSparseByteArrayList.
///
/// Bytes are represented as u8 in Rust.
pub type LongByteArrayConsumer = fn(usize, &Vec<u8>);

/// Consumer for (index, matrix) pairs from HugeSparseLongArrayArrayList.
///
/// # Example
/// ```
/// use gds::collections::HugeSparseLongArrayArrayList;
///
/// let list = HugeSparseLongArrayArrayList::of(vec![]);
/// list.set(10, vec![vec![1, 2], vec![3, 4]]);
///
/// list.for_all(|index, matrix| {
///     println!("Index {} has matrix of length {}", index, matrix.len());
/// });
/// ```
pub type LongLongArrayArrayConsumer = fn(usize, &Vec<Vec<i64>>);

/// Consumer for (index, matrix) pairs from HugeSparseDoubleArrayArrayList.
///
/// Note: This type would be for a double[][] sparse list, which doesn't exist
/// in the current implementation (only used for adjacency lists with long[][]).
pub type LongDoubleArrayArrayConsumer = fn(usize, &Vec<Vec<f64>>);

/// Consumer for (index, matrix) pairs from HugeSparseByteArrayArrayList.
///
/// Represents byte[][] matrices stored sparsely.
pub type LongByteArrayArrayConsumer = fn(usize, &Vec<Vec<u8>>);
