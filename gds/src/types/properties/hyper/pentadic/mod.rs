//! Pentadic module: Dyadic × Triadic composition
//!
//! Module layout mirrors `tetradic`:
//! - `types` : shared type aliases (including `Concepts`)
//! - `traits`: public trait(s) for the PentadicHyperStore
//! - `in_memory`: reference implementation for tests/examples
//! - `prelude`: ergonomic re-exports

pub mod in_memory;
pub mod prelude;
pub mod traits;
pub mod types;

pub use in_memory::*;
pub use prelude::*;
pub use traits::*;
pub use types::*;
