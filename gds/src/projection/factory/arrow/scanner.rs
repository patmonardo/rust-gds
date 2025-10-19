// Arrow2 Scanner System
//
// Provides parallel batch iteration over arrow2 table structures for graph construction.
//
// Design principles:
// - Batch-oriented: Process chunks of rows at once (configurable batch size)
// - Parallel-safe: Multiple scanners can iterate independently
// - Zero-copy: References wrap existing arrow2 structures
// - Memory-bounded: Configurable prefetch and batch sizes
//
// Scanner hierarchy:
// - BatchScanner trait: Abstract scanner interface
// - NodeBatchScanner: Scans node tables in parallel batches
// - EdgeBatchScanner: Scans edge tables in parallel batches
// - RecordConsumer trait: Processes batches of records

use super::reference::{
    ArrowBatchReference, ArrowReference, EdgeTableReference, NodeTableReference,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use thiserror::Error;

// ================================================================================================
// Constants
// ================================================================================================

/// Default batch size for scanning (number of rows per batch)
pub const DEFAULT_BATCH_SIZE: usize = 10_000;

/// Default prefetch size (number of batches to prefetch)
pub const DEFAULT_PREFETCH_SIZE: usize = 4;

// ================================================================================================
// Error Types
// ================================================================================================

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("Scanner exhausted: no more batches available")]
    Exhausted,

    #[error("Invalid batch size: {size} (must be > 0)")]
    InvalidBatchSize { size: usize },

    #[error("Invalid prefetch size: {size} (must be > 0)")]
    InvalidPrefetchSize { size: usize },

    #[error("Consumer rejected batch at index {batch_index}")]
    ConsumerRejected { batch_index: usize },
}

// ================================================================================================
// RecordConsumer Trait - Processes batches of records
// ================================================================================================

/// Consumes batches of records during scanning.
///
/// Implementations typically buffer records and flush when full.
/// Can reject records to signal backpressure (e.g., buffer full).
pub trait RecordConsumer<Reference> {
    /// Offers a record to the consumer.
    ///
    /// Returns `true` if the consumer can accept more records,
    /// `false` if backpressure is needed (e.g., buffer full).
    fn offer(&mut self, reference: &Reference) -> bool;

    /// Resets the consumer's internal state for a new batch.
    ///
    /// Called between batches to prepare for the next batch.
    fn reset(&mut self) {
        // Default: no-op
    }

    /// Flushes any buffered records.
    ///
    /// Called after consuming all batches or when buffer is full.
    fn flush(&mut self) -> Result<(), ScannerError> {
        // Default: no-op
        Ok(())
    }
}

// ================================================================================================
// ScanCursor Trait - Batch iteration cursor
// ================================================================================================

/// Cursor for iterating over batches of records.
///
/// Provides two-phase iteration:
/// 1. `reserve_batch()`: Reserve the next batch
/// 2. `consume_batch()`: Consume the reserved batch
///
/// This separation allows for parallel reservation across multiple cursors.
pub trait ScanCursor: Send {
    /// Reserves the next batch of records.
    ///
    /// Returns `true` if a batch was reserved and needs to be consumed.
    /// Returns `false` if no more batches are available (scanner exhausted).
    fn reserve_batch(&mut self) -> bool;

    /// Consumes the currently reserved batch using a consumer callback.
    ///
    /// Returns `true` if the batch was completely consumed.
    /// Returns `false` if the consumer signaled backpressure (buffer full).
    ///
    /// The callback is invoked with each batch reference.
    fn consume_batch(&mut self, consumer: &mut dyn FnMut(&ArrowBatchReference) -> bool) -> bool;

    /// Returns the current batch index
    fn batch_index(&self) -> usize;
}

// ================================================================================================
// BatchScanner Trait - Main scanner interface
// ================================================================================================

