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

//! User task record combining user, job, and task information.

use super::{JobId, Task};

/// A task associated with a specific user and job.
///
/// This is a simple record type that bundles together the username,
/// job identifier, and the task itself for storage and querying.
#[derive(Debug, Clone)]
pub struct UserTask {
    pub username: String,
    pub job_id: JobId,
    pub task: Task,
}

impl UserTask {
    /// Create a new UserTask.
    pub fn new(username: String, job_id: JobId, task: Task) -> Self {
        Self {
            username,
            job_id,
            task,
        }
    }

    /// Get the username.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get the job ID.
    pub fn job_id(&self) -> &JobId {
        &self.job_id
    }

    /// Get the task.
    pub fn task(&self) -> &Task {
        &self.task
    }
}
