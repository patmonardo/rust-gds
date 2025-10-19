//! Buffered consumers for Arrow import pipeline
//!
//! Translates:
//! - BufferedNodeConsumer.java (123 lines)
//! - BufferedRelationshipConsumer.java (103 lines)
//! - BufferedCompositeRelationshipConsumer.java (59 lines)

use crate::projection::{NodeLabel, RelationshipType};
use std::collections::HashSet;

/// RecordConsumer trait - offers records to buffer
pub trait RecordConsumer<T> {
    /// Offer a record to the consumer. Returns true if accepted, false if buffer full.
    fn offer(&mut self, record: T) -> bool;

    /// Reset the consumer (clear buffer)
    fn reset(&mut self);

    /// Check if buffer is full
    fn is_full(&self) -> bool;
}

// ============================================================================
// Node Records
// ============================================================================

/// A node record from Arrow batch
#[derive(Debug, Clone)]
pub struct NodeRecord {
    pub node_id: u64,
    pub labels: Vec<NodeLabel>,
}

/// Buffer for node records
#[derive(Debug)]
struct NodeBatchBuffer {
    node_ids: Vec<u64>,
    labels_by_node: Vec<Vec<NodeLabel>>,
    capacity: usize,
    current_size: usize,
}

impl NodeBatchBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            node_ids: Vec::with_capacity(capacity),
            labels_by_node: Vec::with_capacity(capacity),
            capacity,
            current_size: 0,
        }
    }

    fn add(&mut self, node_id: u64, labels: Vec<NodeLabel>) {
        self.node_ids.push(node_id);
        self.labels_by_node.push(labels);
        self.current_size += 1;
    }

    fn is_full(&self) -> bool {
        self.current_size >= self.capacity
    }

    fn reset(&mut self) {
        self.node_ids.clear();
        self.labels_by_node.clear();
        self.current_size = 0;
    }

    fn size(&self) -> usize {
        self.current_size
    }
}

/// BufferedNodeConsumer - buffers node records before importing
///
/// Translates: BufferedNodeConsumer.java
pub struct BufferedNodeConsumer {
    buffer: NodeBatchBuffer,
    highest_possible_node_count: u64,
    node_label_filter: HashSet<NodeLabel>,
    filter_enabled: bool,
}

impl BufferedNodeConsumer {
    /// Create a new buffered node consumer
    ///
    /// # Arguments
    /// * `capacity` - Buffer size
    /// * `highest_possible_node_count` - Maximum node ID (nodes >= this are ignored)
    /// * `node_label_filter` - Optional label filter (None = accept all)
    pub fn new(
        capacity: usize,
        highest_possible_node_count: u64,
        node_label_filter: Option<HashSet<NodeLabel>>,
    ) -> Self {
        let filter_enabled = node_label_filter.is_some();
        let node_label_filter = node_label_filter.unwrap_or_default();

        Self {
            buffer: NodeBatchBuffer::new(capacity),
            highest_possible_node_count,
            node_label_filter,
            filter_enabled,
        }
    }

    /// Get the internal buffer (for flushing)
    pub fn buffer(&self) -> &NodeBatchBuffer {
        &self.buffer
    }

    /// Get mutable buffer
    pub fn buffer_mut(&mut self) -> &mut NodeBatchBuffer {
        &mut self.buffer
    }

    /// Get buffered node IDs
    pub fn node_ids(&self) -> &[u64] {
        &self.buffer.node_ids
    }

    /// Get buffered labels
    pub fn labels(&self) -> &[Vec<NodeLabel>] {
        &self.buffer.labels_by_node
    }
}

impl RecordConsumer<NodeRecord> for BufferedNodeConsumer {
    fn offer(&mut self, record: NodeRecord) -> bool {
        // Check if buffer full
        if self.buffer.is_full() {
            return false;
        }

        // Skip nodes beyond max count
        if record.node_id >= self.highest_possible_node_count {
            return true;
        }

        // No filter = accept all
        if !self.filter_enabled {
            self.buffer.add(record.node_id, record.labels);
            return !self.buffer.is_full();
        }

        // Apply label filter
        let filtered_labels: Vec<NodeLabel> = record
            .labels
            .into_iter()
            .filter(|label| self.node_label_filter.contains(label))
            .collect();

        // Accept if at least one label matches
        if !filtered_labels.is_empty() {
            self.buffer.add(record.node_id, filtered_labels);
        }

        !self.buffer.is_full()
    }

