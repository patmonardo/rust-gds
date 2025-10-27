// Base property system
pub mod property;
pub mod property_store;
pub mod property_values;
#[cfg(feature = "triadic_codegen_preview")]
pub mod triadic_invoke;

// NEW: HyperPropertyStores - The Sacred Hierarchy
pub mod hyper;     // Monadic, Dyadic, Triadic, Tetradic, Pentadic

// Specialized property implementations
pub mod graph;
pub mod node;
pub mod relationship;

// Re-export base property system for convenience
pub use property::*;
pub use property_store::*;
pub use property_values::*;
#[cfg(feature = "triadic_codegen_preview")]
pub use triadic_invoke::*;

// Re-export HyperPropertyStores for easy access
pub use hyper::*;
