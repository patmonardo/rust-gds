/// Projection-related types for graph data science.
///
/// This module contains types used for projecting and identifying
/// nodes and relationships in graphs.
///
/// NEW: Projection is now the home of the Eval/GDSL macro system:
/// - eval_macro: The master projector DSL (value_type_table!)
/// - form_processor: Transcendental policy surface (Pure Nama)
/// - functors: Gross ↔ Subtle conversion traits
/// - property_descriptor: Compile-time schema descriptors
// Eval macro must be loaded first (it's a macro_rules! macro)
#[macro_use]
pub mod eval_macro;

// Form Processor (Transcendental / Pure Nama)
pub mod form_processor;

// Functor traits for Gross ↔ Subtle conversions
pub mod functors;

// Pipeline descriptor types (Unity of Five-Fold Brahmachakra)
pub mod pipeline_descriptor;

// Backwards compatibility
#[doc(hidden)]
pub mod program_descriptor {
    pub use super::pipeline_descriptor::*;
}
#[doc(hidden)]
pub mod property_descriptor {
    pub use super::pipeline_descriptor::*;
}

// Five-Fold Brahmachakra components
pub mod computation_descriptor;
pub mod computation_runtime;
pub mod storage_descriptor;
pub mod storage_runtime;

// Prototype value type table (demonstrates the macro)
pub mod value_type_table;

// Existing projection types
pub mod impls;
pub mod node_label;
pub mod orientation;
pub mod relationship_type;
pub mod traits;

// Re-export core projection types
pub use impls::*;
pub use node_label::*;
pub use orientation::*;
pub use relationship_type::*;
pub use traits::*;

// Re-export Form Processor and functors
pub use form_processor::{
    checked_u64_to_usize, widen_f32_to_f64, widen_i32_to_i64, FormProcessorError,
};
pub use functors::{GrossSubtleFunctor, GrossToSubtle, SubtleToGross};

// Five-Fold Brahmachakra re-exports
pub use computation_descriptor::{ComputationDescriptor, ComputationPattern, ComputationSpecies};
pub use computation_runtime::{
    instantiate_computer_from_descriptor, register_computer_factory, ComputeContext, ComputeError,
    ComputeStep, Computer, Messages,
};
pub use pipeline_descriptor::{
    FieldDescriptor, PipelineDescriptor, ProgramDescriptor, PropertyDescriptor, PropertyId,
    StorageHint, StructDescriptor, StructId,
};
pub use storage_descriptor::{
    AccessPattern, BackendTechnology, Compression, ConcurrencyModel, Density, GrowthPolicy,
    Locality, MemoryProfile, Mutability, Persistence, PersistenceConfig, PhysicalGeometry,
    StorageDescriptor, StorageLayout, SyncPolicy,
};
pub use storage_runtime::{
    instantiate_storage_runtime_from_descriptor, register_storage_runtime_factory, AccessMode,
    StorageAccessor, StorageContext, StorageError, StorageRuntime, StorageValue,
};
