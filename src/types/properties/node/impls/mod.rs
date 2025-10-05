// pub mod default_node_property; // No longer needed - using Property<Box<dyn NodePropertyValues>> type alias
pub mod default_node_property_store;
pub mod default_node_property_values;

pub use default_node_property_store::{
    DefaultNodePropertyStore, DefaultNodePropertyStoreBuilder,
};
pub use default_node_property_values::DefaultLongNodePropertyValues;