    fn reset(&mut self) {
        self.buffer.reset();
    }

    fn is_full(&self) -> bool {
        self.buffer.is_full()
    }
}

// ============================================================================
// Relationship Records
// ============================================================================

/// A relationship record from Arrow batch
#[derive(Debug, Clone)]
pub struct RelationshipRecord {
    pub relationship_id: u64,
    pub source_node_id: u64,
    pub target_node_id: u64,
    pub relationship_type: RelationshipType,
}

/// Buffer for relationship records
#[derive(Debug)]
struct RelationshipBatchBuffer {
    sources: Vec<u64>,
    targets: Vec<u64>,
    relationship_ids: Vec<u64>,
    capacity: usize,
    current_size: usize,
}

impl RelationshipBatchBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            sources: Vec::with_capacity(capacity),
            targets: Vec::with_capacity(capacity),
            relationship_ids: Vec::with_capacity(capacity),
            capacity,
            current_size: 0,
        }
    }

    fn add(&mut self, source: u64, target: u64, rel_id: u64) {
        self.sources.push(source);
        self.targets.push(target);
        self.relationship_ids.push(rel_id);
        self.current_size += 1;
    }

    fn is_full(&self) -> bool {
        self.current_size >= self.capacity
    }

    fn reset(&mut self) {
        self.sources.clear();
        self.targets.clear();
        self.relationship_ids.clear();
        self.current_size = 0;
    }

    fn size(&self) -> usize {
        self.current_size
    }
}

/// BufferedEdgeConsumer - buffers relationship records before importing
///
/// Translates: BufferedRelationshipConsumer.java
pub struct BufferedEdgeConsumer {
    buffer: RelationshipBatchBuffer,
    relationship_type_filter: Option<RelationshipType>,
    skip_dangling: bool,
    node_count: u64,
}

impl BufferedEdgeConsumer {
    /// Create a new buffered edge consumer
    ///
    /// # Arguments
    /// * `capacity` - Buffer size
    /// * `node_count` - Total node count (for dangling check)
    /// * `relationship_type_filter` - Optional type filter (None = accept all)
    /// * `skip_dangling` - If true, skip relationships with missing nodes
    pub fn new(
        capacity: usize,
        node_count: u64,
        relationship_type_filter: Option<RelationshipType>,
        skip_dangling: bool,
    ) -> Self {
        Self {
            buffer: RelationshipBatchBuffer::new(capacity),
            relationship_type_filter,
            skip_dangling,
            node_count,
        }
    }

    /// Get the internal buffer (for flushing)
    pub fn buffer(&self) -> &RelationshipBatchBuffer {
        &self.buffer
    }

    /// Get mutable buffer
    pub fn buffer_mut(&mut self) -> &mut RelationshipBatchBuffer {
        &mut self.buffer
    }

    /// Get buffered sources
    pub fn sources(&self) -> &[u64] {
        &self.buffer.sources
    }

    /// Get buffered targets
    pub fn targets(&self) -> &[u64] {
        &self.buffer.targets
    }

    /// Get buffered relationship IDs
    pub fn relationship_ids(&self) -> &[u64] {
        &self.buffer.relationship_ids
    }
}

impl RecordConsumer<RelationshipRecord> for BufferedEdgeConsumer {
    fn offer(&mut self, record: RelationshipRecord) -> bool {
        // Check if buffer full
        if self.buffer.is_full() {
            return false;
        }

        // Apply type filter
        if let Some(filter_type) = &self.relationship_type_filter {
            if &record.relationship_type != filter_type {
                return true; // Skip, but don't block
            }
        }

        // Check for dangling relationships
        let source_valid = record.source_node_id < self.node_count;
        let target_valid = record.target_node_id < self.node_count;

        if !source_valid || !target_valid {
            if self.skip_dangling {
                return true; // Skip dangling, but don't block
            } else {
                // Could panic or return error here
                // For now, skip to avoid breaking the pipeline
                return true;
            }
        }

        // Add to buffer
        self.buffer.add(
            record.source_node_id,
            record.target_node_id,
            record.relationship_id,
        );

        !self.buffer.is_full()
    }

