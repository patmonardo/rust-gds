// Consumer-based Import Tasks (Phase 8 - Option A)
//
// This module provides ImportTask implementations that use BufferedNodeConsumer
// and BufferedEdgeConsumer to stream records from Arrow batches into buffers,
// then flush them into accumulators.
//
// Design:
// - ConsumerNodeImportTask: Uses BufferedNodeConsumer to buffer node records
// - ConsumerEdgeImportTask: Uses BufferedEdgeConsumer to buffer edge records
// - Tasks call process_*_batch_with_consumer adapters from importer module
// - When consumer buffers fill, tasks flush to underlying accumulators
//
// This approach lets us test consumer backpressure and batch processing
// without replacing the existing accumulator infrastructure.

use super::{
    consumer::{
        BufferedEdgeConsumer, BufferedNodeConsumer, RecordConsumer,
    },
    importer::{EdgeAccumulator, NodeAccumulator},
    task::{ImportTask, TaskError, TaskFactory},
    ScanCursor,
};
use std::sync::{Arc, Mutex};

// ================================================================================================
// ConsumerNodeImportTask - Uses BufferedNodeConsumer
// ================================================================================================

/// Import task that streams node records through a BufferedNodeConsumer.
///
/// Reads batches from scanner, offers records to the consumer, and flushes
/// buffered records to the underlying NodeAccumulator when buffer fills or
/// scanning completes.
pub struct ConsumerNodeImportTask {
    task_index: usize,
    consumer: BufferedNodeConsumer,
    accumulator: Arc<Mutex<NodeAccumulator>>,
}

impl ConsumerNodeImportTask {
    /// Creates a new consumer-based node import task.
    ///
    /// # Arguments
    /// * `task_index` - Task index for logging/debugging
    /// * `consumer` - BufferedNodeConsumer instance
    /// * `accumulator` - Shared accumulator for flushing buffered records
    pub fn new(
        task_index: usize,
        consumer: BufferedNodeConsumer,
        accumulator: Arc<Mutex<NodeAccumulator>>,
    ) -> Self {
        Self {
            task_index,
            consumer,
            accumulator,
        }
    }

    /// Flushes buffered node records to the accumulator.
    fn flush_buffer(&mut self) -> Result<u64, TaskError> {
        let node_ids = self.consumer.node_ids();
        let labels = self.consumer.labels();

        if node_ids.is_empty() {
            return Ok(0);
        }

        let mut acc = self
            .accumulator
            .lock()
            .map_err(|e| TaskError::ExecutionFailed {
                message: format!("Failed to acquire accumulator lock: {}", e),
            })?;

        let count = node_ids.len() as u64;

        for (node_id, label_vec) in node_ids.iter().zip(labels.iter()) {
            acc.add_node(*node_id as i64, label_vec.clone());
        }

        // Reset consumer buffer after flush
        self.consumer.reset();

        Ok(count)
    }
}

impl ImportTask for ConsumerNodeImportTask {
    fn execute(&mut self, cursor: &mut dyn ScanCursor) -> Result<(u64, u64), TaskError> {
        let mut total_records = 0u64;

        while cursor.reserve_batch() {
            // Offer batch records to consumer
            let mut fully_consumed = false;

            cursor.consume_batch(&mut |batch| {
                // Call the adapter that streams records to consumer
                match super::importer::process_node_batch_with_consumer(batch, &mut self.consumer) {
                    Ok(accepted) => {
                        fully_consumed = accepted;
                        accepted // Return true if fully consumed
                    }
                    Err(_) => false, // Stop on error
                }
            });

            // If consumer buffer is full, flush it
            if !fully_consumed {
                let flushed = self.flush_buffer()?;
                total_records += flushed;
            }
        }

        // Final flush of any remaining buffered records
        let flushed = self.flush_buffer()?;
        total_records += flushed;

        Ok((total_records, 0)) // No properties in this phase
    }

    fn task_name(&self) -> String {
        format!("consumer-node-import-task-{}", self.task_index)
    }

    fn task_index(&self) -> usize {
        self.task_index
    }
}

// ================================================================================================
// ConsumerEdgeImportTask - Uses BufferedEdgeConsumer
// ================================================================================================

