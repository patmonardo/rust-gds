/// ML model implementations.
///
/// This module contains model trait and concrete implementations.
/// Node-centric: models predict node properties from node features.
///
/// Phase 2.3: decision_tree_stub is a minimal stub for architecture demonstration.
/// Actual model implementations belong in ml-algo package.
pub mod decision_tree_stub;
pub mod model_trait;

// Re-exports
pub use decision_tree_stub::DecisionTreeClassifier;
pub use model_trait::{Model, ModelError, ModelMetadata};
