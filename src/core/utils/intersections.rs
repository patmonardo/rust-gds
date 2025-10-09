// Copyright (c) "Neo4j"
// Neo4j Sweden AB [http://neo4j.com]
//
// This file is part of Neo4j.
//
// Neo4j is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! High-performance intersection and similarity operations for graph algorithms.
//!
//! This module provides optimized implementations for:
//! - Set intersections (multiple algorithms for different use cases)
//! - Vector similarity metrics (cosine, Pearson correlation)
//! - Distance calculations (sum of squared deltas)
//!
//! ## Performance Notes
//! - `intersection3`/`intersection4`: Optimized for sorted arrays (O(n+m) complexity)
//! - `intersection`/`intersection2`: Use hash sets for unsorted data
//! - Vector operations: Optimized for numerical stability and performance

use std::collections::HashSet;

/// High-performance intersection and similarity operations.
pub struct Intersections;

impl Intersections {
    /// Computes intersection size using hash sets.
    ///
    /// Best for: Unsorted data, moderate sizes, when you need exact intersection count.
    ///
    /// # Complexity
    /// - Time: O(n + m) where n, m are set sizes
    /// - Space: O(min(n, m)) for the intersection set
    pub fn intersection(targets1: &HashSet<i64>, targets2: &HashSet<i64>) -> usize {
        let mut intersection_set = targets1.clone();
        intersection_set.retain(|k| targets2.contains(k));
        intersection_set.len()
    }

    /// Computes intersection size by converting arrays to hash sets.
    ///
    /// Best for: Unsorted arrays, when you don't already have sets.
    ///
    /// # Complexity
    /// - Time: O(n + m)
    /// - Space: O(n + m) for both hash sets
    pub fn intersection2(targets1: &[i64], targets2: &[i64]) -> usize {
        let set1: HashSet<i64> = targets1.iter().copied().collect();
        let set2: HashSet<i64> = targets2.iter().copied().collect();
        Self::intersection(&set1, &set2)
    }

    /// Optimized intersection for sorted arrays - merge-like algorithm.
    ///
    /// **REQUIRES:** Both arrays must be sorted in ascending order!
    ///
    /// Best for: Large sorted arrays, when you want maximum performance.
    /// Use case: Graph adjacency lists (neighbors are often sorted by ID)
    ///
    /// # Complexity
    /// - Time: O(n + m) - single pass through both arrays
    /// - Space: O(1) - no additional data structures
    pub fn intersection3(targets1: &[i64], targets2: &[i64]) -> usize {
        let len2 = targets2.len();
        if len2 == 0 {
            return 0;
        }

        let mut off2 = 0;
        let mut intersection = 0;

        for &value1 in targets1 {
            if value1 > targets2[off2] {
                while {
                    off2 += 1;
                    off2 != len2 && value1 > targets2[off2]
                } {}
                if off2 == len2 {
                    return intersection;
                }
            }
            if value1 == targets2[off2] {
                intersection += 1;
                off2 += 1;
                if off2 == len2 {
                    return intersection;
                }
            }
        }

        intersection
    }

    /// Intersection with explicit length parameters - for working with array slices.
    ///
    /// **REQUIRES:** Both arrays must be sorted, len1 <= targets1.len(), len2 <= targets2.len()
    ///
    /// Best for: When you need to process only part of arrays without copying.
    ///
    /// # Complexity
    /// - Time: O(len1 + len2)
    /// - Space: O(1)
    pub fn intersection_arrays_with_length(
        targets1: &[i64],
        targets2: &[i64],
        len1: usize,
        len2: usize,
    ) -> usize {
        debug_assert!(len1 <= targets1.len());
        debug_assert!(len2 <= targets2.len());

        if len2 == 0 {
            return 0;
        }

        let mut off2 = 0;
        let mut intersection = 0;
        let mut idx1 = 0;

        while idx1 < len1 {
            let value1 = targets1[idx1];

            if value1 > targets2[off2] {
                while {
                    off2 += 1;
                    off2 != len2 && value1 > targets2[off2]
                } {}
                if off2 == len2 {
                    return intersection;
                }
            }

            if value1 == targets2[off2] {
                intersection += 1;
                off2 += 1;
                if off2 == len2 {
                    return intersection;
                }
            }

            idx1 += 1;
        }

        intersection
    }

    /// Alternative sorted intersection algorithm with different loop structure.
    ///
    /// **REQUIRES:** Both arrays must be sorted in ascending order!
    ///
    /// Best for: Experimentation with different CPU cache behaviors.
    ///
    /// # Complexity
    /// - Time: O(n + m)
    /// - Space: O(1)
    pub fn intersection4(targets1: &[i64], targets2: &[i64]) -> usize {
        if targets2.is_empty() {
            return 0;
        }

        let mut off2 = 0;
        let mut intersection = 0;

        for off1 in 0..targets1.len() {
            if off2 == targets2.len() {
                return intersection;
            }

            let value1 = targets1[off1];

            if value1 > targets2[off2] {
                while off2 < targets2.len() {
                    if value1 <= targets2[off2] {
                        break;
                    }
                    off2 += 1;
                }
                if off2 == targets2.len() {
                    return intersection;
                }
            }

            if value1 == targets2[off2] {
                intersection += 1;
                off2 += 1;
            }
        }

        intersection
    }

