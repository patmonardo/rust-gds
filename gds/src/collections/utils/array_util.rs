//! Optimized array operations for large-scale data processing
//!
//! This module provides **performance-critical algorithms** designed for graph data science
//! applications that need to process massive arrays efficiently. It provides optimized
//! implementations of search, lookup, and memory allocation algorithms.
//!
//! ## Performance Philosophy
//!
//! 1. **Hybrid Search Strategies**: Binary search for large ranges, linear for small ranges
//! 2. **Memory Alignment Optimization**: Account for platform-specific alignment
//! 3. **Loop Unrolling**: Process multiple elements per iteration for better ILP
//!
//! ## Key Algorithm Categories
//!
//! - **Binary Search Variants**: Standard, first/last occurrence, lookup
//! - **Linear Search Optimizations**: Unrolled loops, early termination
//! - **Memory Allocation Strategies**: Exponential growth, alignment optimization

use crate::mem::Estimate;

/// Utility for optimized array operations
///
/// Provides static methods for search algorithms and memory allocation strategies
/// optimized for graph analytics workloads.
pub struct ArrayUtil;

impl ArrayUtil {
    /// Threshold for switching from binary search to linear search for small ranges
    const LINEAR_SEARCH_LIMIT: usize = 64;

    /// Maximum array length accounting for Rust/platform limitations
    const MAX_ARRAY_LENGTH: usize = usize::MAX - Estimate::BYTES_ARRAY_HEADER;

    /// Performs optimized binary search for existence check in sorted integer arrays.
    ///
    /// This method uses a **hybrid search strategy** that combines binary search for large
    /// ranges with optimized linear search for small ranges. The approach maximizes
    /// performance by leveraging CPU cache locality and reducing branch mispredictions.
    ///
    /// ## Algorithm Strategy
    ///
    /// - **Binary search phase**: Divides search space until range ≤ LINEAR_SEARCH_LIMIT
    /// - **Linear search phase**: Uses early termination for optimal cache performance
    /// - **Early termination**: Exits immediately when sorted order indicates key not present
    ///
    /// ## Performance Characteristics
    ///
    /// - **Time complexity**: O(log n) for large arrays, O(1) for small ranges
    /// - **Space complexity**: O(1) - no additional memory allocation
    /// - **Cache friendly**: Linear search phase maximizes cache hit rates
    ///
    /// # Arguments
    ///
    /// * `arr` - Sorted array to search (must be in ascending order)
    /// * `length` - Number of elements to consider (must be ≤ arr.len())
    /// * `key` - Value to search for
    ///
    /// # Returns
    ///
    /// `true` if key exists in the array, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::ArrayUtil;
    ///
    /// let arr = vec![1, 3, 5, 7, 9, 11, 13, 15];
    /// assert!(ArrayUtil::binary_search(&arr, arr.len(), 7));
    /// assert!(!ArrayUtil::binary_search(&arr, arr.len(), 6));
    /// ```
    pub fn binary_search(arr: &[i64], length: usize, key: i64) -> bool {
        let mut low = 0;
        let mut high = length.wrapping_sub(1);

        // Binary search phase for large ranges
        while high.wrapping_sub(low) > Self::LINEAR_SEARCH_LIMIT {
            let mid = (low + high) / 2;
            let mid_val = arr[mid];

            if mid_val < key {
                low = mid + 1;
            } else if mid_val > key {
                high = mid.wrapping_sub(1);
            } else {
                return true;
            }
        }

        // Linear search phase for small ranges
        Self::linear_search_2(arr, low, high, key)
    }

