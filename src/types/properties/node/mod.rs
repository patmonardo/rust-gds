// Node properties module
//
// Provides property storage and access for node properties.
// Node properties are values indexed by node ID.

pub mod impls;
pub mod node_property;
pub mod node_property_store;
pub mod node_property_values;
pub mod traits;

// Re-export commonly used items
pub use impls::{
    DefaultLongNodePropertyValues, DefaultNodePropertyStore, DefaultNodePropertyStoreBuilder,
};

pub use node_property_store::{NodePropertyStore, NodePropertyStoreBuilder};
pub use node_property_values::NodePropertyValues;
pub use traits::node_property_container::{NodePropertyContainer, NodePropertyContainerExt};
