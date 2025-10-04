pub use super::impls::default_relationship_property_store::DefaultRelationshipPropertyStore;
pub use super::impls::default_relationship_property_store::RelationshipPropertyStoreBuilder;

/// Historical alias maintained for backwards compatibility while the
/// implementation lives in the `impls` module.
pub type RelationshipPropertyStore = DefaultRelationshipPropertyStore;
