// Node properties module
//
// Provides property storage and access for node properties.
// Node properties are values indexed by node ID.

pub mod impls;
pub mod node_property;
pub mod node_property_store;
pub mod node_property_values;
pub mod traits;

pub use impls::*;
pub use node_property::*;
pub use node_property_store::*;
pub use node_property_values::*;
pub use traits::*;
