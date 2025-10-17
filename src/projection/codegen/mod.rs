//! Code Generation - The Projection System
//!
//! This module contains the complete code generation infrastructure for rust-gds,
//! organized into five distinct concerns following the **Five-Fold Brahmachakra** pattern.
//!
//! ## The Five-Fold Brahmachakra
//!
//! ```text
//! Macros (Tools that PROJECT)
//!     ↓
//! Descriptors (Identity/Science - WHAT things ARE)
//!     ↓
//! Runtimes (Difference/Manifestation - HOW things EXECUTE)
//!     ↓
//! Transforms (Maya/Projection - MAPPINGS between extremes)
//!     ↓
//! Procedure Contract (`projection::eval::procedure` - what algorithms implement)
//! ```
//!
//! ## Organization
//!
//! - `macros/` - Code generation TOOLS (eval_macro, config, procedure macros)
//! - `descriptors/` - Compile-time SCHEMAS (property, computation, storage, pipeline, ML)
//! - `runtimes/` - Execution CONTRACTS (Computer, ComputeStep, StorageRuntime, etc.)
//! - `transforms/` - Cross-cutting conversions (TypeProjector, TypeValidator, Functors)
//! - `AlgorithmSpec` now lives under `projection::eval::procedure`
//!
//! ## Usage
//!
//! ```rust,ignore
//! // Get macros
//! use rust_gds::projection::codegen::macros::*;
//!
//! // Get descriptors
//! use rust_gds::projection::codegen::descriptors::*;
//!
//! // Get runtime contracts
//! use rust_gds::projection::codegen::runtimes::*;
//!
//! // Get algorithm contract (lives under the executor runtime)
//! use rust_gds::projection::eval::procedure::AlgorithmSpec;
//!
//! // Get transforms
//! use rust_gds::projection::codegen::transforms::*;
//! ```

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

#[macro_use]
pub mod macros;
pub mod descriptors;
pub mod runtimes;
pub mod transforms;

// ============================================================================
// RE-EXPORTS
// ============================================================================

// Macros (re-exported at crate root via #[macro_export])
pub use macros::*;

// Descriptors
pub use descriptors::{
    ComputationDescriptor, ComputationPattern, ComputationSpecies, FieldDescriptor,
    PropertyDescriptor, PropertyId, StorageHint, StructDescriptor, StructId,
};

// ML Pipeline is THE pipeline (re-export for convenience)
pub use descriptors::PipelineDescriptor;

// Runtimes
pub use runtimes::{
    instantiate_computer_from_descriptor, register_computer_factory, AccessMode, ComputeContext,
    ComputeError, ComputeStep, Computer, ComputerFactory, Messages, StorageAccessor,
    StorageContext, StorageError, StorageRuntime, StorageRuntimeFactory, StorageValue,
};

// Transforms
pub use transforms::{
    AdaptiveProjector, ArrowProjector, HugeArrayProjector, PregelProjector, ProjectionError,
    TypeProjector, TypeValidator, ValidationError,
};
