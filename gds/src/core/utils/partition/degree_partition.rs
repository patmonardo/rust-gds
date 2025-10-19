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

//! A partition that also tracks the number of relationships (degrees) within it.

use super::Partition;

/// A partition that also tracks the number of relationships (degrees) within it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DegreePartition {
    partition: Partition,
    relationship_count: usize,
}

impl DegreePartition {
    /// Creates a new degree partition.
    ///
    /// # Arguments
    /// * `start_node` - The first node ID in this partition
    /// * `node_count` - The number of nodes in this partition
    /// * `relationship_count` - The total number of relationships in this partition
    pub fn new(start_node: usize, node_count: usize, relationship_count: usize) -> Self {
        Self {
            partition: Partition::new(start_node, node_count),
            relationship_count,
        }
    }

    /// Returns the number of relationships in this partition.
    #[inline]
    pub fn relationship_count(&self) -> usize {
        self.relationship_count
    }

    /// Returns the start node ID of this partition.
    #[inline]
    pub fn start_node(&self) -> usize {
        self.partition.start_node()
    }

    /// Returns the number of nodes in this partition.
    #[inline]
    pub fn node_count(&self) -> usize {
        self.partition.node_count()
    }

    /// Creates a new degree partition.
    ///
    /// # Arguments
    /// * `start_node` - The first node ID in the partition
    /// * `node_count` - The number of nodes in the partition
    /// * `total_degree` - The total degree (relationship count) in this partition
    pub fn of(start_node: usize, node_count: usize, total_degree: usize) -> Self {
        Self::new(start_node, node_count, total_degree)
    }

    /// Returns a reference to the underlying partition.
    pub fn as_partition(&self) -> &Partition {
        &self.partition
    }
}

impl std::fmt::Display for DegreePartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DegreePartition{{start:{}, length:{}, relationshipCount={}}}",
            self.start_node(),
            self.node_count(),
            self.relationship_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree_partition_creation() {
        let partition = DegreePartition::new(10, 100, 500);
        assert_eq!(partition.start_node(), 10);
        assert_eq!(partition.node_count(), 100);
        assert_eq!(partition.relationship_count(), 500);
    }

    #[test]
    fn test_degree_partition_of() {
        let partition = DegreePartition::of(5, 50, 250);
        assert_eq!(partition.start_node(), 5);
        assert_eq!(partition.node_count(), 50);
        assert_eq!(partition.relationship_count(), 250);
    }

    #[test]
    fn test_degree_partition_equality() {
        let p1 = DegreePartition::new(10, 100, 500);
        let p2 = DegreePartition::new(10, 100, 500);
        let p3 = DegreePartition::new(10, 100, 501);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }
}
