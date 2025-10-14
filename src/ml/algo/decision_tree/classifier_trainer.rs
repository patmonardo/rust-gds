//! Decision tree classifier trainer.
//!
//! Translated from Java GDS ml-algo DecisionTreeClassifierTrainer.java.
//! This is a literal 1:1 translation following repository translation policy.
//!
//! NOTE: Implementation awaiting HugeIntArray from meta-macro processor.

use crate::ml::algo::decision_tree::{
    DecisionTreeTrainerConfig, FeatureBagger, Features, ImpurityCriterion,
};

// TODO: Uncomment when HugeIntArray is available
// use crate::collections::HugeIntArray;

pub struct DecisionTreeClassifierTrainer {
    #[allow(dead_code)]
    impurity_criterion: Box<dyn ImpurityCriterion>,
    #[allow(dead_code)]
    features: Features,
    // all_labels: HugeIntArray,  // Awaiting HugeIntArray
    #[allow(dead_code)]
    number_of_classes: usize,
    #[allow(dead_code)]
    config: DecisionTreeTrainerConfig,
    #[allow(dead_code)]
    feature_bagger: FeatureBagger,
}

// TODO: Implement when HugeIntArray is available
impl DecisionTreeClassifierTrainer {
    #[allow(dead_code)]
    fn stub() {
        todo!("DecisionTreeClassifierTrainer requires HugeIntArray from meta-macro processor")
    }
}

/*
// Uncomment when HugeIntArray is available:

impl DecisionTreeClassifierTrainer {
    pub fn new(
        impurity_criterion: Box<dyn ImpurityCriterion>,
        features: Features,
        labels: HugeIntArray,
        number_of_classes: usize,
        config: DecisionTreeTrainerConfig,
        feature_bagger: FeatureBagger,
    ) -> Self {
        assert_eq!(labels.size(), features.size());

        Self {
            impurity_criterion,
            features,
            all_labels: labels,
            number_of_classes,
            config,
            feature_bagger,
        }
    }

    pub fn memory_estimation(
        config: &DecisionTreeTrainerConfig,
        number_of_training_samples: usize,
        number_of_classes: usize,
    ) -> usize {
        use crate::ml::algo::decision_tree::{GiniIndex, TreeNode};

        std::mem::size_of::<Self>()
            + DecisionTreeTrainer::<i32>::estimate_tree(
                config,
                number_of_training_samples,
                TreeNode::<i32>::leaf_memory_estimation(),
                GiniIndex::GiniImpurityData::memory_estimation(number_of_classes),
            )
            + std::mem::size_of::<i64>() * number_of_classes
    }
}

impl DecisionTreeTrainer<i32> for DecisionTreeClassifierTrainer {
    fn impurity_criterion(&self) -> &dyn ImpurityCriterion {
        self.impurity_criterion.as_ref()
    }

    fn features(&self) -> &Features {
        &self.features
    }

    fn config(&self) -> &DecisionTreeTrainerConfig {
        &self.config
    }

    fn feature_bagger(&mut self) -> &mut FeatureBagger {
        &mut self.feature_bagger
    }

    fn to_terminal(&self, group: &Group) -> i32 {
        let mut classes_in_group = vec![0i64; self.number_of_classes];
        let array = group.array();

        for i in group.start_idx()..(group.start_idx() + group.size()) {
            classes_in_group[self.all_labels.get(array.get(i) as usize) as usize] += 1;
        }

        let mut max_class_count_in_group = -1i64;
        let mut max_mapped_class = 0;
        for (i, &count) in classes_in_group.iter().enumerate() {
            if count <= max_class_count_in_group {
                continue;
            }
            max_class_count_in_group = count;
            max_mapped_class = i;
        }

        max_mapped_class as i32
    }
}
*/