/// Import task that streams edge records through a BufferedEdgeConsumer.
///
/// Reads batches from scanner, offers records to the consumer, and flushes
/// buffered records to the underlying EdgeAccumulator when buffer fills or
/// scanning completes.
pub struct ConsumerEdgeImportTask {
    task_index: usize,
    consumer: BufferedEdgeConsumer,
    accumulator: Arc<Mutex<EdgeAccumulator>>,
}

impl ConsumerEdgeImportTask {
    /// Creates a new consumer-based edge import task.
    pub fn new(
        task_index: usize,
        consumer: BufferedEdgeConsumer,
        accumulator: Arc<Mutex<EdgeAccumulator>>,
    ) -> Self {
        Self {
            task_index,
            consumer,
            accumulator,
        }
    }

    /// Flushes buffered edge records to the accumulator.
    fn flush_buffer(&mut self) -> Result<u64, TaskError> {
        let sources = self.consumer.sources();
        let targets = self.consumer.targets();
        let _rel_ids = self.consumer.relationship_ids();

        if sources.is_empty() {
            return Ok(0);
        }

        let mut acc = self
            .accumulator
            .lock()
            .map_err(|e| TaskError::ExecutionFailed {
                message: format!("Failed to acquire accumulator lock: {}", e),
            })?;

        let count = sources.len() as u64;

        // For now, we use a placeholder relationship type since the consumer
        // doesn't expose per-relationship types (future enhancement)
        let default_rel_type = crate::projection::RelationshipType::of("RELATED");

        for i in 0..sources.len() {
            acc.add_edge(
                sources[i] as i64,
                targets[i] as i64,
                default_rel_type.clone(),
            );
        }

        // Reset consumer buffer after flush
        self.consumer.reset();

        Ok(count)
    }
}

impl ImportTask for ConsumerEdgeImportTask {
    fn execute(&mut self, cursor: &mut dyn ScanCursor) -> Result<(u64, u64), TaskError> {
        let mut total_records = 0u64;

        while cursor.reserve_batch() {
            let mut fully_consumed = false;

            cursor.consume_batch(&mut |batch| {
                match super::importer::process_edge_batch_with_consumer(batch, &mut self.consumer) {
                    Ok(accepted) => {
                        fully_consumed = accepted;
                        accepted
                    }
                    Err(_) => false,
                }
            });

            // If consumer buffer is full, flush it
            if !fully_consumed {
                let flushed = self.flush_buffer()?;
                total_records += flushed;
            }
        }

        // Final flush of any remaining buffered records
        let flushed = self.flush_buffer()?;
        total_records += flushed;

        Ok((total_records, 0))
    }

    fn task_name(&self) -> String {
        format!("consumer-edge-import-task-{}", self.task_index)
    }

    fn task_index(&self) -> usize {
        self.task_index
    }
}

// ================================================================================================
// Task Factories
// ================================================================================================

/// Factory for creating ConsumerNodeImportTask instances.
pub struct ConsumerNodeImportTaskFactory {
    accumulator: Arc<Mutex<NodeAccumulator>>,
    buffer_capacity: usize,
    max_node_id: u64,
}

impl ConsumerNodeImportTaskFactory {
    /// Creates a new factory.
    ///
    /// # Arguments
    /// * `accumulator` - Shared accumulator for all tasks
    /// * `buffer_capacity` - Consumer buffer size (records per buffer)
    /// * `max_node_id` - Maximum node ID (nodes >= this are ignored)
    pub fn new(
        accumulator: Arc<Mutex<NodeAccumulator>>,
        buffer_capacity: usize,
        max_node_id: u64,
    ) -> Self {
        Self {
            accumulator,
            buffer_capacity,
            max_node_id,
        }
    }
}

impl TaskFactory for ConsumerNodeImportTaskFactory {
    fn create_task(&self, task_index: usize) -> Result<Box<dyn ImportTask>, TaskError> {
        // Create a new consumer for this task (each task gets its own buffer)
        let consumer = BufferedNodeConsumer::new(self.buffer_capacity, self.max_node_id, None);

        Ok(Box::new(ConsumerNodeImportTask::new(
            task_index,
            consumer,
            self.accumulator.clone(),
        )))
    }
}

/// Factory for creating ConsumerEdgeImportTask instances.
pub struct ConsumerEdgeImportTaskFactory {
    accumulator: Arc<Mutex<EdgeAccumulator>>,
    buffer_capacity: usize,
    node_count: u64,
}

