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

use crate::collections::HugeObjectArray;

/// High-performance single-threaded merge sort for huge object arrays.
///
/// Essential for sorting complex objects by computed double values:
/// - Generic object sorting with custom value extraction function
/// - Bottom-up merge sort for optimal performance
/// - Memory-efficient single temporary buffer allocation
/// - Type-safe generic implementation with proper typing
/// - Double-precision value comparison for numerical sorting
///
/// Performance characteristics:
/// - Time complexity: O(n log n) for all cases
/// - Space complexity: O(n) for temporary buffer
/// - Function call overhead: one extraction call per comparison
/// - Cache-friendly: sequential memory access patterns
/// - Type-safe: full Rust generic support
///
/// Algorithm details:
/// - Bottom-up merge sort with doubling merge sizes
/// - Uses closure for value extraction (ToDoubleFunction equivalent)
/// - IEEE 754 double comparison for numerical stability
/// - Iterative approach without recursion overhead
/// - Single temporary array allocation for all merges
///
/// Use Cases:
/// - Sorting graph nodes by computed properties (centrality, degree)
/// - Organizing edges by weight or computed metrics
/// - Ranking entities by complex scoring functions
/// - Sorting analysis results by numerical criteria
/// - Preparing data for ranked access patterns
pub struct HugeSerialObjectMergeSort;

impl HugeSerialObjectMergeSort {
    /// Sorts a huge object array based on double values extracted from objects.
    /// Creates a temporary array internally for merge operations.
    ///
    /// # Arguments
    ///
    /// * `array` - Array of objects to sort (modified in place)
    /// * `to_sort_value` - Function that extracts double values for comparison
    ///
    /// # Performance
    ///
    /// O(n log n) time, O(n) space
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Sort graph nodes by PageRank score
    /// #[derive(Clone)]
    /// struct GraphNode {
    ///     id: i64,
    ///     page_rank: f64,
    ///     degree: usize,
    ///     label: String,
    /// }
    ///
    /// let mut nodes = HugeObjectArray::new(1_000_000);
    /// // ... populate with graph nodes ...
    ///
    /// // Sort by PageRank (descending)
    /// HugeSerialObjectMergeSort::sort(
    ///     &mut nodes,
    ///     |node| -node.page_rank // Negative for descending order
    /// );
    ///
    /// // Result: nodes sorted by PageRank, highest first
    /// println!("Top node: {}, PageRank: {}", nodes.get(0).label, nodes.get(0).page_rank);
    /// ```
    pub fn sort<T, F>(array: &mut HugeObjectArray<T>, to_sort_value: F)
    where
        T: Clone + Default,
        F: Fn(&T) -> f64,
    {
        let size = array.size();
        let mut temp = HugeObjectArray::new(size);
        Self::sort_with_buffer(array, size, to_sort_value, &mut temp);
    }

