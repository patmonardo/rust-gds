//! Neural Network Models
//!
//! This module contains neural network implementations translated from Java GDS.
//! Currently includes Multi-Layer Perceptron (MLP) classifier.

mod classifier;
mod config;
mod data;
mod objective;
mod trainer;

pub use classifier::MLPClassifier;
pub use config::MLPClassifierTrainConfig;
pub use data::MLPClassifierData;
pub use objective::MLPClassifierObjective;
pub use trainer::MLPClassifierTrainer;
