//! Projection-related types for graph data science.
//!
//! This module is intentionally small. Complex codegen and macro machinery
//! lives in the `projection::codegen` submodule to keep the root lightweight.
//!
//! The root re-exports the small, public projection surface used across the crate
//! (labels, relation types, orientations and traits) and leaves heavy internals
//! under `projection::codegen` and other submodules.

// The eval macro and other heavy codegen live under `projection::codegen`.
// We don't declare `eval_macro` at the projection root to avoid attempting to
// load files that live under `projection/codegen`.
// `eval_macro` lives under `projection::codegen`. The macro itself is exported
// with `#[macro_export]` so it is available crate-wide. We no longer forward
// the module at the projection root to prefer a single canonical location.

// Lightweight, widely-used projection types
pub mod impls;
pub mod node_label;
pub mod orientation;
pub mod relationship_type;
pub mod traits;

// Keep form_processor available at the root for convenience
pub mod form_processor;
// Re-export functors from the codegen submodule (they live under codegen)
pub mod functors {
    pub use crate::projection::codegen::functors::*;
}

// Codegen moved into a dedicated submodule to keep the root small
pub mod codegen;

// Re-export the small, stable projection surface
pub use impls::*;
pub use node_label::*;
pub use orientation::*;
pub use relationship_type::*;
pub use traits::*;

// Re-export commonly-used helpers
pub use form_processor::{
    checked_u64_to_usize, widen_f32_to_f64, widen_i32_to_i64, FormProcessorError,
};
pub use functors::{GrossSubtleFunctor, GrossToSubtle, SubtleToGross};

// Re-export select codegen items at the projection root for compatibility
// Explicit re-exports from codegen to avoid ambiguous glob re-exports
pub use codegen::computation_descriptor::{
    ComputationDescriptor, ComputationPattern, ComputationSpecies,
};
pub use codegen::computation_runtime::{
    instantiate_computer_from_descriptor, register_computer_factory, ComputeContext, ComputeError,
    ComputeStep, Computer, Messages,
};
pub use codegen::pipeline_descriptor::{
    FieldDescriptor, PipelineDescriptor, PropertyId, StructDescriptor, StructId,
};
pub use codegen::property_descriptor::{PropertyDescriptor, StorageHint};
pub use codegen::storage_descriptor::{
    AccessPattern, BackendTechnology, Compression, ConcurrencyModel, Density, GrowthPolicy,
    Locality, MemoryProfile, Mutability, Persistence, PersistenceConfig, PhysicalGeometry,
    StorageDescriptor, StorageLayout, SyncPolicy,
};
pub use codegen::storage_runtime::{
    instantiate_storage_runtime_from_descriptor, register_storage_runtime_factory, AccessMode,
    StorageAccessor, StorageContext, StorageError, StorageRuntime, StorageValue,
};
pub use codegen::value_type_table::*;

// --- Compatibility re-exports (forward to codegen) -------------------------
// Some parts of the codebase import `crate::projection::pipeline_descriptor` or
// `crate::projection::storage_descriptor` directly. Forward those names to the
// `projection::codegen` equivalents to minimize changes.
pub mod pipeline_descriptor {
    pub use crate::projection::codegen::pipeline_descriptor::*;
}

pub mod storage_descriptor {
    pub use crate::projection::codegen::storage_descriptor::*;
}

pub mod storage_runtime {
    pub use crate::projection::codegen::storage_runtime::*;
}

pub mod computation_descriptor {
    pub use crate::projection::codegen::computation_descriptor::*;
}

pub mod computation_runtime {
    pub use crate::projection::codegen::computation_runtime::*;
}

pub mod property_descriptor {
    pub use crate::projection::codegen::property_descriptor::*;
}

pub mod value_type_table {
    pub use crate::projection::codegen::value_type_table::*;
}