    /// Returns the index of the first occurrence of a key in a sorted array.
    ///
    /// This method is essential for **handling duplicate values** in sorted data structures
    /// commonly found in graph analytics (e.g., multiple edges between nodes, repeated
    /// timestamps, duplicate property values).
    ///
    /// ## Duplicate Handling Strategy
    ///
    /// When the key is found, the algorithm continues searching leftward to ensure it
    /// returns the **first occurrence** rather than any arbitrary occurrence of the key.
    ///
    /// ## Return Value Semantics
    ///
    /// - **Exact match**: Returns index of first occurrence (>= 0)
    /// - **No match**: Returns `-(insertion_point + 1)` (< 0)
    /// - **Insertion point**: Index where key would be inserted to maintain sort order
    ///
    /// # Arguments
    ///
    /// * `a` - Sorted array to search
    /// * `from_index` - Starting index (inclusive)
    /// * `to_index` - Ending index (exclusive)
    /// * `key` - Value to search for
    ///
    /// # Returns
    ///
    /// Index of first occurrence, or -(insertion_point + 1) if not found
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::ArrayUtil;
    ///
    /// let arr = vec![1, 3, 3, 3, 5, 7, 9];
    /// let idx = ArrayUtil::binary_search_first(&arr, 0, arr.len(), 3);
    /// assert_eq!(idx, 1); // First occurrence of 3
    ///
    /// let not_found = ArrayUtil::binary_search_first(&arr, 0, arr.len(), 4);
    /// assert!(not_found < 0); // Would insert at position 4
    /// assert_eq!(-(not_found + 1), 4);
    /// ```
    pub fn binary_search_first(a: &[i64], from_index: usize, to_index: usize, key: i64) -> isize {
        let mut low = from_index as isize;
        let mut high = to_index as isize - 1;

        while low <= high {
            let mid = ((low + high) / 2) as usize;
            let mid_val = a[mid];

            if mid_val < key {
                low = mid as isize + 1;
            } else if mid_val > key {
                high = mid as isize - 1;
            } else if mid > from_index && a[mid - 1] == key {
                // Key found, but not first occurrence - search leftward
                high = mid as isize - 1;
            } else {
                return mid as isize; // First occurrence found
            }
        }

        -(low + 1) // Key not found
    }

    /// Returns the index of the last occurrence of a key in a sorted array.
    ///
    /// This method complements `binary_search_first` to provide **range boundaries**
    /// for processing duplicate values in sorted data structures.
    ///
    /// ## Duplicate Handling Strategy
    ///
    /// When the key is found, the algorithm continues searching rightward to ensure it
    /// returns the **last occurrence** of the key.
    ///
    /// # Arguments
    ///
    /// * `a` - Sorted array to search
    /// * `from_index` - Starting index (inclusive)
    /// * `to_index` - Ending index (exclusive)
    /// * `key` - Value to search for
    ///
    /// # Returns
    ///
    /// Index of last occurrence, or -(insertion_point + 1) if not found
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::ArrayUtil;
    ///
    /// let arr = vec![1, 3, 3, 3, 5, 7, 9];
    /// let idx = ArrayUtil::binary_search_last(&arr, 0, arr.len(), 3);
    /// assert_eq!(idx, 3); // Last occurrence of 3
    /// ```
    pub fn binary_search_last(a: &[i64], from_index: usize, to_index: usize, key: i64) -> isize {
        let mut low = from_index as isize;
        let mut high = to_index as isize - 1;

        while low <= high {
            let mid = ((low + high) / 2) as usize;
            let mid_val = a[mid];

            if mid_val < key {
                low = mid as isize + 1;
            } else if mid_val > key {
                high = mid as isize - 1;
            } else if mid < to_index - 1 && a[mid + 1] == key {
                // Key found, but not last occurrence - search rightward
                low = mid as isize + 1;
            } else {
                return mid as isize; // Last occurrence found
            }
        }

        -(low + 1) // Key not found
    }

