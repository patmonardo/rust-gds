//! Read-only interface for HugeLongArray providing immutable access.
//!
//! Essential for data protection and API design:
//! - Prevents accidental modifications to shared data
//! - Enables safe sharing of arrays between components
//! - Provides clear immutable semantics in function signatures
//! - Supports defensive programming patterns
//!
//! # Use Cases
//!
//! - Graph adjacency lists (read-only neighbor access)
//! - Precomputed distance arrays (immutable shortest paths)
//! - Node property arrays (read-only feature access)
//! - Sorted index arrays (immutable ordering)
//! - Cached computation results (protected from modification)
//!
//! # Performance Characteristics
//!
//! - Zero-cost abstraction (direct delegation to underlying array)
//! - Same O(1) access time as mutable arrays
//! - No memory overhead (wrapper pattern)
//! - Type-safe immutability guarantees
//!
//! # Data Science Applications
//!
//! - Feature vectors for machine learning (immutable training data)
//! - Graph structures (read-only topology)
//! - Precomputed rankings (PageRank, centrality scores)
//! - Distance matrices (immutable shortest path results)
//! - Time series data (historical values protection)
//!
//! # Examples
//!
//! ```
//! use gds::core::utils::paged::ReadOnlyHugeLongArray;
//! use gds::collections::HugeLongArray;
//!
//! // Create and populate mutable array
//! let mut array = HugeLongArray::new(1000);
//! for i in 0..1000 {
//!     array.set(i, i as i64);
//! }
//!
//! // Create read-only view
//! let read_only = ReadOnlyHugeLongArray::new(array);
//!
//! // Can read
//! assert_eq!(read_only.get(0), 0);
//! assert_eq!(read_only.size(), 1000);
//!
//! // Cannot modify (compile error if uncommented)
//! // read_only.set(0, 42);  // ERROR: no method named `set`
//! ```

use crate::collections::HugeLongArray;

/// Read-only interface for accessing huge long arrays.
///
/// Provides immutable view of underlying data without copy overhead.
/// This is a zero-cost abstraction - it's simply a wrapper that only
/// exposes read-only methods.
///
/// # Type Safety
///
/// In Rust, this wrapper provides compile-time immutability guarantees.
/// Once wrapped, the underlying array cannot be modified through this interface.
///
/// # Examples
///
/// ```
/// use gds::core::utils::paged::ReadOnlyHugeLongArray;
/// use gds::collections::HugeLongArray;
///
/// fn process_immutable_data(data: &ReadOnlyHugeLongArray) {
///     for i in 0..data.size() {
///         println!("Value at {}: {}", i, data.get(i));
///     }
///     // Cannot modify data here - enforced at compile time
/// }
///
/// let array = HugeLongArray::from_vec(vec![1, 2, 3, 4, 5]);
/// let read_only = ReadOnlyHugeLongArray::new(array);
/// process_immutable_data(&read_only);
/// ```
pub struct ReadOnlyHugeLongArray {
    array: HugeLongArray,
}

impl ReadOnlyHugeLongArray {
    /// Creates a read-only wrapper around an existing HugeLongArray.
    ///
    /// This is a zero-cost abstraction - no data copying involved.
    /// The array is moved into the wrapper, preventing further modifications.
    ///
    /// # Arguments
    ///
    /// * `array` - The huge array to wrap as read-only
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ReadOnlyHugeLongArray;
    /// use gds::collections::HugeLongArray;
    ///
    /// // Create and populate mutable array
    /// let mut array = HugeLongArray::new(1000000);
    /// for i in 0..1000000 {
    ///     array.set(i, i as i64);
    /// }
    ///
    /// // Wrap as read-only (array is moved)
    /// let read_only = ReadOnlyHugeLongArray::new(array);
    ///
    /// // Now can only read
    /// assert_eq!(read_only.get(0), 0);
    /// assert_eq!(read_only.size(), 1000000);
    /// ```
    pub fn new(array: HugeLongArray) -> Self {
        Self { array }
    }

    /// Creates a read-only array from literal values.
    ///
    /// Convenient for testing and small datasets.
    ///
    /// # Arguments
    ///
    /// * `values` - Slice of values to store
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ReadOnlyHugeLongArray;
    ///
    /// // Create read-only array for testing
    /// let node_ids = ReadOnlyHugeLongArray::of(&[1, 5, 9, 12, 15]);
    ///
    /// assert_eq!(node_ids.size(), 5);
    /// assert_eq!(node_ids.get(0), 1);
    /// assert_eq!(node_ids.get(4), 15);
    /// ```
    pub fn of(values: &[i64]) -> Self {
        Self {
            array: HugeLongArray::from_vec(values.to_vec()),
        }
    }

    /// Gets the value at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - Array index (0-based)
    ///
    /// # Returns
    ///
    /// The value at the given index
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds
    ///
    /// # Performance
    ///
    /// O(1) access time, same as mutable arrays
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ReadOnlyHugeLongArray;
    ///
    /// let array = ReadOnlyHugeLongArray::of(&[10, 20, 30, 40, 50]);
    /// assert_eq!(array.get(0), 10);
    /// assert_eq!(array.get(2), 30);
    /// assert_eq!(array.get(4), 50);
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> i64 {
        self.array.get(index)
    }

    /// Returns the number of elements in the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ReadOnlyHugeLongArray;
    ///
    /// let array = ReadOnlyHugeLongArray::of(&[1, 2, 3, 4, 5]);
    /// assert_eq!(array.size(), 5);
    /// ```
    #[inline]
    pub fn size(&self) -> usize {
        self.array.size()
    }

