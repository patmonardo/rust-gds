// Phase 4.3: LinkPredictionSplitConfig - Split configuration for link prediction training

use super::ExpectedSetSizes;
use crate::projection::RelationshipType;
use std::collections::HashMap;
use std::marker::PhantomData;

/// Link prediction split configuration.
///
/// # Prim and Proper! 🌟
///
/// **The Core Duality of our ML Pipeline**:
/// - **Prim** = Primitive Values (scalars: testFraction, trainFraction, negativeSamplingRatio)
/// - **Proper** = Property Values (graph properties: relationshipTypes, node properties)
///
/// **Software based on Prim and Proper**!
///
/// This config embodies both:
/// - **Prim**: The Given scalar configuration (fractions, folds, ratios)
/// - **Proper**: The Form configuration (relationship types for splits)
///
/// # The Split Philosophy
///
/// Link prediction training splits relationships into:
/// 1. **Test Set** (testFraction) - For final evaluation
/// 2. **Test Complement** (1 - testFraction) - For training
/// 3. **Train Set** (trainFraction of complement) - For model training
/// 4. **Feature Input** (1 - trainFraction of complement) - For feature extraction
///
/// Each fold in cross-validation further splits the Train Set.
///
/// # Negative Sampling
///
/// For each positive (existing) relationship, we generate negative (non-existing) samples:
/// - **negativeSamplingRatio**: How many negatives per positive (default 1.0 = balanced)
/// - **negativeRelationshipType**: Optional explicit negative edges (overrides ratio)
///
/// # Reserved Relationship Types
///
/// The pipeline creates temporary relationship types:
/// - `_TEST_` - Test set relationships
/// - `_TEST_COMPLEMENT_` - Test complement (train + feature input)
/// - `_TRAIN_` - Training relationships
/// - `_FEATURE_INPUT_` - Feature extraction relationships
///
/// These types **must not** exist in the input graph!
///
/// # Example
///
/// ```text
/// let config = LinkPredictionSplitConfig::builder()
///     .validation_folds(3)
///     .test_fraction(0.1)      // 10% for test
///     .train_fraction(0.1)     // 10% of remaining for train
///     .negative_sampling_ratio(1.0)  // 1:1 positive:negative
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct LinkPredictionSplitConfig {
    // === PRIM: Primitive Configuration Values ===
    /// Number of validation folds for cross-validation (default: 3)
    /// **Prim**: Integer scalar
    validation_folds: u32,

    /// Fraction of relationships to use for test set (default: 0.1)
    /// Range: (0.0, 1.0) exclusive
    /// **Prim**: Double scalar
    test_fraction: f64,

    /// Fraction of test complement to use for train set (default: 0.1)
    /// Range: (0.0, 1.0) exclusive
    /// **Prim**: Double scalar
    train_fraction: f64,

    /// Ratio of negative to positive samples (default: 1.0)
    /// Range: (0.0, ∞) exclusive
    /// **Prim**: Double scalar
    negative_sampling_ratio: f64,

    // === PROPER: Property Configuration Values ===
    /// Optional explicit negative relationship type
    /// If present, overrides negativeSamplingRatio
    /// **Proper**: Relationship type (graph property)
    negative_relationship_type: Option<String>,

    /// Test set relationship type (default: "_TEST_")
    /// **Proper**: Relationship type (reserved)
    test_relationship_type: RelationshipType,

    /// Test complement relationship type (default: "_TEST_COMPLEMENT_")
    /// **Proper**: Relationship type (reserved)
    test_complement_relationship_type: RelationshipType,

    /// Train set relationship type (default: "_TRAIN_")
    /// **Proper**: Relationship type (reserved)
    train_relationship_type: RelationshipType,

    /// Feature input relationship type (default: "_FEATURE_INPUT_")
    /// **Proper**: Relationship type (reserved)
    feature_input_relationship_type: RelationshipType,
}

impl LinkPredictionSplitConfig {
    /// Configuration key for test fraction
    pub const TEST_FRACTION_KEY: &'static str = "testFraction";

    /// Configuration key for train fraction
    pub const TRAIN_FRACTION_KEY: &'static str = "trainFraction";

