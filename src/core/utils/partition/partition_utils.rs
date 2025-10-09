// Copyright (c) "Neo4j"
// Neo4j Sweden AB [http://neo4j.com]
//
// This file is part of Neo4j.
//
// Neo4j is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! High-performance graph partitioning utilities.
//!
//! This module provides various partitioning strategies for dividing graph workloads
//! across multiple threads or workers:
//!
//! - **Range partitioning**: Simple node ID-based splits
//! - **Degree partitioning**: Relationship-aware load balancing
//! - **Number-aligned partitioning**: Memory-efficient page boundaries
//! - **Block-aligned partitioning**: Cache-friendly access patterns
//!
//! # Examples
//!
//! ```
//! use rust_gds::core::utils::partition::{PartitionUtils, Partition};
//!
//! // Range partitioning for 1000 nodes with 4-way concurrency
//! let partitions = PartitionUtils::range_partition(
//!     4,
//!     1000,
//!     |p| p,
//!     None
//! );
//! assert_eq!(partitions.len(), 4);
//! ```

use super::{DegreeFunction, DegreePartition, LazyDegreePartitionIterator, Partition};
use crate::mem::BitUtil;

/// Minimum partition capacity as a fraction of the target batch size.
const MIN_PARTITION_CAPACITY: f64 = 0.67;

/// Default minimum batch size for parallel processing.
pub const DEFAULT_BATCH_SIZE: usize = 10;

/// High-performance graph partitioning utilities.
pub struct PartitionUtils;

impl PartitionUtils {
    // ============================================================================
    // RANGE PARTITIONING
    // ============================================================================

    /// Creates range partitions based on node count and concurrency.
    ///
    /// # Arguments
    /// * `concurrency` - Number of parallel workers
    /// * `node_count` - Total number of nodes to partition
    /// * `task_creator` - Function to create tasks from partitions
    /// * `min_batch_size` - Optional minimum batch size
    ///
    /// # Returns
    /// Vector of tasks created from partitions
    pub fn range_partition<TASK, F>(
        concurrency: usize,
        node_count: usize,
        task_creator: F,
        min_batch_size: Option<usize>,
    ) -> Vec<TASK>
    where
        F: Fn(Partition) -> TASK,
    {
        let batch_size = Self::adjusted_batch_size(
            node_count,
            concurrency,
            min_batch_size.unwrap_or(DEFAULT_BATCH_SIZE),
        );
        Self::range_partition_with_batch_size(node_count, batch_size, task_creator)
    }

    /// Creates range partitions with a specified batch size.
    ///
    /// # Arguments
    /// * `node_count` - Total number of nodes
    /// * `batch_size` - Size of each partition
    /// * `task_creator` - Function to create tasks from partitions
    pub fn range_partition_with_batch_size<TASK, F>(
        node_count: usize,
        batch_size: usize,
        task_creator: F,
    ) -> Vec<TASK>
    where
        F: Fn(Partition) -> TASK,
    {
        Self::tasks(node_count, batch_size, task_creator)
    }

    // ============================================================================
    // NUMBER-ALIGNED PARTITIONING
    // ============================================================================

    /// Creates partitions aligned to a specific number boundary.
    ///
    /// Useful for memory-efficient page-aligned access patterns.
    ///
    /// # Arguments
    /// * `concurrency` - Number of parallel workers
    /// * `node_count` - Total number of nodes
    /// * `align_to` - Alignment boundary (e.g., page size)
    pub fn number_aligned_partitioning(
        concurrency: usize,
        node_count: usize,
        align_to: usize,
    ) -> Vec<Partition> {
        Self::number_aligned_partitioning_with_max_size(
            concurrency,
            node_count,
            align_to,
            usize::MAX,
            |p| p,
        )
    }

