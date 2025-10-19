//! Batch interface for ML in GDS.
//!
//! Translated from Java GDS ml-core Batch.java.
//! This is a literal 1:1 translation following repository translation policy.

/// A batch of elements identified by their IDs.
pub trait Batch {
    /// Iterator over the element IDs in this batch.
    type ElementIdsIter: Iterator<Item = u64>;

    /// Get an iterator over the element IDs in this batch.
    fn element_ids(&self) -> Self::ElementIdsIter;

    /// Get the size of this batch.
    fn size(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementation for testing
    struct MockBatch {
        elements: Vec<u64>,
    }

    impl MockBatch {
        fn new(elements: Vec<u64>) -> Self {
            Self { elements }
        }
    }

    impl Batch for MockBatch {
        type ElementIdsIter = std::vec::IntoIter<u64>;

        fn element_ids(&self) -> Self::ElementIdsIter {
            self.elements.clone().into_iter()
        }

        fn size(&self) -> usize {
            self.elements.len()
        }
    }

    #[test]
    fn test_mock_batch() {
        let batch = MockBatch::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(batch.size(), 5);

        let ids: Vec<u64> = batch.element_ids().collect();
        assert_eq!(ids, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_empty_batch() {
        let batch = MockBatch::new(vec![]);
        assert_eq!(batch.size(), 0);

        let ids: Vec<u64> = batch.element_ids().collect();
        assert_eq!(ids, Vec::<u64>::new());
    }
}
