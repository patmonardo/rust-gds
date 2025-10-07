// Base property system
pub mod property;
pub mod property_store;
pub mod property_values;

// Specialized property implementations
pub mod graph;
pub mod node;
pub mod relationship;

// Re-export base property system for convenience
pub use property::*;
pub use property_store::*;
pub use property_values::*;

// Note: graph, node, and relationship are NOT re-exported here.
// They have complex internal structures that are changing rapidly.
// Import explicitly when needed:
//   use types::properties::node::DefaultNodePropertyStore;
//   use types::properties::relationship::RelationshipCursor;
//
// We'll optimize these exports once the API stabilizes.
