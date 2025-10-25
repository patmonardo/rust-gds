//! Collections Macros: Unified Macro System
//!
//! This module provides the unified macro system for generating Collections
//! implementations, including core macros, backend macros, extension macros,
//! magic macros, and code generators.

pub mod core;
pub mod backends;
pub mod extensions;
pub mod generators;
pub mod storage_descriptor;
pub mod adapter;

pub use core::*;
pub use backends::*;
pub use extensions::*;
pub use generators::*;
pub use storage_descriptor::*;
pub use adapter::*;
