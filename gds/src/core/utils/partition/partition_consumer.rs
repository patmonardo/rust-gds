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

//! Consumer trait for partitions.

use super::Partition;

/// Consumer for partitions that processes each partition.
pub trait PartitionConsumer<P: AsRef<Partition>> {
    /// Process a single partition.
    ///
    /// # Arguments
    /// * `partition` - The partition to process
    fn consume(&mut self, partition: P);
}

// Implement PartitionConsumer for closures
impl<F, P> PartitionConsumer<P> for F
where
    F: FnMut(P),
    P: AsRef<Partition>,
{
    fn consume(&mut self, partition: P) {
        self(partition);
    }
}
