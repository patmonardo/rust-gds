//! Machine Learning models module
//!
//! This module contains core ML model traits and implementations.
//! The hierarchy is:
//!
//! - Base traits (Classifier, Regressor, BaseModelData, ClassifierData, RegressorData)
//! - Training method enum
//! - Feature system (Features trait and implementations)
//! - Trainer config system
//! - Model implementations by type:
//!   - Linear models (linear/logistic regression)
//!   - Tree-based models (random forests)
//!   - Neural networks (MLP)
//! - AutoML system for hyperparameter optimization

pub mod base;
pub mod config;
pub mod features;
pub mod linear;
pub mod logistic_regression;
// pub mod neural;
pub mod training_method;
pub mod trees;

// Core traits - 1:1 with Java GDS
pub use base::{
    BaseModelData, Classifier, ClassifierData, ClassifierTrainer, Features, LegacyBaseModelData,
    ModelData, Regressor, RegressorData, RegressorTrainer,
};
pub use config::{BaseTrainerConfig, ClassAwareTrainerConfig, PenaltyConfig, TrainerConfig};
pub use features::{DenseFeatures, FeaturesFactory, LazyFeatures};
pub use training_method::TrainingMethod;

// Model implementations
pub use linear::{LinearRegressionData, LinearRegressor};
pub use logistic_regression::{LogisticRegressionClassifier, LogisticRegressionData};
// pub use neural::{
//     MLPClassifier, MLPClassifierData, MLPClassifierObjective, MLPClassifierTrainConfig,
//     MLPClassifierTrainer,
// };
pub use trees::{
    DatasetBootstrapper, DecisionTreePredictor, RandomForestClassifier,
    RandomForestClassifierConfig, RandomForestClassifierData, RandomForestClassifierTrainerConfig,
    RandomForestConfig, RandomForestRegressor, RandomForestRegressorConfig,
    RandomForestRegressorData, RandomForestRegressorTrainerConfig,
};
