// Base property system
pub mod property;
pub mod property_store;
pub mod property_values;

// NEW: HyperPropertyStores - The Sacred Hierarchy
// pub mod hyper;     // Monadic, Dyadic, Triadic, Tetradic, Pentadic

// Specialized property implementations
pub mod graph;
pub mod node;
pub mod relationship;

// Re-export base property system for convenience
pub use property::*;
pub use property_store::*;
pub use property_values::*;

// Re-export HyperPropertyStores for easy access
// pub use hyper::*;