impl ConsumerEdgeImportTaskFactory {
    /// Creates a new factory.
    pub fn new(
        accumulator: Arc<Mutex<EdgeAccumulator>>,
        buffer_capacity: usize,
        node_count: u64,
    ) -> Self {
        Self {
            accumulator,
            buffer_capacity,
            node_count,
        }
    }
}

impl TaskFactory for ConsumerEdgeImportTaskFactory {
    fn create_task(&self, task_index: usize) -> Result<Box<dyn ImportTask>, TaskError> {
        let consumer = BufferedEdgeConsumer::new(self.buffer_capacity, self.node_count, None, true);

        Ok(Box::new(ConsumerEdgeImportTask::new(
            task_index,
            consumer,
            self.accumulator.clone(),
        )))
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::NodeLabel;
    use crate::types::graph::id_map::IdMap;

    #[test]
    fn test_consumer_node_task_factory_creation() {
        let acc = Arc::new(Mutex::new(NodeAccumulator::new()));
        let factory = ConsumerNodeImportTaskFactory::new(acc, 100, 1000);

        let task = factory.create_task(0).unwrap();
        assert_eq!(task.task_index(), 0);
        assert_eq!(task.task_name(), "consumer-node-import-task-0");
    }

    #[test]
    fn test_consumer_edge_task_factory_creation() {
        let acc = Arc::new(Mutex::new(EdgeAccumulator::new()));
        let factory = ConsumerEdgeImportTaskFactory::new(acc, 100, 1000);

        let task = factory.create_task(1).unwrap();
        assert_eq!(task.task_index(), 1);
        assert_eq!(task.task_name(), "consumer-edge-import-task-1");
    }

    #[test]
    fn test_consumer_node_task_buffer_flush() {
        let acc = Arc::new(Mutex::new(NodeAccumulator::new()));
        let consumer = BufferedNodeConsumer::new(3, 1000, None);
        let mut task = ConsumerNodeImportTask::new(0, consumer, acc.clone());

        // Manually add records to consumer
        task.consumer.offer(NodeRecord {
            node_id: 100,
            labels: vec![NodeLabel::of("Person")],
        });
        task.consumer.offer(NodeRecord {
            node_id: 101,
            labels: vec![NodeLabel::of("Company")],
        });

        // Flush should move records to accumulator
        let flushed = task.flush_buffer().unwrap();
        assert_eq!(flushed, 2);

        // Verify accumulator received records
        let acc_guard = acc.lock().unwrap();
        assert_eq!(acc_guard.node_count(), 2);
    }

    #[test]
    fn test_consumer_edge_task_buffer_flush() {
        use crate::projection::RelationshipType;

        let acc = Arc::new(Mutex::new(EdgeAccumulator::new()));
        let consumer = BufferedEdgeConsumer::new(3, 1000, None, true);
        let mut task = ConsumerEdgeImportTask::new(0, consumer, acc.clone());

        // Manually add records to consumer
        task.consumer.offer(RelationshipRecord {
            relationship_id: 0,
            source_node_id: 100,
            target_node_id: 101,
            relationship_type: RelationshipType::of("KNOWS"),
        });
        task.consumer.offer(RelationshipRecord {
            relationship_id: 1,
            source_node_id: 101,
            target_node_id: 102,
            relationship_type: RelationshipType::of("KNOWS"),
        });

        // Flush should move records to accumulator
        let flushed = task.flush_buffer().unwrap();
        assert_eq!(flushed, 2);

        // Verify accumulator received records
        let acc_guard = acc.lock().unwrap();
        assert_eq!(acc_guard.edge_count(), 2);
    }

    // ================================================================================================
    // Integration Tests - End-to-end Arrow → TaskRunner → Accumulator
    // ================================================================================================

    #[test]
    fn test_consumer_node_import_end_to_end() {
        use super::super::{NodeBatchScanner, NodeTableReference, TaskRunner};
        use arrow2::array::{Array, Int64Array, Utf8Array};
        use arrow2::chunk::Chunk;
        use arrow2::datatypes::{DataType, Field, Schema};

        // Create a small Arrow node table
        let id_array: Box<dyn Array> = Box::new(Int64Array::from_slice([100, 101, 102]));
        let label_array: Box<dyn Array> = Box::new(Utf8Array::<i32>::from_slice([
            "Person", "Person", "Company",
        ]));

        let chunk = Chunk::new(vec![id_array, label_array]);
        let schema = Arc::new(Schema::from(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("label", DataType::Utf8, false),
        ]));

        let node_table = Arc::new(NodeTableReference::new("nodes", chunk, schema).unwrap());

        // Create scanner with small batch size to test multiple batches
        let scanner = Arc::new(NodeBatchScanner::new(node_table, 2).unwrap());

        // Create accumulator and factory
        let accumulator = Arc::new(Mutex::new(NodeAccumulator::new()));
        let factory = Arc::new(ConsumerNodeImportTaskFactory::new(
            accumulator.clone(),
            10,   // buffer capacity
            1000, // max node id
        ));

        // Run import with TaskRunner
        let runner = TaskRunner::new(2).unwrap();
        let result = runner.run_import(scanner, factory).unwrap();

        // Verify import results
        assert_eq!(result.total_records_imported, 3);
        assert_eq!(result.tasks_completed, 2);

        // Verify accumulator received all nodes
        let node_count = {
            let acc_guard = accumulator.lock().unwrap();
            acc_guard.node_count()
        };
        assert_eq!(node_count, 3);

        // Build IdMap and verify (consumes accumulator)
        let acc = Arc::try_unwrap(accumulator).unwrap().into_inner().unwrap();
        let id_map = acc.build_id_map();
        assert_eq!(id_map.node_count(), 3);
    }

