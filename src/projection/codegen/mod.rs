// Ensure the eval macro is declared first so it is available to subsequent
// modules inside `codegen` (the macro is used by `value_type_table`).
#[macro_use]
pub mod eval_macro;

// Lightweight in-repo config generation macro (demo)
pub mod config_macro;

pub mod computation_descriptor;
pub mod computation_runtime;
pub mod functors;
pub mod pipeline_descriptor;
pub mod property_descriptor;
pub mod storage_descriptor;
pub mod storage_runtime;
pub mod value_type_table;

// Re-exports for convenience when referencing codegen items directly.
pub use computation_descriptor::*;
pub use computation_runtime::*;
pub use functors::*;
// Explicit re-exports from pipeline_descriptor
pub use pipeline_descriptor::{
    FieldDescriptor, PipelineDescriptor, PropertyId, StructDescriptor, StructId,
};

// Explicit re-exports from property_descriptor
pub use property_descriptor::{PropertyDescriptor, StorageHint};
pub use storage_descriptor::*;
pub use storage_runtime::*;
pub use value_type_table::*;