    /// Performs binary search and returns the exact index of the found element.
    ///
    /// This method provides **index-based access** for sorted arrays where you need
    /// the exact position of the found element rather than just existence confirmation.
    ///
    /// # Arguments
    ///
    /// * `arr` - Sorted array to search
    /// * `length` - Number of elements to consider
    /// * `key` - Value to search for
    ///
    /// # Returns
    ///
    /// Index of the key if found, or negative value indicating insertion point
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::ArrayUtil;
    ///
    /// let arr = vec![1, 3, 5, 7, 9];
    /// assert_eq!(ArrayUtil::binary_search_index(&arr, arr.len(), 5), 2);
    /// ```
    pub fn binary_search_index(arr: &[i64], length: usize, key: i64) -> isize {
        let mut low = 0;
        let mut high = length.wrapping_sub(1);

        // Binary search phase
        while high.wrapping_sub(low) > Self::LINEAR_SEARCH_LIMIT {
            let mid = (low + high) / 2;
            let mid_val = arr[mid];

            if mid_val < key {
                low = mid + 1;
            } else if mid_val > key {
                high = mid.wrapping_sub(1);
            } else {
                return mid as isize;
            }
        }

        // Linear search phase with index return
        Self::linear_search_2_index(arr, low, high, key)
    }

    /// Optimized linear search for small ranges with early termination.
    ///
    /// This method is used by hybrid search algorithms when the search range
    /// becomes small enough that linear search outperforms binary search due
    /// to better cache locality and reduced branch overhead.
    ///
    /// ## Early Termination Optimization
    ///
    /// Since the array is sorted, the search can terminate as soon as an element
    /// larger than the key is encountered, avoiding unnecessary comparisons.
    fn linear_search_2(arr: &[i64], low: usize, high: usize, key: i64) -> bool {
        if low >= arr.len() || high >= arr.len() {
            return false;
        }

        for &value in arr.iter().skip(low).take(high - low + 1) {
            if value == key {
                return true;
            }
            if value > key {
                return false; // Early termination for sorted arrays
            }
        }
        false
    }
    /// Optimized linear search with index return and early termination.
    fn linear_search_2_index(arr: &[i64], low: usize, high: usize, key: i64) -> isize {
        if low >= arr.len() || high >= arr.len() {
            return -((high + 1) as isize) - 1;
        }

        for (offset, &value) in arr.iter().skip(low).take(high - low + 1).enumerate() {
            let i = low + offset;
            if value == key {
                return i as isize;
            }
            if value > key {
                return -((i as isize) + 1); // Insertion point
            }
        }
        -((high + 1) as isize) - 1 // Would insert after high
    }

    /// High-performance linear search with loop unrolling optimization.
    ///
    /// This method uses **loop unrolling** to process 4 elements per iteration,
    /// reducing loop overhead and improving instruction-level parallelism.
    /// This is particularly effective for modern CPUs with multiple execution units.
    ///
    /// ## Loop Unrolling Benefits
    ///
    /// - **Reduced branch overhead**: Fewer loop condition checks
    /// - **Better instruction pipelining**: More work per instruction fetch
    /// - **Cache optimization**: Sequential access pattern maximizes cache hits
    ///
    /// # Arguments
    ///
    /// * `arr` - Array to search (not required to be sorted)
    /// * `length` - Number of elements to consider
    /// * `key` - Value to search for
    ///
    /// # Returns
    ///
    /// `true` if key found, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::ArrayUtil;
    ///
    /// let arr = vec![5, 2, 8, 1, 9, 3, 7, 4];
    /// assert!(ArrayUtil::linear_search(&arr, arr.len(), 7));
    /// assert!(!ArrayUtil::linear_search(&arr, arr.len(), 10));
    /// ```
    pub fn linear_search(arr: &[i64], length: usize, key: i64) -> bool {
        let mut i = 0;

        // Process 4 elements per iteration (loop unrolling)
        while i + 4 <= length {
            if arr[i] == key || arr[i + 1] == key || arr[i + 2] == key || arr[i + 3] == key {
                return true;
            }
            i += 4;
        }

        // Handle remaining elements
        while i < length {
            if arr[i] == key {
                return true;
            }
            i += 1;
        }
        false
    }

