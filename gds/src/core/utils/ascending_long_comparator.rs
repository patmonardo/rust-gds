//! Indirect sorting comparator for ascending order.
//!
//! Provides high-performance indirect sorting where index arrays are sorted
//! based on values in a separate data array, without moving the original data.
//!
//! # Indirect Sorting
//!
//! **Traditional sorting** physically rearranges array elements, which is expensive for:
//! - Large objects (graph nodes with many properties)
//! - Synchronized arrays (node IDs with corresponding edge lists)
//! - Memory-mapped data (persistent graph storage)
//! - Cache-sensitive operations (avoiding memory fragmentation)
//!
//! **Indirect sorting** creates a permutation index array representing sorted order
//! without touching the original data.
//!
//! # Performance Benefits
//!
//! - Zero data movement - only index manipulation
//! - Cache-friendly - better memory locality
//! - Preserves relationships - parallel arrays stay synchronized
//! - Memory efficient - no temporary copies of large data
//!
//! # Graph Analytics Applications
//!
//! - **Node Ranking**: Sort nodes by PageRank score without moving node data
//! - **Temporal Processing**: Sort edges by timestamp while preserving metadata
//! - **Community Detection**: Group nodes by community without disrupting adjacency
//! - **Edge Weighting**: Sort edges for MST algorithms (Kruskal's, Prim's)
//!
//! # Examples
//!
//! ```
//! use gds::core::utils::AscendingLongComparator;
//!
//! // Sort nodes by PageRank scores using indirect sorting
//! let pagerank_scores = vec![0.1, 0.4, 0.2, 0.3];
//! let comparator = AscendingLongComparator::new(&pagerank_scores);
//!
//! // Create index array [0, 1, 2, 3]
//! let mut indices: Vec<usize> = (0..pagerank_scores.len()).collect();
//!
//! // Sort indices by the values they point to
//! indices.sort_by(|&a, &b| comparator.compare(a, b));
//!
//! // Indices now represent sorted order: [0, 2, 3, 1]
//! // Access sorted values: 0.1, 0.2, 0.3, 0.4
//! assert_eq!(indices, vec![0, 2, 3, 1]);
//! ```

use std::cmp::Ordering;

/// Indirect comparator for sorting indices based on ascending i64 values.
///
/// This comparator enables indirect sorting where an array of indices is sorted
/// based on the values they reference in a separate data array. The original
/// data array remains unchanged.
///
/// # Mathematical Properties
///
/// Implements a total ordering with:
/// - **Reflexive**: compare(a, a) = Equal
/// - **Antisymmetric**: if compare(a, b) = Less then compare(b, a) = Greater
/// - **Transitive**: if compare(a, b) = Less and compare(b, c) = Less then compare(a, c) = Less
/// - **Total**: for any a, b, exactly one of Less, Greater, or Equal holds
///
/// # Thread Safety
///
/// The comparator is read-only and thread-safe as long as the underlying
/// array is not modified during sorting operations.
///
/// # Examples
///
/// ```
/// use gds::core::utils::AscendingLongComparator;
/// use std::cmp::Ordering;
///
/// // Graph node degrees for hub analysis
/// let node_degrees = vec![12, 5, 23, 8, 15, 3, 19];
/// let comparator = AscendingLongComparator::new(&node_degrees);
///
/// // Compare two nodes by their degrees
/// assert_eq!(comparator.compare(1, 0), Ordering::Less); // 5 < 12
/// assert_eq!(comparator.compare(2, 6), Ordering::Greater); // 23 > 19
/// assert_eq!(comparator.compare(1, 1), Ordering::Equal); // 5 == 5
///
/// // Sort indices by ascending degree
/// let mut indices: Vec<usize> = (0..node_degrees.len()).collect();
/// indices.sort_by(|&a, &b| comparator.compare(a, b));
///
/// // Low-degree nodes first
/// assert_eq!(indices[0], 5); // degree 3
/// assert_eq!(indices[1], 1); // degree 5
/// assert_eq!(indices[6], 2); // degree 23 (highest)
/// ```
pub struct AscendingLongComparator<'a> {
    array: &'a [i64],
}