    /// Creates partitions aligned to a boundary with maximum partition size.
    ///
    /// # Arguments
    /// * `concurrency` - Number of parallel workers
    /// * `node_count` - Total number of nodes
    /// * `align_to` - Alignment boundary
    /// * `max_partition_size` - Maximum size of any partition
    /// * `task_creator` - Function to create tasks from partitions
    pub fn number_aligned_partitioning_with_max_size<TASK, F>(
        concurrency: usize,
        node_count: usize,
        align_to: usize,
        max_partition_size: usize,
        task_creator: F,
    ) -> Vec<TASK>
    where
        F: Fn(Partition) -> TASK,
    {
        if max_partition_size < align_to {
            panic!(
                "Maximum size of a partition must be at least as much as its desired alignment \
                 but got align={} and maxPartitionSize={}",
                align_to, max_partition_size
            );
        }

        let initial_batch_size = Self::adjusted_batch_size(node_count, concurrency, align_to);
        let remainder = initial_batch_size % align_to;
        let mut adjusted_batch_size = if remainder == 0 {
            initial_batch_size
        } else {
            initial_batch_size + (align_to - remainder)
        };

        if adjusted_batch_size > max_partition_size {
            let overflow = max_partition_size % align_to;
            adjusted_batch_size = max_partition_size - overflow;
        }

        Self::tasks(node_count, adjusted_batch_size, task_creator)
    }

    // ============================================================================
    // DEGREE PARTITIONING
    // ============================================================================

    /// Creates degree-aware partitions for load-balanced parallel processing.
    ///
    /// This is the primary method for creating partitions that balance work
    /// based on the degree distribution of nodes in the graph.
    ///
    /// # Arguments
    /// * `node_count` - Total number of nodes
    /// * `relationship_count` - Total number of relationships
    /// * `degrees` - Function to get node degrees
    /// * `concurrency` - Number of parallel workers
    /// * `task_creator` - Function to create tasks from partitions
    /// * `min_batch_size` - Optional minimum batch size
    pub fn degree_partition<TASK, F>(
        node_count: usize,
        relationship_count: usize,
        degrees: Box<dyn DegreeFunction>,
        concurrency: usize,
        task_creator: F,
        min_batch_size: Option<usize>,
    ) -> Vec<TASK>
    where
        F: Fn(DegreePartition) -> TASK,
    {
        if concurrency == 1 {
            return vec![task_creator(DegreePartition::new(
                0,
                node_count,
                relationship_count,
            ))];
        }

        let batch_size = std::cmp::max(
            min_batch_size.unwrap_or(DEFAULT_BATCH_SIZE),
            BitUtil::ceil_div(relationship_count, concurrency),
        );

        Self::degree_partition_with_batch_size(node_count, degrees, batch_size, task_creator)
    }

    /// Creates degree partitions with a specified batch size.
    ///
    /// # Arguments
    /// * `node_count` - Total number of nodes
    /// * `degrees` - Function to get node degrees
    /// * `batch_size` - Target number of relationships per partition
    /// * `task_creator` - Function to create tasks from partitions
    pub fn degree_partition_with_batch_size<TASK, F>(
        node_count: usize,
        degrees: Box<dyn DegreeFunction>,
        batch_size: usize,
        task_creator: F,
    ) -> Vec<TASK>
    where
        F: Fn(DegreePartition) -> TASK,
    {
        let mut partitions = Vec::new();
        let mut start = 0;

        debug_assert!(batch_size > 0);

        let min_partition_size = (batch_size as f64 * MIN_PARTITION_CAPACITY).round() as usize;

        while start < node_count {
            let mut partition_size = 0;
            let mut node_id = start.saturating_sub(1);

            // Find the next partition boundary
            while node_id < node_count - 1
                && (node_id + 1).saturating_sub(start) < Partition::MAX_NODE_COUNT
            {
                let degree = degrees.degree(node_id + 1);
                let partition_is_large_enough = partition_size >= min_partition_size;

                if partition_size + degree > batch_size && partition_is_large_enough {
                    break;
                }

                node_id += 1;
                partition_size += degree;
            }

            let end = node_id + 1;
            partitions.push(DegreePartition::of(start, end - start, partition_size));
            start = end;
        }

        // Merge small last partition with previous one if needed
        let min_last_partition_size = (0.2 * batch_size as f64).round() as usize;
        if partitions.len() > 1
            && partitions.last().unwrap().relationship_count() < min_last_partition_size
        {
            let last_partition = partitions.pop().unwrap();
            let partition_to_merge = partitions.pop().unwrap();

            let merged_partition = DegreePartition::of(
                partition_to_merge.start_node(),
                last_partition.node_count() + partition_to_merge.node_count(),
                partition_to_merge.relationship_count() + last_partition.relationship_count(),
            );

            partitions.push(merged_partition);
        }

        partitions.into_iter().map(task_creator).collect()
    }

