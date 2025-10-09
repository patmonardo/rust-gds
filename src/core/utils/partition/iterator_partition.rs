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

//! A partition that's defined by an iterator over node IDs.

/// A partition that's defined by an iterator over node IDs.
pub struct IteratorPartition<I>
where
    I: Iterator<Item = usize>,
{
    iterator: I,
    length: usize,
}

impl<I> IteratorPartition<I>
where
    I: Iterator<Item = usize>,
{
    /// Creates a new iterator partition.
    ///
    /// # Arguments
    /// * `iterator` - Iterator over node IDs
    /// * `length` - Number of nodes in the partition
    pub fn new(iterator: I, length: usize) -> Self {
        Self { iterator, length }
    }

    /// Returns the number of nodes in this partition.
    #[inline]
    pub fn length(&self) -> usize {
        self.length
    }

    /// Returns true if the partition is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Applies the given consumer function to each node ID in the partition.
    ///
    /// # Arguments
    /// * `consumer` - Function that accepts a node ID
    pub fn consume<F>(mut self, mut consumer: F)
    where
        F: FnMut(usize),
    {
        for _ in 0..self.length {
            if let Some(node_id) = self.iterator.next() {
                consumer(node_id);
            } else {
                break;
            }
        }
    }

    /// Materializes the partition into a vector.
    /// For testing purposes only.
    #[cfg(test)]
    pub fn materialize(self) -> Vec<usize> {
        let mut values = Vec::with_capacity(self.length);
        self.consume(|value| values.push(value));
        values
    }
}

impl<I> std::fmt::Display for IteratorPartition<I>
where
    I: Iterator<Item = usize>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IteratorPartition{{length={}}}", self.length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_partition_creation() {
        let data = vec![1, 2, 3, 4, 5];
        let partition = IteratorPartition::new(data.into_iter(), 5);
        assert_eq!(partition.length(), 5);
        assert!(!partition.is_empty());
    }

    #[test]
    fn test_iterator_partition_consume() {
        let data = vec![10, 20, 30];
        let partition = IteratorPartition::new(data.into_iter(), 3);

        let mut collected = Vec::new();
        partition.consume(|id| collected.push(id));

        assert_eq!(collected, vec![10, 20, 30]);
    }

    #[test]
    fn test_iterator_partition_materialize() {
        let data = vec![1, 2, 3, 4];
        let partition = IteratorPartition::new(data.clone().into_iter(), 4);

        let materialized = partition.materialize();
        assert_eq!(materialized, data);
    }

    #[test]
    fn test_iterator_partition_empty() {
        let data: Vec<usize> = vec![];
        let partition = IteratorPartition::new(data.into_iter(), 0);
        assert!(partition.is_empty());
    }
}
