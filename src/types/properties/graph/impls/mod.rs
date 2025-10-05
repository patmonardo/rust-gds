// pub mod default_graph_property; // No longer needed - using Property<Box<dyn GraphPropertyValues>> type alias
pub mod default_graph_property_store;
pub mod default_graph_property_values;

pub use default_graph_property_store::{
    DefaultGraphPropertyStore, DefaultGraphPropertyStoreBuilder,
};
pub use default_graph_property_values::{
    DefaultDoubleGraphPropertyValues, DefaultLongGraphPropertyValues,
};
