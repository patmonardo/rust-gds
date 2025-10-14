//! Decision tree regressor trainer.
//!
//! Translated from Java GDS ml-algo DecisionTreeRegressorTrainer.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::collections::HugeDoubleArray;
use crate::ml::algo::decision_tree::{
    splitter::Features, DecisionTreeTrainer, DecisionTreeTrainerConfig, FeatureBagger, Group,
    ImpurityCriterion, MSEImpurityData, SplitMeanSquaredError, TreeNode,
};
use std::sync::Arc;

pub struct DecisionTreeRegressorTrainer {
    targets: Arc<HugeDoubleArray>,
    features: Features,
    config: DecisionTreeTrainerConfig,
    feature_bagger: FeatureBagger,
}

impl DecisionTreeRegressorTrainer {
    pub fn new(
        targets: HugeDoubleArray,
        features: Features,
        config: DecisionTreeTrainerConfig,
        feature_bagger: FeatureBagger,
    ) -> Self {
        assert_eq!(targets.size(), features.size());

        Self {
            targets: Arc::new(targets),
            features,
            config,
            feature_bagger,
        }
    }

    pub fn memory_estimation(
        config: &DecisionTreeTrainerConfig,
        number_of_training_samples: usize,
    ) -> usize {
        std::mem::size_of::<Self>()
            + DecisionTreeRegressorTrainer::estimate_tree_static(
                config,
                number_of_training_samples,
                TreeNode::<f64>::leaf_memory_estimation(),
                MSEImpurityData::memory_estimation(),
            )
    }

    // Static helper for memory estimation (trait method can't be called without self)
    fn estimate_tree_static(
        config: &DecisionTreeTrainerConfig,
        number_of_training_samples: usize,
        leaf_node_size_in_bytes: usize,
        size_of_impurity_data: usize,
    ) -> usize {
        let predictor_estimation = Self::estimate_tree_memory_static(
            config,
            number_of_training_samples,
            leaf_node_size_in_bytes,
        );

        let normalized_max_depth = config
            .max_depth()
            .min(1.max(number_of_training_samples.saturating_sub(config.min_split_size()) + 2));
        let max_items_on_stack = 2 * normalized_max_depth;
        let max_stack_size = std::mem::size_of::<
            std::collections::VecDeque<crate::ml::algo::decision_tree::StackRecord<f64>>,
        >() + std::mem::size_of::<
            crate::ml::algo::decision_tree::StackRecord<f64>,
        >() * max_items_on_stack
            + (std::mem::size_of::<i64>() * number_of_training_samples / max_items_on_stack)
                * max_items_on_stack;

        let splitter_estimation = crate::ml::algo::decision_tree::Splitter::memory_estimation(
            number_of_training_samples,
            size_of_impurity_data,
        );

        predictor_estimation + max_stack_size + splitter_estimation
    }

    fn estimate_tree_memory_static(
        config: &DecisionTreeTrainerConfig,
        number_of_training_samples: usize,
        leaf_node_size_in_bytes: usize,
    ) -> usize {
        if number_of_training_samples == 0 {
            return 0;
        }

        let max_num_leaf_nodes = (2.0_f64.powi(config.max_depth() as i32))
            .min((number_of_training_samples as f64) / (config.min_leaf_size() as f64))
            .min(2.0 * (number_of_training_samples as f64) / (config.min_split_size() as f64))
            .ceil() as usize;

        std::mem::size_of::<crate::ml::algo::decision_tree::DecisionTreePredictor<f64>>()
            + (1..=max_num_leaf_nodes).sum::<usize>() * leaf_node_size_in_bytes
            + (0..max_num_leaf_nodes.saturating_sub(1)).sum::<usize>()
                * TreeNode::<f64>::split_memory_estimation()
    }
}

impl DecisionTreeTrainer<f64> for DecisionTreeRegressorTrainer {
    fn impurity_criterion(&self) -> Box<dyn ImpurityCriterion> {
        Box::new(SplitMeanSquaredError::new(self.targets.clone()))
    }

    fn features(&self) -> Features {
        self.features.clone()
    }

    fn config(&self) -> &DecisionTreeTrainerConfig {
        &self.config
    }

    fn feature_bagger(&self) -> &FeatureBagger {
        &self.feature_bagger
    }

    fn to_terminal(&self, group: &Group) -> f64 {
        let array = group.array();
        let mut sum = 0.0;
        for i in group.start_idx()..(group.start_idx() + group.size()) {
            sum += self.targets.get(array.get(i) as usize);
        }
        sum / group.size() as f64
    }
}
