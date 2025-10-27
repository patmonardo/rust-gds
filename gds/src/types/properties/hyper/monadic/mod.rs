//! Monadic hyper-layer: universal building block (n=1)
//!
//! Canonical layout: types, traits, property_values, in_memory, prelude

pub mod in_memory;
pub mod prelude;
pub mod property_values;
pub mod traits;
pub mod types;

pub use in_memory::*;
pub use prelude::*;
pub use property_values::*;
pub use traits::*;
pub use types::*;
