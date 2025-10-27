//! Collections Backends: Backend Implementations
//!
//! This module provides backend implementations for Collections:
//! - Huge: Paged arrays for billions of elements (RAM)
//! - Vec: Simple wrappers around std::vec::Vec (RAM)
//! - Arrow: Future disk-backed storage (stub for now)
//!
//! The `factory` module provides helpers for creating backends from configuration.

pub mod huge;
pub mod vec;
pub mod arrow;
pub mod factory;

#[allow(ambiguous_glob_reexports)]
pub use huge::*;
#[allow(ambiguous_glob_reexports)]
pub use vec::*;
#[allow(ambiguous_glob_reexports)]
pub use arrow::*;
#[allow(ambiguous_glob_reexports)]
pub use factory::*;
