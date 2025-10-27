//! Triadic hyper-schema layer: graph structure (n=3)
//!
//! Canonical layout: types, traits, schema, prelude

pub mod prelude;
pub mod schema;
pub mod traits;
pub mod types;

pub use prelude::*;
pub use schema::*;
pub use types::*;

