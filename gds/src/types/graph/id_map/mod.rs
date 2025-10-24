#![allow(clippy::module_inception)]

mod batch_node_iterable;
mod filtered_id_map;
mod id_map;
mod node_iterator;
mod partial_id_map;
mod simple;

pub use batch_node_iterable::{BatchNodeIterable, NodeIdBatch, NodeIdBatchIter};
pub use filtered_id_map::FilteredIdMap;
pub use id_map::{IdMap, NodeLabelConsumer, NOT_FOUND, NO_TYPE, START_NODE_ID};
pub use node_iterator::{NodeConsumer, NodeIdIterator, NodeIterator, NodeIteratorExt};
pub use partial_id_map::{EmptyPartialIdMap, PartialIdMap};
pub use simple::SimpleIdMap;

pub use crate::types::concurrency::Concurrency;

// =============================================================================
// JAVA GDS ALIGNMENT: Everything is i64 (matches Java Long)
// =============================================================================
// 
// This replaces the SUSPECT dual-type system with Java GDS alignment:
// - Java GDS: Everything is Long (signed 64-bit)
// - Rust GDS: Everything is i64 (signed 64-bit)
// - Serialization: One type for JSON/messaging
// - Algorithm portability: Java algorithms work directly
// - Mental model clarity: One type rules them all

/// Node identifier type (matches Java GDS Long)
pub type NodeId = i64;

/// Property value type (matches Java GDS Long) 
pub type PropertyValue = i64;

/// Algorithm weight type (matches Java GDS Long)
pub type Weight = i64;

/// Count and size type (matches Java GDS Long)
pub type Count = i64;

/// Floating point value type (only when you need actual floating point)
pub type FloatValue = f64;

// =============================================================================
// LEGACY ALIASES (for backward compatibility during transition)
// =============================================================================
// 
// These will be removed after full migration to Java GDS alignment
// TODO: Remove these aliases after updating all 228 usages

/// @deprecated Use NodeId instead
pub type MappedNodeId = NodeId;

/// @deprecated Use NodeId instead  
pub type OriginalNodeId = NodeId;
