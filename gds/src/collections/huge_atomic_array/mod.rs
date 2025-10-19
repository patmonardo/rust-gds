pub mod huge_atomic_double_array;
/// Huge atomic arrays supporting thread-safe operations on massive datasets.
///
/// This module provides atomic variants of huge arrays with lock-free operations
/// for concurrent access patterns common in parallel graph algorithms.
pub mod huge_atomic_long_array;

pub use huge_atomic_double_array::HugeAtomicDoubleArray;
pub use huge_atomic_long_array::HugeAtomicLongArray;
