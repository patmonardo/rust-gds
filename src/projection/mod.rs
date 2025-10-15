//! Projection-related types for graph data science.
//!
//! Simple module containing lightweight projection types (labels, relationship types,
//! orientations) and delegating heavy codegen machinery to the `codegen` submodule.
//!
//! ## Architecture
//!
//! - **projection/** - Core projection types (this module)
//! - **projection/codegen/** - Code generation descriptors and runtime
//! - **native/** - Native implementations, including form_processor
//!
//! ## Tomorrow's Work Focus
//!
//! We'll be working primarily in two folders:
//! 1. **codegen/** - Generate descriptors and specifications
//! 2. **native/** - Implement runtime execution (form_processor is the nexus)

// ------------------------------------------------------------------------
// Core projection types (simple, widely used)
// ------------------------------------------------------------------------
pub mod impls;
pub mod node_label;
pub mod orientation;
pub mod relationship_type;
pub mod traits;

// Re-export the stable projection surface
pub use impls::*;
pub use node_label::*;
pub use orientation::*;
pub use relationship_type::*;
pub use traits::*;

// ------------------------------------------------------------------------
// Native implementation layer (form_processor, native_factory)
// ------------------------------------------------------------------------
pub mod native;

// ------------------------------------------------------------------------
// Heavy codegen machinery (isolated in submodule)
// ------------------------------------------------------------------------
pub mod codegen;

// Re-export commonly used codegen types
// pub use codegen::functors::{GrossSubtleFunctor, GrossToSubtle, SubtleToGross};  // Form processor dependency
pub use codegen::property_descriptor;

// That's it! Everything else stays under codegen:: or native::.
// If you need ComputationDescriptor, use: crate::projection::codegen::computation_descriptor::ComputationDescriptor
// If you need StorageDescriptor, use: crate::projection::codegen::storage_descriptor::StorageDescriptor
// If you need form_processor, use: crate::projection::native::form_processor
