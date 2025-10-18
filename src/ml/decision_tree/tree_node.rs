//! Tree node for decision trees in GDS.
//!
//! Translated from Java GDS ml-algo TreeNode.java.
//! This is a literal 1:1 translation following repository translation policy.

use std::fmt;

/// A node in a decision tree, either a split node or a leaf.
///
/// Corresponds to TreeNode<PREDICTION> in Java GDS.
#[derive(Clone, Debug)]
pub struct TreeNode<P> {
    prediction: Option<P>,
    feature_index: i32,
    threshold_value: f64,
    left_child: Option<Box<TreeNode<P>>>,
    right_child: Option<Box<TreeNode<P>>>,
}

impl<P: Clone> TreeNode<P> {
    /// Create a split node with a feature index and threshold.
    pub fn new_split(index: usize, value: f64) -> Self {
        Self {
            prediction: None,
            feature_index: index as i32,
            threshold_value: value,
            left_child: None,
            right_child: None,
        }
    }

    /// Create a leaf node with a prediction.
    pub fn new_leaf(prediction: P) -> Self {
        Self {
            prediction: Some(prediction),
            feature_index: -1,
            threshold_value: 0.0,
            left_child: None,
            right_child: None,
        }
    }

    pub fn split_memory_estimation() -> usize {
        std::mem::size_of::<TreeNode<()>>()
    }

    pub fn leaf_memory_estimation() -> usize
    where
        P: Sized,
    {
        std::mem::size_of::<TreeNode<P>>() + std::mem::size_of::<P>()
    }

    pub fn set_prediction(&mut self, prediction: P) {
        self.prediction = Some(prediction);
    }

    pub fn prediction(&self) -> Option<&P> {
        self.prediction.as_ref()
    }

    pub fn feature_index(&self) -> i32 {
        self.feature_index
    }

    pub fn threshold_value(&self) -> f64 {
        self.threshold_value
    }

    pub fn left_child(&self) -> Option<&TreeNode<P>> {
        self.left_child.as_deref()
    }

    pub fn left_child_mut(&mut self) -> Option<&mut TreeNode<P>> {
        self.left_child.as_deref_mut()
    }

    pub fn set_left_child(&mut self, child: TreeNode<P>) {
        self.left_child = Some(Box::new(child));
    }

    pub fn has_left_child(&self) -> bool {
        self.left_child.is_some()
    }

    pub fn right_child(&self) -> Option<&TreeNode<P>> {
        self.right_child.as_deref()
    }

    pub fn right_child_mut(&mut self) -> Option<&mut TreeNode<P>> {
        self.right_child.as_deref_mut()
    }

    pub fn set_right_child(&mut self, child: TreeNode<P>) {
        self.right_child = Some(Box::new(child));
    }

    pub fn has_right_child(&self) -> bool {
        self.right_child.is_some()
    }

    /// Renders the tree into a human readable representation.
    pub fn render(&self) -> String
    where
        P: fmt::Display,
    {
        let mut sb = String::new();
        Self::render_recursive(&mut sb, Some(self), 0);
        sb
    }

    fn render_recursive(sb: &mut String, node: Option<&TreeNode<P>>, depth: usize)
    where
        P: fmt::Display,
    {
        let Some(node) = node else { return };

        if depth > 0 {
            sb.push_str(&"\t".repeat(depth - 1));
            sb.push_str("|-- ");
        }

        sb.push_str(&format!("{}\n", node));

        Self::render_recursive(sb, node.left_child(), depth + 1);
        Self::render_recursive(sb, node.right_child(), depth + 1);
    }
}

impl<P: fmt::Display> fmt::Display for TreeNode<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Node: prediction {:?}, featureIndex {}, splitValue {}",
            self.prediction.as_ref().map(|p| p.to_string()),
            self.feature_index,
            self.threshold_value
        )
    }
}

impl<P: PartialEq> PartialEq for TreeNode<P> {
    fn eq(&self, other: &Self) -> bool {
        self.feature_index == other.feature_index
            && self.threshold_value == other.threshold_value
            && self.prediction == other.prediction
            && self.left_child == other.left_child
            && self.right_child == other.right_child
    }
}

impl<P: Eq> Eq for TreeNode<P> {}
