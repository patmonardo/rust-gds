//! Feature extraction module for ML in GDS.
//!
//! This module contains the core feature extraction system translated from Java GDS ml-core.
//!
//! ## Core Components (1:1 from Java)
//!
//! - **FeatureExtractor** - Marker trait with `dimension()` method
//! - **ScalarFeatureExtractor** - Extracts scalar (f64) features
//! - **ArrayFeatureExtractor** - Extracts array (Vec<f64>) features  
//! - **FeatureConsumer** - Interface for consuming extracted features
//! - **FeatureExtraction** - Utility functions for orchestrating extraction
//!
//! ## Translation Notes
//!
//! This is a literal 1:1 translation from Java GDS ml-core/features package.
//! - Java's static utility class → Rust module with functions
//! - Java's instanceof → Rust trait downcasting (to be implemented)
//! - Java's FeatureConsumer.NOOP → Rust NoopConsumer constant

pub mod array_feature_extractor;
pub mod bias_feature;
pub mod feature_consumer;
pub mod feature_extraction;
pub mod feature_extractor;
pub mod scalar_feature_extractor;
pub mod scalar_property_extractor;

// Re-export core types

pub use array_feature_extractor::*;
pub use bias_feature::*;
pub use feature_consumer::*;
pub use feature_extraction::*;
pub use feature_extractor::*;
pub use scalar_feature_extractor::*;
pub use scalar_property_extractor::*;
