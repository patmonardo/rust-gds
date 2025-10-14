//! Base decision tree trainer.
//!
//! Translated from Java GDS ml-algo DecisionTreeTrainer.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::collections::HugeLongArray;
use crate::ml::algo::decision_tree::{
    splitter::Features, DecisionTreePredictor, DecisionTreeTrainerConfig, FeatureBagger, Group,
    ImpurityCriterion, Splitter, StackRecord, TreeNode,
};
use std::collections::VecDeque;

pub trait DecisionTreeTrainer<P: Clone> {
    fn impurity_criterion(&self) -> Box<dyn ImpurityCriterion>;
    fn features(&self) -> Features;
    fn config(&self) -> &DecisionTreeTrainerConfig;
    fn feature_bagger(&self) -> &FeatureBagger;
    fn to_terminal(&self, group: &Group) -> P;

    fn estimate_tree(
        config: &DecisionTreeTrainerConfig,
        number_of_training_samples: usize,
        leaf_node_size_in_bytes: usize,
        size_of_impurity_data: usize,
    ) -> usize {
        let predictor_estimation =
            Self::estimate_tree_memory(config, number_of_training_samples, leaf_node_size_in_bytes);

        let normalized_max_depth = config
            .max_depth()
            .min(1.max(number_of_training_samples.saturating_sub(config.min_split_size()) + 2));
        let max_items_on_stack = 2 * normalized_max_depth;
        let max_stack_size = std::mem::size_of::<VecDeque<StackRecord<P>>>()
            + std::mem::size_of::<StackRecord<P>>() * max_items_on_stack
            + (std::mem::size_of::<i64>() * number_of_training_samples / max_items_on_stack)
                * max_items_on_stack;

        let splitter_estimation =
            Splitter::memory_estimation(number_of_training_samples, size_of_impurity_data);

        predictor_estimation + max_stack_size + splitter_estimation
    }

    fn estimate_tree_memory(
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

        std::mem::size_of::<DecisionTreePredictor<P>>()
            + (1..=max_num_leaf_nodes).sum::<usize>() * leaf_node_size_in_bytes
            + (0..max_num_leaf_nodes.saturating_sub(1)).sum::<usize>()
                * TreeNode::<P>::split_memory_estimation()
    }

    fn train(&mut self, train_set_indices: &[i64]) -> DecisionTreePredictor<P> {
        let mut splitter = Splitter::new(
            train_set_indices.len(),
            self.impurity_criterion(),
            self.feature_bagger().clone(),
            self.features(),
            self.config().min_leaf_size(),
        );

        let mut stack = VecDeque::new();
        let root: TreeNode<P>;

        {
            let mut mutable_train_set_indices = HugeLongArray::new(train_set_indices.len());
            for (i, &val) in train_set_indices.iter().enumerate() {
                mutable_train_set_indices.set(i, val);
            }

            let impurity_data = self.impurity_criterion().group_impurity(
                &mutable_train_set_indices,
                0,
                mutable_train_set_indices.size(),
            );

            root = self.split_and_push(
                &mut stack,
                &mut splitter,
                &Group::new(
                    mutable_train_set_indices,
                    0,
                    train_set_indices.len(),
                    impurity_data,
                ),
                1,
            );
        }

        let max_depth = self.config().max_depth();
        let min_split_size = self.config().min_split_size();

        while let Some(mut record) = stack.pop_front() {
            let depth = record.depth();
            let split = record.split_owned();
            let groups = split.into_groups();
            let (left_group, right_group) = groups.into_parts();

            let left_child = if depth >= max_depth || left_group.size() < min_split_size {
                TreeNode::new_leaf(self.to_terminal(&left_group))
            } else {
                self.split_and_push(&mut stack, &mut splitter, &left_group, depth + 1)
            };

            let right_child = if depth >= max_depth || right_group.size() < min_split_size {
                TreeNode::new_leaf(self.to_terminal(&right_group))
            } else {
                self.split_and_push(&mut stack, &mut splitter, &right_group, depth + 1)
            };

            record.node_mut().set_left_child(left_child);
            record.node_mut().set_right_child(right_child);
        }

        DecisionTreePredictor::new(root)
    }

    fn split_and_push(
        &self,
        stack: &mut VecDeque<StackRecord<P>>,
        splitter: &mut Splitter,
        group: &Group,
        depth: usize,
    ) -> TreeNode<P> {
        assert!(group.size() > 0);
        assert!(depth >= 1);

        if group.size() < self.config().min_split_size() {
            return TreeNode::new_leaf(self.to_terminal(group));
        }

        let split = splitter.find_best_split(group);
        if split.groups().right().size() == 0 {
            let groups = split.into_groups();
            let (left_group, _) = groups.into_parts();
            return TreeNode::new_leaf(self.to_terminal(&left_group));
        } else if split.groups().left().size() == 0 {
            let groups = split.into_groups();
            let (_, right_group) = groups.into_parts();
            return TreeNode::new_leaf(self.to_terminal(&right_group));
        }

        let node = TreeNode::new_split(split.index(), split.value());
        stack.push_back(StackRecord::new(node.clone(), split, depth));

        node
    }
}
