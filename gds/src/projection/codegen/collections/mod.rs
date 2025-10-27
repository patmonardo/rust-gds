//! Collections Codegen: Unified Collections Macro System
//!
//! **CONSOLIDATION COMPLETE**: All Collections-related macros now live here.
//! This module was previously split between `collections/macros/` (Level 0) and
//! `projection/codegen/collections/` (Projection). We've unified everything under
//! projection/codegen for clarity and to prepare for @reality migration.
//!
//! This module provides active collection-related macros:
//! - **Vec backend macros** (lightweight trait impl for Vec-backed types)
//! - **Huge backend macros** (lightweight trait impl for HugeArray types)
//!
//! **Philosophy**: Collections are the **typed columns** that back PropertyValues.
//! Every PropertyValues implementation needs a corresponding collection type for storage.
//!
//! **Architecture**:
//! - `vec_collections!` → Implements Collections trait for Vec-backed types
//! - `huge_collections!` → Implements Collections trait for HugeArray-backed types
//!
//! **Future**: Additional backends (sparse, atomic, cursors) can be added as needed.

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