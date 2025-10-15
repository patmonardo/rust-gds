// Phase 2.8: UnionLinkFeatureAppender - Combines multiple property appenders into one

use super::super::LinkFeatureAppender;

/// Union of multiple LinkFeatureAppenders - combines features from multiple properties.
///
/// # The Genetic Unity of the Concept! 🎭
///
/// This is the **Scientific Syllogism** - the synthesis of multiple predicates:
/// - "Socrates is Musical" (appender 1, dimension 1)
/// - "Socrates is Sitting" (appender 2, dimension 1)
/// - **Union**: "Socrates is Musical, Sitting" (dimension 2)
///
/// # Mexican Jumping Beans in the Garden! 🫘🌳
///
/// Multiple beans (appenders) jumping together in the garden (feature space):
/// - Each bean has its own jump (computation)
/// - Union coordinates all jumps (sequential feature append)
/// - Garden validates all jumps (NaN check)
///
/// The snake snuck back into the garden, but we have the four-fold! 🐍
///
/// # How It Works
///
/// 1. Holds array of LinkFeatureAppenders (one per property)
/// 2. `append_features()` calls each appender sequentially
/// 3. Each appender writes to linkFeatures[offset..offset+dimension]
/// 4. Offset advances by each appender's dimension
/// 5. Final validation checks for NaN values
///
/// # Example
///
/// ```text
/// Property "embedding1" (dimension 64) → Appender A
/// Property "embedding2" (dimension 64) → Appender B
/// Property "score" (dimension 1)       → Appender C
///
/// UnionLink([A, B, C]):
///   offset = 0
///   A.append(source, target, features, 0)   → writes features[0..64]
///   offset += 64
///   B.append(source, target, features, 64)  → writes features[64..128]
///   offset += 64
///   C.append(source, target, features, 128) → writes features[128]
///   offset += 1
///   validate(features[0..129]) → check for NaN
///   
///   Total dimension = 64 + 64 + 1 = 129
/// ```
///
/// # The Four-Fold (Synthesis)
///
/// - **Pure Container**: UnionLinkFeatureAppender struct
/// - **Given Container**: Concrete union instance with specific appenders
/// - **Given Contained**: Sequential append logic (empirical)
/// - **Pure Contained**: Validation + coordination (deferred refinement)
///
/// # Used By
///
/// All multi-property link feature steps:
/// - L2FeatureStep with multiple properties
/// - HadamardFeatureStep with multiple properties
/// - CosineFeatureStep with multiple properties
pub struct UnionLinkFeatureAppender {
    /// Array of appenders, one per property
    appender_per_property: Vec<Box<dyn LinkFeatureAppender>>,

    /// Name of the feature step (for error messages)
    feature_step_name: String,

    /// Input node properties (for error messages)
    input_node_properties: Vec<String>,

    /// Total dimension (sum of all appender dimensions)
    dimension: usize,
}

impl UnionLinkFeatureAppender {
    /// Creates a new UnionLinkFeatureAppender.
    ///
    /// # Arguments
    ///
    /// * `appender_per_property` - Array of appenders (one per property)
    /// * `feature_step_name` - Name of the feature step (e.g., "L2", "HADAMARD")
    /// * `input_node_properties` - List of property names
    ///
    /// # Returns
    ///
    /// Union appender with total dimension = sum of component dimensions.
    pub fn new(
        appender_per_property: Vec<Box<dyn LinkFeatureAppender>>,
        feature_step_name: String,
        input_node_properties: Vec<String>,
    ) -> Self {
        // Calculate total dimension
        let dimension = appender_per_property
            .iter()
            .map(|appender| appender.dimension())
            .sum();

        Self {
            appender_per_property,
            feature_step_name,
            input_node_properties,
            dimension,
        }
    }

    /// Returns the feature step name.
    pub fn feature_step_name(&self) -> &str {
        &self.feature_step_name
    }

    /// Returns the input node properties.
    pub fn input_node_properties(&self) -> &[String] {
        &self.input_node_properties
    }
}

