//! Collections Macros: Unified Macro System
//!
//! This module provides the unified macro system for generating Collections
//! implementations, including core macros, backend macros, extension macros,
//! magic macros, and code generators.

pub mod core;
pub mod backends;
pub mod adapter;

pub use core::*;
pub use backends::*;