    /// Returns a lazy iterator of many small degree partitions.
    ///
    /// This is useful for streaming partitions without materializing them all at once.
    ///
    /// # Arguments
    /// * `node_count` - Total number of nodes
    /// * `relationship_count` - Total number of relationships
    /// * `concurrency` - Number of parallel workers
    /// * `degrees` - Function to get node degrees
    pub fn degree_partition_stream(
        node_count: usize,
        relationship_count: usize,
        concurrency: usize,
        degrees: Box<dyn DegreeFunction>,
    ) -> LazyDegreePartitionIterator {
        if concurrency == 1 {
            // For single-threaded, create a simple iterator that yields one partition
            LazyDegreePartitionIterator::with_partition_size(
                node_count,
                relationship_count,
                degrees,
            )
        } else {
            LazyDegreePartitionIterator::new(node_count, relationship_count, concurrency, degrees)
        }
    }

    // ============================================================================
    // UTILITY METHODS
    // ============================================================================

    /// Calculates adjusted batch size based on concurrency and constraints.
    ///
    /// # Arguments
    /// * `node_count` - Total number of nodes
    /// * `concurrency` - Number of parallel workers
    /// * `min_batch_size` - Minimum batch size constraint
    fn adjusted_batch_size(node_count: usize, concurrency: usize, min_batch_size: usize) -> usize {
        let batch_size = BitUtil::ceil_div(node_count, concurrency);
        std::cmp::max(batch_size, min_batch_size)
    }

    /// Creates tasks from partitions.
    fn tasks<TASK, F>(node_count: usize, batch_size: usize, task_creator: F) -> Vec<TASK>
    where
        F: Fn(Partition) -> TASK,
    {
        let expected_capacity = BitUtil::ceil_div(node_count, batch_size);
        let mut result = Vec::with_capacity(expected_capacity);

        let mut i = 0;
        while i < node_count {
            result.push(task_creator(Partition::of(
                i,
                Self::actual_batch_size(i, batch_size, node_count),
            )));
            i += batch_size;
        }

        result
    }

    /// Calculates actual batch size accounting for the last partition.
    fn actual_batch_size(start_node: usize, batch_size: usize, node_count: usize) -> usize {
        if start_node + batch_size < node_count {
            batch_size
        } else {
            node_count - start_node
        }
    }