    /// High-performance linear search with index return and loop unrolling.
    ///
    /// # Arguments
    ///
    /// * `arr` - Array to search
    /// * `length` - Number of elements to consider
    /// * `key` - Value to search for
    ///
    /// # Returns
    ///
    /// Index if found, or negative value if not found
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::ArrayUtil;
    ///
    /// let arr = vec![5, 2, 8, 1, 9, 3, 7, 4];
    /// assert_eq!(ArrayUtil::linear_search_index(&arr, arr.len(), 7), 6);
    /// ```
    pub fn linear_search_index(arr: &[i64], length: usize, key: i64) -> isize {
        let mut i = 0;

        // Process 4 elements per iteration
        while i + 4 <= length {
            if arr[i] == key {
                return i as isize;
            }
            if arr[i + 1] == key {
                return (i + 1) as isize;
            }
            if arr[i + 2] == key {
                return (i + 2) as isize;
            }
            if arr[i + 3] == key {
                return (i + 3) as isize;
            }
            i += 4;
        }

        // Handle remaining elements
        while i < length {
            if arr[i] == key {
                return i as isize;
            }
            i += 1;
        }
        -(length as isize) - 1
    }

    /// Specialized binary lookup for finding insertion points in sorted arrays.
    ///
    /// This method is designed for **range queries and insertion point finding**
    /// rather than exact matches. It returns the index where `(ids[idx] <= id) && (ids[idx + 1] > id)`,
    /// making it ideal for bucketing operations and range-based algorithms.
    ///
    /// ## Return Value Semantics
    ///
    /// - **Range position**: Returns index where value fits in the sorted order
    /// - **Exact match**: Returns the matching index
    /// - **Too small**: Returns -1 if value is smaller than all elements
    /// - **Too large**: Returns `length - 1` if value is larger than all elements
    ///
    /// # Arguments
    ///
    /// * `id` - Value to find position for
    /// * `ids` - Sorted array to search in
    ///
    /// # Returns
    ///
    /// Index where value fits, or -1 if smaller than all elements
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::ArrayUtil;
    ///
    /// let buckets = vec![0, 10, 20, 30, 40];
    /// assert_eq!(ArrayUtil::binary_lookup(15, &buckets), 1); // Between 10 and 20
    /// assert_eq!(ArrayUtil::binary_lookup(30, &buckets), 3); // Exact match
    /// assert_eq!(ArrayUtil::binary_lookup(-5, &buckets), -1); // Before all
    /// ```
    pub fn binary_lookup(id: i64, ids: &[i64]) -> isize {
        let length = ids.len();
        let mut low = 0isize;
        let mut high = length as isize - 1;

        while low <= high {
            let mid = ((low + high) / 2) as usize;
            let mid_val = ids[mid];

            if mid_val < id {
                low = mid as isize + 1;
            } else if mid_val > id {
                high = mid as isize - 1;
            } else {
                return mid as isize; // Exact match
            }
        }
        low - 1 // Insertion point
    }

    /// Creates a new vector filled with the specified value.
    ///
    /// This utility method provides **efficient array initialization** for numerical
    /// computations where arrays need to be pre-filled with default values.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to fill the array with
    /// * `length` - The desired array length
    ///
    /// # Returns
    ///
    /// A new `Vec<i64>` of the specified length filled with the value
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::ArrayUtil;
    ///
    /// let arr = ArrayUtil::fill(42, 5);
    /// assert_eq!(arr, vec![42, 42, 42, 42, 42]);
    /// ```
    pub fn fill(value: i64, length: usize) -> Vec<i64> {
        vec![value; length]
    }

    /// Checks if an array contains a specific value using linear search.
    ///
    /// This method provides **simple containment checking** for unsorted arrays
    /// or when the overhead of sorting is not justified for one-time searches.
    ///
    /// # Arguments
    ///
    /// * `array` - Array to search
    /// * `value` - Value to search for
    ///
    /// # Returns
    ///
    /// `true` if array contains the value, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::ArrayUtil;
    ///
    /// let arr = vec![1, 5, 3, 9, 2];
    /// assert!(ArrayUtil::contains(&arr, 9));
    /// assert!(!ArrayUtil::contains(&arr, 7));
    /// ```
    pub fn contains(array: &[i64], value: i64) -> bool {
        array.contains(&value)
    }

