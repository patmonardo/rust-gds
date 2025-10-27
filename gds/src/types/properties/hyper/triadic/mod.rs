//! Triadic module: three-level composition (meta/node/link)
//!
//! This module exposes the triadic property store and a small, parallel
//! module surface that mirrors `tetradic` / `pentadic` (types/traits/in_memory/prelude).

pub mod in_memory;
pub mod prelude;
pub mod traits;
pub mod types;

pub use in_memory::*;
pub use prelude::*;
