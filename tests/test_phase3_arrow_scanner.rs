// Integration tests for Phase 3: Arrow2 Scanner System
//
// Tests the batch scanner system (NodeBatchScanner, EdgeBatchScanner) that provides
// parallel batch iteration over arrow2 table structures.

#[cfg(test)]
#[cfg(feature = "arrow")]
mod arrow_scanner_tests {
    use arrow2::array::{Array, Int64Array, Utf8Array};
    use arrow2::chunk::Chunk;
    use arrow2::datatypes::{DataType, Field, Schema};
    use rust_gds::projection::factory::arrow::{
        ArrowBatchReference, BatchScanner, EdgeBatchScanner, EdgeTableReference, NodeBatchScanner,
        NodeTableReference, ScannerError, DEFAULT_BATCH_SIZE,
    };
    use std::sync::Arc;

    // =============================================================================================
    // Helper Functions
    // =============================================================================================

    fn create_node_table(row_count: usize) -> Arc<NodeTableReference> {
        let ids: Vec<i64> = (1..=row_count as i64).collect();
        let labels: Vec<Option<&str>> = (0..row_count)
            .map(|i| Some(if i % 2 == 0 { "Person" } else { "Organization" }))
            .collect();

        let id_array: Box<dyn Array> = Box::new(Int64Array::from_vec(ids));
        let label_array: Box<dyn Array> = Box::new(Utf8Array::<i32>::from(labels));

        let chunk = Chunk::new(vec![id_array, label_array]);
        let schema = Arc::new(Schema::from(vec![
            Field::new("id", DataType::Int64, false),
            Field::new("label", DataType::Utf8, false),
        ]));

        Arc::new(NodeTableReference::new("nodes", chunk, schema).unwrap())
    }

    fn create_edge_table(row_count: usize) -> Arc<EdgeTableReference> {
        let sources: Vec<i64> = (1..=row_count as i64).collect();
        let targets: Vec<i64> = sources
            .iter()
            .map(|&s| (s % row_count as i64) + 1)
            .collect();
        let types: Vec<Option<&str>> = (0..row_count).map(|_| Some("KNOWS")).collect();

        let source_array: Box<dyn Array> = Box::new(Int64Array::from_vec(sources));
        let target_array: Box<dyn Array> = Box::new(Int64Array::from_vec(targets));
        let type_array: Box<dyn Array> = Box::new(Utf8Array::<i32>::from(types));

        let chunk = Chunk::new(vec![source_array, target_array, type_array]);
        let schema = Arc::new(Schema::from(vec![
            Field::new("source", DataType::Int64, false),
            Field::new("target", DataType::Int64, false),
            Field::new("type", DataType::Utf8, false),
        ]));

        Arc::new(EdgeTableReference::new("edges", chunk, schema).unwrap())
    }

    // Simple counter for testing
    struct BatchCounter {
        batch_count: usize,
        total_rows_seen: usize,
        max_batches: Option<usize>,
    }

    impl BatchCounter {
        fn new() -> Self {
            Self {
                batch_count: 0,
                total_rows_seen: 0,
                max_batches: None,
            }
        }

        fn with_max(max_batches: usize) -> Self {
            Self {
                batch_count: 0,
                total_rows_seen: 0,
                max_batches: Some(max_batches),
            }
        }

        fn consume_batch(&mut self, batch: &ArrowBatchReference) -> bool {
            self.batch_count += 1;
            self.total_rows_seen += batch.len();

            // Return true if we CAN accept more (haven't EXCEEDED max yet)
            if let Some(max) = self.max_batches {
                self.batch_count <= max
            } else {
                true
            }
        }
        fn batch_count(&self) -> usize {
            self.batch_count
        }

        fn total_rows_seen(&self) -> usize {
            self.total_rows_seen
        }
    }

    // =============================================================================================
    // NodeBatchScanner Tests
    // =============================================================================================

    #[test]
    fn test_node_batch_scanner_creation() {
        let node_table = create_node_table(100);
        let scanner = NodeBatchScanner::new(node_table, 25).unwrap();

        assert_eq!(scanner.batch_size(), 25);
        assert_eq!(scanner.total_rows(), 100);
        assert_eq!(scanner.batch_count(), 4); // 100 rows / 25 batch_size = 4 batches
    }

    #[test]
    fn test_node_batch_scanner_with_defaults() {
        let node_table = create_node_table(100);
        let scanner = NodeBatchScanner::with_defaults(node_table).unwrap();

        assert_eq!(scanner.batch_size(), DEFAULT_BATCH_SIZE);
    }