/// Abstract scanner interface for batch iteration over arrow2 tables.
///
/// Implementations provide cursors for parallel batch iteration.
pub trait BatchScanner: Send + Sync {
    /// Creates a new scan cursor for batch iteration.
    ///
    /// Multiple cursors can be created for parallel scanning.
    fn create_cursor(&self) -> Box<dyn ScanCursor>;

    /// Returns the estimated store size in bytes.
    fn store_size(&self) -> usize;

    /// Returns the batch size (number of rows per batch).
    fn batch_size(&self) -> usize;

    /// Returns the total number of rows to scan.
    fn total_rows(&self) -> usize;

    /// Returns the number of batches (ceiling division).
    fn batch_count(&self) -> usize {
        (self.total_rows() + self.batch_size() - 1) / self.batch_size()
    }
}

// ================================================================================================
// NodeBatchScanner - Scans node tables in parallel batches
// ================================================================================================

/// Scanner for node tables stored as arrow2 chunks.
///
/// Provides parallel batch iteration over node records with configurable batch size.
///
/// Example:
/// ```rust,ignore
/// let scanner = NodeBatchScanner::new(node_table, 10_000)?;
/// let mut cursor = scanner.create_cursor();
///
/// while cursor.reserve_batch() {
///     cursor.consume_batch(&mut consumer);
/// }
/// ```
pub struct NodeBatchScanner {
    node_table: Arc<NodeTableReference>,
    batch_size: usize,
    total_rows: usize,
    next_batch: Arc<AtomicUsize>,
}

impl NodeBatchScanner {
    /// Creates a new node batch scanner.
    pub fn new(
        node_table: Arc<NodeTableReference>,
        batch_size: usize,
    ) -> Result<Self, ScannerError> {
        if batch_size == 0 {
            return Err(ScannerError::InvalidBatchSize { size: batch_size });
        }

        let total_rows = node_table.as_ref().row_count();

        Ok(Self {
            node_table,
            batch_size,
            total_rows,
            next_batch: Arc::new(AtomicUsize::new(0)),
        })
    }

    /// Creates a new node batch scanner with default batch size.
    pub fn with_defaults(node_table: Arc<NodeTableReference>) -> Result<Self, ScannerError> {
        Self::new(node_table, DEFAULT_BATCH_SIZE)
    }
}

impl BatchScanner for NodeBatchScanner {
    fn create_cursor(&self) -> Box<dyn ScanCursor> {
        Box::new(NodeScanCursor {
            node_table: Arc::clone(&self.node_table),
            batch_size: self.batch_size,
            total_rows: self.total_rows,
            next_batch: Arc::clone(&self.next_batch),
            current_batch_index: 0,
            current_batch_start: 0,
            current_batch_end: 0,
        })
    }

    fn store_size(&self) -> usize {
        // Estimate: sum of all array sizes
        self.node_table
            .chunk()
            .arrays()
            .iter()
            .map(|arr| {
                // Rough estimate: 8 bytes per value + overhead
                arr.len() * 8
            })
            .sum()
    }

    fn batch_size(&self) -> usize {
        self.batch_size
    }

    fn total_rows(&self) -> usize {
        self.total_rows
    }
}

// ================================================================================================
// NodeScanCursor - Cursor for node batch iteration
// ================================================================================================

struct NodeScanCursor {
    node_table: Arc<NodeTableReference>,
    batch_size: usize,
    total_rows: usize,
    next_batch: Arc<AtomicUsize>,
    current_batch_index: usize,
    current_batch_start: usize,
    current_batch_end: usize,
}

impl ScanCursor for NodeScanCursor {
    fn reserve_batch(&mut self) -> bool {
        // Atomically reserve the next batch
        let batch_index = self.next_batch.fetch_add(1, Ordering::SeqCst);

        let batch_start = batch_index * self.batch_size;
        if batch_start >= self.total_rows {
            return false; // No more batches
        }

        let batch_end = (batch_start + self.batch_size).min(self.total_rows);

        self.current_batch_index = batch_index;
        self.current_batch_start = batch_start;
        self.current_batch_end = batch_end;

        true
    }

