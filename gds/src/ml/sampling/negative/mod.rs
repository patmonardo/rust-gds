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

//! Negative sampling for machine learning.
//!
//! 1:1 translation of org.neo4j.gds.ml.negativeSampling package from Java GDS.

mod factory;
mod negative_sampler;
mod random;
mod user_input;

pub use factory::create_sampler;
pub use negative_sampler::{NegativeSampler, NEGATIVE};
pub use random::RandomNegativeSampler;
pub use user_input::UserInputNegativeSampler;
