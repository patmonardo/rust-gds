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

//! Job identifier for tracking running tasks.

use std::fmt;
use uuid::Uuid;

/// Unique identifier for a job/task execution.
///
/// Used to track and manage running tasks in the progress tracking system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JobId {
    value: String,
}

impl JobId {
    /// Empty job ID for special cases.
    pub const EMPTY: JobId = JobId {
        value: String::new(),
    };

    /// Create a new JobId with auto-generated UUID.
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4().to_string(),
        }
    }

    /// Create JobId from existing UUID string.
    pub fn from_uuid(uuid: String) -> Self {
        Self { value: uuid }
    }

    /// Get the string representation.
    pub fn as_string(&self) -> &str {
        &self.value
    }
}

impl Default for JobId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for JobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<String> for JobId {
    fn from(value: String) -> Self {
        Self::from_uuid(value)
    }
}

impl From<&str> for JobId {
    fn from(value: &str) -> Self {
        Self::from_uuid(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_job_id() {
        let id1 = JobId::new();
        let id2 = JobId::new();

        assert_ne!(id1, id2);
        assert!(!id1.as_string().is_empty());
    }

    #[test]
    fn test_from_uuid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = JobId::from_uuid(uuid_str.to_string());

        assert_eq!(id.as_string(), uuid_str);
    }

    #[test]
    fn test_empty() {
        let empty = JobId::EMPTY;
        assert_eq!(empty.as_string(), "");
    }

    #[test]
    fn test_equality() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id1 = JobId::from_uuid(uuid_str.to_string());
        let id2 = JobId::from_uuid(uuid_str.to_string());

        assert_eq!(id1, id2);
    }
}
