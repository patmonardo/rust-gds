// Base property system
pub mod property;
pub mod property_store;
pub mod property_values;
pub mod typed_property_values;
pub mod backend;
pub mod factory;
#[cfg(feature = "triadic_codegen_preview")]
pub mod triadic_invoke;

// Specialized property implementations
pub mod graph;
pub mod node;
pub mod relationship;

// Re-export base property system for convenience
pub use property::*;
pub use property_store::*;
pub use property_values::*;
pub use typed_property_values::*;
pub use backend::*;
pub use factory::*;
#[cfg(feature = "triadic_codegen_preview")]
pub use triadic_invoke::*;
