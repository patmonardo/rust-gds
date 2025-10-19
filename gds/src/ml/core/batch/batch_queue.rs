//! Batch queue for parallel processing in ML in GDS.
//!
//! Translated from Java GDS ml-core BatchQueue.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::{Batch, RangeBatch};

/// Default batch size for processing.
pub const DEFAULT_BATCH_SIZE: usize = 100;

/// Trait object-compatible batch interface.
///
/// This allows BatchQueue to return different batch implementations
/// without fixing the iterator type at compile time.
pub trait AnyBatch {
    /// Get a boxed iterator over element IDs.
    fn element_ids_boxed(&self) -> Box<dyn Iterator<Item = u64> + '_>;

    /// Get the size of this batch.
    fn size(&self) -> usize;
}

/// Blanket implementation: any Batch can be an AnyBatch.
impl<T: Batch> AnyBatch for T {
    fn element_ids_boxed(&self) -> Box<dyn Iterator<Item = u64> + '_> {
        Box::new(self.element_ids())
    }

    fn size(&self) -> usize {
        Batch::size(self)
    }
}

/// Abstract batch queue for managing batches in parallel processing.
pub trait BatchQueue {
    /// Pop the next batch from the queue.
    ///
    /// Returns a type-erased batch that can contain any iterator implementation.
    fn pop(&mut self) -> Option<Box<dyn AnyBatch>>;

    /// Get the total size of all elements.
    fn total_size(&self) -> u64;

    /// Get the batch size.
    fn batch_size(&self) -> usize;

    /// Get the current batch index.
    fn current_batch(&self) -> u64;
}

/// Compute the optimal batch size based on total size, minimum batch size, and concurrency.
pub fn compute_batch_size(total_size: u64, min_batch_size: usize, concurrency: usize) -> usize {
    // Simplified version - in real implementation would use ParallelUtil.adjustedBatchSize equivalent
    let adjusted = (total_size as usize)
        .saturating_div(concurrency)
        .max(min_batch_size);
    adjusted.min(usize::MAX)
}

/// Create a consecutive batch queue with default batch size.
pub fn consecutive(total_size: u64) -> Box<dyn BatchQueue> {
    consecutive_with_batch_size(total_size, DEFAULT_BATCH_SIZE)
}

/// Create a consecutive batch queue with computed batch size.
pub fn consecutive_with_concurrency(
    total_size: u64,
    min_batch_size: usize,
    concurrency: usize,
) -> Box<dyn BatchQueue> {
    let batch_size = compute_batch_size(total_size, min_batch_size, concurrency);
    consecutive_with_batch_size(total_size, batch_size)
}

/// Create a consecutive batch queue with specified batch size.
pub fn consecutive_with_batch_size(total_size: u64, batch_size: usize) -> Box<dyn BatchQueue> {
    Box::new(ConsecutiveBatchQueue::new(total_size, batch_size))
}

/// Consecutive batch queue implementation.
///
/// Matches Java's ConsecutiveBatchQueue which tracks batches by index
/// and creates RangeBatch instances on demand.
pub struct ConsecutiveBatchQueue {
    total_size: u64,
    batch_size: usize,
    current_batch: u64,
}

impl ConsecutiveBatchQueue {
    fn new(total_size: u64, batch_size: usize) -> Self {
        Self {
            total_size,
            batch_size,
            current_batch: 0,
        }
    }
}

impl BatchQueue for ConsecutiveBatchQueue {
    fn pop(&mut self) -> Option<Box<dyn AnyBatch>> {
        if self.current_batch * self.batch_size as u64 >= self.total_size {
            return None;
        }

        let batch = RangeBatch::new(
            self.current_batch * self.batch_size as u64,
            self.batch_size,
            self.total_size,
        );

        self.current_batch += 1;

        Some(Box::new(batch))
    }
    fn total_size(&self) -> u64 {
        self.total_size
    }

    fn batch_size(&self) -> usize {
        self.batch_size
    }

    fn current_batch(&self) -> u64 {
        self.current_batch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consecutive_batch_queue() {
        let mut queue = consecutive_with_batch_size(10, 3);

        // First batch: [0, 1, 2]
        let batch1 = queue.pop().unwrap();
        assert_eq!(batch1.size(), 3);
        let ids1: Vec<u64> = batch1.element_ids_boxed().collect();
        assert_eq!(ids1, vec![0, 1, 2]);

        // Second batch: [3, 4, 5]
        let batch2 = queue.pop().unwrap();
        assert_eq!(batch2.size(), 3);
        let ids2: Vec<u64> = batch2.element_ids_boxed().collect();
        assert_eq!(ids2, vec![3, 4, 5]);

        // Third batch: [6, 7, 8]
        let batch3 = queue.pop().unwrap();
        assert_eq!(batch3.size(), 3);
        let ids3: Vec<u64> = batch3.element_ids_boxed().collect();
        assert_eq!(ids3, vec![6, 7, 8]);

        // Fourth batch: [9] (remaining)
        let batch4 = queue.pop().unwrap();
        assert_eq!(batch4.size(), 1);
        let ids4: Vec<u64> = batch4.element_ids_boxed().collect();
        assert_eq!(ids4, vec![9]);

        // No more batches
        assert!(queue.pop().is_none());
    }

    #[test]
    fn test_compute_batch_size() {
        assert_eq!(compute_batch_size(100, 10, 4), 25);
        assert_eq!(compute_batch_size(10, 5, 2), 5);
        assert_eq!(compute_batch_size(1, 1, 1), 1);
    }

    #[test]
    fn test_empty_queue() {
        let mut queue = consecutive_with_batch_size(0, 5);
        assert!(queue.pop().is_none());
    }
}
