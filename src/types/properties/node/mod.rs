// Node properties module
//
// Provides property storage and access for node properties.
// Node properties are values indexed by node ID.

pub mod impls;
pub mod node_property;
pub mod node_property_container;
pub mod node_property_store;
pub mod node_property_values;
pub mod traits;

// Re-export public API
pub use node_property::NodeProperty;
pub use node_property_container::{
    EmptyNodePropertyContainer, NodePropertyContainer, NodePropertyContainerExt,
};
pub use node_property_store::{NodePropertyStore, NodePropertyStoreBuilder};
pub use node_property_values::{
    DefaultDoubleArrayNodePropertyValues, DefaultDoubleNodePropertyValues,
    DefaultFloatArrayNodePropertyValues, DefaultLongArrayNodePropertyValues,
    DefaultLongNodePropertyValues, DoubleArrayNodePropertyValues, DoubleNodePropertyValues,
    FloatArrayNodePropertyValues, LongArrayNodePropertyValues, LongNodePropertyValues,
    NodePropertyValues,
};
