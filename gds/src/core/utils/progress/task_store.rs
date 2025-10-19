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

//! Core storage abstraction for task management.

use super::{JobId, Task, TaskStoreListener, UserTask};

/// Interface for storing and querying user tasks.
///
/// The TaskStore provides the persistence layer for tracking running tasks.
/// It supports queries by username, job ID, or both.
pub trait TaskStore: Send + Sync {
    /// Store a task for a user and job.
    fn store(&self, username: String, job_id: JobId, task: Task);

    /// Remove a task for a user and job.
    fn remove(&self, username: &str, job_id: &JobId);

    /// Query all tasks.
    fn query_all(&self) -> Vec<UserTask>;

    /// Query tasks by job ID.
    fn query_by_job_id(&self, job_id: &JobId) -> Vec<UserTask>;

    /// Query tasks by username.
    fn query_by_username(&self, username: &str) -> Vec<UserTask>;

    /// Query specific task by username and job ID.
    fn query(&self, username: &str, job_id: &JobId) -> Option<UserTask>;

    /// Check if store is empty.
    fn is_empty(&self) -> bool;

    /// Get total task count.
    fn task_count(&self) -> usize;

    /// Add a listener for task store events.
    fn add_listener(&self, listener: Box<dyn TaskStoreListener>);
}
