//! Pentadic hyper-schema layer: assertions and certification (n=5)
//!
//! Canonical layout: types, traits, schema, prelude

pub mod prelude;
pub mod schema;
pub mod traits;
pub mod types;

pub use prelude::*;
pub use schema::*;
pub use types::*;

