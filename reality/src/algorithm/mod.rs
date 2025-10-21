//! Algorithm Genetic Constituents
//!
//! This module contains the genetic constituents that give rise to Algorithms:
//! the logical structures that determine how a concept unfolds from thought into
//! concrete Storage + Computation manifestation.
//!
//! - `type_projector.rs` - Maya as Dialectical Absolute (PropertyDescriptor → Storage/Computation)
//! - `type_validator.rs` - Inference as Brahman-Knowing (Values → PropertyDescriptor)
//! - `functors.rs` - Gross ↔ Subtle conversions (Form Processor dependency)
//!
//! ## The Genetic Process
//!
//! An Algorithm is the Concept that subsumes:
//! - **Storage Runtime** (Being There - what persists)
//! - **Computation Runtime** (Ephemeral Nothing - what transforms)
//!
//! These genetic constituents extract and validate the essence:
//!
//! - **TypeProjector**: Maps Form (PropertyDescriptor) → Manifestations (Storage, Computation)
//! - **TypeValidator**: Maps Values → Form (inference, validation)
//! - **Functors**: Maps Gross (PropertyValues) ↔ Subtle (GdsValue)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gds::projection::codegen::transforms::*;
//!
//! // Project property descriptor to storage backend
//! let projector = HugeArrayProjector::new();
//! let storage_desc = projector.project_to_storage(&property_desc)?;
//!
//! // Infer descriptor from values
//! let inferred = TypeValidator::infer_from_values(&values)?;
//! ```

pub mod sum_aggregation;
pub mod type_projector;
pub mod type_validator;
// pub mod functors;  // Form processor dependency - commented out

// Re-exports
pub use sum_aggregation::{
    AggregationError, AggregationResult, AggregationSource, AggregationType, SumAggregation,
    SumAggregationMembership, SumAggregationProcedure,
};
pub use type_projector::{
    AdaptiveProjector, ArrowProjector, HugeArrayProjector, PregelProjector, ProjectionError,
    TypeProjector,
};
pub use type_validator::{TypeValidator, ValidationError};
// pub use functors::*;  // Form processor dependency - commented out
