//! Singleton batch implementation for ML in GDS.
//!
//! Translated from Java GDS ml-core SingletonBatch.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::Batch;

/// An iterator that yields exactly one element ID.
pub struct SingletonIterator {
    id: u64,
    yielded: bool,
}

impl SingletonIterator {
    fn new(id: u64) -> Self {
        Self { id, yielded: false }
    }
}

impl Iterator for SingletonIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded {
            None
        } else {
            self.yielded = true;
            Some(self.id)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = if self.yielded { 0 } else { 1 };
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for SingletonIterator {}

/// A batch that contains exactly one element ID.
pub struct SingletonBatch {
    id: u64,
}

impl SingletonBatch {
    /// Create a new SingletonBatch containing a single element ID.
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Batch for SingletonBatch {
    type ElementIdsIter = SingletonIterator;

    fn element_ids(&self) -> Self::ElementIdsIter {
        SingletonIterator::new(self.id)
    }

    fn size(&self) -> usize {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton_batch() {
        let batch = SingletonBatch::new(42);

        assert_eq!(batch.size(), 1);
        let collected: Vec<u64> = batch.element_ids().collect();
        assert_eq!(collected, vec![42]);
    }

    #[test]
    fn test_singleton_iterator_multiple_calls() {
        let mut iter = SingletonIterator::new(99);

        // First call should return the element
        assert_eq!(iter.next(), Some(99));

        // Subsequent calls should return None
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_singleton_iterator_size_hint() {
        let mut iter = SingletonIterator::new(123);

        // Before yielding
        assert_eq!(iter.size_hint(), (1, Some(1)));

        // After yielding
        iter.next();
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn test_singleton_batch_different_ids() {
        let batch1 = SingletonBatch::new(0);
        let batch2 = SingletonBatch::new(1000);
        let batch3 = SingletonBatch::new(u64::MAX);

        assert_eq!(batch1.element_ids().collect::<Vec<_>>(), vec![0]);
        assert_eq!(batch2.element_ids().collect::<Vec<_>>(), vec![1000]);
        assert_eq!(batch3.element_ids().collect::<Vec<_>>(), vec![u64::MAX]);
    }
}