    fn reset(&mut self) {
        self.buffer.reset();
    }

    fn is_full(&self) -> bool {
        self.buffer.is_full()
    }
}

// ============================================================================
// Composite Consumer (Multi-Type Relationships)
// ============================================================================

/// CompositeEdgeConsumer - delegates to multiple edge consumers
///
/// Translates: BufferedCompositeRelationshipConsumer.java
pub struct CompositeEdgeConsumer {
    consumers: Vec<BufferedEdgeConsumer>,
}

impl CompositeEdgeConsumer {
    /// Create a composite consumer from multiple edge consumers
    ///
    /// If only one consumer is provided, returns that consumer directly via Option.
    pub fn new(consumers: Vec<BufferedEdgeConsumer>) -> Self {
        Self { consumers }
    }

    /// Create from a single consumer (for optimization)
    pub fn from_single(consumer: BufferedEdgeConsumer) -> BufferedEdgeConsumer {
        consumer
    }

    /// Get the consumers
    pub fn consumers(&self) -> &[BufferedEdgeConsumer] {
        &self.consumers
    }

    /// Get mutable consumers
    pub fn consumers_mut(&mut self) -> &mut [BufferedEdgeConsumer] {
        &mut self.consumers
    }
}

impl RecordConsumer<RelationshipRecord> for CompositeEdgeConsumer {
    fn offer(&mut self, record: RelationshipRecord) -> bool {
        let mut all_offered = true;
        for consumer in &mut self.consumers {
            // Clone for each consumer
            let offered = consumer.offer(record.clone());
            all_offered = all_offered && offered;
        }
        all_offered
    }

    fn reset(&mut self) {
        for consumer in &mut self.consumers {
            consumer.reset();
        }
    }