    /// Calculates optimal array size for dynamic growth with memory alignment optimization.
    ///
    /// This method implements a **sophisticated growth strategy** that balances memory
    /// efficiency with performance by considering:
    /// - **Exponential growth**: Minimizes reallocation frequency
    /// - **Memory alignment**: Optimizes for CPU cache line boundaries
    /// - **Platform differences**: Accounts for 32-bit vs 64-bit architectures
    /// - **GC optimization**: Reduces garbage collection pressure
    ///
    /// ## Growth Strategy
    ///
    /// The algorithm grows arrays by approximately 12.5% (1/8th) beyond the minimum
    /// required size, providing a good balance between memory usage and reallocation
    /// frequency. This is more conservative than typical 2x growth but reduces
    /// memory waste for large arrays common in graph analytics.
    ///
    /// ## Memory Alignment Optimization
    ///
    /// The method rounds array sizes to optimal boundaries based on element size
    /// and platform architecture to maximize cache performance and minimize
    /// memory fragmentation.
    ///
    /// # Arguments
    ///
    /// * `min_target_size` - Minimum required array size
    /// * `bytes_per_element` - Size of each array element in bytes
    ///
    /// # Returns
    ///
    /// Optimal array size >= min_target_size
    ///
    /// # Panics
    ///
    /// Panics if min_target_size is negative or exceeds maximum array size
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::ArrayUtil;
    ///
    /// let new_capacity = ArrayUtil::oversize(1000, 8);
    /// assert!(new_capacity >= 1000);
    /// assert!(new_capacity <= 1200); // ~12.5% growth
    /// ```
    pub fn oversize(min_target_size: usize, bytes_per_element: usize) -> usize {
        if min_target_size == 0 {
            return 0; // Wait until at least one element is requested
        }

        if min_target_size > Self::MAX_ARRAY_LENGTH {
            panic!(
                "Requested array size {} exceeds maximum array size ({})",
                min_target_size,
                Self::MAX_ARRAY_LENGTH
            );
        }

        // Asymptotic exponential growth by 1/8th
        let mut extra = min_target_size >> 3;

        if extra < 3 {
            // For very small arrays, grow faster to amortize reallocation overhead
            extra = 3;
        }

        let new_size = min_target_size + extra;

        // Check for overflow and maximum size constraints
        if new_size.wrapping_add(7) < new_size || new_size + 7 > Self::MAX_ARRAY_LENGTH {
            return Self::MAX_ARRAY_LENGTH;
        }

        // Memory alignment optimization based on platform and element size
        if Estimate::BYTES_OBJECT_REF == 8 {
            // 64-bit platform: round up to 8-byte alignment
            match bytes_per_element {
                4 => (new_size + 1) & 0x7fff_fffe, // Round up to multiple of 2
                2 => (new_size + 3) & 0x7fff_fffc, // Round up to multiple of 4
                1 => (new_size + 7) & 0x7fff_fff8, // Round up to multiple of 8
                _ => new_size,                     // 8 bytes or other: no rounding needed
            }
        } else {
            // 32-bit platform: different alignment strategy
            match bytes_per_element {
                1 => ((new_size + 3) & 0x7fff_fff8) + 4,
                2 => ((new_size + 1) & 0x7fff_fffc) + 2,
                4 => (new_size & 0x7fff_fffe) + 1,
                _ => new_size, // 8 bytes or other
            }
        }
    }

