//! Property Codegen: The Smart Converter Foundation
//!
//! This module consolidates all property-related macros that generate:
//! - PropertyValues implementations (typed storage backends)
//! - Smart converter traits (type-driven dispatch)
//! - Cursor accessors (zero-copy typed getters)
//! - Selector validation (ValueType compatibility)
//!
//! **Philosophy**: Every property column is a Smart Converter that accepts all queries
//! and either returns exact type (zero-cost), converts compatible types (i64→f64), 
//! or throws error if incompatible (Long→LongArray).
//!
//! **Aesthetic**: First-class terms via barrel imports:
//! - `projection::codegen::property_values_impl!` (not buried in submodules)
//! - `projection::codegen::smart_converter!` (primary reality)
//! - `projection::codegen::cursor_accessors!` (unified access)

/// Core PropertyValues implementation macros
/// 
/// Generates typed storage backends for each ValueType with proper
/// element counting and type conversion policies.
#[macro_use]
mod property_values;

/// Smart converter trait macros
/// 
/// Generates type-driven dispatch for `get<T>()` pattern that
/// eliminates namespace pollution while maintaining type safety.
#[macro_use]
mod smart_converter;

/// Cursor accessor macros
/// 
/// Generates zero-copy typed getters for relationship/node cursors
/// with Arrow/HugeArray backend support.
#[macro_use]
mod cursor_accessors;

/// Selector validation macros
/// 
/// Generates spec-time ValueType compatibility checks for weights,
/// keys, and other property constraints.
#[macro_use]
mod selector_validation;

/// Typed PropertyValues macros
/// 
/// Generates typed PropertyValues implementations for each ValueType
/// with dense Vec, Arrow-backed, and sparse variants.
#[macro_use]
mod typed_property_values;

/// Triadic PropertyStore matrix macros (Level × Component × ValueTypes)
#[macro_use]
mod triadic_macros;

// Re-export all macros at module level for barrel imports
pub use property_values::*;
pub use smart_converter::*;
pub use selector_validation::*;
// Note: cursor_accessors and typed_property_values are available but not yet used
// pub use cursor_accessors::*;
// pub use typed_property_values::*;
