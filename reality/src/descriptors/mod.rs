//! Descriptor Types - Compile-Time Schemas
//!
//! This module contains all descriptor types that define WHAT things ARE
//! (Identity/Science pole of the Five-Fold Brahmachakra).
//!
//! ## The Five-Fold Descriptor System
//!
//! 1. **PropertyDescriptor** (CENTER ॐ) - The Form itself
//! 2. **ComputationDescriptor** - How computation manifests
//! 3. **StorageDescriptor** - How storage manifests  
//! 4. **ML Descriptors** - Machine learning pipeline schemas (including PipelineDescriptor)
//!
//! ## Organization
//!
//! - `property.rs` - PropertyDescriptor (the center, the Form)
//! - `computation.rs` - ComputationDescriptor (computation identity)
//! - `storage.rs` - StorageDescriptor (storage identity)
//! - `ml/` - ML-specific descriptors (models, pipelines, steps, training)
//!   - `ml/pipeline.rs` - PipelineDescriptor (ML training workflows)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gds::projection::codegen::descriptors::*;
//!
//! // Define a property
//! let prop_desc = PropertyDescriptor::new(1, "pagerank", ValueType::Double)
//!     .with_storage_hint(StorageHint::FixedWidth);
//!
//! // Define a computation
//! let comp_desc = ComputationDescriptor::new(
//!     1,
//!     "PageRank",
//!     ComputationSpecies::Bsp,
//!     ComputationPattern::VertexCentric,
//! );
//! ```

pub mod computation;
pub mod ml;
pub mod procedure;
pub mod property;
pub mod storage;

// Re-exports
pub use computation::{ComputationDescriptor, ComputationPattern, ComputationSpecies};
pub use procedure::{ProcedureCategory, ProcedureConfigFormat, ProcedureDescriptor, ProcedureMode};
pub use property::{
    FieldDescriptor, PropertyDescriptor, PropertyId, StorageHint, StructDescriptor, StructId,
};
pub use storage::*; // Many storage-related enums

// ML Pipeline is THE pipeline - re-export for convenience
pub use ml::PipelineDescriptor;
