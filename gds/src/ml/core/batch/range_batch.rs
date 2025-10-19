//! Range batch implementation for ML in GDS.
//!
//! Translated from Java GDS ml-core RangeBatch.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::Batch;

/// An iterator that yields consecutive IDs from start to end (exclusive).
pub struct RangeIterator {
    current: u64,
    end: u64,
}

impl RangeIterator {
    fn new(start: u64, end: u64) -> Self {
        Self {
            current: start,
            end,
        }
    }
}

impl Iterator for RangeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let id = self.current;
            self.current += 1;
            Some(id)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.end - self.current) as usize;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for RangeIterator {}

/// A batch that represents a consecutive range of element IDs.
pub struct RangeBatch {
    start_id: u64,
    end_id: u64,
    size: usize,
}

impl RangeBatch {
    /// Create a new RangeBatch.
    ///
    /// # Arguments
    /// * `start_id` - The starting ID (inclusive)
    /// * `batch_size` - The desired batch size
    /// * `node_count` - The total number of nodes (to prevent overflow)
    pub fn new(start_id: u64, batch_size: usize, node_count: u64) -> Self {
        let end_id = std::cmp::min(node_count, start_id + batch_size as u64);
        let size = (end_id - start_id) as usize;

        Self {
            start_id,
            end_id,
            size,
        }
    }
}

impl Batch for RangeBatch {
    type ElementIdsIter = RangeIterator;

    fn element_ids(&self) -> Self::ElementIdsIter {
        RangeIterator::new(self.start_id, self.end_id)
    }

    fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_batch() {
        let batch = RangeBatch::new(10, 5, 100);

        assert_eq!(batch.size(), 5);
        let collected: Vec<u64> = batch.element_ids().collect();
        assert_eq!(collected, vec![10, 11, 12, 13, 14]);
    }

    #[test]
    fn test_range_batch_at_end() {
        let batch = RangeBatch::new(95, 10, 100);

        assert_eq!(batch.size(), 5); // Limited by node_count
        let collected: Vec<u64> = batch.element_ids().collect();
        assert_eq!(collected, vec![95, 96, 97, 98, 99]);
    }

    #[test]
    fn test_range_batch_exact_size() {
        let batch = RangeBatch::new(0, 3, 3);

        assert_eq!(batch.size(), 3);
        let collected: Vec<u64> = batch.element_ids().collect();
        assert_eq!(collected, vec![0, 1, 2]);
    }

    #[test]
    fn test_range_batch_empty() {
        let batch = RangeBatch::new(100, 5, 100);

        assert_eq!(batch.size(), 0);
        let collected: Vec<u64> = batch.element_ids().collect();
        assert_eq!(collected, Vec::<u64>::new());
    }

    #[test]
    fn test_range_iterator_size_hint() {
        let mut iter = RangeIterator::new(10, 15);
        assert_eq!(iter.size_hint(), (5, Some(5)));

        iter.next();
        assert_eq!(iter.size_hint(), (4, Some(4)));

        iter.next();
        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(2)));
    }
}
