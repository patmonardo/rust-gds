//! Indirect comparison for zero-copy sorting algorithms
//!
//! Defines the contract for **indirect sorting operations** where comparisons are
//! performed on **indices** rather than actual data elements. This enables
//! **zero-copy sorting algorithms** that rearrange index arrays while leaving the
//! original data structures completely untouched.
//!
//! ## The Indirect Sorting Paradigm
//!
//! **Traditional sorting** operates directly on data elements - moves actual data.
//! **Indirect sorting** operates on indices that point to data - only moves indices.
//!
//! ## Graph Analytics Applications
//!
//! Critical for high-performance graph processing:
//! - Node ranking without data movement
//! - Temporal graph processing (sort edges by timestamp)
//! - Memory-intensive analytics (sort without moving large feature vectors)

use std::cmp::Ordering;

/// Defines comparison between elements by their indices in underlying data structure.
///
/// This trait enables **zero-copy sorting** by comparing elements through their
/// indices rather than moving the actual data. Essential for graph analytics where
/// data structures are large and expensive to move.
///
/// ## Mathematical Properties
///
/// Implementations **must satisfy** total ordering requirements:
///
/// - **Reflexivity**: `compare(i, i) == Ordering::Equal`
/// - **Antisymmetry**: If `compare(i, j) == Less` then `compare(j, i) == Greater`
/// - **Transitivity**: If `compare(i, j) == Less` and `compare(j, k) == Less` then `compare(i, k) == Less`
///
/// ## Performance Requirements
///
/// - **O(1) time complexity** - constant time comparison operation
/// - **Deterministic behavior** - same inputs always produce same output
/// - **Thread-safe access** - safe for concurrent read operations
///
/// # Examples
///
/// ```
/// use gds::collections::IndirectComparator;
/// use std::cmp::Ordering;
///
/// // Basic numeric comparison by index
/// struct NumericComparator<'a> {
///     data: &'a [i64],
/// }
///
/// impl<'a> IndirectComparator for NumericComparator<'a> {
///     fn compare(&self, index_a: usize, index_b: usize) -> Ordering {
///         self.data[index_a].cmp(&self.data[index_b])
///     }
/// }
///
/// let scores = vec![85, 92, 78, 96, 88];
/// let comparator = NumericComparator { data: &scores };
///
/// assert_eq!(comparator.compare(0, 1), Ordering::Less);  // 85 < 92
/// assert_eq!(comparator.compare(3, 1), Ordering::Greater); // 96 > 92
/// assert_eq!(comparator.compare(2, 2), Ordering::Equal);   // 78 == 78
/// ```
///
/// # Advanced Example: Multi-Criteria Sorting
///
/// ```
/// use gds::collections::IndirectComparator;
/// use std::cmp::Ordering;
///
/// struct NodeRankComparator<'a> {
///     page_rank_scores: &'a [f64],
///     betweenness_centrality: &'a [f64],
///     clustering_coefficient: &'a [f64],
/// }
///
/// impl<'a> IndirectComparator for NodeRankComparator<'a> {
///     fn compare(&self, index_a: usize, index_b: usize) -> Ordering {
///         // Primary criterion: PageRank (descending - higher is better)
///         let page_rank_diff = self.page_rank_scores[index_b] - self.page_rank_scores[index_a];
///         if page_rank_diff.abs() > 1e-10 {
///             return if page_rank_diff < 0.0 { Ordering::Less } else { Ordering::Greater };
///         }
///
///         // Secondary criterion: Betweenness centrality (descending)
///         let betweenness_diff = self.betweenness_centrality[index_b] - self.betweenness_centrality[index_a];
///         if betweenness_diff.abs() > 1e-10 {
///             return if betweenness_diff < 0.0 { Ordering::Less } else { Ordering::Greater };
///         }
///
///         // Tertiary criterion: Clustering coefficient (ascending)
///         let clustering_diff = self.clustering_coefficient[index_a] - self.clustering_coefficient[index_b];
///         if clustering_diff.abs() <= 1e-10 {
///             Ordering::Equal
///         } else if clustering_diff < 0.0 {
///             Ordering::Less
///         } else {
///             Ordering::Greater
///         }
///     }
/// }
/// ```
pub trait IndirectComparator {
    /// Compares two elements by their indices in the underlying data structure.
    ///
    /// This method defines the **ordering relationship** between elements at the specified
    /// indices without accessing or moving the actual data.
    ///
    /// # Arguments
    ///
    /// * `index_a` - First index to compare (must be valid for underlying data)
    /// * `index_b` - Second index to compare (must be valid for underlying data)
    ///
    /// # Returns
    ///
    /// * `Ordering::Less` - Element at `index_a` should come before element at `index_b`
    /// * `Ordering::Equal` - Elements are equivalent in sort order
    /// * `Ordering::Greater` - Element at `index_a` should come after element at `index_b`
    ///
    /// # Panics
    ///
    /// Implementations may panic if indices are out of bounds
    fn compare(&self, index_a: usize, index_b: usize) -> Ordering;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SimpleComparator<'a> {
        values: &'a [i64],
    }

    impl<'a> IndirectComparator for SimpleComparator<'a> {
        fn compare(&self, index_a: usize, index_b: usize) -> Ordering {
            self.values[index_a].cmp(&self.values[index_b])
        }
    }

    #[test]
    fn test_basic_comparison() {
        let values = vec![5, 2, 8, 1, 9];
        let comparator = SimpleComparator { values: &values };

        assert_eq!(comparator.compare(0, 1), Ordering::Greater); // 5 > 2
        assert_eq!(comparator.compare(1, 3), Ordering::Greater); // 2 > 1
        assert_eq!(comparator.compare(3, 4), Ordering::Less); // 1 < 9
        assert_eq!(comparator.compare(0, 0), Ordering::Equal); // 5 == 5
    }

    #[test]
    fn test_indirect_sorting() {
        let values = vec![5, 2, 8, 1, 9];
        let comparator = SimpleComparator { values: &values };

        let mut indices: Vec<usize> = (0..values.len()).collect();
        indices.sort_by(|&a, &b| comparator.compare(a, b));

        // Indices should be sorted to: [3, 1, 0, 2, 4]
        // Which gives values: [1, 2, 5, 8, 9]
        assert_eq!(indices, vec![3, 1, 0, 2, 4]);

        let sorted_values: Vec<i64> = indices.iter().map(|&i| values[i]).collect();
        assert_eq!(sorted_values, vec![1, 2, 5, 8, 9]);
    }
}
