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

// Property descriptor types
pub mod property_descriptor;

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
pub use property_descriptor::{PropertyDescriptor, PropertyId, StorageHint, StructDescriptor};