    #[test]
    fn test_node_batch_scanner_invalid_batch_size() {
        let node_table = create_node_table(100);
        let result = NodeBatchScanner::new(node_table, 0);

        assert!(matches!(
            result,
            Err(ScannerError::InvalidBatchSize { size: 0 })
        ));
    }

    #[test]
    fn test_node_scan_cursor_single_batch() {
        let node_table = create_node_table(10);
        let scanner = NodeBatchScanner::new(node_table, 20).unwrap();
        let mut cursor = scanner.create_cursor();
        let mut counter = BatchCounter::new();

        // Reserve and consume first batch (all 10 rows)
        assert!(cursor.reserve_batch());
        assert_eq!(cursor.batch_index(), 0);
        cursor.consume_batch(&mut |batch| counter.consume_batch(batch));

        assert_eq!(counter.batch_count(), 1);

        // Try to reserve second batch (should fail - only 1 batch total)
        assert!(!cursor.reserve_batch());
    }

    #[test]
    fn test_node_scan_cursor_multiple_batches() {
        let node_table = create_node_table(50);
        // Batch size of 10 means 5 batches
        let scanner = NodeBatchScanner::new(node_table, 10).unwrap();
        assert_eq!(scanner.batch_count(), 5);

        let mut cursor = scanner.create_cursor();
        let mut counter = BatchCounter::new();

        // Consume all 5 batches
        while cursor.reserve_batch() {
            cursor.consume_batch(&mut |batch| counter.consume_batch(batch));
        }

        assert_eq!(counter.batch_count(), 5);
    }

    #[test]
    fn test_node_scan_cursor_partial_last_batch() {
        let node_table = create_node_table(25); // 25 rows
                                                // Batch size of 10 means 3 batches (10, 10, 5 rows)
        let scanner = NodeBatchScanner::new(node_table, 10).unwrap();
        assert_eq!(scanner.batch_count(), 3);

        let mut cursor = scanner.create_cursor();
        let mut counter = BatchCounter::new();

        // Consume all batches
        while cursor.reserve_batch() {
            cursor.consume_batch(&mut |batch| counter.consume_batch(batch));
        }

        assert_eq!(counter.batch_count(), 3);
        // With range semantics, batches report correct slice lengths (10, 10, 5)
        assert_eq!(counter.total_rows_seen(), 25); // 10 + 10 + 5 = 25 rows
    }
    #[test]
    fn test_node_scan_cursor_backpressure() {
        let node_table = create_node_table(50);
        let scanner = NodeBatchScanner::new(node_table, 10).unwrap();
        let mut cursor = scanner.create_cursor();

        // Counter that accepts exactly 3 batches, then signals backpressure
        let mut counter = BatchCounter::with_max(3);

        // Consume batches until backpressure
        let mut batches_attempted = 0;
        while cursor.reserve_batch() {
            let accepted = cursor.consume_batch(&mut |batch| counter.consume_batch(batch));
            batches_attempted += 1;
            if !accepted {
                break; // Backpressure - stop consuming
            }
        }

        // With max=3 and logic "count <= max", we accept batches 1,2,3 and reject batch 4
        // So batch_count should be 4 (we count before returning), and 3 were accepted
        assert!(counter.batch_count() <= 4); // Up to 4 batches seen
        assert_eq!(batches_attempted, 4); // 4 batches attempted (3 accepted, 1 rejected)
    }

    #[test]
    fn test_node_batch_scanner_store_size() {
        let node_table = create_node_table(100);
        let scanner = NodeBatchScanner::new(node_table, 25).unwrap();

        let size = scanner.store_size();
        assert!(size > 0);
        // Should be roughly 100 rows * 2 columns * 8 bytes
        assert!(size >= 1600);
    }

    // =============================================================================================
    // EdgeBatchScanner Tests
    // =============================================================================================

    #[test]
    fn test_edge_batch_scanner_creation() {
        let edge_table = create_edge_table(100);
        let scanner = EdgeBatchScanner::new(edge_table, 25).unwrap();

        assert_eq!(scanner.batch_size(), 25);
        assert_eq!(scanner.total_rows(), 100);
        assert_eq!(scanner.batch_count(), 4);
    }

    #[test]
    fn test_edge_batch_scanner_with_defaults() {
        let edge_table = create_edge_table(100);
        let scanner = EdgeBatchScanner::with_defaults(edge_table).unwrap();

        assert_eq!(scanner.batch_size(), DEFAULT_BATCH_SIZE);
    }

    #[test]
    fn test_edge_batch_scanner_invalid_batch_size() {
        let edge_table = create_edge_table(100);
        let result = EdgeBatchScanner::new(edge_table, 0);

        assert!(matches!(
            result,
            Err(ScannerError::InvalidBatchSize { size: 0 })
        ));
    }

