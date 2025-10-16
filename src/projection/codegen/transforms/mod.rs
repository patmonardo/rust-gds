//! Cross-Cutting Transformations
//!
//! This module contains conversion utilities that operate across
//! the descriptor and runtime spaces:
//!
//! - `type_projector.rs` - Maya as Dialectical Absolute (PropertyDescriptor → Storage/Computation)
//! - `type_validator.rs` - Inference as Brahman-Knowing (Values → PropertyDescriptor)
//! - `functors.rs` - Gross ↔ Subtle conversions (Form Processor dependency)
//!
//! ## The Transcendental Function
//!
//! While `descriptors/` and `runtime/` define the extremes (WHAT and HOW),
//! `transforms/` provides the MAPPINGS between them - the projective geometry
//! of the type system.
//!
//! - **TypeProjector**: Maps Form (PropertyDescriptor) → Manifestations (Storage, Computation)
//! - **TypeValidator**: Maps Values → Form (inference, validation)
//! - **Functors**: Maps Gross (PropertyValues) ↔ Subtle (GdsValue)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use rust_gds::projection::codegen::transforms::*;
//!
//! // Project property descriptor to storage backend
//! let projector = HugeArrayProjector::new();
//! let storage_desc = projector.project_to_storage(&property_desc)?;
//!
//! // Infer descriptor from values
//! let inferred = TypeValidator::infer_from_values(&values)?;
//! ```

pub mod type_projector;
pub mod type_validator;
// pub mod functors;  // Form processor dependency - commented out

// Re-exports
pub use type_projector::{
    AdaptiveProjector, ArrowProjector, HugeArrayProjector, PregelProjector, ProjectionError,
    TypeProjector,
};
pub use type_validator::{TypeValidator, ValidationError};
// pub use functors::*;  // Form processor dependency - commented out
