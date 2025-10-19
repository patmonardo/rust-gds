// Phase 1.3: ExpectedSetSizes - Split size calculations for link prediction

/// Expected sizes of train/test/validation splits for link prediction.
///
/// Used by `LinkPredictionSplitConfig` to validate that relationship counts
/// are sufficient for the requested split configuration. Link prediction requires
/// both positive examples (existing relationships) and negative examples (non-existent
/// relationships generated via negative sampling).
///
/// # Split Strategy
///
/// Link prediction splits relationships (not nodes) into:
/// 1. **Test set**: Hold-out for final evaluation
/// 2. **Train set**: Used for model training  
/// 3. **Feature input**: Used for computing link features (may overlap with train)
/// 4. **Test complement**: Remaining relationships (train + feature input)
/// 5. **Validation folds**: K-fold splits of train set for cross-validation
///
/// # Example
///
/// ```rust,ignore
/// let sizes = ExpectedSetSizes {
///     test_size: 1000,
///     feature_input_size: 8000,
///     train_size: 1000,
///     test_complement_size: 9000,
///     validation_fold_size: 333,
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectedSetSizes {
    /// Number of relationships in test set (positive + negative examples)
    pub test_size: u64,

    /// Number of relationships available for feature computation
    pub feature_input_size: u64,

    /// Number of relationships in training set (positive + negative examples)
    pub train_size: u64,

    /// Number of relationships NOT in test set (feature_input_size + train_size - overlap)
    pub test_complement_size: u64,

    /// Number of relationships per validation fold in cross-validation
    pub validation_fold_size: u64,
}

impl ExpectedSetSizes {
    /// Creates a new ExpectedSetSizes with the given values.
    pub fn new(
        test_size: u64,
        feature_input_size: u64,
        train_size: u64,
        test_complement_size: u64,
        validation_fold_size: u64,
    ) -> Self {
        Self {
            test_size,
            feature_input_size,
            train_size,
            test_complement_size,
            validation_fold_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expected_set_sizes_creation() {
        let sizes = ExpectedSetSizes::new(1000, 8000, 1000, 9000, 333);

        assert_eq!(sizes.test_size, 1000);
        assert_eq!(sizes.feature_input_size, 8000);
        assert_eq!(sizes.train_size, 1000);
        assert_eq!(sizes.test_complement_size, 9000);
        assert_eq!(sizes.validation_fold_size, 333);
    }

    #[test]
    fn test_struct_literal_creation() {
        let sizes = ExpectedSetSizes {
            test_size: 500,
            feature_input_size: 4000,
            train_size: 500,
            test_complement_size: 4500,
            validation_fold_size: 166,
        };

        assert_eq!(sizes.test_size, 500);
        assert_eq!(sizes.train_size, 500);
    }

    #[test]
    fn test_copy_trait() {
        let sizes1 = ExpectedSetSizes::new(100, 800, 100, 900, 33);
        let sizes2 = sizes1; // Copy

        // Both still accessible (Copy trait works)
        assert_eq!(sizes1.test_size, 100);
        assert_eq!(sizes2.test_size, 100);
    }

    #[test]
    fn test_clone_trait() {
        let sizes1 = ExpectedSetSizes::new(200, 1600, 200, 1800, 66);
        let sizes2 = sizes1.clone();

        assert_eq!(sizes1, sizes2);
    }

    #[test]
    fn test_equality() {
        let sizes1 = ExpectedSetSizes::new(1000, 8000, 1000, 9000, 333);
        let sizes2 = ExpectedSetSizes::new(1000, 8000, 1000, 9000, 333);
        let sizes3 = ExpectedSetSizes::new(1000, 8000, 1000, 9000, 334);

        assert_eq!(sizes1, sizes2);
        assert_ne!(sizes1, sizes3);
    }

    #[test]
    fn test_zero_sizes() {
        let sizes = ExpectedSetSizes::new(0, 0, 0, 0, 0);

        assert_eq!(sizes.test_size, 0);
        assert_eq!(sizes.train_size, 0);
    }

    #[test]
    fn test_large_sizes() {
        let sizes = ExpectedSetSizes::new(1_000_000, 8_000_000, 1_000_000, 9_000_000, 333_333);

        assert_eq!(sizes.test_size, 1_000_000);
        assert_eq!(sizes.validation_fold_size, 333_333);
    }
}
