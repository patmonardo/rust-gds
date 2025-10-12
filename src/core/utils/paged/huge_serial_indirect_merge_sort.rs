/*
 * Copyright (c) "Neo4j"
 * Neo4j Sweden AB [http://neo4j.com]
 *
 * This file is part of Neo4j.
 *
 * Neo4j is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::collections::HugeLongArray;

/// High-performance single-threaded indirect merge sort for huge long arrays.
///
/// Essential for sorting indices by their associated values:
/// - Indirect sorting: sorts array indices based on external value function
/// - Memory-efficient: doesn't move actual data, only reorders indices
/// - Bottom-up merge sort: iterative approach without recursion overhead
/// - Custom comparison: uses external function to determine sort order
/// - Single-threaded: optimized for sequential execution
///
/// Performance characteristics:
/// - Time complexity: O(n log n) for all cases
/// - Space complexity: O(n) for temporary buffer
/// - No recursion overhead: iterative bottom-up approach
/// - Cache-friendly: sequential memory access patterns
/// - Function call overhead: one function call per comparison
///
/// Algorithm details:
/// - Bottom-up merge sort with doubling merge sizes
/// - Starts with single elements, doubles size each iteration
/// - Merges adjacent sorted ranges into larger sorted ranges
/// - Uses external function to get sortable values from indices
/// - Continues until entire array is sorted
///
/// Use Cases:
/// - Sorting node indices by property values (degree, PageRank, etc.)
/// - Creating sorted index arrays for ranked access
/// - Organizing data by computed metrics without data movement
/// - Building lookup tables with custom sort orders
/// - Preparing indices for binary search on computed values
pub struct HugeSerialIndirectMergeSort;

impl HugeSerialIndirectMergeSort {
    /// Sorts an array of indices based on values from an external function.
    /// Creates a temporary array internally for merge operations.
    ///
    /// # Arguments
    ///
    /// * `array` - Array of indices to sort (modified in place)
    /// * `to_sort_value` - Function that maps indices to sortable double values
    ///
    /// # Performance
    ///
    /// O(n log n) time, O(n) space
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Sort node indices by their PageRank scores
    /// let mut node_indices = HugeLongArray::new(1_000_000);
    /// for i in 0..1_000_000 {
    ///     node_indices.set(i, i as i64);
    /// }
    ///
    /// let page_rank_scores = compute_page_rank(); // Returns HashMap<i64, f64>
    ///
    /// // Sort indices by PageRank values (descending via negation)
    /// HugeSerialIndirectMergeSort::sort(&mut node_indices, |node_id| {
    ///     -page_rank_scores.get(&node_id).unwrap_or(&0.0)
    /// });
    ///
    /// // Now node_indices[0] contains the index of the highest PageRank node
    /// let top_node = node_indices.get(0);
    /// ```
    pub fn sort<F>(array: &mut HugeLongArray, to_sort_value: F)
    where
        F: Fn(i64) -> f64,
    {
        let size = array.size();
        let mut temp = HugeLongArray::new(size);
        Self::sort_with_buffer(array, size, to_sort_value, &mut temp);
    }

    /// Sorts an array with explicit size and reusable temporary buffer.
    /// More efficient when sorting multiple arrays or when size < array.size().
    ///
    /// # Arguments
    ///
    /// * `array` - Array of indices to sort (modified in place)
    /// * `size` - Number of elements to sort (can be less than array size)
    /// * `to_sort_value` - Function that maps indices to sortable double values
    /// * `temp` - Temporary buffer for merge operations (must be at least size elements)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Sort node indices by degree (number of connections)
    /// let mut node_indices = HugeLongArray::new(5_000_000);
    /// let mut temp_buffer = HugeLongArray::new(5_000_000);
    /// let node_degrees = compute_node_degrees(); // HashMap<i64, i64>
    ///
    /// // Initialize indices
    /// for i in 0..5_000_000 {
    ///     node_indices.set(i, i as i64);
    /// }
    ///
    /// // Sort by node degree (ascending)
    /// HugeSerialIndirectMergeSort::sort_with_buffer(
    ///     &mut node_indices,
    ///     5_000_000,
    ///     |node_id| *node_degrees.get(&node_id).unwrap_or(&0) as f64,
    ///     &mut temp_buffer,
    /// );
    ///
    /// // Result: node_indices sorted by degree
    /// // node_indices[0] = lowest degree node
    /// // node_indices[4_999_999] = highest degree node
    /// ```
    pub fn sort_with_buffer<F>(
        array: &mut HugeLongArray,
        size: usize,
        to_sort_value: F,
        temp: &mut HugeLongArray,
    ) where
        F: Fn(i64) -> f64,
    {
        let mut temp_size = 1usize;

        // Bottom-up merge sort: start with single elements, double size each iteration
        while temp_size < size {
            let mut i = 0usize;

            // Process all ranges of current temp_size
            while i < size {
                let left_start = i;
                let left_end = i + temp_size - 1;
                let right_start = i + temp_size;
                let mut right_end = i + 2 * temp_size - 1;

                // If there's no right range, we're done with this iteration
                if right_start >= size {
                    break;
                }

                // Clamp right end to array bounds
                if right_end >= size {
                    right_end = size - 1;
                }

                // Merge left and right ranges
                Self::merge(
                    array,
                    temp,
                    &to_sort_value,
                    left_start,
                    left_end,
                    right_start,
                    right_end,
                );

                // Copy merged result back to array
                for j in 0..=(right_end - left_start) {
                    array.set(i + j, temp.get(j));
                }

                i += 2 * temp_size;
            }

            temp_size *= 2;
        }
    }

    /// Merges two sorted ranges using external value function for comparison.
    ///
    /// # Arguments
    ///
    /// * `array` - Source array containing both ranges
    /// * `temp` - Temporary array for merge output
    /// * `to_sort_value` - Function to get comparable values from indices
    /// * `left_start` - Start of left range (inclusive)
    /// * `left_end` - End of left range (inclusive)
    /// * `right_start` - Start of right range (inclusive)
    /// * `right_end` - End of right range (inclusive)
    fn merge<F>(
        array: &HugeLongArray,
        temp: &mut HugeLongArray,
        to_sort_value: &F,
        left_start: usize,
        left_end: usize,
        right_start: usize,
        right_end: usize,
    ) where
        F: Fn(i64) -> f64,
    {
        let mut idx = 0usize;
        let mut left = left_start;
        let mut right = right_start;

        // Merge while both ranges have elements
        while left <= left_end && right <= right_end {
            let ls_idx = array.get(left);
            let rs_idx = array.get(right);

            if to_sort_value(ls_idx) <= to_sort_value(rs_idx) {
                temp.set(idx, ls_idx);
                idx += 1;
                left += 1;
            } else {
                temp.set(idx, rs_idx);
                idx += 1;
                right += 1;
            }
        }

        // Copy remaining elements from left range
        while left <= left_end {
            temp.set(idx, array.get(left));
            idx += 1;
            left += 1;
        }

        // Copy remaining elements from right range
        while right <= right_end {
            temp.set(idx, array.get(right));
            idx += 1;
            right += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_value() {
        // Create array with indices 0..9
        let mut indices = HugeLongArray::new(10);
        for i in 0..10 {
            indices.set(i, i as i64);
        }

        // Values in reverse order (index 0 -> value 9.0, index 9 -> value 0.0)
        let values = |idx: i64| (9 - idx) as f64;

        HugeSerialIndirectMergeSort::sort(&mut indices, values);

        // After sorting, indices should be reversed (sorted by increasing values)
        for i in 0..10 {
            assert_eq!(indices.get(i), (9 - i) as i64);
        }
    }

    #[test]
    fn test_sort_partial_array() {
        let mut indices = HugeLongArray::new(10);
        for i in 0..10 {
            indices.set(i, i as i64);
        }

        let values = |idx: i64| (9 - idx) as f64;
        let mut temp = HugeLongArray::new(10);

        // Sort only first 5 elements
        HugeSerialIndirectMergeSort::sort_with_buffer(&mut indices, 5, values, &mut temp);

        // First 5 should be sorted
        assert_eq!(indices.get(0), 4);
        assert_eq!(indices.get(1), 3);
        assert_eq!(indices.get(2), 2);
        assert_eq!(indices.get(3), 1);
        assert_eq!(indices.get(4), 0);

        // Last 5 should be unchanged
        for i in 5..10 {
            assert_eq!(indices.get(i), i as i64);
        }
    }

    #[test]
    fn test_sort_duplicate_values() {
        let mut indices = HugeLongArray::new(6);
        for i in 0..6 {
            indices.set(i, i as i64);
        }

        // All elements have same value - stable sort should preserve order
        let values = |_idx: i64| 42.0;

        HugeSerialIndirectMergeSort::sort(&mut indices, values);

        // Order should be preserved (stable sort)
        for i in 0..6 {
            assert_eq!(indices.get(i), i as i64);
        }
    }

    #[test]
    fn test_sort_already_sorted() {
        let mut indices = HugeLongArray::new(10);
        for i in 0..10 {
            indices.set(i, i as i64);
        }

        // Values already in order
        let values = |idx: i64| idx as f64;

        HugeSerialIndirectMergeSort::sort(&mut indices, values);

        // Should remain unchanged
        for i in 0..10 {
            assert_eq!(indices.get(i), i as i64);
        }
    }

    #[test]
    fn test_sort_with_negative_values() {
        let mut indices = HugeLongArray::new(5);
        for i in 0..5 {
            indices.set(i, i as i64);
        }

        // Mix of positive and negative values
        let values = |idx: i64| match idx {
            0 => 5.0,
            1 => -3.0,
            2 => 0.0,
            3 => -10.0,
            4 => 2.0,
            _ => 0.0,
        };

        HugeSerialIndirectMergeSort::sort(&mut indices, values);

        // Should be sorted: -10.0, -3.0, 0.0, 2.0, 5.0
        assert_eq!(indices.get(0), 3); // -10.0
        assert_eq!(indices.get(1), 1); // -3.0
        assert_eq!(indices.get(2), 2); // 0.0
        assert_eq!(indices.get(3), 4); // 2.0
        assert_eq!(indices.get(4), 0); // 5.0
    }

    #[test]
    fn test_sort_large_array() {
        let size = 1000usize;
        let mut indices = HugeLongArray::new(size);
        for i in 0..size {
            indices.set(i, i as i64);
        }

        // Random-ish values using simple hash
        let values = |idx: i64| ((idx * 2654435761i64) % 1000) as f64;

        HugeSerialIndirectMergeSort::sort(&mut indices, values);

        // Verify sorted order
        for i in 0..(size - 1) {
            let curr_idx = indices.get(i);
            let next_idx = indices.get(i + 1);
            assert!(
                values(curr_idx) <= values(next_idx),
                "Not sorted at position {}: {} > {}",
                i,
                values(curr_idx),
                values(next_idx)
            );
        }
    }
}