    /// Creates a builder for LinkPredictionSplitConfig.
    ///
    /// # The Prim and Proper Builder!
    ///
    /// Constructs config with both Prim (scalars) and Proper (types).
    pub fn builder() -> LinkPredictionSplitConfigBuilder {
        LinkPredictionSplitConfigBuilder::new()
    }

    /// Returns the number of validation folds.
    /// **Prim**: Integer scalar getter
    pub fn validation_folds(&self) -> u32 {
        self.validation_folds
    }

    /// Returns the test fraction.
    /// **Prim**: Double scalar getter
    pub fn test_fraction(&self) -> f64 {
        self.test_fraction
    }

    /// Returns the train fraction.
    /// **Prim**: Double scalar getter
    pub fn train_fraction(&self) -> f64 {
        self.train_fraction
    }

    /// Returns the negative sampling ratio.
    /// **Prim**: Double scalar getter
    pub fn negative_sampling_ratio(&self) -> f64 {
        self.negative_sampling_ratio
    }

    /// Returns the optional negative relationship type.
    /// **Proper**: Relationship type getter
    pub fn negative_relationship_type(&self) -> Option<&str> {
        self.negative_relationship_type.as_deref()
    }

    /// Returns the test relationship type.
    /// **Proper**: Reserved type getter
    pub fn test_relationship_type(&self) -> &RelationshipType {
        &self.test_relationship_type
    }

    /// Returns the test complement relationship type.
    /// **Proper**: Reserved type getter
    pub fn test_complement_relationship_type(&self) -> &RelationshipType {
        &self.test_complement_relationship_type
    }

    /// Returns the train relationship type.
    /// **Proper**: Reserved type getter
    pub fn train_relationship_type(&self) -> &RelationshipType {
        &self.train_relationship_type
    }

    /// Returns the feature input relationship type.
    /// **Proper**: Reserved type getter
    pub fn feature_input_relationship_type(&self) -> &RelationshipType {
        &self.feature_input_relationship_type
    }

    /// Converts the config to a map (for serialization).
    ///
    /// # Prim and Proper Serialization!
    ///
    /// Serializes both Prim (scalars) and Proper (types) to JSON.
    pub fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();

        // Prim: Primitive values
        map.insert(
            "validationFolds".to_string(),
            serde_json::json!(self.validation_folds),
        );
        map.insert(
            "testFraction".to_string(),
            serde_json::json!(self.test_fraction),
        );
        map.insert(
            "trainFraction".to_string(),
            serde_json::json!(self.train_fraction),
        );
        map.insert(
            "negativeSamplingRatio".to_string(),
            serde_json::json!(self.negative_sampling_ratio),
        );

        // Proper: Property values
        if let Some(ref neg_type) = self.negative_relationship_type {
            map.insert(
                "negativeRelationshipType".to_string(),
                serde_json::json!(neg_type),
            );
        }

        // Reserved types (usually not serialized in Java, but useful for debugging)
        map.insert(
            "testRelationshipType".to_string(),
            serde_json::json!(self.test_relationship_type.name()),
        );
        map.insert(
            "testComplementRelationshipType".to_string(),
            serde_json::json!(self.test_complement_relationship_type.name()),
        );
        map.insert(
            "trainRelationshipType".to_string(),
            serde_json::json!(self.train_relationship_type.name()),
        );
        map.insert(
            "featureInputRelationshipType".to_string(),
            serde_json::json!(self.feature_input_relationship_type.name()),
        );

