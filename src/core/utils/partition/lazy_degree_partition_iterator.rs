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

//! Lazy iterator that generates degree partitions on-demand.

use super::{DegreeFunction, DegreePartition};
use crate::mem::BitUtil;

/// This is a good guess to achieve smaller partitions
const DIVISION_FACTOR: usize = 10;

/// Lazy iterator that generates degree partitions on-demand.
pub struct LazyDegreePartitionIterator {
    node_count: usize,
    partition_size: usize,
    degrees: Box<dyn DegreeFunction>,
    next_start_node: usize,
}

impl LazyDegreePartitionIterator {
    /// Creates a new lazy degree partition iterator.
    ///
    /// # Arguments
    /// * `node_count` - Total number of nodes
    /// * `relationship_count` - Total number of relationships
    /// * `concurrency` - Concurrency level
    /// * `degrees` - Function to get node degrees
    pub fn new(
        node_count: usize,
        relationship_count: usize,
        concurrency: usize,
        degrees: Box<dyn DegreeFunction>,
    ) -> Self {
        let num_relationships_in_partition =
            BitUtil::ceil_div(relationship_count, concurrency * DIVISION_FACTOR);

        Self {
            node_count,
            partition_size: num_relationships_in_partition,
            degrees,
            next_start_node: 0,
        }
    }

    /// Creates a new lazy degree partition iterator with explicit partition size.
    pub fn with_partition_size(
        node_count: usize,
        partition_size: usize,
        degrees: Box<dyn DegreeFunction>,
    ) -> Self {
        Self {
            node_count,
            partition_size,
            degrees,
            next_start_node: 0,
        }
    }
}

impl Iterator for LazyDegreePartitionIterator {
    type Item = DegreePartition;

    fn next(&mut self) -> Option<Self::Item> {
        let mut node_id = self.next_start_node;

        if node_id >= self.node_count {
            return None;
        }

        let mut rels_in_partition = 0;
        let start_node = node_id;

        while node_id < self.node_count {
            let next_relationship_count = self.degrees.degree(node_id);

            rels_in_partition += next_relationship_count;
            if rels_in_partition > self.partition_size {
                self.next_start_node = node_id + 1;
                return Some(DegreePartition::of(
                    start_node,
                    self.next_start_node - start_node,
                    rels_in_partition,
                ));
            }
            node_id += 1;
        }

        self.next_start_node = node_id + 1;
        Some(DegreePartition::of(
            start_node,
            self.node_count - start_node,
            rels_in_partition,
        ))
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
    fn test_lazy_degree_partition_iterator() {
        let degrees = Box::new(ConstantDegree(10));
        let iterator = LazyDegreePartitionIterator::with_partition_size(100, 100, degrees);

        let partitions: Vec<_> = iterator.collect();

        assert!(!partitions.is_empty());
        let total_nodes: usize = partitions.iter().map(|p| p.node_count()).sum();
        assert_eq!(total_nodes, 100);
    }

    #[test]
    fn test_lazy_degree_partition_with_concurrency() {
        let degrees = Box::new(ConstantDegree(5));
        let iterator = LazyDegreePartitionIterator::new(1000, 5000, 4, degrees);

        let partitions: Vec<_> = iterator.collect();

        assert!(!partitions.is_empty());
        let total_nodes: usize = partitions.iter().map(|p| p.node_count()).sum();
        assert_eq!(total_nodes, 1000);
    }
}