    /// Calculates optimal array size for huge arrays exceeding standard size limits.
    ///
    /// This method extends the `oversize` algorithm to handle **massive arrays**
    /// that exceed the limitations of standard Rust vectors. It's designed
    /// for graph analytics applications that need to process datasets with
    /// billions of elements.
    ///
    /// ## Huge Array Characteristics
    ///
    /// - **Extended range**: Supports sizes beyond standard limits
    /// - **Memory optimization**: Maintains alignment benefits for huge allocations
    /// - **Performance preservation**: Same growth strategy as standard oversize
    ///
    /// # Arguments
    ///
    /// * `min_target_size` - Minimum required array size (can exceed standard limits)
    /// * `bytes_per_element` - Size of each array element in bytes
    ///
    /// # Returns
    ///
    /// Optimal array size >= min_target_size
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::collections::ArrayUtil;
    ///
    /// let huge_capacity = ArrayUtil::oversize_huge(10_000_000, 8);
    /// assert!(huge_capacity >= 10_000_000);
    /// ```
    pub fn oversize_huge(min_target_size: usize, bytes_per_element: usize) -> usize {
        if min_target_size == 0 {
            return 0;
        }

        // Asymptotic exponential growth by 1/8th
        let mut extra = min_target_size >> 3;

        if extra < 3 {
            extra = 3;
        }

        let new_size = min_target_size + extra;

        // Memory alignment for huge arrays
        if Estimate::BYTES_OBJECT_REF == 8 {
            // 64-bit platform alignment
            match bytes_per_element {
                4 => (new_size + 1) & 0x7fff_fffe,
                2 => (new_size + 3) & 0x7fff_fffc,
                1 => (new_size + 7) & 0x7fff_fff8,
                _ => new_size, // 8 bytes or other
            }
        } else {
            // 32-bit platform alignment
            match bytes_per_element {
                2 => (new_size + 1) & 0x7fff_fffe,
                1 => (new_size + 3) & 0x7fff_fffc,
                _ => new_size, // 4, 8 bytes or other
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        assert!(ArrayUtil::binary_search(&arr, arr.len(), 7));
        assert!(ArrayUtil::binary_search(&arr, arr.len(), 1));
        assert!(ArrayUtil::binary_search(&arr, arr.len(), 19));
        assert!(!ArrayUtil::binary_search(&arr, arr.len(), 6));
        assert!(!ArrayUtil::binary_search(&arr, arr.len(), 20));
    }

    #[test]
    fn test_binary_search_first() {
        let arr = vec![1, 3, 3, 3, 5, 7, 9];
        let idx = ArrayUtil::binary_search_first(&arr, 0, arr.len(), 3);
        assert_eq!(idx, 1); // First occurrence

        let not_found = ArrayUtil::binary_search_first(&arr, 0, arr.len(), 4);
        assert!(not_found < 0);
        assert_eq!(-(not_found + 1), 4); // Would insert at position 4
    }

    #[test]
    fn test_binary_search_last() {
        let arr = vec![1, 3, 3, 3, 5, 7, 9];
        let idx = ArrayUtil::binary_search_last(&arr, 0, arr.len(), 3);
        assert_eq!(idx, 3); // Last occurrence
    }

    #[test]
    fn test_binary_lookup() {
        let buckets = vec![0, 10, 20, 30, 40];
        assert_eq!(ArrayUtil::binary_lookup(15, &buckets), 1);
        assert_eq!(ArrayUtil::binary_lookup(30, &buckets), 3);
        assert_eq!(ArrayUtil::binary_lookup(-5, &buckets), -1);
    }

    #[test]
    fn test_linear_search() {
        let arr = vec![5, 2, 8, 1, 9, 3, 7, 4];
        assert!(ArrayUtil::linear_search(&arr, arr.len(), 7));
        assert!(!ArrayUtil::linear_search(&arr, arr.len(), 10));
    }

    #[test]
    fn test_oversize() {
        let size = ArrayUtil::oversize(1000, 8);
        assert!(size >= 1000);
        assert!(size <= 1200); // ~12.5% growth

        assert_eq!(ArrayUtil::oversize(0, 8), 0);
    }

    #[test]
    #[should_panic]
    fn test_oversize_too_large() {
        ArrayUtil::oversize(usize::MAX, 8);
    }
}
