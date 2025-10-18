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

//! NegativeSampler trait definition.
//!
//! 1:1 translation of NegativeSampler.java interface from Java GDS.

use crate::projection::factory::RelationshipsBuilder;

/// Constant value for negative samples
pub const NEGATIVE: f64 = 0.0;

/// Trait for negative sampling strategies.
/// 1:1 translation of NegativeSampler interface from Java GDS.
pub trait NegativeSampler: Send + Sync {
    /// Produces negative samples and adds them to the test and train set builders.
    ///
    /// # Arguments
    /// * `test_set_builder` - Builder for test set relationships
    /// * `train_set_builder` - Builder for train set relationships
    fn produce_negative_samples(
        &self,
        test_set_builder: &mut dyn RelationshipsBuilder,
        train_set_builder: &mut dyn RelationshipsBuilder,
    );
}
