//! Random forest implementations for classification and regression
//!
//! 1:1 translation of org.neo4j.gds.ml.models.randomforest package from Java GDS.
//!
//! Contains:
//! - Base RandomForest traits and configs
//! - Dataset bootstrapping utilities
//! - Classification implementation
//! - Regression implementation
//! - Trainers (TODO: translate from Java)

mod bootstrapper;
mod classifier;
mod config;
mod regressor;

pub use bootstrapper::*;
pub use classifier::*;
pub use config::*;
pub use regressor::*;

// TODO: Translate trainers from Java GDS:
// - RandomForestClassifierTrainer
// - RandomForestRegressorTrainer
