//! Neural network implementations
//!
//! Contains:
//! - MLP (Multi-Layer Perceptron) classifier
//! - Core neural network functions (activation, loss)
//! - Training infrastructure

mod functions;
mod mlp;

pub use mlp::{MLPClassifier, MLPClassifierConfig, MLPClassifierData, MLPClassifierTrainer};
