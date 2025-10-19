// Phase 1.2: LinkFeatureAppender - Feature appending for link pairs

// TODO: Replace u64 with proper NodeId type when available

/// Appends features for a specific graph's link pairs.
///
/// Responsible for computing and appending features for (source, target) node pairs.
/// Instances are graph-specific and should not be reused across different graphs.
///
/// # Feature Computation
///
/// Link features are computed from node property pairs using mathematical operations:
/// - **Hadamard**: element-wise multiplication `v1[i] * v2[i]`
/// - **Cosine**: angular similarity `dot(v1, v2) / (norm(v1) * norm(v2))`
/// - **L2**: Euclidean distance `sqrt(sum((v1[i] - v2[i])^2))`
/// - **SameCategory**: categorical equality `v1 == v2 ? 1.0 : 0.0`
///
/// # Examples
///
/// ```rust,ignore
/// let appender = HadamardFeatureAppender::new(node_properties);
/// let mut features = vec![0.0; appender.dimension()];
/// appender.append_features(source_id, target_id, &mut features, 0);
/// ```
pub trait LinkFeatureAppender: Send + Sync {
    /// Appends features for the (source, target) pair to the linkFeatures array.
    ///
    /// # Arguments
    ///
    /// * `source` - Source node ID
    /// * `target` - Target node ID  
    /// * `link_features` - Feature array to append to
    /// * `offset` - Starting position in link_features where features should be written
    ///
    /// # Behavior
    ///
    /// Writes `dimension()` feature values starting at `link_features[offset]`.
    /// Does not check array bounds - caller must ensure sufficient capacity.
    fn append_features(&self, source: u64, target: u64, link_features: &mut [f64], offset: usize);

    /// Returns the number of features this appender will add.
    ///
    /// Used to:
    /// - Pre-allocate feature arrays
    /// - Calculate offsets for multiple appenders
    /// - Validate feature dimensions
    fn dimension(&self) -> usize;

    /// Returns whether the feature computation is symmetric.
    ///
    /// If `true`, `f(source, target) == f(target, source)` for all node pairs.
    /// This allows optimization: compute once, reuse for both directions.
    ///
    /// # Symmetry by Feature Type
    ///
    /// - **Hadamard**: symmetric (multiplication is commutative)
    /// - **Cosine**: symmetric (dot product is commutative)
    /// - **L2**: symmetric (distance is symmetric)
    /// - **SameCategory**: symmetric (equality is symmetric)
    ///
    /// Default: `true` (most link features are symmetric)
    fn is_symmetric(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestAppender {
        dim: usize,
        symmetric: bool,
    }

    impl LinkFeatureAppender for TestAppender {
        fn append_features(
            &self,
            source: NodeId,
            target: NodeId,
            link_features: &mut [f64],
            offset: usize,
        ) {
            // Test implementation: just write source + target as features
            for i in 0..self.dim {
                link_features[offset + i] = (source + target) as f64 + i as f64;
            }
        }

        fn dimension(&self) -> usize {
            self.dim
        }

        fn is_symmetric(&self) -> bool {
            self.symmetric
        }
    }

    #[test]
    fn test_appender_basic() {
        let appender = TestAppender {
            dim: 3,
            symmetric: true,
        };

        assert_eq!(appender.dimension(), 3);
        assert!(appender.is_symmetric());
    }

    #[test]
    fn test_append_features() {
        let appender = TestAppender {
            dim: 2,
            symmetric: true,
        };
        let mut features = vec![0.0; 5];

        // Append at offset 1
        appender.append_features(10, 20, &mut features, 1);

        assert_eq!(features[0], 0.0); // Before offset - unchanged
        assert_eq!(features[1], 30.0); // 10 + 20 + 0
        assert_eq!(features[2], 31.0); // 10 + 20 + 1
        assert_eq!(features[3], 0.0); // After features - unchanged
    }

    #[test]
    fn test_symmetric_flag() {
        let symmetric = TestAppender {
            dim: 1,
            symmetric: true,
        };
        let asymmetric = TestAppender {
            dim: 1,
            symmetric: false,
        };

        assert!(symmetric.is_symmetric());
        assert!(!asymmetric.is_symmetric());
    }

    #[test]
    fn test_multiple_appends() {
        let appender = TestAppender {
            dim: 2,
            symmetric: true,
        };
        let mut features = vec![0.0; 6];

        // First append at offset 0
        appender.append_features(1, 2, &mut features, 0);
        // Second append at offset 2
        appender.append_features(3, 4, &mut features, 2);
        // Third append at offset 4
        appender.append_features(5, 6, &mut features, 4);

        assert_eq!(features[0], 3.0); // 1 + 2 + 0
        assert_eq!(features[1], 4.0); // 1 + 2 + 1
        assert_eq!(features[2], 7.0); // 3 + 4 + 0
        assert_eq!(features[3], 8.0); // 3 + 4 + 1
        assert_eq!(features[4], 11.0); // 5 + 6 + 0
        assert_eq!(features[5], 12.0); // 5 + 6 + 1
    }

    #[test]
    fn test_dimension_zero() {
        let appender = TestAppender {
            dim: 0,
            symmetric: true,
        };
        let mut features = vec![1.0, 2.0, 3.0];

        appender.append_features(10, 20, &mut features, 1);

        // Zero-dimensional appender doesn't modify anything
        assert_eq!(features, vec![1.0, 2.0, 3.0]);
    }
}
