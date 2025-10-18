//! Decision tree predictor.
//!
//! Translated from Java GDS ml-algo DecisionTreePredictor.java.
//! This is a literal 1:1 translation following repository translation policy.

use crate::ml::decision_tree::TreeNode;

#[derive(Debug)]
pub struct DecisionTreePredictor<P> {
    pub root: TreeNode<P>,
}

impl<P: Clone> DecisionTreePredictor<P> {
    pub fn new(root: TreeNode<P>) -> Self {
        Self { root }
    }

    pub fn predict(&self, features: &[f64]) -> &P {
        assert!(!features.is_empty());

        let mut node = &self.root;

        while node.has_left_child() {
            assert!(features.len() > node.feature_index() as usize);
            assert!(node.has_right_child());

            if features[node.feature_index() as usize] < node.threshold_value() {
                node = node.left_child().unwrap();
            } else {
                node = node.right_child().unwrap();
            }
        }

        node.prediction().unwrap()
    }
}

impl<P: PartialEq> PartialEq for DecisionTreePredictor<P> {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl<P: Eq> Eq for DecisionTreePredictor<P> {}
