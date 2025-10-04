pub use super::impls::default_graph_property_store::{
    DefaultGraphPropertyStore, GraphPropertyStoreBuilder,
};

/// Alias maintained for backward compatibility with the previous graph store
/// type.
pub type GraphPropertyStore = DefaultGraphPropertyStore;
