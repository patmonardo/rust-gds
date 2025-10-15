// Phase 1.4: FeaturesAndLabels - Training data container for link prediction

use std::marker::PhantomData;

// TODO: Replace with real types when available
pub type Features = PhantomData<()>;
pub type HugeIntArray = PhantomData<()>;

/// Pairs extracted features with their corresponding labels for training.
///
/// Used during link prediction training to bundle:
/// - **Features**: Computed link features (e.g., Hadamard, Cosine) for relationship pairs
/// - **Labels**: Binary labels (1 = positive/exists, 0 = negative/doesn't exist)
///
/// # Link Prediction Training Data
///
/// Link prediction is binary classification:
/// - **Positive examples**: Existing relationships (label = 1)
/// - **Negative examples**: Non-existent relationships from negative sampling (label = 0)
///
/// Features are computed for both positive and negative examples using link functions
/// on node properties.
///
/// # Example Flow
///
/// ```text
/// 1. Split relationships → train/test sets
/// 2. Generate negative samples → train_positive + train_negative
/// 3. Extract link features → Features (for all train pairs)
/// 4. Assign labels → HugeIntArray (1 for positive, 0 for negative)
/// 5. Bundle → FeaturesAndLabels
/// 6. Train classifier → Logistic Regression, Random Forest, etc.
/// ```
#[derive(Debug, Clone)]
pub struct FeaturesAndLabels {
    /// Extracted link features for training examples
    features: Features,

    /// Binary labels: 1 = relationship exists, 0 = doesn't exist
    labels: HugeIntArray,

    /// Cached size (number of training examples)
    size: u64,
}

impl FeaturesAndLabels {
    /// Creates a new FeaturesAndLabels from features and labels.
    ///
    /// # Arguments
    ///
    /// * `features` - Computed link features
    /// * `labels` - Binary labels (1/0)
    /// * `size` - Number of training examples
    pub fn new(features: Features, labels: HugeIntArray, size: u64) -> Self {
        Self {
            features,
            labels,
            size,
        }
    }

    /// Returns the features.
    pub fn features(&self) -> &Features {
        &self.features
    }

    /// Returns the labels.
    pub fn labels(&self) -> &HugeIntArray {
        &self.labels
    }

    /// Returns the number of training examples.
    ///
    /// Equal to the number of relationship pairs (positive + negative).
    pub fn size(&self) -> u64 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_features_and_labels_creation() {
        let features = PhantomData;
        let labels = PhantomData;
        let data = FeaturesAndLabels::new(features, labels, 1000);

        assert_eq!(data.size(), 1000);
    }

    #[test]
    fn test_accessors() {
        let features = PhantomData;
        let labels = PhantomData;
        let data = FeaturesAndLabels::new(features, labels, 500);

        let _features = data.features();
        let _labels = data.labels();
        assert_eq!(data.size(), 500);
    }

    #[test]
    fn test_clone() {
        let features = PhantomData;
        let labels = PhantomData;
        let data1 = FeaturesAndLabels::new(features, labels, 100);
        let data2 = data1.clone();

        assert_eq!(data1.size(), data2.size());
    }

    #[test]
    fn test_zero_size() {
        let features = PhantomData;
        let labels = PhantomData;
        let data = FeaturesAndLabels::new(features, labels, 0);

        assert_eq!(data.size(), 0);
    }

    #[test]
    fn test_large_size() {
        let features = PhantomData;
        let labels = PhantomData;
        let data = FeaturesAndLabels::new(features, labels, 1_000_000);

        assert_eq!(data.size(), 1_000_000);
    }

    #[test]
    fn test_typical_sizes() {
        // Typical link prediction dataset sizes
        let small = FeaturesAndLabels::new(PhantomData, PhantomData, 10_000);
        let medium = FeaturesAndLabels::new(PhantomData, PhantomData, 100_000);
        let large = FeaturesAndLabels::new(PhantomData, PhantomData, 1_000_000);

        assert_eq!(small.size(), 10_000);
        assert_eq!(medium.size(), 100_000);
        assert_eq!(large.size(), 1_000_000);
    }
}