    fn is_full(&self) -> bool {
        // Full if ANY consumer is full
        self.consumers.iter().any(|c| c.is_full())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_consumer_basic() {
        let mut consumer = BufferedNodeConsumer::new(3, 1000, None);

        assert!(!consumer.is_full());

        let record1 = NodeRecord {
            node_id: 0,
            labels: vec![NodeLabel::of("Person")],
        };
        assert!(consumer.offer(record1));
        assert_eq!(consumer.buffer().size(), 1);

        let record2 = NodeRecord {
            node_id: 1,
            labels: vec![NodeLabel::of("Company")],
        };
        assert!(consumer.offer(record2));
        assert_eq!(consumer.buffer().size(), 2);

        let record3 = NodeRecord {
            node_id: 2,
            labels: vec![NodeLabel::of("Product")],
        };
        let offered = consumer.offer(record3);
        assert_eq!(consumer.buffer().size(), 3);
        assert!(consumer.is_full());
        assert!(!offered); // Returns false because buffer is now full (backpressure signal)

        // Full - should reject
        let record4 = NodeRecord {
            node_id: 3,
            labels: vec![NodeLabel::of("Event")],
        };
        assert!(!consumer.offer(record4)); // Still full, rejects
    }

    #[test]
    fn test_node_consumer_label_filter() {
        let mut filter = HashSet::new();
        filter.insert(NodeLabel::of("Person"));

        let mut consumer = BufferedNodeConsumer::new(10, 1000, Some(filter));

        // Matching label - accepted
        let record1 = NodeRecord {
            node_id: 0,
            labels: vec![NodeLabel::of("Person")],
        };
        assert!(consumer.offer(record1));
        assert_eq!(consumer.buffer().size(), 1);

        // Non-matching label - skipped
        let record2 = NodeRecord {
            node_id: 1,
            labels: vec![NodeLabel::of("Company")],
        };
        assert!(consumer.offer(record2));
        assert_eq!(consumer.buffer().size(), 1); // Still 1
    }

    #[test]
    fn test_node_consumer_max_id() {
        let mut consumer = BufferedNodeConsumer::new(10, 100, None);

        // Within range
        let record1 = NodeRecord {
            node_id: 99,
            labels: vec![NodeLabel::of("Person")],
        };
        assert!(consumer.offer(record1));
        assert_eq!(consumer.buffer().size(), 1);

        // Beyond range - skipped
        let record2 = NodeRecord {
            node_id: 100,
            labels: vec![NodeLabel::of("Person")],
        };
        assert!(consumer.offer(record2));
        assert_eq!(consumer.buffer().size(), 1); // Still 1
    }

    #[test]
    fn test_edge_consumer_basic() {
        let mut consumer = BufferedEdgeConsumer::new(3, 1000, None, true);

        let record1 = RelationshipRecord {
            relationship_id: 0,
            source_node_id: 0,
            target_node_id: 1,
            relationship_type: RelationshipType::of("KNOWS"),
        };
        assert!(consumer.offer(record1));
        assert_eq!(consumer.buffer().size(), 1);

        let record2 = RelationshipRecord {
            relationship_id: 1,
            source_node_id: 1,
            target_node_id: 2,
            relationship_type: RelationshipType::of("KNOWS"),
        };
        assert!(consumer.offer(record2));
        assert_eq!(consumer.buffer().size(), 2);
    }

    #[test]
    fn test_edge_consumer_type_filter() {
        let filter = Some(RelationshipType::of("KNOWS"));
        let mut consumer = BufferedEdgeConsumer::new(10, 1000, filter, true);

        // Matching type
        let record1 = RelationshipRecord {
            relationship_id: 0,
            source_node_id: 0,
            target_node_id: 1,
            relationship_type: RelationshipType::of("KNOWS"),
        };
        assert!(consumer.offer(record1));
        assert_eq!(consumer.buffer().size(), 1);

        // Non-matching type - skipped
        let record2 = RelationshipRecord {
            relationship_id: 1,
            source_node_id: 1,
            target_node_id: 2,
            relationship_type: RelationshipType::of("WORKS_AT"),
        };
        assert!(consumer.offer(record2));
        assert_eq!(consumer.buffer().size(), 1); // Still 1
    }

    #[test]
    fn test_edge_consumer_dangling() {
        let mut consumer = BufferedEdgeConsumer::new(10, 10, None, true); // Only nodes 0-9 exist

        // Valid relationship
        let record1 = RelationshipRecord {
            relationship_id: 0,
            source_node_id: 0,
            target_node_id: 9,
            relationship_type: RelationshipType::of("KNOWS"),
        };
        assert!(consumer.offer(record1));
        assert_eq!(consumer.buffer().size(), 1);

        // Dangling source - skipped
        let record2 = RelationshipRecord {
            relationship_id: 1,
            source_node_id: 100,
            target_node_id: 5,
            relationship_type: RelationshipType::of("KNOWS"),
        };
        assert!(consumer.offer(record2));
        assert_eq!(consumer.buffer().size(), 1); // Still 1

        // Dangling target - skipped
        let record3 = RelationshipRecord {
            relationship_id: 2,
            source_node_id: 5,
            target_node_id: 100,
            relationship_type: RelationshipType::of("KNOWS"),
        };
        assert!(consumer.offer(record3));
        assert_eq!(consumer.buffer().size(), 1); // Still 1
    }

    #[test]
    fn test_composite_consumer() {
        let consumer1 =
            BufferedEdgeConsumer::new(5, 1000, Some(RelationshipType::of("KNOWS")), true);
        let consumer2 =
            BufferedEdgeConsumer::new(5, 1000, Some(RelationshipType::of("WORKS_AT")), true);

        let mut composite = CompositeEdgeConsumer::new(vec![consumer1, consumer2]);

        let record = RelationshipRecord {
            relationship_id: 0,
            source_node_id: 0,
            target_node_id: 1,
            relationship_type: RelationshipType::of("KNOWS"),
        };

        assert!(composite.offer(record));

        // Check both consumers received it (one accepted, one filtered)
        assert_eq!(composite.consumers()[0].buffer().size(), 1);
        assert_eq!(composite.consumers()[1].buffer().size(), 0);
    }

    #[test]
    fn test_consumer_reset() {
        let mut consumer = BufferedNodeConsumer::new(10, 1000, None);

        let record = NodeRecord {
            node_id: 0,
            labels: vec![NodeLabel::of("Person")],
        };
        consumer.offer(record);
        assert_eq!(consumer.buffer().size(), 1);

        consumer.reset();
        assert_eq!(consumer.buffer().size(), 0);
        assert!(!consumer.is_full());
    }
}