    #[test]
    fn test_edge_scan_cursor_single_batch() {
        let edge_table = create_edge_table(10);
        let scanner = EdgeBatchScanner::new(edge_table, 20).unwrap();
        let mut cursor = scanner.create_cursor();
        let mut counter = BatchCounter::new();

        // Reserve and consume first batch
        assert!(cursor.reserve_batch());
        cursor.consume_batch(&mut |batch| counter.consume_batch(batch));

        assert_eq!(counter.batch_count(), 1);

        // No more batches
        assert!(!cursor.reserve_batch());
    }

    #[test]
    fn test_edge_scan_cursor_multiple_batches() {
        let edge_table = create_edge_table(50);
        let scanner = EdgeBatchScanner::new(edge_table, 10).unwrap();
        assert_eq!(scanner.batch_count(), 5);

        let mut cursor = scanner.create_cursor();
        let mut counter = BatchCounter::new();

        // Consume all batches
        while cursor.reserve_batch() {
            cursor.consume_batch(&mut |batch| counter.consume_batch(batch));
        }

        assert_eq!(counter.batch_count(), 5);
        // With range semantics, batches report correct slice lengths (10 each)
        assert_eq!(counter.total_rows_seen(), 50); // 5 batches × 10 rows each = 50 rows
    }

    #[test]
    fn test_edge_batch_scanner_store_size() {
        let edge_table = create_edge_table(100);
        let scanner = EdgeBatchScanner::new(edge_table, 25).unwrap();

        let size = scanner.store_size();
        assert!(size > 0);
        // Should be roughly 100 rows * 3 columns * 8 bytes
        assert!(size >= 2400);
    }

    // =============================================================================================
    // Parallel Scanning Tests (Multiple Cursors)
    // =============================================================================================

    #[test]
    fn test_parallel_node_scanning_multiple_cursors() {
        let node_table = create_node_table(100);
        let scanner = NodeBatchScanner::new(node_table, 10).unwrap();
        assert_eq!(scanner.batch_count(), 10); // 10 batches total

        // Create two cursors
        let mut cursor1 = scanner.create_cursor();
        let mut cursor2 = scanner.create_cursor();

        let mut counter1 = BatchCounter::new();
        let mut counter2 = BatchCounter::new();

        // Cursor 1 consumes some batches
        for _ in 0..3 {
            if cursor1.reserve_batch() {
                cursor1.consume_batch(&mut |batch| counter1.consume_batch(batch));
            }
        }

        // Cursor 2 consumes remaining batches
        while cursor2.reserve_batch() {
            cursor2.consume_batch(&mut |batch| counter2.consume_batch(batch));
        }

        // Together they should have consumed all 10 batches
        assert_eq!(counter1.batch_count() + counter2.batch_count(), 10);
    }

    #[test]
    fn test_parallel_edge_scanning_multiple_cursors() {
        let edge_table = create_edge_table(100);
        let scanner = EdgeBatchScanner::new(edge_table, 10).unwrap();
        assert_eq!(scanner.batch_count(), 10);

        // Create two cursors
        let mut cursor1 = scanner.create_cursor();
        let mut cursor2 = scanner.create_cursor();

        let mut counter1 = BatchCounter::new();
        let mut counter2 = BatchCounter::new();

        // Interleaved consumption
        for _ in 0..5 {
            if cursor1.reserve_batch() {
                cursor1.consume_batch(&mut |batch| counter1.consume_batch(batch));
            }
            if cursor2.reserve_batch() {
                cursor2.consume_batch(&mut |batch| counter2.consume_batch(batch));
            }
        }

        // Together they consumed all 10 batches
        assert_eq!(counter1.batch_count() + counter2.batch_count(), 10);
    }

    // =============================================================================================
    // Batch Reference Tests
    // =============================================================================================

    #[test]
    fn test_batch_reference_content_access() {
        let node_table = create_node_table(5);
        let scanner = NodeBatchScanner::new(node_table, 10).unwrap();
        let mut cursor = scanner.create_cursor();

        cursor.reserve_batch();
        cursor.consume_batch(&mut |batch| {
            // Test batch properties
            assert_eq!(batch.batch_index(), 0);
            assert_eq!(batch.len(), 5);
            assert!(!batch.is_empty());

            // Test column access
            let id_col = batch.int64_column(0).unwrap();
            assert_eq!(id_col.len(), 5);
            assert_eq!(id_col.value(0), 1);
            assert_eq!(id_col.value(4), 5);

            let label_col = batch.utf8_column(1).unwrap();
            assert_eq!(label_col.len(), 5);
            assert_eq!(label_col.value(0), "Person");

            true
        });
    }
}