    /// Computes sum of squared differences between two vectors.
    ///
    /// Best for: Euclidean distance calculations, k-means clustering, outlier detection.
    ///
    /// Mathematical formula: Σ(vector1[i] - vector2[i])²
    ///
    /// # Complexity
    /// - Time: O(len)
    /// - Space: O(1)
    pub fn sum_square_delta_f64(vector1: &[f64], vector2: &[f64], len: usize) -> f64 {
        let mut result = 0.0;
        for i in 0..len {
            let delta = vector1[i] - vector2[i];
            result += delta * delta;
        }
        result
    }

    /// Float32 version of sum_square_delta for memory-efficient operations.
    ///
    /// Best for: Large datasets where memory usage matters more than precision.
    pub fn sum_square_delta_f32(vector1: &[f32], vector2: &[f32], len: usize) -> f32 {
        let mut result = 0.0;
        for i in 0..len {
            let delta = vector1[i] - vector2[i];
            result += delta * delta;
        }
        result
    }

    /// Computes sum of squared deltas between one vector and multiple vectors.
    ///
    /// Best for: Finding nearest neighbors, batch similarity calculations.
    ///
    /// # Returns
    /// Array where result[j] = sum_square_delta(vector1, vector2[j], len)
    ///
    /// # Complexity
    /// - Time: O(len * vectors) where vectors = vector2.len()
    /// - Space: O(vectors) for result array
    pub fn sum_square_deltas(vector1: &[f64], vector2: &[&[f64]], len: usize) -> Vec<f64> {
        let vectors = vector2.len();
        let mut result = vec![0.0; vectors];

        for i in 0..len {
            let v1 = vector1[i];
            for j in 0..vectors {
                let delta = v1 - vector2[j][i];
                result[j] += delta * delta;
            }
        }

        result
    }

    /// Computes Pearson correlation coefficient between two vectors.
    ///
    /// Best for: Measuring linear relationships, recommendation systems, feature correlation.
    ///
    /// # Returns
    /// Value between -1 and 1, where:
    /// - 1 = perfect positive correlation
    /// - 0 = no linear correlation
    /// - -1 = perfect negative correlation
    ///
    /// # Complexity
    /// - Time: O(len) - two passes through data
    /// - Space: O(1)
    pub fn pearson(vector1: &[f64], vector2: &[f64], len: usize) -> f64 {
        let mut vector1_sum = 0.0;
        let mut vector2_sum = 0.0;

        for i in 0..len {
            vector1_sum += vector1[i];
            vector2_sum += vector2[i];
        }

        let vector1_mean = vector1_sum / len as f64;
        let vector2_mean = vector2_sum / len as f64;

        let mut dot_product_minus_mean = 0.0;
        let mut x_length = 0.0;
        let mut y_length = 0.0;

        for i in 0..len {
            let vector1_delta = vector1[i] - vector1_mean;
            let vector2_delta = vector2[i] - vector2_mean;

            dot_product_minus_mean += vector1_delta * vector2_delta;
            x_length += vector1_delta * vector1_delta;
            y_length += vector2_delta * vector2_delta;
        }

        let result = dot_product_minus_mean / (x_length * y_length).sqrt();
        if result.is_nan() {
            0.0
        } else {
            result
        }
    }

    /// Computes cosine similarity between two vectors.
    ///
    /// Best for: Document similarity, recommendation systems, high-dimensional data.
    ///
    /// # Returns
    /// Value between -1 and 1, where:
    /// - 1 = vectors point in same direction
    /// - 0 = vectors are orthogonal
    /// - -1 = vectors point in opposite directions
    ///
    /// Mathematical formula: (v1 · v2) / (||v1|| × ||v2||)
    ///
    /// # Complexity
    /// - Time: O(len) - single pass
    /// - Space: O(1)
    pub fn cosine_f64(vector1: &[f64], vector2: &[f64], len: usize) -> f64 {
        let mut dot_product = 0.0;
        let mut x_length = 0.0;
        let mut y_length = 0.0;

        for i in 0..len {
            let weight1 = vector1[i];
            let weight2 = vector2[i];

            dot_product += weight1 * weight2;
            x_length += weight1 * weight1;
            y_length += weight2 * weight2;
        }

        dot_product / (x_length * y_length).sqrt()
    }

    /// Float32 version of cosine similarity for memory-efficient operations.
    ///
    /// Best for: Large datasets, GPU acceleration, memory-constrained environments.
    pub fn cosine_f32(vector1: &[f32], vector2: &[f32], len: usize) -> f32 {
        let mut dot_product = 0.0;
        let mut x_length = 0.0;
        let mut y_length = 0.0;

        for i in 0..len {
            let weight1 = vector1[i];
            let weight2 = vector2[i];

            dot_product += weight1 * weight2;
            x_length += weight1 * weight1;
            y_length += weight2 * weight2;
        }

        (dot_product / (x_length * y_length).sqrt()) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection3_sorted() {
        let arr1 = vec![1, 3, 5, 7, 9];
        let arr2 = vec![2, 3, 5, 8, 9];

        let result = Intersections::intersection3(&arr1, &arr2);
        assert_eq!(result, 3); // 3, 5, 9
    }

    #[test]
    fn test_cosine_similarity() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![2.0, 4.0, 6.0];

        let result = Intersections::cosine_f64(&v1, &v2, 3);
        assert!((result - 1.0).abs() < 1e-10); // Perfect correlation
    }
}