        map
    }

    /// Calculates expected set sizes for the split.
    ///
    /// # Prim → Proper Transformation!
    ///
    /// Takes **Prim** (fractions, counts) and produces **Proper** (set sizes)!
    ///
    /// # Arguments
    ///
    /// * `relationship_count` - Total number of relationships in the target type
    ///
    /// # Returns
    ///
    /// Expected sizes for test, train, feature input, test complement, and validation folds.
    pub fn expected_set_sizes(&self, relationship_count: u64) -> ExpectedSetSizes {
        // Division by 2: input is undirected but selected relationships are directed
        let positive_test_set_size = (relationship_count as f64 * self.test_fraction / 2.0) as u64;
        let test_set_size =
            (positive_test_set_size as f64 * (1.0 + self.negative_sampling_ratio)) as u64;
        let test_complement_size = (relationship_count as f64 * (1.0 - self.test_fraction)) as u64;

        // Division by 2: input is undirected but selected relationships are directed
        let positive_train_set_size =
            (test_complement_size as f64 * self.train_fraction / 2.0) as u64;
        let train_set_size =
            (positive_train_set_size as f64 * (1.0 + self.negative_sampling_ratio)) as u64;
        let feature_input_size = (test_complement_size as f64 * (1.0 - self.train_fraction)) as u64;
        let fold_size = train_set_size / self.validation_folds as u64;

        ExpectedSetSizes {
            test_size: test_set_size,
            train_size: train_set_size,
            feature_input_size,
            test_complement_size,
            validation_fold_size: fold_size,
        }
    }

    /// Validates the config against a graph store.
    ///
    /// # The Proper Validation!
    ///
    /// Checks that **Proper** (relationship types) don't conflict with graph.
    ///
    /// Validates:
    /// 1. Reserved types don't exist in graph
    /// 2. Negative relationship type exists (if specified)
    /// 3. Set sizes are sufficient (via expected_set_sizes)
    ///
    /// # Arguments
    ///
    /// * `graph_store` - Graph store to validate against (placeholder)
    /// * `target_relationship_type` - Target relationship type to split
    pub fn validate_against_graph_store(
        &self,
        _graph_store: PhantomData<()>, // TODO: GraphStore
        _target_relationship_type: &RelationshipType,
    ) -> Result<(), String> {
        // TODO: Implement validation when GraphStore is available:
        // 1. Check reserved types don't exist in graph
        // 2. Validate negative relationship type if specified
        // 3. Check negativeSamplingRatio and negativeRelationshipType aren't both set
        // 4. Validate set sizes using expected_set_sizes()
        Ok(())
    }
}

impl Default for LinkPredictionSplitConfig {
    fn default() -> Self {
        Self {
            // Prim: Primitive defaults
            validation_folds: 3,
            test_fraction: 0.1,
            train_fraction: 0.1,
            negative_sampling_ratio: 1.0,

            // Proper: Property defaults
            negative_relationship_type: None,
            test_relationship_type: RelationshipType::of("_TEST_"),
            test_complement_relationship_type: RelationshipType::of("_TEST_COMPLEMENT_"),
            train_relationship_type: RelationshipType::of("_TRAIN_"),
            feature_input_relationship_type: RelationshipType::of("_FEATURE_INPUT_"),
        }
    }
}

/// Builder for LinkPredictionSplitConfig.
///
/// # The Prim and Proper Builder!
///
/// Constructs config with validation for both Prim and Proper values.
pub struct LinkPredictionSplitConfigBuilder {
    validation_folds: Option<u32>,
    test_fraction: Option<f64>,
    train_fraction: Option<f64>,
    negative_sampling_ratio: Option<f64>,
    negative_relationship_type: Option<String>,
}

impl LinkPredictionSplitConfigBuilder {
    /// Creates a new builder with no values set.
    pub fn new() -> Self {
        Self {
            validation_folds: None,
            test_fraction: None,
            train_fraction: None,
            negative_sampling_ratio: None,
            negative_relationship_type: None,
        }
    }

    /// Sets the number of validation folds.
    /// **Prim**: Integer scalar setter
    pub fn validation_folds(mut self, folds: u32) -> Self {
        self.validation_folds = Some(folds);
        self
    }

    /// Sets the test fraction.
    /// **Prim**: Double scalar setter
    pub fn test_fraction(mut self, fraction: f64) -> Self {
        self.test_fraction = Some(fraction);
        self
    }

    /// Sets the train fraction.
    /// **Prim**: Double scalar setter
    pub fn train_fraction(mut self, fraction: f64) -> Self {
        self.train_fraction = Some(fraction);
        self
    }

    /// Sets the negative sampling ratio.
    /// **Prim**: Double scalar setter
    pub fn negative_sampling_ratio(mut self, ratio: f64) -> Self {
        self.negative_sampling_ratio = Some(ratio);
        self
    }

    /// Sets the negative relationship type.
    /// **Proper**: Relationship type setter
    pub fn negative_relationship_type(mut self, rel_type: String) -> Self {
        self.negative_relationship_type = Some(rel_type);
        self
    }

