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

//! RelationshipsBuilder trait - construction interface for building relationship sets.
//!
//! 1:1 translation of org.neo4j.gds.core.loading.construction.RelationshipsBuilder
//! from Java GDS.
//!
//! NOTE: This is currently a minimal trait stub to support ML negative sampling.
//! Full implementation will be added when we translate the graph loading construction layer.

/// Trait for building relationships from internal mapped node IDs.
///
/// 1:1 translation of RelationshipsBuilder.java interface from Java GDS.
///
/// This builder accepts relationships using internal (root/mapped) node IDs
/// and optionally property values.
pub trait RelationshipsBuilder: Send + Sync {
    /// Add a relationship from internal node IDs with a property value.
    ///
    /// # Arguments
    /// * `source` - Internal/mapped source node ID
    /// * `target` - Internal/mapped target node ID
    /// * `property_value` - Relationship property value
    fn add_from_internal(&mut self, source: u64, target: u64, property_value: f64);

    /// Add a relationship from internal node IDs with multiple property values.
    ///
    /// # Arguments
    /// * `source` - Internal/mapped source node ID
    /// * `target` - Internal/mapped target node ID
    /// * `property_values` - Array of relationship property values
    fn add_from_internal_with_properties(
        &mut self,
        source: u64,
        target: u64,
        property_values: &[f64],
    );

    /// Add a relationship from internal node IDs with no properties.
    ///
    /// # Arguments
    /// * `source` - Internal/mapped source node ID
    /// * `target` - Internal/mapped target node ID
    fn add_from_internal_no_property(&mut self, source: u64, target: u64);
}
