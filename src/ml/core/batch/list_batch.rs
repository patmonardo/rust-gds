//! List batch implementation for ML in GDS.
//!
//! Translated from Java GDS ml-core ListBatch.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::Batch;

/// A batch that contains a list of element IDs.
pub struct ListBatch {
    ids: Vec<u64>,
}

impl ListBatch {
    /// Create a new ListBatch from a vector of IDs.
    pub fn new(ids: Vec<u64>) -> Self {
        Self { ids }
    }

    /// Create a new ListBatch from a slice of IDs.
    pub fn from_slice(ids: &[u64]) -> Self {
        Self::new(ids.to_vec())
    }
}

impl Batch for ListBatch {
    type ElementIdsIter = std::vec::IntoIter<u64>;

    fn element_ids(&self) -> Self::ElementIdsIter {
        self.ids.clone().into_iter()
    }

    fn size(&self) -> usize {
        self.ids.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_batch() {
        let ids = vec![1, 5, 10, 15, 20];
        let batch = ListBatch::new(ids);

        assert_eq!(batch.size(), 5);
        let collected: Vec<u64> = batch.element_ids().collect();
        assert_eq!(collected, vec![1, 5, 10, 15, 20]);
    }

    #[test]
    fn test_list_batch_from_slice() {
        let ids = [1, 5, 10, 15, 20];
        let batch = ListBatch::from_slice(&ids);

        assert_eq!(batch.size(), 5);
        let collected: Vec<u64> = batch.element_ids().collect();
        assert_eq!(collected, vec![1, 5, 10, 15, 20]);
    }

    #[test]
    fn test_empty_list_batch() {
        let batch = ListBatch::new(vec![]);

        assert_eq!(batch.size(), 0);
        let collected: Vec<u64> = batch.element_ids().collect();
        assert_eq!(collected, Vec::<u64>::new());
    }
}
