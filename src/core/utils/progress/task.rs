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

//! Task abstraction for progress tracking.
//!
//! This is a placeholder - the full Task hierarchy will be implemented
//! in the tasks/ submodule.

/// Marker for unknown task volume.
pub const UNKNOWN_VOLUME: usize = usize::MAX;

/// Represents a task that can be tracked for progress.
///
/// This is a simplified placeholder. The full Task trait hierarchy
/// will be implemented in the tasks/ module.
#[derive(Debug, Clone)]
pub struct Task {
    pub description: String,
    pub volume: usize,
}

impl Task {
    /// Unknown volume constant.
    pub const UNKNOWN_VOLUME: usize = UNKNOWN_VOLUME;

    /// Create a new task with known volume.
    pub fn new(description: String, volume: usize) -> Self {
        Self {
            description,
            volume,
        }
    }

    /// Create a task with unknown volume.
    pub fn with_unknown_volume(description: String) -> Self {
        Self {
            description,
            volume: UNKNOWN_VOLUME,
        }
    }

    /// Get task description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get task volume.
    pub fn volume(&self) -> usize {
        self.volume
    }

    /// Check if volume is known.
    pub fn has_known_volume(&self) -> bool {
        self.volume != UNKNOWN_VOLUME
    }
}