    fn consume_batch(&mut self, consumer: &mut dyn FnMut(&ArrowBatchReference) -> bool) -> bool {
        // Create a batch reference with the logical sub-range
        let batch_ref = ArrowBatchReference::new(
            self.node_table.chunk(),
            self.node_table.as_ref().schema(),
            self.current_batch_index,
            self.current_batch_start,
            self.current_batch_end,
        );

        // Invoke the consumer callback with the batch
        consumer(&batch_ref)
    }

    fn batch_index(&self) -> usize {
        self.current_batch_index
    }
}

// ================================================================================================
// EdgeBatchScanner - Scans edge tables in parallel batches
// ================================================================================================

/// Scanner for edge tables stored as arrow2 chunks.
///
/// Provides parallel batch iteration over edge records with configurable batch size.
///
/// Example:
/// ```rust,ignore
/// let scanner = EdgeBatchScanner::new(edge_table, 10_000)?;
/// let mut cursor = scanner.create_cursor();
///
/// while cursor.reserve_batch() {
///     cursor.consume_batch(&mut consumer);
/// }
/// ```
pub struct EdgeBatchScanner {
    edge_table: Arc<EdgeTableReference>,
    batch_size: usize,
    total_rows: usize,
    next_batch: Arc<AtomicUsize>,
}

impl EdgeBatchScanner {
    /// Creates a new edge batch scanner.
    pub fn new(
        edge_table: Arc<EdgeTableReference>,
        batch_size: usize,
    ) -> Result<Self, ScannerError> {
        if batch_size == 0 {
            return Err(ScannerError::InvalidBatchSize { size: batch_size });
        }

        let total_rows = edge_table.as_ref().row_count();

        Ok(Self {
            edge_table,
            batch_size,
            total_rows,
            next_batch: Arc::new(AtomicUsize::new(0)),
        })
    }

    /// Creates a new edge batch scanner with default batch size.
    pub fn with_defaults(edge_table: Arc<EdgeTableReference>) -> Result<Self, ScannerError> {
        Self::new(edge_table, DEFAULT_BATCH_SIZE)
    }
}

impl BatchScanner for EdgeBatchScanner {
    fn create_cursor(&self) -> Box<dyn ScanCursor> {
        Box::new(EdgeScanCursor {
            edge_table: Arc::clone(&self.edge_table),
            batch_size: self.batch_size,
            total_rows: self.total_rows,
            next_batch: Arc::clone(&self.next_batch),
            current_batch_index: 0,
            current_batch_start: 0,
            current_batch_end: 0,
        })
    }

    fn store_size(&self) -> usize {
        // Estimate: sum of all array sizes
        self.edge_table
            .chunk()
            .arrays()
            .iter()
            .map(|arr| arr.len() * 8)
            .sum()
    }

    fn batch_size(&self) -> usize {
        self.batch_size
    }

    fn total_rows(&self) -> usize {
        self.total_rows
    }
}

// ================================================================================================
// EdgeScanCursor - Cursor for edge batch iteration
// ================================================================================================

struct EdgeScanCursor {
    edge_table: Arc<EdgeTableReference>,
    batch_size: usize,
    total_rows: usize,
    next_batch: Arc<AtomicUsize>,
    current_batch_index: usize,
    current_batch_start: usize,
    current_batch_end: usize,
}

impl ScanCursor for EdgeScanCursor {
    fn reserve_batch(&mut self) -> bool {
        // Atomically reserve the next batch
        let batch_index = self.next_batch.fetch_add(1, Ordering::SeqCst);

        let batch_start = batch_index * self.batch_size;
        if batch_start >= self.total_rows {
            return false; // No more batches
        }

        let batch_end = (batch_start + self.batch_size).min(self.total_rows);

        self.current_batch_index = batch_index;
        self.current_batch_start = batch_start;
        self.current_batch_end = batch_end;

        true
    }

