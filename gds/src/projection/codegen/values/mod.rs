//! Value Codegen: The Primitive Values Macro System
//!
//! This module consolidates all GdsValue-related macros that generate:
//! - Scalar value implementations (Long, Double, String, Boolean)
//! - Array value implementations (with/without conversion)
//! - Binary values with MIME metadata
//! - PrimitiveValues factory for runtime type creation
//!
//! **Philosophy**: Macro-driven codegen ensures consistency across ValueType variants,
//! eliminates boilerplate, and enables future Arrow/Polars compatibility through
//! uniform accessor patterns.
//!
//! **Aesthetic**: First-class terms via barrel imports:
//! - `projection::codegen::gds_value_scalar!` (primary reality)
//! - `projection::codegen::generate_primitive_values!` (unified generation)

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

// Re-export all macros at module level for barrel imports
// Note: These macros are available but not yet actively used in the current codebase
// pub use scalar_macros::*;
// pub use array_macros::*;
// pub use binary_macros::*;
// pub use factory_macros::*;
// pub use primitive_generator::*;
