//! Tests for decision tree implementation.
//!
//! Phase 1 unit tests - no external dependencies required.

use crate::ml::decision_tree::TreeNode;

#[cfg(test)]
mod tree_node_tests {
    use super::*;

    #[test]
    fn test_leaf_node_creation() {
        let leaf = TreeNode::new_leaf(42.0);
        assert_eq!(leaf.prediction(), Some(&42.0));
        assert_eq!(leaf.feature_index(), -1);
        assert!(!leaf.has_left_child());
        assert!(!leaf.has_right_child());
    }

    #[test]
    fn test_split_node_creation() {
        let split = TreeNode::<f64>::new_split(2, 5.5);
        assert_eq!(split.feature_index(), 2);
        assert_eq!(split.threshold_value(), 5.5);
        assert_eq!(split.prediction(), None);
        assert!(!split.has_left_child());
        assert!(!split.has_right_child());
    }

    #[test]
    fn test_set_children() {
        let mut root = TreeNode::new_split(0, 1.5);

        root.set_left_child(TreeNode::new_leaf(10.0));
        root.set_right_child(TreeNode::new_leaf(20.0));

        assert!(root.has_left_child());
        assert!(root.has_right_child());

        assert_eq!(root.left_child().unwrap().prediction(), Some(&10.0));
        assert_eq!(root.right_child().unwrap().prediction(), Some(&20.0));
    }

    #[test]
    fn test_mutable_child_access() {
        let mut root = TreeNode::new_split(0, 1.5);
        root.set_left_child(TreeNode::new_leaf(10.0));

        // Modify left child
        if let Some(left) = root.left_child_mut() {
            left.set_prediction(99.0);
        }

        assert_eq!(root.left_child().unwrap().prediction(), Some(&99.0));
    }

    #[test]
    fn test_tree_rendering() {
        let mut root = TreeNode::new_split(0, 1.5);
        root.set_left_child(TreeNode::new_leaf(10.0));
        root.set_right_child(TreeNode::new_leaf(20.0));

        let rendered = root.render();

        // Check that all nodes are present in output
        assert!(rendered.contains("featureIndex 0"));
        assert!(rendered.contains("splitValue 1.5"));
        // Predictions appear as Some("10") not "10"
        assert!(rendered.contains("Some(\"10\")") || rendered.contains("10"));
        assert!(rendered.contains("Some(\"20\")") || rendered.contains("20"));
    }

    #[test]
    fn test_deep_tree_rendering() {
        let mut root = TreeNode::new_split(0, 10.0);

        let mut left = TreeNode::new_split(1, 5.0);
        left.set_left_child(TreeNode::new_leaf(1.0));
        left.set_right_child(TreeNode::new_leaf(2.0));

        root.set_left_child(left);
        root.set_right_child(TreeNode::new_leaf(3.0));

        let rendered = root.render();

        // Should show tree structure with indentation
        assert!(rendered.contains("featureIndex 0"));
        assert!(rendered.contains("featureIndex 1"));
        assert!(rendered.contains("|--"));
    }

    #[test]
    fn test_tree_equality() {
        let tree1 = TreeNode::new_leaf(42.0);
        let tree2 = TreeNode::new_leaf(42.0);
        let tree3 = TreeNode::new_leaf(99.0);

        assert_eq!(tree1, tree2);
        assert_ne!(tree1, tree3);
    }

    #[test]
    fn test_memory_estimation() {
        let leaf_size = TreeNode::<f64>::leaf_memory_estimation();
        let split_size = TreeNode::<f64>::split_memory_estimation();

        // Split nodes should not be larger than leaf nodes
        assert!(split_size <= leaf_size);
        assert!(leaf_size > 0);
        assert!(split_size > 0);
    }
}

use crate::ml::decision_tree::ClassifierImpurityCriterionType;
use crate::ml::decision_tree::{DecisionTreePredictor, DecisionTreeTrainerConfig};
use crate::ml::decision_tree::{FeatureBagger, ImpurityData, MSEImpurityData};

#[cfg(test)]
mod predictor_tests {
    use super::*;

