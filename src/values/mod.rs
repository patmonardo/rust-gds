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
// Note: macros are automatically available due to #[macro_use], no need for pub use macros::*
pub use impls::*;
pub use primitive_values::*;
pub use traits::*;
