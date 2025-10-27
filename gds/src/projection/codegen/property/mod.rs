//! Property Codegen: Universal Adapter System
//!
//! This module provides the macro system for generating property value adapters
//! from the ValueType table. All adapters are generic over Collections backends,
//! enabling runtime backend selection (Vec, Huge, Arrow).
//!
//! **Philosophy**: Universal adapter pattern where all property columns accept
//! all queries and either return exact type (zero-cost), convert compatible types
//! (i64→f64), or throw error if incompatible.
//!
//! **Architecture**:
//! ```
//! ValueType Table (Master Controller)
//!     ↓
//! Universal Adapter Macros (triadic_macros.rs)
//!     ↓
//! Trait Implementation Helpers (property_values.rs)
//!     ↓
//! Generated Adapters (DefaultLongNodePropertyValues<C>, etc.)
//! ```

/// Core PropertyValues implementation macros
/// 
/// Generates typed storage backends for each ValueType with proper
/// element counting and type conversion policies.
#[macro_use]
mod property_values;

/// Triadic PropertyStore matrix macros
/// 
/// Generates property adapters for Node, Relationship, and Graph properties
/// using the universal adapter pattern with Collections backends.
#[macro_use]
mod triadic_macros;

// Re-export all macros at module level for barrel imports
pub use property_values::*;
