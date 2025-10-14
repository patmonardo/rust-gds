//! Mapped batch implementation for ML in GDS.
//!
//! Translated from Java GDS ml-core MappedBatch.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::{Batch, BatchTransformer};

/// A batch that applies a transformation to element IDs from a delegate batch.
pub struct MappedBatch<B, T> {
    delegate: B,
    transformer: T,
}

impl<B, T> MappedBatch<B, T>
where
    B: Batch,
    T: BatchTransformer,
{
    /// Create a new MappedBatch with a delegate batch and transformer.
    pub fn new(delegate: B, transformer: T) -> Self {
        Self {
            delegate,
            transformer,
        }
    }
}

impl<B, T> Batch for MappedBatch<B, T>
where
    B: Batch,
    T: BatchTransformer + Copy,
{
    type ElementIdsIter = MappedIterator<B::ElementIdsIter, T>;

    fn element_ids(&self) -> Self::ElementIdsIter {
        MappedIterator {
            iter: self.delegate.element_ids(),
            transformer: self.transformer,
        }
    }

    fn size(&self) -> usize {
        self.delegate.size()
    }
}

/// Iterator that applies a transformation to element IDs.
pub struct MappedIterator<I, T> {
    iter: I,
    transformer: T,
}

impl<I, T> Iterator for MappedIterator<I, T>
where
    I: Iterator<Item = u64>,
    T: BatchTransformer,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|id| self.transformer.apply(id))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::core::batch::{IdentityBatchTransformer, SimpleBatch};

    #[test]
    fn test_mapped_batch_with_identity() {
        let delegate = SimpleBatch::new(vec![1, 2, 3, 4, 5]);
        let transformer = IdentityBatchTransformer;
        let mapped = MappedBatch::new(delegate, transformer);

        assert_eq!(mapped.size(), 5);
        let ids: Vec<u64> = mapped.element_ids().collect();
        assert_eq!(ids, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mapped_batch_with_offset() {
        #[derive(Copy, Clone)]
        struct OffsetTransformer {
            offset: u64,
        }

        impl BatchTransformer for OffsetTransformer {
            fn apply(&self, index: u64) -> u64 {
                index + self.offset
            }
        }

        let delegate = SimpleBatch::new(vec![1, 2, 3]);
        let transformer = OffsetTransformer { offset: 10 };
        let mapped = MappedBatch::new(delegate, transformer);

        assert_eq!(mapped.size(), 3);
        let ids: Vec<u64> = mapped.element_ids().collect();
        assert_eq!(ids, vec![11, 12, 13]);
    }

    #[test]
    fn test_mapped_iterator_size_hint() {
        let delegate = SimpleBatch::new(vec![1, 2, 3, 4, 5]);
        let transformer = IdentityBatchTransformer;
        let mapped = MappedBatch::new(delegate, transformer);

        let mut iter = mapped.element_ids();
        assert_eq!(iter.size_hint(), (5, Some(5)));

        // Consume one element
        let _ = iter.next();
        assert_eq!(iter.size_hint(), (4, Some(4)));
    }
}
