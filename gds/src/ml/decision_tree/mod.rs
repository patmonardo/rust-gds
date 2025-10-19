//! Decision tree algorithms for ML in GDS.
//!
//! Translated from Java GDS ml-algo decisiontree package.
//! This is a literal 1:1 translation following repository translation policy.

mod classifier_trainer;
mod entropy;
mod feature_bagger;
mod gini_index;
mod impurity_criterion;
mod predictor;
mod regressor_trainer;
mod split_mse;
mod splitter;
mod trainer;
mod trainer_config;
mod tree_node;
mod types;

#[cfg(test)]
mod tests;

pub use classifier_trainer::DecisionTreeClassifierTrainer;
pub use entropy::{Entropy, EntropyImpurityData};
pub use feature_bagger::FeatureBagger;
pub use gini_index::{GiniImpurityData, GiniIndex};
pub use impurity_criterion::{ImpurityCriterion, ImpurityData, ImpurityDataAny};
pub use predictor::DecisionTreePredictor;
pub use regressor_trainer::DecisionTreeRegressorTrainer;
pub use split_mse::{MSEImpurityData, SplitMeanSquaredError};
pub use splitter::{Features, Splitter};
pub use trainer::DecisionTreeTrainer;
pub use trainer_config::{DecisionTreeTrainerConfig, DecisionTreeTrainerConfigBuilder};
pub use tree_node::TreeNode;
pub use types::{ClassifierImpurityCriterionType, Group, Groups, Split, StackRecord};