    fn consume_batch(&mut self, consumer: &mut dyn FnMut(&ArrowBatchReference) -> bool) -> bool {
        // Create a batch reference with the logical sub-range
        let batch_ref = ArrowBatchReference::new(
            self.edge_table.chunk(),
            self.edge_table.as_ref().schema(),
            self.current_batch_index,
            self.current_batch_start,
            self.current_batch_end,
        );

        // Invoke the consumer callback with the batch
        consumer(&batch_ref)
    }

    fn batch_index(&self) -> usize {
        self.current_batch_index
    }
}

// ================================================================================================
// Tests
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use arrow2::array::{Array, Int64Array, Utf8Array};
    use arrow2::chunk::Chunk;
    use arrow2::datatypes::{DataType, Field, Schema};

    // Simple counter for testing
    struct BatchCounter {
        count: usize,
        max_count: Option<usize>,
    }

    impl BatchCounter {
        fn new() -> Self {
            Self {
                count: 0,
                max_count: None,
            }
        }

        fn with_max(max_count: usize) -> Self {
            Self {
                count: 0,
                max_count: Some(max_count),
            }
        }

        fn count(&self) -> usize {
            self.count
        }

        fn consume_batch(&mut self, _batch: &ArrowBatchReference) -> bool {
            if let Some(max) = self.max_count {
                if self.count < max {
                    self.count += 1;
                    true
                } else {
                    false
                }
            } else {
                self.count += 1;
                true
            }
        }
    }

    fn create_test_node_table() -> Arc<NodeTableReference> {
        let id_array: Box<dyn Array> = Box::new(Int64Array::from_slice([1, 2, 3, 4, 5]));
        let label_array: Box<dyn Array> = Box::new(Utf8Array::<i32>::from_slice([
            "Person",
            "Person",
            "Organization",
            "Person",
            "Organization",
        ]));

        let chunk = Chunk::new(vec![id_array, label_array]);
        let schema = Arc::new(Schema::from(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("label", DataType::Utf8, false),
        ]));

        Arc::new(NodeTableReference::new("nodes", chunk, schema).unwrap())
    }

    fn create_test_edge_table() -> Arc<EdgeTableReference> {
        let source_array: Box<dyn Array> = Box::new(Int64Array::from_slice([1, 2, 3, 4]));
        let target_array: Box<dyn Array> = Box::new(Int64Array::from_slice([2, 3, 4, 1]));
        let type_array: Box<dyn Array> = Box::new(Utf8Array::<i32>::from_slice([
            "KNOWS", "WORKS_AT", "KNOWS", "KNOWS",
        ]));

        let chunk = Chunk::new(vec![source_array, target_array, type_array]);
        let schema = Arc::new(Schema::from(vec![
            Field::new("source", DataType::Int64, false),
            Field::new("target", DataType::Int64, false),
            Field::new("type", DataType::Utf8, false),
        ]));

        Arc::new(EdgeTableReference::new("edges", chunk, schema).unwrap())
    }

    #[test]
    fn test_node_batch_scanner_creation() {
        let node_table = create_test_node_table();
        let scanner = NodeBatchScanner::new(node_table, 10).unwrap();

        assert_eq!(scanner.batch_size(), 10);
        assert_eq!(scanner.total_rows(), 5);
        assert_eq!(scanner.batch_count(), 1); // 5 rows / 10 batch_size = 1 batch
    }

    #[test]
    fn test_node_batch_scanner_invalid_batch_size() {
        let node_table = create_test_node_table();
        let result = NodeBatchScanner::new(node_table, 0);

        assert!(matches!(result, Err(ScannerError::InvalidBatchSize { .. })));
    }

    #[test]
    fn test_node_scan_cursor_reserve_and_consume() {
        let node_table = create_test_node_table();
        let scanner = NodeBatchScanner::new(node_table, 10).unwrap();
        let mut cursor = scanner.create_cursor();
        let mut counter = BatchCounter::new();

        // Reserve first batch
        assert!(cursor.reserve_batch());
        assert_eq!(cursor.batch_index(), 0);

        // Consume first batch
        let consumed = cursor.consume_batch(&mut |batch| counter.consume_batch(batch));
        assert!(consumed);
        assert_eq!(counter.count(), 1); // One batch consumed

        // Try to reserve second batch (should fail - only 1 batch total)
        assert!(!cursor.reserve_batch());
    }

    #[test]
    fn test_node_scan_cursor_multiple_batches() {
        let node_table = create_test_node_table();
        // Batch size of 2 means 3 batches (2, 2, 1 rows)
        let scanner = NodeBatchScanner::new(node_table, 2).unwrap();
        assert_eq!(scanner.batch_count(), 3);

        let mut cursor = scanner.create_cursor();
        let mut counter = BatchCounter::new();

        // Consume all 3 batches
        let mut batches_consumed = 0;
        while cursor.reserve_batch() {
            cursor.consume_batch(&mut |batch| counter.consume_batch(batch));
            batches_consumed += 1;
        }

        assert_eq!(batches_consumed, 3);
        assert_eq!(counter.count(), 3); // 3 batches consumed
    }

    #[test]
    fn test_edge_batch_scanner_creation() {
        let edge_table = create_test_edge_table();
        let scanner = EdgeBatchScanner::new(edge_table, 10).unwrap();

        assert_eq!(scanner.batch_size(), 10);
        assert_eq!(scanner.total_rows(), 4);
        assert_eq!(scanner.batch_count(), 1);
    }

    #[test]
    fn test_edge_scan_cursor_reserve_and_consume() {
        let edge_table = create_test_edge_table();
        let scanner = EdgeBatchScanner::new(edge_table, 10).unwrap();
        let mut cursor = scanner.create_cursor();
        let mut counter = BatchCounter::new();

        // Reserve and consume first batch
        assert!(cursor.reserve_batch());
        let consumed = cursor.consume_batch(&mut |batch| counter.consume_batch(batch));
        assert!(consumed);
        assert_eq!(counter.count(), 1);

        // No more batches
        assert!(!cursor.reserve_batch());
    }

    #[test]
    fn test_consumer_backpressure() {
        let node_table = create_test_node_table();
        let scanner = NodeBatchScanner::new(node_table, 2).unwrap();
        let mut cursor = scanner.create_cursor();

        // Counter that rejects after 2 batches
        let mut counter = BatchCounter::with_max(2);

        // First batch - accepted
        assert!(cursor.reserve_batch());
        let consumed = cursor.consume_batch(&mut |batch| counter.consume_batch(batch));
        assert!(consumed);

        // Second batch - accepted
        assert!(cursor.reserve_batch());
        let consumed = cursor.consume_batch(&mut |batch| counter.consume_batch(batch));
        assert!(consumed);

        // Third batch - rejected (count == max_count)
        assert!(cursor.reserve_batch());
        let consumed = cursor.consume_batch(&mut |batch| counter.consume_batch(batch));
        assert!(!consumed);

        assert_eq!(counter.count(), 2);
    }

    #[test]
    fn test_batch_scanner_store_size() {
        let node_table = create_test_node_table();
        let scanner = NodeBatchScanner::new(node_table, 10).unwrap();

        let size = scanner.store_size();
        assert!(size > 0);
        // Should be roughly 5 rows * 2 columns * 8 bytes = 80 bytes
        assert!(size >= 80);
    }

    #[test]
    fn test_default_batch_scanner() {
        let node_table = create_test_node_table();
        let scanner = NodeBatchScanner::with_defaults(node_table).unwrap();

        assert_eq!(scanner.batch_size(), DEFAULT_BATCH_SIZE);
    }
}
