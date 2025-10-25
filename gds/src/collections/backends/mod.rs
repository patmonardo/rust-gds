//! Collections Backends: Backend Implementations
//!
//! This module provides backend implementations for Collections:
//! - Huge: Paged arrays for billions of elements (RAM)
//! - Vec: Simple wrappers around std::vec::Vec (RAM)
//! - Arrow: Future disk-backed storage (stub for now)

pub mod huge;
pub mod vec;
pub mod arrow;

pub use huge::*;
pub use vec::*;
pub use arrow::*;