    /// Returns actual batch sizes for range partitioning.
    ///
    /// Useful for understanding the distribution of work across partitions.
    pub fn range_partition_actual_batch_sizes(
        concurrency: usize,
        node_count: usize,
        min_batch_size: Option<usize>,
    ) -> Vec<usize> {
        let batch_size = Self::adjusted_batch_size(
            node_count,
            concurrency,
            min_batch_size.unwrap_or(DEFAULT_BATCH_SIZE),
        );

        let expected_capacity = BitUtil::ceil_div(node_count, batch_size);
        let mut batch_sizes = Vec::with_capacity(expected_capacity);

        let mut i = 0;
        while i < node_count {
            batch_sizes.push(Self::actual_batch_size(i, batch_size, node_count));
            i += batch_size;
        }

        batch_sizes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ConstantDegree(usize);

    impl DegreeFunction for ConstantDegree {
        fn degree(&self, _node: usize) -> usize {
            self.0
        }
    }

    #[test]
    fn test_range_partition() {
        let partitions = PartitionUtils::range_partition(4, 1000, |p| p, None);

        assert_eq!(partitions.len(), 4);
        let total_nodes: usize = partitions.iter().map(|p| p.node_count()).sum();
        assert_eq!(total_nodes, 1000);
    }

    #[test]
    fn test_range_partition_with_batch_size() {
        let partitions = PartitionUtils::range_partition_with_batch_size(1000, 100, |p| p);

        assert_eq!(partitions.len(), 10);
        assert_eq!(partitions[0].start_node(), 0);
        assert_eq!(partitions[0].node_count(), 100);
    }

    #[test]
    fn test_number_aligned_partitioning() {
        let partitions = PartitionUtils::number_aligned_partitioning(4, 1000, 64);

        // Each partition should be aligned to 64
        for partition in &partitions {
            if partition.start_node() > 0 {
                assert_eq!(partition.start_node() % 64, 0);
            }
        }
    }

    #[test]
    #[should_panic(expected = "Maximum size of a partition")]
    fn test_number_aligned_partitioning_invalid_max_size() {
        PartitionUtils::number_aligned_partitioning_with_max_size(4, 1000, 100, 50, |p| p);
    }

    #[test]
    fn test_degree_partition() {
        let degrees = Box::new(ConstantDegree(10));
        let partitions = PartitionUtils::degree_partition(1000, 10000, degrees, 4, |p| p, None);

        assert!(!partitions.is_empty());
        let total_nodes: usize = partitions.iter().map(|p| p.node_count()).sum();
        assert_eq!(total_nodes, 1000);
    }

    #[test]
    fn test_degree_partition_single_thread() {
        let degrees = Box::new(ConstantDegree(10));
        let partitions = PartitionUtils::degree_partition(1000, 10000, degrees, 1, |p| p, None);

        assert_eq!(partitions.len(), 1);
        assert_eq!(partitions[0].node_count(), 1000);
        assert_eq!(partitions[0].relationship_count(), 10000);
    }

    #[test]
    fn test_degree_partition_with_batch_size() {
        let degrees = Box::new(ConstantDegree(5));
        let partitions =
            PartitionUtils::degree_partition_with_batch_size(1000, degrees, 500, |p| p);

        assert!(!partitions.is_empty());
        // Each partition should have roughly 500 relationships (100 nodes * 5 degree)
        for partition in &partitions {
            assert!(partition.relationship_count() <= 505); // Allow some tolerance
        }
    }

    #[test]
    fn test_degree_partition_stream() {
        let degrees = Box::new(ConstantDegree(10));
        let mut stream = PartitionUtils::degree_partition_stream(1000, 10000, 4, degrees);

        let partitions: Vec<_> = stream.collect();
        assert!(!partitions.is_empty());

        let total_nodes: usize = partitions.iter().map(|p| p.node_count()).sum();
        assert_eq!(total_nodes, 1000);
    }

    #[test]
    fn test_adjusted_batch_size() {
        assert_eq!(PartitionUtils::adjusted_batch_size(1000, 4, 100), 250);
        assert_eq!(PartitionUtils::adjusted_batch_size(1000, 20, 100), 100);
    }

    #[test]
    fn test_actual_batch_size() {
        assert_eq!(PartitionUtils::actual_batch_size(0, 100, 1000), 100);
        assert_eq!(PartitionUtils::actual_batch_size(950, 100, 1000), 50);
    }

    #[test]
    fn test_range_partition_actual_batch_sizes() {
        let batch_sizes = PartitionUtils::range_partition_actual_batch_sizes(4, 1000, None);

        assert_eq!(batch_sizes.len(), 4);
        let total: usize = batch_sizes.iter().sum();
        assert_eq!(total, 1000);
    }

    #[test]
    fn test_partition_merge_small_last() {
        // Test that small last partitions get merged
        let degrees = Box::new(ConstantDegree(10));
        let partitions =
            PartitionUtils::degree_partition_with_batch_size(105, degrees, 1000, |p| p);

        // Should merge the small last partition
        let last = partitions.last().unwrap();
        assert!(last.relationship_count() >= 200); // Merged partition should be reasonable size
    }
}
