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

//! Task execution status enumeration.

use std::fmt;

/// Represents the lifecycle states of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Pending,
    Running,
    Finished,
    Canceled,
    Failed,
}

impl Status {
    /// Check if status represents a terminal state (task is done).
    pub fn is_terminal(self) -> bool {
        matches!(self, Status::Finished | Status::Canceled | Status::Failed)
    }

    /// Check if status represents an active state (task is in progress).
    pub fn is_active(self) -> bool {
        self == Status::Running
    }

    /// Check if status represents a waiting state (task hasn't started).
    pub fn is_pending(self) -> bool {
        self == Status::Pending
    }

    /// Check if status represents successful completion.
    pub fn is_successful(self) -> bool {
        self == Status::Finished
    }

    /// Check if status represents a failure state.
    pub fn is_failed(self) -> bool {
        matches!(self, Status::Failed | Status::Canceled)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "PENDING"),
            Status::Running => write!(f, "RUNNING"),
            Status::Finished => write!(f, "FINISHED"),
            Status::Canceled => write!(f, "CANCELED"),
            Status::Failed => write!(f, "FAILED"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_states() {
        assert!(!Status::Pending.is_terminal());
        assert!(!Status::Running.is_terminal());
        assert!(Status::Finished.is_terminal());
        assert!(Status::Canceled.is_terminal());
        assert!(Status::Failed.is_terminal());
    }

    #[test]
    fn test_active_state() {
        assert!(!Status::Pending.is_active());
        assert!(Status::Running.is_active());
        assert!(!Status::Finished.is_active());
    }

    #[test]
    fn test_successful_completion() {
        assert!(!Status::Pending.is_successful());
        assert!(!Status::Running.is_successful());
        assert!(Status::Finished.is_successful());
        assert!(!Status::Canceled.is_successful());
        assert!(!Status::Failed.is_successful());
    }
}