    /// Sorts an object array with explicit size and reusable temporary buffer.
    /// More efficient when sorting multiple arrays or when size < array.size().
    ///
    /// # Arguments
    ///
    /// * `array` - Array of objects to sort (modified in place)
    /// * `size` - Number of elements to sort (can be less than array size)
    /// * `to_sort_value` - Function that extracts double values for comparison
    /// * `temp` - Temporary buffer for merge operations (must be at least size elements)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// #[derive(Clone)]
    /// struct Edge {
    ///     source: i64,
    ///     target: i64,
    ///     weight: f64,
    ///     edge_type: String,
    /// }
    ///
    /// let mut edges = HugeObjectArray::new(5_000_000);
    /// let mut temp_buffer = HugeObjectArray::new(5_000_000);
    ///
    /// // Sort by edge weight (ascending)
    /// HugeSerialObjectMergeSort::sort_with_buffer(
    ///     &mut edges,
    ///     5_000_000,
    ///     |edge| edge.weight,
    ///     &mut temp_buffer,
    /// );
    ///
    /// // Result: edges sorted by weight, lightest first
    /// println!("Lightest edge: {}", edges.get(0).weight);
    /// println!("Heaviest edge: {}", edges.get(4_999_999).weight);
    /// ```
    pub fn sort_with_buffer<T, F>(
        array: &mut HugeObjectArray<T>,
        size: usize,
        to_sort_value: F,
        temp: &mut HugeObjectArray<T>,
    ) where
        T: Clone + Default,
        F: Fn(&T) -> f64,
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
                    array.set(i + j, temp.get(j).clone());
                }

                i += 2 * temp_size;
            }

            temp_size *= 2;
        }
    }

    /// Merges two sorted ranges of objects using extracted double values for comparison.
    ///
    /// # Arguments
    ///
    /// * `array` - Source array containing both ranges
    /// * `temp` - Temporary array for merge output
    /// * `to_sort_value` - Function to extract comparable double values from objects
    /// * `left_start` - Start of left range (inclusive)
    /// * `left_end` - End of left range (inclusive)
    /// * `right_start` - Start of right range (inclusive)
    /// * `right_end` - End of right range (inclusive)
    fn merge<T, F>(
        array: &HugeObjectArray<T>,
        temp: &mut HugeObjectArray<T>,
        to_sort_value: &F,
        left_start: usize,
        left_end: usize,
        right_start: usize,
        right_end: usize,
    ) where
        T: Clone + Default,
        F: Fn(&T) -> f64,
    {
        let mut idx = 0usize;
        let mut left = left_start;
        let mut right = right_start;

        // Merge while both ranges have elements
        while left <= left_end && right <= right_end {
            let left_obj = array.get(left);
            let right_obj = array.get(right);
            let left_value = to_sort_value(left_obj);
            let right_value = to_sort_value(right_obj);

            // Use IEEE 754 double comparison for numerical stability
            if Self::double_compare(left_value, right_value) <= 0 {
                temp.set(idx, left_obj.clone());
                idx += 1;
                left += 1;
            } else {
                temp.set(idx, right_obj.clone());
                idx += 1;
                right += 1;
            }
        }

        // Copy remaining elements from left range
        while left <= left_end {
            temp.set(idx, array.get(left).clone());
            idx += 1;
            left += 1;
        }

        // Copy remaining elements from right range
        while right <= right_end {
            temp.set(idx, array.get(right).clone());
            idx += 1;
            right += 1;
        }
    }

    /// IEEE 754 compliant double comparison.
    /// Handles NaN, Infinity, and -0.0 correctly.
    ///
    /// # Arguments
    ///
    /// * `a` - First double value
    /// * `b` - Second double value
    ///
    /// # Returns
    ///
    /// -1 if a < b, 0 if a == b, 1 if a > b
    fn double_compare(a: f64, b: f64) -> i32 {
        // Handle NaN cases
        if a.is_nan() && b.is_nan() {
            return 0;
        }
        if a.is_nan() {
            return 1;
        }
        if b.is_nan() {
            return -1;
        }

        // Handle infinity cases
        if a == f64::INFINITY && b == f64::INFINITY {
            return 0;
        }
        if a == f64::NEG_INFINITY && b == f64::NEG_INFINITY {
            return 0;
        }
        if a == f64::INFINITY {
            return 1;
        }
        if b == f64::INFINITY {
            return -1;
        }
        if a == f64::NEG_INFINITY {
            return -1;
        }
        if b == f64::NEG_INFINITY {
            return 1;
        }

        // Handle -0.0 vs 0.0
        if a == 0.0 && b == 0.0 {
            if a.is_sign_negative() && b.is_sign_positive() {
                return -1;
            }
            if a.is_sign_positive() && b.is_sign_negative() {
                return 1;
            }
            return 0;
        }

        // Regular comparison
        if a < b {
            -1
        } else if a > b {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Default)]
    struct TestNode {
        id: i64,
        value: f64,
    }

    #[test]
    fn test_sort_by_value() {
        let mut array = HugeObjectArray::new(5);
        array.set(0, TestNode { id: 0, value: 5.0 });
        array.set(1, TestNode { id: 1, value: 2.0 });
        array.set(2, TestNode { id: 2, value: 8.0 });
        array.set(3, TestNode { id: 3, value: 1.0 });
        array.set(4, TestNode { id: 4, value: 4.0 });

        HugeSerialObjectMergeSort::sort(&mut array, |node| node.value);

        // Should be sorted by value: 1.0, 2.0, 4.0, 5.0, 8.0
        assert_eq!(array.get(0).value, 1.0);
        assert_eq!(array.get(1).value, 2.0);
        assert_eq!(array.get(2).value, 4.0);
        assert_eq!(array.get(3).value, 5.0);
        assert_eq!(array.get(4).value, 8.0);
    }

    #[test]
    fn test_sort_descending() {
        let mut array = HugeObjectArray::new(5);
        array.set(0, TestNode { id: 0, value: 5.0 });
        array.set(1, TestNode { id: 1, value: 2.0 });
        array.set(2, TestNode { id: 2, value: 8.0 });
        array.set(3, TestNode { id: 3, value: 1.0 });
        array.set(4, TestNode { id: 4, value: 4.0 });

        // Sort descending by negating values
        HugeSerialObjectMergeSort::sort(&mut array, |node| -node.value);

        // Should be sorted by value: 8.0, 5.0, 4.0, 2.0, 1.0
        assert_eq!(array.get(0).value, 8.0);
        assert_eq!(array.get(1).value, 5.0);
        assert_eq!(array.get(2).value, 4.0);
        assert_eq!(array.get(3).value, 2.0);
        assert_eq!(array.get(4).value, 1.0);
    }

    #[test]
    fn test_sort_with_duplicates() {
        let mut array = HugeObjectArray::new(6);
        array.set(0, TestNode { id: 0, value: 3.0 });
        array.set(1, TestNode { id: 1, value: 1.0 });
        array.set(2, TestNode { id: 2, value: 3.0 });
        array.set(3, TestNode { id: 3, value: 2.0 });
        array.set(4, TestNode { id: 4, value: 1.0 });
        array.set(5, TestNode { id: 5, value: 2.0 });

        HugeSerialObjectMergeSort::sort(&mut array, |node| node.value);

        // Should be sorted: 1.0, 1.0, 2.0, 2.0, 3.0, 3.0
        assert_eq!(array.get(0).value, 1.0);
        assert_eq!(array.get(1).value, 1.0);
        assert_eq!(array.get(2).value, 2.0);
        assert_eq!(array.get(3).value, 2.0);
        assert_eq!(array.get(4).value, 3.0);
        assert_eq!(array.get(5).value, 3.0);
    }

    #[test]
    fn test_sort_already_sorted() {
        let mut array = HugeObjectArray::new(5);
        array.set(0, TestNode { id: 0, value: 1.0 });
        array.set(1, TestNode { id: 1, value: 2.0 });
        array.set(2, TestNode { id: 2, value: 3.0 });
        array.set(3, TestNode { id: 3, value: 4.0 });
        array.set(4, TestNode { id: 4, value: 5.0 });

        HugeSerialObjectMergeSort::sort(&mut array, |node| node.value);

        // Should remain sorted
        for i in 0..5 {
            assert_eq!(array.get(i).value, (i + 1) as f64);
        }
    }

    #[test]
    fn test_sort_with_negative_values() {
        let mut array = HugeObjectArray::new(5);
        array.set(0, TestNode { id: 0, value: 5.0 });
        array.set(1, TestNode { id: 1, value: -3.0 });
        array.set(2, TestNode { id: 2, value: 0.0 });
        array.set(
            3,
            TestNode {
                id: 3,
                value: -10.0,
            },
        );
        array.set(4, TestNode { id: 4, value: 2.0 });

        HugeSerialObjectMergeSort::sort(&mut array, |node| node.value);

        // Should be sorted: -10.0, -3.0, 0.0, 2.0, 5.0
        assert_eq!(array.get(0).value, -10.0);
        assert_eq!(array.get(1).value, -3.0);
        assert_eq!(array.get(2).value, 0.0);
        assert_eq!(array.get(3).value, 2.0);
        assert_eq!(array.get(4).value, 5.0);
    }

    #[test]
    fn test_sort_partial_array() {
        let mut array = HugeObjectArray::new(10);
        for i in 0..10 {
            array.set(
                i,
                TestNode {
                    id: i as i64,
                    value: (9 - i) as f64,
                },
            );
        }

        let mut temp = HugeObjectArray::new(10);

        // Sort only first 5 elements
        HugeSerialObjectMergeSort::sort_with_buffer(&mut array, 5, |node| node.value, &mut temp);

        // First 5 should be sorted
        assert_eq!(array.get(0).value, 5.0);
        assert_eq!(array.get(1).value, 6.0);
        assert_eq!(array.get(2).value, 7.0);
        assert_eq!(array.get(3).value, 8.0);
        assert_eq!(array.get(4).value, 9.0);

        // Last 5 should be unchanged
        for i in 5..10 {
            assert_eq!(array.get(i).value, (9 - i) as f64);
        }
    }

    #[test]
    fn test_double_compare() {
        // Regular comparisons
        assert_eq!(HugeSerialObjectMergeSort::double_compare(1.0, 2.0), -1);
        assert_eq!(HugeSerialObjectMergeSort::double_compare(2.0, 1.0), 1);
        assert_eq!(HugeSerialObjectMergeSort::double_compare(1.0, 1.0), 0);

        // NaN comparisons
        assert_eq!(
            HugeSerialObjectMergeSort::double_compare(f64::NAN, f64::NAN),
            0
        );
        assert_eq!(HugeSerialObjectMergeSort::double_compare(f64::NAN, 1.0), 1);
        assert_eq!(HugeSerialObjectMergeSort::double_compare(1.0, f64::NAN), -1);

        // Infinity comparisons
        assert_eq!(
            HugeSerialObjectMergeSort::double_compare(f64::INFINITY, f64::INFINITY),
            0
        );
        assert_eq!(
            HugeSerialObjectMergeSort::double_compare(f64::NEG_INFINITY, f64::NEG_INFINITY),
            0
        );
        assert_eq!(
            HugeSerialObjectMergeSort::double_compare(f64::INFINITY, 1.0),
            1
        );
        assert_eq!(
            HugeSerialObjectMergeSort::double_compare(1.0, f64::INFINITY),
            -1
        );
        assert_eq!(
            HugeSerialObjectMergeSort::double_compare(f64::NEG_INFINITY, 1.0),
            -1
        );
        assert_eq!(
            HugeSerialObjectMergeSort::double_compare(1.0, f64::NEG_INFINITY),
            1
        );

        // Zero comparisons
        assert_eq!(HugeSerialObjectMergeSort::double_compare(0.0, 0.0), 0);
        assert_eq!(HugeSerialObjectMergeSort::double_compare(-0.0, 0.0), -1);
        assert_eq!(HugeSerialObjectMergeSort::double_compare(0.0, -0.0), 1);
    }
}
