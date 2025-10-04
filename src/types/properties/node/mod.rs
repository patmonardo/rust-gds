// Node properties module
//
// Provides property storage and access for node properties.
// Node properties are values indexed by node ID.

pub mod impls;
pub mod node_property;
pub mod node_property_store;
pub mod node_property_values;
pub mod traits;

// Re-export public API
pub use impls::{
    DefaultDoubleArrayNodePropertyValues, DefaultDoubleNodePropertyValues,
    DefaultFloatArrayNodePropertyValues, DefaultLongArrayNodePropertyValues,
    DefaultLongNodePropertyValues, NodePropertyStoreBuilder,
};
pub use node_property::NodeProperty;
pub use node_property_store::NodePropertyStore;
pub use node_property_values::{
    DoubleArrayNodePropertyValues, DoubleNodePropertyValues, FloatArrayNodePropertyValues,
    LongArrayNodePropertyValues, LongNodePropertyValues, NodePropertyValues,
};
pub use traits::{EmptyNodePropertyContainer, NodePropertyContainer, NodePropertyContainerExt};