    #[test]
    fn test_predict_leaf_only() {
        let root = TreeNode::new_leaf(42.0);
        let predictor = DecisionTreePredictor::new(root);

        let features = vec![1.0, 2.0, 3.0];
        assert_eq!(*predictor.predict(&features), 42.0);
    }

    #[test]
    fn test_predict_simple_split() {
        let mut root = TreeNode::new_split(0, 5.0);
        root.set_left_child(TreeNode::new_leaf(10.0));
        root.set_right_child(TreeNode::new_leaf(20.0));

        let predictor = DecisionTreePredictor::new(root);

        // Feature[0] < 5.0 -> left (10.0)
        assert_eq!(*predictor.predict(&[3.0, 0.0, 0.0]), 10.0);

        // Feature[0] >= 5.0 -> right (20.0)
        assert_eq!(*predictor.predict(&[7.0, 0.0, 0.0]), 20.0);

        // Exactly at threshold -> right
        assert_eq!(*predictor.predict(&[5.0, 0.0, 0.0]), 20.0);
    }

    #[test]
    fn test_predict_deep_tree() {
        // Build a 3-level tree testing multiple features
        let mut root = TreeNode::new_split(0, 10.0);

        let mut left = TreeNode::new_split(1, 5.0);
        left.set_left_child(TreeNode::new_leaf(1.0));
        left.set_right_child(TreeNode::new_leaf(2.0));

        let mut right = TreeNode::new_split(1, 15.0);
        right.set_left_child(TreeNode::new_leaf(3.0));
        right.set_right_child(TreeNode::new_leaf(4.0));

        root.set_left_child(left);
        root.set_right_child(right);

        let predictor = DecisionTreePredictor::new(root);

        // Test all four leaf paths
        assert_eq!(*predictor.predict(&[5.0, 3.0]), 1.0); // left-left: [0]<10, [1]<5
        assert_eq!(*predictor.predict(&[5.0, 7.0]), 2.0); // left-right: [0]<10, [1]>=5
        assert_eq!(*predictor.predict(&[15.0, 12.0]), 3.0); // right-left: [0]>=10, [1]<15
        assert_eq!(*predictor.predict(&[15.0, 20.0]), 4.0); // right-right: [0]>=10, [1]>=15
    }

    #[test]
    fn test_predict_unbalanced_tree() {
        // Left side is leaf, right side has subtree
        let mut root = TreeNode::new_split(0, 5.0);
        root.set_left_child(TreeNode::new_leaf(100.0));

        let mut right = TreeNode::new_split(1, 10.0);
        right.set_left_child(TreeNode::new_leaf(200.0));
        right.set_right_child(TreeNode::new_leaf(300.0));
        root.set_right_child(right);

        let predictor = DecisionTreePredictor::new(root);

        assert_eq!(*predictor.predict(&[3.0, 0.0]), 100.0); // left leaf
        assert_eq!(*predictor.predict(&[7.0, 8.0]), 200.0); // right-left
        assert_eq!(*predictor.predict(&[7.0, 12.0]), 300.0); // right-right
    }

    #[test]
    #[should_panic]
    fn test_predict_empty_features_panics() {
        let root = TreeNode::new_leaf(42.0);
        let predictor = DecisionTreePredictor::new(root);

        // Should panic - features cannot be empty
        let _ = predictor.predict(&[]);
    }

    #[test]
    fn test_predictor_equality() {
        let tree1 = TreeNode::new_leaf(42.0);
        let tree2 = TreeNode::new_leaf(42.0);
        let tree3 = TreeNode::new_leaf(99.0);

        let pred1 = DecisionTreePredictor::new(tree1);
        let pred2 = DecisionTreePredictor::new(tree2);
        let pred3 = DecisionTreePredictor::new(tree3);

        assert_eq!(pred1, pred2);
        assert_ne!(pred1, pred3);
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = DecisionTreeTrainerConfig::default();

        assert_eq!(config.max_depth(), usize::MAX);
        assert_eq!(config.min_split_size(), 2);
        assert_eq!(config.min_leaf_size(), 1);
    }

