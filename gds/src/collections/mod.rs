//! Collections: Unified Data Structures for Graph Data Science
//!
//! This module provides a unified Collections API across multiple backends:
//! - **Huge**: Paged arrays for billions of elements
//! - **Vec**: Enhanced standard library vectors
//! - **Arrow**: Apache Arrow columnar arrays
//! - **Extensions**: ndarray, GPU, distributed, compression, encryption
//! - **Magic**: Auto-optimization, AI-powered features
//!
//! ## Architecture
//!
//! ```text
//! Application Layer (PropertyValues)
//!         ↓
//! Adapter Layer (UniversalPropertyValues)
//!         ↓
//! Collections Layer (Huge/Vec/Arrow/Extensions)
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use crate::collections::{HugeIntArray, VecInt, ArrowInt};
//! use crate::collections::Collections;
//! use crate::collections::UniversalPropertyValues;
//!
//! // All implement same interface
//! let huge: HugeIntArray = HugeIntArray::new(1000);
//! let vec: VecInt = VecInt::new();
//! let arrow: ArrowInt = ArrowInt::new();
//!
//! // Same API for all
//! let sum1 = huge.sum();
//! let sum2 = vec.sum();
//! let sum3 = arrow.sum();
//! ```

// Core traits
pub mod traits;

// HyperStore: Pure Being (Part 1 of Objective Logic)
pub mod hyper_store;

// Backend implementations
pub mod backends;

// Extension implementations
pub mod extensions;

// NOTE: Macro system has been moved to projection/codegen/collections/
// All collections macros (vec_collections!, huge_collections!, etc.) now live there
// This prepares for @reality migration as the foundation for data science

// Utilities
pub mod utils;

// Universal adapter
pub mod adapter;

// Re-export commonly used types
pub use traits::*;
pub use backends::*;
pub use extensions::*;
pub use utils::*;
pub use adapter::*;

// NOTE: Macros are now in projection::codegen::collections
// Use: `use crate::{vec_collections, huge_collections};` instead

// Re-export legacy modules for backward compatibility
pub mod bit_set;
pub mod huge_sparse_array;
pub mod huge_sparse_list;
pub mod indirect_comparator;
pub mod long_multiset;
pub mod primitive;

// Re-export types from core for backward compatibility
pub use crate::core::utils::paged::HugeAtomicBitSet;

// Backend selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionsBackend {
    Huge,   // Paged arrays
    Vec,    // Enhanced vectors
    Arrow,  // Apache Arrow
    Std,    // Standard library
}

impl Default for CollectionsBackend {
    fn default() -> Self {
        Self::Vec
    }
}