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

//! Core utilities for rust-gds.
//!
//! This module provides foundational utilities used throughout the library:
//! - Time and clock services
//! - Progress tracking and timing
//! - Partitioning strategies
//! - Queue implementations
//! - Data shuffling utilities
//! - Bit manipulation and raw value encoding
//! - Set intersections and vector similarity operations
//! - Cache-efficient binary search (Eytzinger layout)
//! - Lazy batch collection for parallel processing

pub mod array_layout;
pub mod clock_service;
pub mod intersections;
pub mod lazy_batch_collection;
pub mod progress_timer;
pub mod raw_values;
pub mod time_util;

pub mod paged;
pub mod partition;
pub mod progress;
pub mod queue;
pub mod shuffle;

// Re-exports for convenience
pub use array_layout::{ArrayLayout, LayoutAndSecondary};
pub use clock_service::{Clock, ClockService};
pub use intersections::Intersections;
pub use lazy_batch_collection::LazyBatchCollection;
pub use progress_timer::ProgressTimer;
pub use raw_values::RawValues;
pub use shuffle::{Random as ShuffleRandom, ShuffleUtil, SplittableRandom};
pub use time_util::{TimeUtil, ZoneId};
