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

pub type MappedNodeId = u64;
pub type OriginalNodeId = i64;
