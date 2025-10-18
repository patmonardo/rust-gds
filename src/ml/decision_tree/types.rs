//! Type definitions for decision tree algorithms.
//!
//! Translated from Java GDS ml-algo Group.java, Groups.java, Split.java interfaces.
//! This is a literal 1:1 translation following repository translation policy.

use crate::collections::HugeLongArray;
use crate::ml::decision_tree::{ImpurityData, TreeNode};

/// A group of training samples with associated impurity data.
pub struct Group {
    array: HugeLongArray,
    start_idx: usize,
    size: usize,
    impurity_data: Box<dyn ImpurityData>,
}

impl Group {
    pub fn new(
        array: HugeLongArray,
        start_idx: usize,
        size: usize,
        impurity_data: Box<dyn ImpurityData>,
    ) -> Self {
        Self {
            array,
            start_idx,
            size,
            impurity_data,
        }
    }

    pub fn array(&self) -> &HugeLongArray {
        &self.array
    }

    pub fn start_idx(&self) -> usize {
        self.start_idx
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn impurity_data(&self) -> &dyn ImpurityData {
        self.impurity_data.as_ref()
    }

    pub fn impurity_data_mut(&mut self) -> &mut dyn ImpurityData {
        self.impurity_data.as_mut()
    }
}

/// Left and right child groups after a split.
pub struct Groups {
    left: Group,
    right: Group,
}

impl Groups {
    pub fn new(left: Group, right: Group) -> Self {
        Self { left, right }
    }

    pub fn left(&self) -> &Group {
        &self.left
    }

    pub fn right(&self) -> &Group {
        &self.right
    }

    /// Consume self and return both groups as a tuple.
    pub fn into_parts(self) -> (Group, Group) {
        (self.left, self.right)
    }
}

/// A split decision with feature index, threshold, and resulting groups.
pub struct Split {
    index: usize,
    value: f64,
    groups: Groups,
}

impl Split {
    pub fn new(index: usize, value: f64, groups: Groups) -> Self {
        Self {
            index,
            value,
            groups,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn groups(&self) -> &Groups {
        &self.groups
    }

    pub fn into_groups(self) -> Groups {
        self.groups
    }
}

/// Stack record for depth-first tree construction.
pub struct StackRecord<P> {
    node: TreeNode<P>,
    split: Option<Split>,
    depth: usize,
}

impl<P> StackRecord<P> {
    pub fn new(node: TreeNode<P>, split: Split, depth: usize) -> Self {
        Self {
            node,
            split: Some(split),
            depth,
        }
    }

    pub fn node_mut(&mut self) -> &mut TreeNode<P> {
        &mut self.node
    }

    pub fn split_owned(&mut self) -> Split {
        self.split.take().expect("Split already consumed")
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

/// Impurity criterion types for classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassifierImpurityCriterionType {
    Gini,
    Entropy,
}

impl ClassifierImpurityCriterionType {
    pub fn parse(input: &str) -> Result<Self, String> {
        match input.to_uppercase().as_str() {
            "GINI" => Ok(Self::Gini),
            "ENTROPY" => Ok(Self::Entropy),
            _ => Err(format!(
                "Impurity criterion `{}` is not supported. Must be one of: GINI, ENTROPY.",
                input
            )),
        }
    }
}

impl std::fmt::Display for ClassifierImpurityCriterionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gini => write!(f, "GINI"),
            Self::Entropy => write!(f, "ENTROPY"),
        }
    }
}
