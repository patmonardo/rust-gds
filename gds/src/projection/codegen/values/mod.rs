//! Value Codegen: The Primitive Values Macro System
//!
//! This module consolidates all value-related codegen macros:
//! - **ValueType Table**: Master controller mapping ValueType → Rust types
//! - **Scalar/Array Macros**: GdsValue implementations (Long, Double, String, Boolean)
//! - **Binary Macros**: Binary values with MIME metadata
//! - **Factory Macros**: PrimitiveValues factory for runtime type creation
//!
//! **Philosophy**: Macro-driven codegen ensures consistency across ValueType variants,
//! eliminates boilerplate, and enables Arrow/Polars compatibility through
//! uniform accessor patterns.
//!
//! **Architecture**: The ValueType table is the single source of truth that drives
//! all property value adapter generation across Node/Relationship/Graph levels.

#[macro_use]
mod value_type_table;
#[macro_use]
mod scalar_macros;
#[macro_use]
mod array_macros;
#[macro_use]
mod binary_macros;
#[macro_use]
mod factory_macros;
#[macro_use]
mod primitive_generator;

// Re-export the ValueType table for use across the codebase

// Re-export all macros at module level for barrel imports
// Note: These macros are available but not yet actively used in the current codebase
// pub use scalar_macros::*;
// pub use array_macros::*;
// pub use binary_macros::*;
// pub use factory_macros::*;
// pub use primitive_generator::*;
