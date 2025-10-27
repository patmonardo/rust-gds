//! Collections Codegen: Unified Collections Macro System
//!
//! **CONSOLIDATION COMPLETE**: All Collections-related macros now live here.
//! This module was previously split between `collections/macros/` (Level 0) and
//! `projection/codegen/collections/` (Projection). We've unified everything under
//! projection/codegen for clarity and to prepare for @reality migration.
//!
//! This module provides all collection-related macros:
//! - **Vec backend macros** (lightweight trait impl for Vec-backed types)
//! - **HugeArray generators** (typed columns for billions of elements)
//! - **Sparse collection variants** (memory-efficient for sparse data)
//! - **Atomic collection variants** (thread-safe operations)
//! - **Cursor support** (zero-copy iteration over pages)
//!
//! **Philosophy**: Collections are the **typed columns** that back PropertyValues.
//! Every PropertyValues implementation needs a corresponding collection type for storage.
//!
//! **Architecture**:
//! - Lightweight macros (`vec_collections!`) → Implement Collections trait on existing structs
//! - Heavy generators (`huge_primitive_array!`) → Generate complete new collection types
//!
//! **Future**: This will move to @reality as the foundation for data science.

/// Vec backend macros (ACTIVE - lightweight trait impl)
/// 
/// Implements Collections trait for Vec-backed types with a `data: Vec<T>` field.
/// This is the "Level 0" infrastructure - just trait implementation, no type generation.
/// 
/// **Moved from**: `collections/macros/backends/vec.rs`
#[macro_use]
mod vec_backend;

/// Huge backend macros (ACTIVE - lightweight trait impl)
/// 
/// Implements Collections trait for HugeArray-backed types (paged arrays for billions of elements).
/// Similar to vec_backend, but for HugeArrays - just trait implementation, no type generation.
/// 
/// **Moved from**: `collections/macros/backends/huge.rs`
#[macro_use]
mod huge_backend;

/// HugeArray generator macros (FUTURE EXPANSION)
/// 
/// Generates typed columns supporting billions of elements with automatic
/// single-page vs multi-page selection based on size.
#[macro_use]
mod huge_array;

/// Sparse collection macros (FUTURE EXPANSION)
/// 
/// Generates memory-efficient variants for sparse data patterns
/// with optional elements and compressed storage.
#[macro_use]
mod sparse_collection;

/// Atomic collection macros (FUTURE EXPANSION)
/// 
/// Generates thread-safe variants for concurrent access patterns
/// with atomic operations and lock-free algorithms.
#[macro_use]
mod atomic_collection;

/// Cursor support macros (FUTURE EXPANSION)
/// 
/// Generates zero-copy iteration support for all collection types
/// with page-aware cursors and efficient traversal.
#[macro_use]
mod cursor_support;

// Re-export active backend macros
pub use vec_backend::*;
pub use huge_backend::*;

// Future expansion macros (not yet activated)
// pub use huge_array::*;
// pub use sparse_collection::*;
// pub use atomic_collection::*;
// pub use cursor_support::*;