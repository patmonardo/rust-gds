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

//! Represents a partition of nodes in a graph.

/// Represents a partition of nodes in a graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Partition {
    start_node: usize,
    node_count: usize,
}

impl Partition {
    /// Maximum number of nodes in a partition.
    pub const MAX_NODE_COUNT: usize = (i32::MAX as usize - 32) >> 1;

    /// Creates a new partition.
    ///
    /// # Arguments
    /// * `start_node` - The first node ID in this partition
    /// * `node_count` - The number of nodes in this partition
    pub fn new(start_node: usize, node_count: usize) -> Self {
        Self {
            start_node,
            node_count,
        }
    }

    /// Returns the start node ID of this partition.
    #[inline]
    pub fn start_node(&self) -> usize {
        self.start_node
    }

    /// Returns the number of nodes in this partition.
    #[inline]
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    /// Applies the given consumer function to each node ID in the partition.
    ///
    /// # Arguments
    /// * `consumer` - Function that accepts a node ID
    pub fn consume<F>(&self, mut consumer: F)
    where
        F: FnMut(usize),
    {
        let start_node = self.start_node;
        let end_node = start_node + self.node_count;
        for id in start_node..end_node {
            consumer(id);
        }
    }

    /// Creates a new partition with the specified start node and count.
    ///
    /// # Arguments
    /// * `start_node` - The first node ID in the partition
    /// * `node_count` - The number of nodes in the partition
    pub fn of(start_node: usize, node_count: usize) -> Self {
        Self::new(start_node, node_count)
    }

    /// Returns an iterator over the node IDs in this partition.
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.start_node..(self.start_node + self.node_count)
    }
}

impl std::fmt::Display for Partition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Partition{{start:{}, length:{}}}",
            self.start_node, self.node_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_creation() {
        let partition = Partition::new(10, 100);
        assert_eq!(partition.start_node(), 10);
        assert_eq!(partition.node_count(), 100);
    }

    #[test]
    fn test_partition_of() {
        let partition = Partition::of(5, 50);
        assert_eq!(partition.start_node(), 5);
        assert_eq!(partition.node_count(), 50);
    }

    #[test]
    fn test_partition_consume() {
        let partition = Partition::new(0, 5);
        let mut collected = Vec::new();
        partition.consume(|id| collected.push(id));
        assert_eq!(collected, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_partition_iter() {
        let partition = Partition::new(10, 5);
        let collected: Vec<_> = partition.iter().collect();
        assert_eq!(collected, vec![10, 11, 12, 13, 14]);
    }

    #[test]
    fn test_partition_equality() {
        let p1 = Partition::new(10, 100);
        let p2 = Partition::new(10, 100);
        let p3 = Partition::new(10, 101);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }
}