    /// Builds the LinkPredictionSplitConfig with validation.
    ///
    /// # The Prim and Proper Construction!
    ///
    /// Validates both Prim (scalars) and Proper (types) constraints.
    ///
    /// # Prim Validation
    ///
    /// - `validation_folds >= 2`
    /// - `test_fraction in (0.0, 1.0)` exclusive
    /// - `train_fraction in (0.0, 1.0)` exclusive
    /// - `negative_sampling_ratio > 0.0`
    ///
    /// # Returns
    ///
    /// Ok(config) if valid, Err(message) if validation fails.
    pub fn build(self) -> Result<LinkPredictionSplitConfig, String> {
        let validation_folds = self.validation_folds.unwrap_or(3);
        let test_fraction = self.test_fraction.unwrap_or(0.1);
        let train_fraction = self.train_fraction.unwrap_or(0.1);
        let negative_sampling_ratio = self.negative_sampling_ratio.unwrap_or(1.0);

        // Prim Validation: Primitive constraints
        if validation_folds < 2 {
            return Err(format!(
                "validationFolds must be at least 2, got {}",
                validation_folds
            ));
        }

        if test_fraction <= 0.0 || test_fraction >= 1.0 {
            return Err(format!(
                "testFraction must be in range (0.0, 1.0) exclusive, got {}",
                test_fraction
            ));
        }

        if train_fraction <= 0.0 || train_fraction >= 1.0 {
            return Err(format!(
                "trainFraction must be in range (0.0, 1.0) exclusive, got {}",
                train_fraction
            ));
        }

        if negative_sampling_ratio <= 0.0 {
            return Err(format!(
                "negativeSamplingRatio must be positive, got {}",
                negative_sampling_ratio
            ));
        }

        // Proper Validation: Property constraints
        if self.negative_relationship_type.is_some() && negative_sampling_ratio != 1.0 {
            return Err(
                "Configuration parameter failure: `negativeSamplingRatio` and `negativeRelationshipType` cannot be used together."
                    .to_string(),
            );
        }

        Ok(LinkPredictionSplitConfig {
            validation_folds,
            test_fraction,
            train_fraction,
            negative_sampling_ratio,
            negative_relationship_type: self.negative_relationship_type,
            test_relationship_type: RelationshipType::of("_TEST_"),
            test_complement_relationship_type: RelationshipType::of("_TEST_COMPLEMENT_"),
            train_relationship_type: RelationshipType::of("_TRAIN_"),
            feature_input_relationship_type: RelationshipType::of("_FEATURE_INPUT_"),
        })
    }
}