    #[test]
    fn test_consumer_edge_import_end_to_end() {
        use super::super::{EdgeBatchScanner, EdgeTableReference, TaskRunner};
        use crate::types::graph::id_map::SimpleIdMap;
        use arrow2::array::{Array, Int64Array, Utf8Array};
        use arrow2::chunk::Chunk;
        use arrow2::datatypes::{DataType, Field, Schema};

        // Create a small Arrow edge table
        let source_array: Box<dyn Array> = Box::new(Int64Array::from_slice([100, 101, 102]));
        let target_array: Box<dyn Array> = Box::new(Int64Array::from_slice([101, 102, 100]));
        let type_array: Box<dyn Array> =
            Box::new(Utf8Array::<i32>::from_slice(["KNOWS", "KNOWS", "WORKS_AT"]));

        let chunk = Chunk::new(vec![source_array, target_array, type_array]);
        let schema = Arc::new(Schema::from(vec![
            Field::new("source", DataType::Int64, false),
            Field::new("target", DataType::Int64, false),
            Field::new("type", DataType::Utf8, false),
        ]));

        let edge_table = Arc::new(EdgeTableReference::new("edges", chunk, schema).unwrap());

        // Create scanner
        let scanner = Arc::new(EdgeBatchScanner::new(edge_table, 2).unwrap());

        // Create accumulator and factory
        let accumulator = Arc::new(Mutex::new(EdgeAccumulator::new()));
        let factory = Arc::new(ConsumerEdgeImportTaskFactory::new(
            accumulator.clone(),
            10,   // buffer capacity
            1000, // node count (for dangling check)
        ));

        // Run import
        let runner = TaskRunner::new(2).unwrap();
        let result = runner.run_import(scanner, factory).unwrap();

        // Verify import results
        assert_eq!(result.total_records_imported, 3);
        assert_eq!(result.tasks_completed, 2);

        // Verify accumulator received all edges
        let edge_count = {
            let acc_guard = accumulator.lock().unwrap();
            acc_guard.edge_count()
        };
        assert_eq!(edge_count, 3);

        // Build topology and verify (consumes accumulator)
        let id_map = SimpleIdMap::from_original_ids([100, 101, 102]);
        let acc = Arc::try_unwrap(accumulator).unwrap().into_inner().unwrap();
        let topologies = acc.build_topology(&id_map).unwrap();

        // Note: Current implementation uses default "RELATED" type for all edges
        // (relationship type tracking per edge will be added in a future enhancement)
        assert_eq!(topologies.len(), 1); // All edges use default type for now

        // Verify we have 3 relationships total
        let total_rels: usize = topologies.values().map(|t| t.relationship_count()).sum();
        assert_eq!(total_rels, 3);
    }
}