    /// Converts the huge array to a standard Vec.
    ///
    /// **WARNING**: Only use for small arrays or testing!
    /// For large arrays, this will allocate significant memory.
    ///
    /// # Returns
    ///
    /// Vec containing all elements
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ReadOnlyHugeLongArray;
    ///
    /// let array = ReadOnlyHugeLongArray::of(&[1, 2, 3, 4, 5]);
    /// let vec = array.to_vec();
    /// assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    /// ```
    #[cfg(test)]
    pub fn to_vec(&self) -> Vec<i64> {
        (0..self.size()).map(|i| self.array.get(i)).collect()
    }

    /// Returns a reference to the underlying array (read-only).
    ///
    /// This allows calling other read-only methods on HugeLongArray
    /// that might not be exposed through this wrapper.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ReadOnlyHugeLongArray;
    ///
    /// let array = ReadOnlyHugeLongArray::of(&[1, 2, 3]);
    /// let inner = array.inner();
    /// assert_eq!(inner.size(), 3);
    /// ```
    #[inline]
    pub fn inner(&self) -> &HugeLongArray {
        &self.array
    }

    /// Consumes the wrapper and returns the underlying array.
    ///
    /// This allows regaining mutable access if needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::paged::ReadOnlyHugeLongArray;
    /// use gds::collections::HugeLongArray;
    ///
    /// let array = HugeLongArray::from_vec(vec![1, 2, 3]);
    /// let read_only = ReadOnlyHugeLongArray::new(array);
    ///
    /// // Later, if mutable access is needed
    /// let mut array = read_only.into_inner();
    /// array.set(0, 42);
    /// ```
    pub fn into_inner(self) -> HugeLongArray {
        self.array
    }
}

// Implement Clone if needed (creates a new copy)
impl Clone for ReadOnlyHugeLongArray {
    fn clone(&self) -> Self {
        // Note: This clones the underlying array
        // For large arrays, consider using Arc<ReadOnlyHugeLongArray> instead
        Self {
            array: {
                let mut new_array = HugeLongArray::new(self.array.size());
                for i in 0..self.array.size() {
                    new_array.set(i, self.array.get(i));
                }
                new_array
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_from_array() {
        let array = HugeLongArray::from_vec(vec![1, 2, 3, 4, 5]);
        let read_only = ReadOnlyHugeLongArray::new(array);

        assert_eq!(read_only.size(), 5);
        assert_eq!(read_only.get(0), 1);
        assert_eq!(read_only.get(4), 5);
    }

    #[test]
    fn test_of_values() {
        let read_only = ReadOnlyHugeLongArray::of(&[10, 20, 30]);

        assert_eq!(read_only.size(), 3);
        assert_eq!(read_only.get(0), 10);
        assert_eq!(read_only.get(1), 20);
        assert_eq!(read_only.get(2), 30);
    }

    #[test]
    fn test_to_vec() {
        let read_only = ReadOnlyHugeLongArray::of(&[1, 2, 3, 4, 5]);
        let vec = read_only.to_vec();

        assert_eq!(vec, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_inner() {
        let read_only = ReadOnlyHugeLongArray::of(&[1, 2, 3]);
        let inner = read_only.inner();

        assert_eq!(inner.size(), 3);
        assert_eq!(inner.get(1), 2);
    }

    #[test]
    fn test_into_inner() {
        let array = HugeLongArray::from_vec(vec![1, 2, 3]);
        let read_only = ReadOnlyHugeLongArray::new(array);

        let mut array = read_only.into_inner();
        array.set(0, 42);

        assert_eq!(array.get(0), 42);
    }

    #[test]
    fn test_large_array() {
        let mut array = HugeLongArray::new(10000);
        for i in 0..10000 {
            array.set(i, i as i64);
        }

        let read_only = ReadOnlyHugeLongArray::new(array);

        assert_eq!(read_only.size(), 10000);
        assert_eq!(read_only.get(0), 0);
        assert_eq!(read_only.get(5000), 5000);
        assert_eq!(read_only.get(9999), 9999);
    }

    #[test]
    fn test_clone() {
        let read_only1 = ReadOnlyHugeLongArray::of(&[1, 2, 3]);
        let read_only2 = read_only1.clone();

        assert_eq!(read_only1.size(), read_only2.size());
        assert_eq!(read_only1.get(0), read_only2.get(0));
        assert_eq!(read_only1.get(2), read_only2.get(2));
    }

    #[test]
    fn test_immutability_through_api() {
        let array = HugeLongArray::from_vec(vec![1, 2, 3]);
        let read_only = ReadOnlyHugeLongArray::new(array);

        // Can only read - no set method available
        let _value = read_only.get(0);
        let _size = read_only.size();

        // The following would cause a compile error:
        // read_only.set(0, 42);  // ERROR: no method named `set`
    }

    #[test]
    fn test_pass_as_reference() {
        fn process_readonly(arr: &ReadOnlyHugeLongArray) -> i64 {
            let mut sum = 0;
            for i in 0..arr.size() {
                sum += arr.get(i);
            }
            sum
        }

        let read_only = ReadOnlyHugeLongArray::of(&[1, 2, 3, 4, 5]);
        let sum = process_readonly(&read_only);

        assert_eq!(sum, 15);
    }
}
