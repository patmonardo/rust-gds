// Macros must be loaded first with #[macro_use] so they're available to other modules
#[macro_use]
pub mod macros;

// Now load modules that use the macros
pub mod impls;
pub mod traits;

// Finally load the factory (which uses macros and impls)
#[macro_use]
pub mod primitive_values;

// Re-export everything for convenient access
pub use impls::*;
pub use primitive_values::*;
pub use traits::*;

// NOTE: form_processor has moved to src/projection
// Import it via: use crate::projection::form_processor;