    #[test]
    fn test_config_builder_all_fields() {
        let config = DecisionTreeTrainerConfig::builder()
            .max_depth(10)
            .min_split_size(5)
            .min_leaf_size(2)
            .build()
            .expect("valid config");

        assert_eq!(config.max_depth(), 10);
        assert_eq!(config.min_split_size(), 5);
        assert_eq!(config.min_leaf_size(), 2);
    }

    #[test]
    fn test_config_builder_partial() {
        let config = DecisionTreeTrainerConfig::builder()
            .max_depth(20)
            .build()
            .expect("valid config");

        assert_eq!(config.max_depth(), 20);
        assert_eq!(config.min_split_size(), 2); // default
        assert_eq!(config.min_leaf_size(), 1); // default
    }

    #[test]
    fn test_config_validation_min_leaf_equal_to_split() {
        let result = DecisionTreeTrainerConfig::builder()
            .min_split_size(5)
            .min_leaf_size(5) // Invalid: must be < min_split_size
            .build();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("minLeafSize"));
        assert!(err.contains("minSplitSize"));
    }

    #[test]
    fn test_config_validation_min_leaf_greater_than_split() {
        let result = DecisionTreeTrainerConfig::builder()
            .min_split_size(5)
            .min_leaf_size(10) // Invalid
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_valid_boundary() {
        let result = DecisionTreeTrainerConfig::builder()
            .min_split_size(5)
            .min_leaf_size(4) // Valid: 4 < 5
            .build();

        assert!(result.is_ok());
    }

    #[test]
    #[should_panic(expected = "maxDepth must be at least 1")]
    fn test_config_max_depth_zero_panics() {
        let _ = DecisionTreeTrainerConfig::builder().max_depth(0).build();
    }

    #[test]
    #[should_panic(expected = "minSplitSize must be at least 2")]
    fn test_config_min_split_size_too_small_panics() {
        let _ = DecisionTreeTrainerConfig::builder()
            .min_split_size(1)
            .build();
    }

    #[test]
    #[should_panic(expected = "minLeafSize must be at least 1")]
    fn test_config_min_leaf_size_zero_panics() {
        let _ = DecisionTreeTrainerConfig::builder()
            .min_leaf_size(0)
            .build();
    }
}

#[cfg(test)]
mod feature_bagger_tests {
    use super::*;

    #[test]
    fn test_feature_bagger_deterministic() {
        let mut bagger1 = FeatureBagger::new(42, 10, 0.5);
        let mut bagger2 = FeatureBagger::new(42, 10, 0.5);

        let sample1 = bagger1.sample();
        let sample2 = bagger2.sample();

        // Same seed should produce same samples
        assert_eq!(sample1, sample2);
    }

    #[test]
    fn test_feature_bagger_sample_size() {
        let mut bagger = FeatureBagger::new(42, 100, 0.3);
        let sample = bagger.sample();

        // Should sample 30% of 100 features = 30
        assert_eq!(sample.len(), 30);

        // All indices should be in valid range
        assert!(sample.iter().all(|&idx| idx < 100));
    }

    #[test]
    fn test_feature_bagger_different_seeds() {
        let mut bagger1 = FeatureBagger::new(42, 10, 0.5);
        let mut bagger2 = FeatureBagger::new(99, 10, 0.5);

        let sample1 = bagger1.sample();
        let sample2 = bagger2.sample();

        // Different seeds should (very likely) produce different samples
        assert_ne!(sample1, sample2);
    }

    #[test]
    fn test_feature_bagger_multiple_samples() {
        let mut bagger = FeatureBagger::new(42, 20, 0.5);

        let sample1 = bagger.sample();
        let sample2 = bagger.sample();

        // Consecutive samples from same bagger should differ (randomness)
        // With 50% sampling of 20 features, extremely unlikely to be identical
        assert_ne!(sample1, sample2);
    }