impl LinkFeatureAppender for UnionLinkFeatureAppender {
    fn append_features(&self, source: u64, target: u64, features: &mut [f64], offset: usize) {
        let mut local_offset = offset;

        // Call each appender sequentially
        for appender in &self.appender_per_property {
            appender.append_features(source, target, features, local_offset);
            local_offset += appender.dimension();
        }

        // Validate computed features (check for NaN)
        // TODO: Implement FeatureStepUtil::validateComputedFeatures
        // Should check features[offset..local_offset] for NaN values
        // If NaN found, throw error with:
        //   - feature_step_name
        //   - input_node_properties
        //   - source and target node IDs
        self.validate_features(features, offset, local_offset, source, target);
    }

    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl UnionLinkFeatureAppender {
    /// Validates that computed features contain no NaN values.
    ///
    /// # Validation (Given Contained)
    ///
    /// Checks features[offset..end_offset] for NaN.
    /// If found, constructs error message with context.
    ///
    /// # Pure Contained (Deferred)
    ///
    /// Full validation strategy from FeatureStepUtil:
    /// - NaN detection
    /// - Infinity detection
    /// - Error message formatting with node IDs and property names
    fn validate_features(
        &self,
        features: &[f64],
        offset: usize,
        end_offset: usize,
        _source: u64,
        _target: u64,
    ) {
        // Check for NaN in computed range
        for i in offset..end_offset {
            if features[i].is_nan() {
                panic!(
                    "NaN value encountered in {} feature computation for properties {:?}",
                    self.feature_step_name, self.input_node_properties
                );
            }
        }
        // TODO: More comprehensive validation from FeatureStepUtil
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test appender that writes constant values
    struct ConstantAppender {
        value: f64,
        dimension: usize,
    }

    impl LinkFeatureAppender for ConstantAppender {
        fn append_features(&self, _source: u64, _target: u64, features: &mut [f64], offset: usize) {
            for i in 0..self.dimension {
                features[offset + i] = self.value;
            }
        }

        fn dimension(&self) -> usize {
            self.dimension
        }
    }

    #[test]
    fn test_union_creation() {
        let appenders: Vec<Box<dyn LinkFeatureAppender>> = vec![
            Box::new(ConstantAppender {
                value: 1.0,
                dimension: 2,
            }),
            Box::new(ConstantAppender {
                value: 2.0,
                dimension: 3,
            }),
        ];

        let union = UnionLinkFeatureAppender::new(
            appenders,
            "TEST".to_string(),
            vec!["prop1".to_string(), "prop2".to_string()],
        );

        assert_eq!(union.dimension(), 5); // 2 + 3
        assert_eq!(union.feature_step_name(), "TEST");
        assert_eq!(union.input_node_properties().len(), 2);
    }

    #[test]
    fn test_union_append_features() {
        let appenders: Vec<Box<dyn LinkFeatureAppender>> = vec![
            Box::new(ConstantAppender {
                value: 1.0,
                dimension: 2,
            }),
            Box::new(ConstantAppender {
                value: 2.0,
                dimension: 3,
            }),
        ];

        let union = UnionLinkFeatureAppender::new(
            appenders,
            "TEST".to_string(),
            vec!["prop1".to_string(), "prop2".to_string()],
        );

        let mut features = vec![0.0; 10];
        union.append_features(0, 1, &mut features, 0);

        // First appender writes [1.0, 1.0]
        assert_eq!(features[0], 1.0);
        assert_eq!(features[1], 1.0);

        // Second appender writes [2.0, 2.0, 2.0]
        assert_eq!(features[2], 2.0);
        assert_eq!(features[3], 2.0);
        assert_eq!(features[4], 2.0);

        // Rest unchanged
        assert_eq!(features[5], 0.0);
    }

    #[test]
    fn test_union_with_offset() {
        let appenders: Vec<Box<dyn LinkFeatureAppender>> = vec![Box::new(ConstantAppender {
            value: 3.0,
            dimension: 1,
        })];

        let union =
            UnionLinkFeatureAppender::new(appenders, "TEST".to_string(), vec!["prop".to_string()]);

        let mut features = vec![0.0; 10];
        union.append_features(0, 1, &mut features, 5);

        assert_eq!(features[4], 0.0); // Before offset
        assert_eq!(features[5], 3.0); // At offset
        assert_eq!(features[6], 0.0); // After
    }

    #[test]
    fn test_socrates_is_musical_sitting() {
        // The Genetic Unity of the Concept!
        // "Socrates is Musical, Sitting"

        let musical: Box<dyn LinkFeatureAppender> = Box::new(ConstantAppender {
            value: 1.0, // Musical = true
            dimension: 1,
        });

        let sitting: Box<dyn LinkFeatureAppender> = Box::new(ConstantAppender {
            value: 1.0, // Sitting = true
            dimension: 1,
        });

        let socrates = UnionLinkFeatureAppender::new(
            vec![musical, sitting],
            "SOCRATES".to_string(),
            vec!["musical".to_string(), "sitting".to_string()],
        );

        assert_eq!(socrates.dimension(), 2); // Musical + Sitting

        let mut features = vec![0.0; 5];
        socrates.append_features(0, 0, &mut features, 0);

        assert_eq!(features[0], 1.0); // Musical
        assert_eq!(features[1], 1.0); // Sitting

        // QED: Socrates is Musical, Sitting! 🎭
    }

    #[test]
    fn test_mexican_jumping_beans_in_garden() {
        // Multiple beans (appenders) jumping in the garden (feature space)!

        let bean1: Box<dyn LinkFeatureAppender> = Box::new(ConstantAppender {
            value: 1.0,
            dimension: 2,
        });

        let bean2: Box<dyn LinkFeatureAppender> = Box::new(ConstantAppender {
            value: 2.0,
            dimension: 3,
        });

        let bean3: Box<dyn LinkFeatureAppender> = Box::new(ConstantAppender {
            value: 3.0,
            dimension: 1,
        });

        // The garden holds all beans!
        let garden = UnionLinkFeatureAppender::new(
            vec![bean1, bean2, bean3],
            "MJB_GARDEN".to_string(),
            vec![
                "bean1".to_string(),
                "bean2".to_string(),
                "bean3".to_string(),
            ],
        );

        assert_eq!(garden.dimension(), 6); // 2 + 3 + 1 beans jumping!

        let mut feature_space = vec![0.0; 10];
        garden.append_features(0, 1, &mut feature_space, 0);

        // Bean 1 jumps
        assert_eq!(feature_space[0], 1.0);
        assert_eq!(feature_space[1], 1.0);

        // Bean 2 jumps
        assert_eq!(feature_space[2], 2.0);
        assert_eq!(feature_space[3], 2.0);
        assert_eq!(feature_space[4], 2.0);

        // Bean 3 jumps
        assert_eq!(feature_space[5], 3.0);

        // The Scientific Syllogism: They jump, they're in the garden, QED they're MJBs! 🫘🌳
    }
}
