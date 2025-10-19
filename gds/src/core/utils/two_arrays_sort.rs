//! High-performance simultaneous sorting of two related arrays.
//!
//! Essential for graph algorithms where you need to maintain relationships between:
//! - Node IDs and their weights/properties
//! - Edge targets and their weights
//! - Neighbor lists with associated distances/scores
//!
//! Uses indirect sorting to minimize data movement - sorts indices first,
//! then reorders both arrays in a single pass for optimal cache performance.

use super::AscendingLongComparator;

/// Sorts two arrays simultaneously based on values of the first (i64) array.
///
/// The second array's elements are reordered to maintain correspondence with
/// the first array's sorted order.
///
/// # Arguments
///
/// * `long_array` - Array of i64 values (e.g., neighbor IDs)
/// * `double_array` - Array of f64 values (e.g., edge weights)
/// * `length` - Number of elements to sort (allows partial sorting)
///
/// # Time Complexity
///
/// O(n log n) for sorting + O(n) for reordering
///
/// # Space Complexity
///
/// O(n) for the ordering indices
///
/// # Examples
///
/// ```
/// use rust_gds::core::utils::sort_double_array_by_long_values;
///
/// let mut node_ids = vec![4, 1, 8];
/// let mut weights = vec![0.5, 1.9, 0.9];
///
/// sort_double_array_by_long_values(&mut node_ids, &mut weights, 3);
///
/// assert_eq!(node_ids, vec![1, 4, 8]);
/// assert_eq!(weights, vec![1.9, 0.5, 0.9]);
/// ```
///
/// ```
/// use rust_gds::core::utils::sort_double_array_by_long_values;
///
/// // Sort neighbor list by node IDs while maintaining edge weights
/// let mut neighbors = vec![100, 50, 200, 75];
/// let mut edge_weights = vec![0.8, 0.3, 0.9, 0.1];
///
/// sort_double_array_by_long_values(&mut neighbors, &mut edge_weights, 4);
///
/// assert_eq!(neighbors, vec![50, 75, 100, 200]);
/// assert_eq!(edge_weights, vec![0.3, 0.1, 0.8, 0.9]);
/// ```
pub fn sort_double_array_by_long_values(
    long_array: &mut [i64],
    double_array: &mut [f64],
    length: usize,
) {
    assert!(
        long_array.len() >= length,
        "long_array too short for specified length"
    );
    assert!(
        double_array.len() >= length,
        "double_array too short for specified length"
    );

    // Create indirect sort order using merge sort
    let order = indirect_mergesort(&long_array[..length]);

    // Reorder both arrays according to sorted indices
    reorder(order, long_array, double_array, length);
}

