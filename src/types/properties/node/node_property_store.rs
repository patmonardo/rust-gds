use super::node_property_values::NodePropertyValues;
use crate::types::properties::property_store::GenericPropertyStore;

/// Represents a store for node properties.
/// Each property is identified by a string key and holds NodePropertyValues.
pub type NodePropertyStore = GenericPropertyStore<Box<dyn NodePropertyValues>>;