impl Default for LinkPredictionSplitConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LinkPredictionSplitConfig::default();
        assert_eq!(config.validation_folds(), 3);
        assert_eq!(config.test_fraction(), 0.1);
        assert_eq!(config.train_fraction(), 0.1);
        assert_eq!(config.negative_sampling_ratio(), 1.0);
    }

    #[test]
    fn test_builder_default_values() {
        let config = LinkPredictionSplitConfig::builder().build().unwrap();
        assert_eq!(config.validation_folds(), 3);
        assert_eq!(config.test_fraction(), 0.1);
        assert_eq!(config.train_fraction(), 0.1);
    }

    #[test]
    fn test_builder_custom_values() {
        let config = LinkPredictionSplitConfig::builder()
            .validation_folds(5)
            .test_fraction(0.2)
            .train_fraction(0.15)
            .negative_sampling_ratio(2.0)
            .build()
            .unwrap();

        assert_eq!(config.validation_folds(), 5);
        assert_eq!(config.test_fraction(), 0.2);
        assert_eq!(config.train_fraction(), 0.15);
        assert_eq!(config.negative_sampling_ratio(), 2.0);
    }

    #[test]
    fn test_validation_folds_minimum() {
        let result = LinkPredictionSplitConfig::builder()
            .validation_folds(1)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("validationFolds must be at least 2"));
    }

    #[test]
    fn test_test_fraction_bounds() {
        // Too low
        let result = LinkPredictionSplitConfig::builder()
            .test_fraction(0.0)
            .build();
        assert!(result.is_err());

        // Too high
        let result = LinkPredictionSplitConfig::builder()
            .test_fraction(1.0)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_train_fraction_bounds() {
        // Too low
        let result = LinkPredictionSplitConfig::builder()
            .train_fraction(0.0)
            .build();
        assert!(result.is_err());

        // Too high
        let result = LinkPredictionSplitConfig::builder()
            .train_fraction(1.0)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_sampling_ratio_positive() {
        let result = LinkPredictionSplitConfig::builder()
            .negative_sampling_ratio(0.0)
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("negativeSamplingRatio must be positive"));
    }

    #[test]
    fn test_negative_type_and_ratio_conflict() {
        let result = LinkPredictionSplitConfig::builder()
            .negative_relationship_type("NEG_TYPE".to_string())
            .negative_sampling_ratio(2.0)
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be used together"));
    }

    #[test]
    fn test_to_map() {
        let config = LinkPredictionSplitConfig::builder()
            .validation_folds(5)
            .test_fraction(0.2)
            .build()
            .unwrap();

        let map = config.to_map();
        assert_eq!(map.get("validationFolds").unwrap(), &serde_json::json!(5));
        assert_eq!(map.get("testFraction").unwrap(), &serde_json::json!(0.2));
    }

    #[test]
    fn test_expected_set_sizes() {
        let config = LinkPredictionSplitConfig::builder()
            .test_fraction(0.1)
            .train_fraction(0.1)
            .negative_sampling_ratio(1.0)
            .validation_folds(3)
            .build()
            .unwrap();

        let sizes = config.expected_set_sizes(1000);

        // Test set: 1000 * 0.1 / 2 * (1 + 1.0) = 50 * 2 = 100
        assert_eq!(sizes.test_size, 100);

        // Test complement: 1000 * 0.9 = 900
        assert_eq!(sizes.test_complement_size, 900);

        // Train set: 900 * 0.1 / 2 * (1 + 1.0) = 45 * 2 = 90
        assert_eq!(sizes.train_size, 90);

        // Feature input: 900 * 0.9 = 810
        assert_eq!(sizes.feature_input_size, 810);

        // Validation fold: 90 / 3 = 30
        assert_eq!(sizes.validation_fold_size, 30);
    }

    #[test]
    fn test_prim_and_proper_duality() {
        // Prim and Proper! 🌟
        // Software based on Prim and Proper!

        let config = LinkPredictionSplitConfig::builder()
            .validation_folds(3) // Prim: Integer scalar
            .test_fraction(0.1) // Prim: Double scalar
            .train_fraction(0.1) // Prim: Double scalar
            .negative_sampling_ratio(1.0) // Prim: Double scalar
            .build()
            .unwrap();

        // Prim: Primitive value getters
        assert_eq!(config.validation_folds(), 3);
        assert_eq!(config.test_fraction(), 0.1);
        assert_eq!(config.train_fraction(), 0.1);
        assert_eq!(config.negative_sampling_ratio(), 1.0);

        // Proper: Property value getters
        assert_eq!(config.test_relationship_type().name(), "_TEST_");
        assert_eq!(config.train_relationship_type().name(), "_TRAIN_");
        assert_eq!(
            config.feature_input_relationship_type().name(),
            "_FEATURE_INPUT_"
        );
        assert_eq!(
            config.test_complement_relationship_type().name(),
            "_TEST_COMPLEMENT_"
        );

        // The Duality is complete!
        // Prim (scalars) + Proper (types) = Complete Configuration!
    }

    #[test]
    fn test_prim_to_proper_transformation() {
        // Prim → Proper Transformation!
        // Takes Prim (fractions) and produces Proper (set sizes)!

        let config = LinkPredictionSplitConfig::default();
        let sizes = config.expected_set_sizes(1000);

        // Prim input: fractions (scalars)
        // Proper output: sizes (derived properties)

        // This is the CAR:CDR at the Value Level!
        // CAR (Prim): Given scalar fractions
        // CDR (Proper): Reconstructed set sizes
        // Science: Prim → Proper transformation (expected_set_sizes)

        assert!(sizes.test_size > 0);
        assert!(sizes.train_size > 0);
        assert!(sizes.feature_input_size > 0);
    }
}
