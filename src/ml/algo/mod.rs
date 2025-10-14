//! ML Algorithm implementations for GDS.
//!
//! This module contains machine learning algorithms built on top of ml-core primitives.

pub mod decision_tree;

pub use decision_tree::{
    DecisionTreeClassifierTrainer, DecisionTreePredictor, DecisionTreeRegressorTrainer,
    DecisionTreeTrainer, DecisionTreeTrainerConfig, DecisionTreeTrainerConfigBuilder,
};