impl<'a> AscendingLongComparator<'a> {
    /// Creates a new ascending comparator for an i64 array.
    ///
    /// Initializes the comparator to work with the specified array for indirect sorting.
    /// The comparator maintains a read-only reference to the array and provides
    /// comparison operations based on the values at given indices.
    ///
    /// # Arguments
    ///
    /// * `array` - The array to compare elements from
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::AscendingLongComparator;
    ///
    /// // Edge weights for MST algorithms
    /// let edge_weights = vec![80, 20, 90, 10, 50, 70];
    /// let comparator = AscendingLongComparator::new(&edge_weights);
    ///
    /// // Use for Kruskal's algorithm: process edges in ascending weight order
    /// let mut edge_indices: Vec<usize> = (0..edge_weights.len()).collect();
    /// edge_indices.sort_by(|&a, &b| comparator.compare(a, b));
    ///
    /// // Lightest edges first
    /// assert_eq!(edge_weights[edge_indices[0]], 10);
    /// assert_eq!(edge_weights[edge_indices[1]], 20);
    /// ```
    pub fn new(array: &'a [i64]) -> Self {
        Self { array }
    }

    /// Compares two array elements by their indices using ascending order.
    ///
    /// This is the core comparison operation for indirect sorting algorithms.
    /// It compares values in the original array without moving them.
    ///
    /// # Arguments
    ///
    /// * `index_a` - First index to compare
    /// * `index_b` - Second index to compare
    ///
    /// # Returns
    ///
    /// * `Ordering::Less` if `array[index_a] < array[index_b]`
    /// * `Ordering::Greater` if `array[index_a] > array[index_b]`
    /// * `Ordering::Equal` if `array[index_a] == array[index_b]`
    ///
    /// # Panics
    ///
    /// Panics if either index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::AscendingLongComparator;
    /// use std::cmp::Ordering;
    ///
    /// let values = vec![100, 50, 200, 75];
    /// let comparator = AscendingLongComparator::new(&values);
    ///
    /// assert_eq!(comparator.compare(0, 1), Ordering::Greater); // 100 > 50
    /// assert_eq!(comparator.compare(1, 2), Ordering::Less);    // 50 < 200
    /// assert_eq!(comparator.compare(1, 1), Ordering::Equal);   // 50 == 50
    /// ```
    #[inline]
    pub fn compare(&self, index_a: usize, index_b: usize) -> Ordering {
        let a = self.array[index_a];
        let b = self.array[index_b];

        if a < b {
            Ordering::Less
        } else if a > b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// Gets the underlying array being compared.
    ///
    /// Provides read-only access to the array for debugging and introspection.
    ///
    /// # Returns
    ///
    /// Reference to the underlying array.
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::core::utils::AscendingLongComparator;
    ///
    /// let scores = vec![10, 40, 20, 30];
    /// let comparator = AscendingLongComparator::new(&scores);
    ///
    /// // Access underlying data for validation
    /// let data = comparator.array();
    /// assert_eq!(data.len(), 4);
    /// assert_eq!(data[0], 10);
    /// ```
    #[inline]
    pub fn array(&self) -> &[i64] {
        self.array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_comparison() {
        let values = vec![100, 50, 200, 75];
        let comparator = AscendingLongComparator::new(&values);

        assert_eq!(comparator.compare(0, 1), Ordering::Greater); // 100 > 50
        assert_eq!(comparator.compare(1, 2), Ordering::Less); // 50 < 200
        assert_eq!(comparator.compare(1, 1), Ordering::Equal); // 50 == 50
        assert_eq!(comparator.compare(2, 3), Ordering::Greater); // 200 > 75
    }

    #[test]
    fn test_indirect_sorting() {
        let pagerank_scores = vec![15, 35, 8, 42];
        let comparator = AscendingLongComparator::new(&pagerank_scores);

        let mut indices: Vec<usize> = (0..pagerank_scores.len()).collect();
        indices.sort_by(|&a, &b| comparator.compare(a, b));

        // Should be sorted by ascending PageRank: [8, 15, 35, 42]
        assert_eq!(indices, vec![2, 0, 1, 3]);

        // Verify sorted order
        for i in 0..indices.len() - 1 {
            let curr_val = pagerank_scores[indices[i]];
            let next_val = pagerank_scores[indices[i + 1]];
            assert!(curr_val <= next_val);
        }
    }

    #[test]
    fn test_negative_values() {
        let values = vec![-10, 5, -20, 15, 0];
        let comparator = AscendingLongComparator::new(&values);

        let mut indices: Vec<usize> = (0..values.len()).collect();
        indices.sort_by(|&a, &b| comparator.compare(a, b));

        // Should be: [-20, -10, 0, 5, 15]
        assert_eq!(indices, vec![2, 0, 4, 1, 3]);
    }

    #[test]
    fn test_equal_values() {
        let values = vec![10, 20, 10, 30, 20];
        let comparator = AscendingLongComparator::new(&values);

        assert_eq!(comparator.compare(0, 2), Ordering::Equal); // Both 10
        assert_eq!(comparator.compare(1, 4), Ordering::Equal); // Both 20

        let mut indices: Vec<usize> = (0..values.len()).collect();
        indices.sort_by(|&a, &b| comparator.compare(a, b));

        // Equal values maintain relative order (stable sort)
        // Should be: [10, 10, 20, 20, 30]
        assert_eq!(values[indices[0]], 10);
        assert_eq!(values[indices[1]], 10);
        assert_eq!(values[indices[2]], 20);
        assert_eq!(values[indices[3]], 20);
        assert_eq!(values[indices[4]], 30);
    }

    #[test]
    fn test_already_sorted() {
        let values = vec![1, 2, 3, 4, 5];
        let comparator = AscendingLongComparator::new(&values);

        let mut indices: Vec<usize> = (0..values.len()).collect();
        indices.sort_by(|&a, &b| comparator.compare(a, b));

        // Should remain [0, 1, 2, 3, 4]
        assert_eq!(indices, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_reverse_sorted() {
        let values = vec![50, 40, 30, 20, 10];
        let comparator = AscendingLongComparator::new(&values);

        let mut indices: Vec<usize> = (0..values.len()).collect();
        indices.sort_by(|&a, &b| comparator.compare(a, b));

        // Should reverse: [4, 3, 2, 1, 0]
        assert_eq!(indices, vec![4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_single_element() {
        let values = vec![42];
        let comparator = AscendingLongComparator::new(&values);

        assert_eq!(comparator.compare(0, 0), Ordering::Equal);

        let mut indices: Vec<usize> = (0..values.len()).collect();
        indices.sort_by(|&a, &b| comparator.compare(a, b));

        assert_eq!(indices, vec![0]);
    }

    #[test]
    fn test_edge_weights_mst() {
        // Kruskal's algorithm scenario
        let edge_weights = vec![80, 20, 90, 10, 50, 70];
        let comparator = AscendingLongComparator::new(&edge_weights);

        let mut edge_indices: Vec<usize> = (0..edge_weights.len()).collect();
        edge_indices.sort_by(|&a, &b| comparator.compare(a, b));

        // Lightest edges first for MST
        assert_eq!(edge_weights[edge_indices[0]], 10);
        assert_eq!(edge_weights[edge_indices[1]], 20);
        assert_eq!(edge_weights[edge_indices[2]], 50);
        assert_eq!(edge_weights[edge_indices[3]], 70);
        assert_eq!(edge_weights[edge_indices[4]], 80);
        assert_eq!(edge_weights[edge_indices[5]], 90);
    }

    #[test]
    fn test_node_degrees() {
        // Hub identification scenario
        let node_degrees = vec![12, 5, 23, 8, 15, 3, 19];
        let comparator = AscendingLongComparator::new(&node_degrees);

        let mut indices: Vec<usize> = (0..node_degrees.len()).collect();
        indices.sort_by(|&a, &b| comparator.compare(a, b));

        // Low-degree nodes first
        assert_eq!(indices[0], 5); // degree 3
        assert_eq!(indices[1], 1); // degree 5
        assert_eq!(indices[6], 2); // degree 23 (highest)
    }

    #[test]
    fn test_array_access() {
        let values = vec![100, 200, 300];
        let comparator = AscendingLongComparator::new(&values);

        let array = comparator.array();
        assert_eq!(array.len(), 3);
        assert_eq!(array[0], 100);
        assert_eq!(array[1], 200);
        assert_eq!(array[2], 300);
    }

    #[test]
    fn test_large_values() {
        let values = vec![i64::MAX, i64::MIN, 0, 1, -1];
        let comparator = AscendingLongComparator::new(&values);

        let mut indices: Vec<usize> = (0..values.len()).collect();
        indices.sort_by(|&a, &b| comparator.compare(a, b));

        // Should be: [MIN, -1, 0, 1, MAX]
        assert_eq!(indices, vec![1, 4, 2, 3, 0]);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_a() {
        let values = vec![1, 2, 3];
        let comparator = AscendingLongComparator::new(&values);

        comparator.compare(5, 1); // Should panic
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds_b() {
        let values = vec![1, 2, 3];
        let comparator = AscendingLongComparator::new(&values);

        comparator.compare(1, 5); // Should panic
    }
}
