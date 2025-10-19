/// Feature assembly and transformation system.
///
/// This module provides traits and implementations for transforming
/// graph properties into ML-ready features. Mirrors Java GDS feature system.
///
/// # Architecture
///
/// - `Transformation` trait: Transform property values → feature values
/// - `FeatureAssembler`: Orchestrate feature assembly from multiple properties
/// - `validation`: Validate features (check for NaN, dimension mismatches)
///
/// # Usage
///
/// ```ignore
/// use gds::projection::eval::form::features::*;
///
/// // Create assembler
/// let assembler = DefaultFeatureAssembler::new();
///
/// // Assemble feature from properties
/// let feature = assembler.assemble(&properties, &step_descriptor)?;
///
/// // Validate
/// validation::validate_features(&feature)?;
/// ```
pub mod assembler;
pub mod transformation;
pub mod validation;

// Re-exports
pub use assembler::{DefaultFeatureAssembler, FeatureAssembler};
pub use transformation::{
    CombineTransformation, IdentityTransformation, NormalizationStrategy, NormalizeTransformation,
    Transformation,
};
pub use validation::{property_dimension, validate_features};