    #[test]
    fn test_feature_bagger_full_features() {
        let mut bagger = FeatureBagger::new(42, 10, 1.0);
        let sample = bagger.sample();

        // Should sample all features
        assert_eq!(sample.len(), 10);

        // Should contain all indices 0..10
        let mut sorted = sample.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_feature_bagger_small_ratio() {
        let mut bagger = FeatureBagger::new(42, 100, 0.01);
        let sample = bagger.sample();

        // Should sample ceil(0.01 * 100) = 1 feature
        assert_eq!(sample.len(), 1);
        assert!(sample[0] < 100);
    }

    #[test]
    #[should_panic(expected = "Invalid maxFeaturesRatio")]
    fn test_feature_bagger_zero_ratio_panics() {
        let _ = FeatureBagger::new(42, 10, 0.0);
    }

    #[test]
    fn test_feature_bagger_memory_estimation() {
        let estimate = FeatureBagger::memory_estimation(50);
        assert!(estimate > 0);

        // Should be roughly: size_of(FeatureBagger) + 50 * size_of(usize)
        let min_expected = std::mem::size_of::<usize>() * 50;
        assert!(estimate >= min_expected);
    }
}

#[cfg(test)]
mod mse_impurity_tests {
    use super::*;

    #[test]
    fn test_mse_impurity_data_creation() {
        let data = MSEImpurityData::new(
            0.5,   // impurity
            100.0, // sum_of_squares
            10.0,  // sum
            5,     // group_size
        );

        assert_eq!(data.impurity(), 0.5);
        assert_eq!(data.group_size(), 5);
        assert_eq!(data.sum(), 10.0);
        assert_eq!(data.sum_of_squares(), 100.0);
    }

    #[test]
    fn test_mse_impurity_data_setters() {
        let mut data = MSEImpurityData::new(0.0, 0.0, 0.0, 0);

        data.set_impurity(1.5);
        data.set_sum(20.0);
        data.set_sum_of_squares(450.0);
        data.set_group_size(10);

        assert_eq!(data.impurity(), 1.5);
        assert_eq!(data.sum(), 20.0);
        assert_eq!(data.sum_of_squares(), 450.0);
        assert_eq!(data.group_size(), 10);
    }

    #[test]
    fn test_mse_copy_to() {
        let source = MSEImpurityData::new(0.5, 100.0, 10.0, 5);
        let mut target = MSEImpurityData::new(0.0, 0.0, 0.0, 0);

        source.copy_to(&mut target);

        assert_eq!(target.impurity(), 0.5);
        assert_eq!(target.sum(), 10.0);
        assert_eq!(target.sum_of_squares(), 100.0);
        assert_eq!(target.group_size(), 5);
    }

    #[test]
    fn test_mse_memory_estimation() {
        let estimate = MSEImpurityData::memory_estimation();
        assert!(estimate > 0);
        assert!(estimate >= std::mem::size_of::<MSEImpurityData>());
    }
}

#[cfg(test)]
mod types_tests {
    use super::*;

    #[test]
    fn test_criterion_type_parse_gini() {
        let result = ClassifierImpurityCriterionType::parse("GINI");
        assert_eq!(result, Ok(ClassifierImpurityCriterionType::Gini));

        let result = ClassifierImpurityCriterionType::parse("gini");
        assert_eq!(result, Ok(ClassifierImpurityCriterionType::Gini));
    }

    #[test]
    fn test_criterion_type_parse_entropy() {
        let result = ClassifierImpurityCriterionType::parse("ENTROPY");
        assert_eq!(result, Ok(ClassifierImpurityCriterionType::Entropy));

        let result = ClassifierImpurityCriterionType::parse("entropy");
        assert_eq!(result, Ok(ClassifierImpurityCriterionType::Entropy));
    }

    #[test]
    fn test_criterion_type_parse_invalid() {
        let result = ClassifierImpurityCriterionType::parse("invalid");
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.contains("invalid"));
        assert!(err.contains("GINI"));
        assert!(err.contains("ENTROPY"));
    }

    #[test]
    fn test_criterion_type_display() {
        assert_eq!(ClassifierImpurityCriterionType::Gini.to_string(), "GINI");
        assert_eq!(
            ClassifierImpurityCriterionType::Entropy.to_string(),
            "ENTROPY"
        );
    }
}
