//! Machine Learning module for GDS.
//!
//! Contains:
//! - Core ML primitives (tensors, variables, functions)
//! - Model traits and implementations (linear, trees, neural)
//! - Algorithm implementations (decision trees, etc.)
//! - Training and evaluation infrastructure
//! - Node prediction and classification
//! - Sampling and splitting strategies

// Core functionality
pub mod core;
pub mod metrics;
pub mod models;

// Algorithm implementations
pub mod decision_tree;
pub mod gradient_descent;
pub mod link_models;
pub mod node_classification;
pub mod node_prediction;

// Training infrastructure
pub mod sampling;
pub mod splitting;
pub mod training;
pub mod training_method;

// Curated re-exports for common surface area
pub use core::tensor::{size_in_bytes, Matrix, Scalar, Tensor, Vector};
pub use models::{
    BaseModelData, Classifier, ClassifierData, ClassifierTrainer, DenseFeatures, Features,
    FeaturesFactory, LazyFeatures, ModelData, Regressor, RegressorData, RegressorTrainer,
    TrainingMethod,
};

/// Shared ML prelude mirroring the Java convenience imports.
pub mod prelude {
    pub use super::core::dimensions::{
        is_scalar, is_vector, matrix, render, scalar, total_size, vector,
    };
    pub use super::{
        size_in_bytes, BaseModelData, Classifier, ClassifierData, ClassifierTrainer, DenseFeatures,
        Features, FeaturesFactory, LazyFeatures, Matrix, ModelData, Regressor, RegressorData,
        RegressorTrainer, Scalar, Tensor, TrainingMethod, Vector,
    };
}
