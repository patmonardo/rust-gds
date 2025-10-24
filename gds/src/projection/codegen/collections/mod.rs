//! Collections Codegen: The Typed Column Foundation
//!
//! This module consolidates all collection-related macros that generate:
//! - HugeArray implementations (typed columns for billions of elements)
//! - Sparse collection variants (memory-efficient for sparse data)
//! - Atomic collection variants (thread-safe operations)
//! - Cursor support (zero-copy iteration over pages)
//!
//! **Philosophy**: Collections are the **flip side** of PropertyValues - they're the actual
//! typed columns that back property stores. Every PropertyValues implementation needs
//! a corresponding collection type for storage.
//!
//! **Aesthetic**: First-class terms via barrel imports:
//! - `projection::codegen::huge_array!` (not buried in submodules)
//! - `projection::codegen::sparse_collection!` (primary reality)
//! - `projection::codegen::atomic_collection!` (unified access)

/// HugeArray implementation macros
/// 
/// Generates typed columns supporting billions of elements with automatic
/// single-page vs multi-page selection based on size.
#[macro_use]
mod huge_array;

/// Sparse collection macros
/// 
/// Generates memory-efficient variants for sparse data patterns
/// with optional elements and compressed storage.
#[macro_use]
mod sparse_collection;

/// Atomic collection macros
/// 
/// Generates thread-safe variants for concurrent access patterns
/// with atomic operations and lock-free algorithms.
#[macro_use]
mod atomic_collection;

/// Cursor support macros
/// 
/// Generates zero-copy iteration support for all collection types
/// with page-aware cursors and efficient traversal.
#[macro_use]
mod cursor_support;

// Re-export all macros at module level for barrel imports
// Note: These modules are available but not yet actively used
// pub use huge_array::*;
// pub use sparse_collection::*;
// pub use atomic_collection::*;
// pub use cursor_support::*;