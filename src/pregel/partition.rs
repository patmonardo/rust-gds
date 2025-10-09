//! Partition - Work batch representation for Pregel computation
//!
//! A partition represents a batch of nodes to be processed together.

use std::ops::Range;

/// Represents a batch of nodes to be processed in a Pregel computation.
///
/// A partition is a contiguous range of node IDs that can be processed
/// by a single worker thread. The partition provides a `consume` method
/// to iterate over all nodes in the batch.
///
/// # Example
///
/// ```
/// use rust_gds::pregel::Partition;
///
/// let partition = Partition::new(0, 100);
/// let mut count = 0;
///
/// partition.consume(|node_id| {
///     count += 1;
///     // Process node_id
/// });
///
/// assert_eq!(count, 100);
/// assert_eq!(partition.node_count(), 100);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Partition {
    start_node: u64,
    node_count: usize,
}

impl Partition {
    /// Create a new partition with the given start node and count.
    ///
    /// # Arguments
    ///
    /// * `start_node` - The first node ID in this partition
    /// * `node_count` - The number of nodes in this partition
    ///
    /// # Example
    ///
    /// ```
    /// use rust_gds::pregel::Partition;
    ///
    /// let partition = Partition::new(0, 1000);
    /// assert_eq!(partition.start_node(), 0);
    /// assert_eq!(partition.node_count(), 1000);
    /// ```
    pub fn new(start_node: u64, node_count: usize) -> Self {
        Self {
            start_node,
            node_count,
        }
    }

    /// Get the first node ID in this partition.
    pub fn start_node(&self) -> u64 {
        self.start_node
    }

    /// Get the number of nodes in this partition.
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    /// Get the range of node IDs in this partition.
    ///
    /// Returns a range [start, end) where end is exclusive.
    pub fn range(&self) -> Range<u64> {
        self.start_node..(self.start_node + self.node_count as u64)
    }

    /// Execute a function for each node in this partition.
    ///
    /// # Arguments
    ///
    /// * `f` - Function to execute for each node ID
    ///
    /// # Example
    ///
    /// ```
    /// use rust_gds::pregel::Partition;
    ///
    /// let partition = Partition::new(10, 5);
    /// let mut ids = Vec::new();
    ///
    /// partition.consume(|node_id| {
    ///     ids.push(node_id);
    /// });
    ///
    /// assert_eq!(ids, vec![10, 11, 12, 13, 14]);
    /// ```
    pub fn consume<F>(&self, mut f: F)
    where
        F: FnMut(u64),
    {
        for node_id in self.range() {
            f(node_id);
        }
    }
}

impl From<Range<u64>> for Partition {
    fn from(range: Range<u64>) -> Self {
        let start_node = range.start;
        let node_count = (range.end - range.start) as usize;
        Self::new(start_node, node_count)
    }
}

impl From<Range<usize>> for Partition {
    fn from(range: Range<usize>) -> Self {
        let start_node = range.start as u64;
        let node_count = range.end - range.start;
        Self::new(start_node, node_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_partition() {
        let partition = Partition::new(0, 100);
        assert_eq!(partition.start_node(), 0);
        assert_eq!(partition.node_count(), 100);
    }

    #[test]
    fn test_partition_range() {
        let partition = Partition::new(50, 150);
        let range = partition.range();
        assert_eq!(range, 50..200);
    }

    #[test]
    fn test_consume() {
        let partition = Partition::new(10, 5);
        let mut visited = Vec::new();

        partition.consume(|node_id| {
            visited.push(node_id);
        });

        assert_eq!(visited, vec![10, 11, 12, 13, 14]);
    }

    #[test]
    fn test_consume_empty() {
        let partition = Partition::new(0, 0);
        let mut count = 0;

        partition.consume(|_| {
            count += 1;
        });

        assert_eq!(count, 0);
    }

    #[test]
    fn test_from_range_u64() {
        let range = 10u64..20u64;
        let partition = Partition::from(range);

        assert_eq!(partition.start_node(), 10);
        assert_eq!(partition.node_count(), 10);
    }

    #[test]
    fn test_from_range_usize() {
        let range = 100usize..250usize;
        let partition = Partition::from(range);

        assert_eq!(partition.start_node(), 100);
        assert_eq!(partition.node_count(), 150);
    }
}
