// Base property system
pub mod property;
pub mod property_store;
pub mod property_values;
#[cfg(feature = "triadic_codegen_preview")]
pub mod triadic_invoke;

// NEW: Collections First PropertyStores (Experimental)
pub mod monadic;   // Universal single-level store
pub mod triadic;   // Three-level composed store (Meta/Node/Link)

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

// Re-export Collections First systems for easy access
pub use monadic::*;
pub use triadic::*;