/// Sorts two arrays simultaneously based on values of the second (f64) array.
///
/// Useful when edge weights are the primary sort key.
///
/// # Examples
///
/// ```
/// use rust_gds::core::utils::sort_long_array_by_double_values;
///
/// let mut node_ids = vec![100, 50, 200, 75];
/// let mut weights = vec![0.8, 0.3, 0.9, 0.1];
///
/// sort_long_array_by_double_values(&mut node_ids, &mut weights, 4);
///
/// // Sorted by weights: 0.1, 0.3, 0.8, 0.9
/// assert_eq!(weights, vec![0.1, 0.3, 0.8, 0.9]);
/// assert_eq!(node_ids, vec![75, 50, 100, 200]);
/// ```
pub fn sort_long_array_by_double_values(
    long_array: &mut [i64],
    double_array: &mut [f64],
    length: usize,
) {
    assert!(
        long_array.len() >= length,
        "long_array too short for specified length"
    );
    assert!(
        double_array.len() >= length,
        "double_array too short for specified length"
    );

    // Create index array and sort by double values
    let mut indices: Vec<usize> = (0..length).collect();
    indices.sort_by(|&a, &b| {
        double_array[a]
            .partial_cmp(&double_array[b])
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Reorder both arrays
    reorder(indices, long_array, double_array, length);
}

/// Sorts in descending order by long values.
///
/// Useful for algorithms that need largest-first processing.
///
/// # Examples
///
/// ```
/// use rust_gds::core::utils::sort_double_array_by_long_values_desc;
///
/// let mut node_ids = vec![4, 1, 8];
/// let mut weights = vec![0.5, 1.9, 0.9];
///
/// sort_double_array_by_long_values_desc(&mut node_ids, &mut weights, 3);
///
/// assert_eq!(node_ids, vec![8, 4, 1]);
/// assert_eq!(weights, vec![0.9, 0.5, 1.9]);
/// ```
pub fn sort_double_array_by_long_values_desc(
    long_array: &mut [i64],
    double_array: &mut [f64],
    length: usize,
) {
    assert!(
        long_array.len() >= length,
        "long_array too short for specified length"
    );
    assert!(
        double_array.len() >= length,
        "double_array too short for specified length"
    );

    // Create index array and sort descending
    let mut indices: Vec<usize> = (0..length).collect();
    indices.sort_by(|&a, &b| long_array[b].cmp(&long_array[a]));

    reorder(indices, long_array, double_array, length);
}

/// Efficient in-place reordering of two arrays based on a permutation.
///
/// Uses cycle-following algorithm to minimize temporary storage.
///
/// # Algorithm Explanation
///
/// - For each position, follow the cycle of swaps needed
/// - Mark processed positions to avoid duplicate work
/// - Handles arbitrary permutations efficiently
///
/// # Time Complexity: O(n)
/// # Space Complexity: O(1) additional (modifies order array)
fn reorder(mut order: Vec<usize>, long_array: &mut [i64], double_array: &mut [f64], length: usize) {
    for i in 0..length {
        // Skip if this element is already in correct position
        if order[i] == i {
            continue;
        }

        // Store the values that will be displaced
        let init_long = long_array[i];
        let init_double = double_array[i];
        let mut current_idx = i;

        // Follow the cycle of swaps
        while order[current_idx] != i {
            let next_idx = order[current_idx];

            // Move values from next position to current position
            long_array[current_idx] = long_array[next_idx];
            double_array[current_idx] = double_array[next_idx];

            // Mark current position as processed
            order[current_idx] = current_idx;
            current_idx = next_idx;
        }

        // Complete the cycle by placing initial values
        long_array[current_idx] = init_long;
        double_array[current_idx] = init_double;
        order[current_idx] = current_idx;
    }
}

/// Indirect merge sort implementation.
///
/// Returns a permutation array representing the sorted order of indices
/// without modifying the original array.
///
/// This is a stable sort that uses the AscendingLongComparator strategy.
fn indirect_mergesort(array: &[i64]) -> Vec<usize> {
    let n = array.len();
    let mut indices: Vec<usize> = (0..n).collect();

    if n <= 1 {
        return indices;
    }

    let comparator = AscendingLongComparator::new(array);
    let mut temp = vec![0; n];

    mergesort_impl(&mut indices, &mut temp, 0, n, &comparator);

    indices
}

/// Recursive merge sort implementation for indices.
fn mergesort_impl(
    indices: &mut [usize],
    temp: &mut [usize],
    left: usize,
    right: usize,
    comparator: &AscendingLongComparator,
) {
    if right - left <= 1 {
        return;
    }

    let mid = left + (right - left) / 2;

    mergesort_impl(indices, temp, left, mid, comparator);
    mergesort_impl(indices, temp, mid, right, comparator);

    merge(indices, temp, left, mid, right, comparator);
}

/// Merge two sorted subarrays of indices.
fn merge(
    indices: &mut [usize],
    temp: &mut [usize],
    left: usize,
    mid: usize,
    right: usize,
    comparator: &AscendingLongComparator,
) {
    let mut i = left;
    let mut j = mid;
    let mut k = left;

    while i < mid && j < right {
        if comparator.compare(indices[i], indices[j]) != std::cmp::Ordering::Greater {
            temp[k] = indices[i];
            i += 1;
        } else {
            temp[k] = indices[j];
            j += 1;
        }
        k += 1;
    }

    while i < mid {
        temp[k] = indices[i];
        i += 1;
        k += 1;
    }

    while j < right {
        temp[k] = indices[j];
        j += 1;
        k += 1;
    }

    indices[left..right].copy_from_slice(&temp[left..right]);
}

/// Utility function to check if two arrays are sorted together correctly.
///
/// Useful for testing and validation.
///
/// # Examples
///
/// ```
/// use rust_gds::core::utils::is_correctly_sorted;
///
/// let node_ids = vec![1, 4, 8];
/// let weights = vec![1.9, 0.5, 0.9];
///
/// assert!(is_correctly_sorted(&node_ids, &weights, 3, true));
/// ```
pub fn is_correctly_sorted(
    long_array: &[i64],
    _double_array: &[f64],
    length: usize,
    ascending: bool,
) -> bool {
    for i in 1..length {
        let current = long_array[i];
        let previous = long_array[i - 1];

        if ascending && current < previous {
            return false;
        }
        if !ascending && current > previous {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_sort_by_long() {
        let mut node_ids = vec![4, 1, 8];
        let mut weights = vec![0.5, 1.9, 0.9];

        sort_double_array_by_long_values(&mut node_ids, &mut weights, 3);

        assert_eq!(node_ids, vec![1, 4, 8]);
        assert_eq!(weights, vec![1.9, 0.5, 0.9]);
    }

    #[test]
    fn test_neighbor_list_sort() {
        let mut neighbors = vec![100, 50, 200, 75];
        let mut edge_weights = vec![0.8, 0.3, 0.9, 0.1];

        sort_double_array_by_long_values(&mut neighbors, &mut edge_weights, 4);

        assert_eq!(neighbors, vec![50, 75, 100, 200]);
        assert_eq!(edge_weights, vec![0.3, 0.1, 0.8, 0.9]);
    }

    #[test]
    fn test_sort_by_double_values() {
        let mut node_ids = vec![100, 50, 200, 75];
        let mut weights = vec![0.8, 0.3, 0.9, 0.1];

        sort_long_array_by_double_values(&mut node_ids, &mut weights, 4);

        assert_eq!(weights, vec![0.1, 0.3, 0.8, 0.9]);
        assert_eq!(node_ids, vec![75, 50, 100, 200]);
    }

    #[test]
    fn test_descending_sort() {
        let mut node_ids = vec![4, 1, 8];
        let mut weights = vec![0.5, 1.9, 0.9];

        sort_double_array_by_long_values_desc(&mut node_ids, &mut weights, 3);

        assert_eq!(node_ids, vec![8, 4, 1]);
        assert_eq!(weights, vec![0.9, 0.5, 1.9]);
    }

    #[test]
    fn test_single_element() {
        let mut node_ids = vec![42];
        let mut weights = vec![3.14];

        sort_double_array_by_long_values(&mut node_ids, &mut weights, 1);

        assert_eq!(node_ids, vec![42]);
        assert_eq!(weights, vec![3.14]);
    }

    #[test]
    fn test_already_sorted() {
        let mut node_ids = vec![1, 2, 3, 4, 5];
        let mut weights = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        sort_double_array_by_long_values(&mut node_ids, &mut weights, 5);

        assert_eq!(node_ids, vec![1, 2, 3, 4, 5]);
        assert_eq!(weights, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut node_ids = vec![5, 4, 3, 2, 1];
        let mut weights = vec![5.0, 4.0, 3.0, 2.0, 1.0];

        sort_double_array_by_long_values(&mut node_ids, &mut weights, 5);

        assert_eq!(node_ids, vec![1, 2, 3, 4, 5]);
        assert_eq!(weights, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_duplicate_keys() {
        let mut node_ids = vec![3, 1, 3, 2, 1];
        let mut weights = vec![3.0, 1.0, 3.5, 2.0, 1.5];

        sort_double_array_by_long_values(&mut node_ids, &mut weights, 5);

        assert_eq!(node_ids, vec![1, 1, 2, 3, 3]);
        // Stable sort preserves relative order of equal keys
        assert_eq!(weights, vec![1.0, 1.5, 2.0, 3.0, 3.5]);
    }

    #[test]
    fn test_negative_values() {
        let mut node_ids = vec![-5, 10, -3, 0, 7];
        let mut weights = vec![5.0, 10.0, 3.0, 0.0, 7.0];

        sort_double_array_by_long_values(&mut node_ids, &mut weights, 5);

        assert_eq!(node_ids, vec![-5, -3, 0, 7, 10]);
        assert_eq!(weights, vec![5.0, 3.0, 0.0, 7.0, 10.0]);
    }

    #[test]
    fn test_partial_sort() {
        let mut node_ids = vec![5, 2, 8, 1, 9, 3];
        let mut weights = vec![5.0, 2.0, 8.0, 1.0, 9.0, 3.0];

        // Only sort first 4 elements
        sort_double_array_by_long_values(&mut node_ids, &mut weights, 4);

        assert_eq!(node_ids[0..4], [1, 2, 5, 8]);
        assert_eq!(weights[0..4], [1.0, 2.0, 5.0, 8.0]);
        // Last 2 elements unchanged
        assert_eq!(node_ids[4..6], [9, 3]);
        assert_eq!(weights[4..6], [9.0, 3.0]);
    }

    #[test]
    fn test_is_correctly_sorted_ascending() {
        let node_ids = vec![1, 4, 8];
        let weights = vec![1.9, 0.5, 0.9];

        assert!(is_correctly_sorted(&node_ids, &weights, 3, true));
    }

    #[test]
    fn test_is_correctly_sorted_descending() {
        let node_ids = vec![8, 4, 1];
        let weights = vec![0.9, 0.5, 1.9];

        assert!(is_correctly_sorted(&node_ids, &weights, 3, false));
    }

    #[test]
    fn test_is_not_sorted() {
        let node_ids = vec![4, 1, 8];
        let weights = vec![0.5, 1.9, 0.9];

        assert!(!is_correctly_sorted(&node_ids, &weights, 3, true));
    }

    #[test]
    fn test_large_array() {
        let n = 1000;
        let mut node_ids: Vec<i64> = (0..n).rev().collect();
        let mut weights: Vec<f64> = (0..n).rev().map(|x| x as f64).collect();

        sort_double_array_by_long_values(&mut node_ids, &mut weights, n as usize);

        // Should be sorted ascending
        for i in 0..(n as usize) - 1 {
            assert!(node_ids[i] < node_ids[i + 1]);
            assert_eq!(node_ids[i], weights[i] as i64);
        }
    }

    #[test]
    #[should_panic(expected = "long_array too short")]
    fn test_panic_on_short_long_array() {
        let mut node_ids = vec![1, 2];
        let mut weights = vec![1.0, 2.0, 3.0];

        sort_double_array_by_long_values(&mut node_ids, &mut weights, 3);
    }

    #[test]
    #[should_panic(expected = "double_array too short")]
    fn test_panic_on_short_double_array() {
        let mut node_ids = vec![1, 2, 3];
        let mut weights = vec![1.0, 2.0];

        sort_double_array_by_long_values(&mut node_ids, &mut weights, 3);
    }

    #[test]
    fn test_indirect_mergesort() {
        let array = vec![4, 1, 8, 3, 5];
        let order = indirect_mergesort(&array);

        // Order should represent sorted indices: [1, 3, 0, 4, 2]
        // Because array[1]=1, array[3]=3, array[0]=4, array[4]=5, array[2]=8
        let sorted_values: Vec<i64> = order.iter().map(|&i| array[i]).collect();
        assert_eq!(sorted_values, vec![1, 3, 4, 5, 8]);
    }

    #[test]
    fn test_csr_adjacency_format() {
        // Typical CSR format: targets and weights need to stay synchronized
        let mut targets = vec![10, 5, 15, 3, 20];
        let mut weights = vec![1.5, 2.0, 0.5, 3.0, 1.0];

        sort_double_array_by_long_values(&mut targets, &mut weights, 5);

        // Sorted by target IDs
        assert_eq!(targets, vec![3, 5, 10, 15, 20]);
        assert_eq!(weights, vec![3.0, 2.0, 1.5, 0.5, 1.0]);
    }
}
