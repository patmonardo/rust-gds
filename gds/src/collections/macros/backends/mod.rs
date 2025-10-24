//! Backend Collections Macros: Backend-Specific Implementations
//!
//! This module provides backend-specific macros for generating Collections
//! implementations for different backends (Vec, Huge, Arrow, etc.).

pub mod vec;
pub mod huge;
pub use vec::*;
pub use huge::*;
