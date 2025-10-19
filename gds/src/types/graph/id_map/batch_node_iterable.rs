use super::MappedNodeId;

/// Representation of a contiguous batch of mapped node identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeIdBatch {
    pub start: MappedNodeId,
    pub length: usize,
}

impl NodeIdBatch {
    pub fn new(start: MappedNodeId, length: usize) -> Self {
        Self { start, length }
    }

    /// Returns an iterator over the identifiers in this batch.
    pub fn iter(&self) -> NodeIdBatchIter {
        NodeIdBatchIter {
            current: self.start,
            remaining: self.length,
        }
    }
}

/// Iterator implementation for [`NodeIdBatch`].
pub struct NodeIdBatchIter {
    current: MappedNodeId,
    remaining: usize,
}

impl Iterator for NodeIdBatchIter {
    type Item = MappedNodeId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let value = self.current;
        self.current += 1;
        self.remaining -= 1;
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for NodeIdBatchIter {}

/// Trait for producing iterables over node identifiers grouped into batches.
pub trait BatchNodeIterable: Send + Sync {
    fn batch_iterables(&self, batch_size: usize) -> Vec<NodeIdBatch>;
}
