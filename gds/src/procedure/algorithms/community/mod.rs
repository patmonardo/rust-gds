//! Community detection algorithms - Faithful 1:1 translation from Java GDS
//!
//! This module contains faithful translations of Java GDS community algorithms:
//! - `CommunityCompanion.java` → `community_companion.rs`
//! - `ConsecutiveLongNodePropertyValues.java` → `consecutive_long_node_property_values.rs`
//! - `LongIfChangedNodePropertyValues.java` → `long_if_changed_node_property_values.rs`

pub mod community_companion;
pub mod consecutive_long_node_property_values;
pub mod long_if_changed_node_property_values;

// Re-export the translated types
pub use community_companion::*;
pub use consecutive_long_node_property_values::*;
pub use long_if_changed_node_property_values::*;