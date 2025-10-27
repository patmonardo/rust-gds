//! Tetradic module: Dyad:Dyad plane
//!
//! Module layout:
//! - `types` : shared type aliases (including `Concepts`)
//! - `traits`: public trait(s) for the TetradicHyperStore
//! - `in_memory`: a small reference implementation for tests/examples
//! - `prelude`: ergonomic re-exports

pub mod in_memory;
pub mod prelude;
pub mod traits;
pub mod types;

pub use in_memory::*;
pub use prelude::*;
