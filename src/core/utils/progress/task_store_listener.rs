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

//! Listener interface for TaskStore lifecycle events.

use super::{JobId, UserTask};

/// Listener for task store events.
///
/// Implementations can observe when tasks are added, removed, or when
/// the store is cleared.
pub trait TaskStoreListener: Send + Sync {
    /// Called when a task is added to the store.
    fn on_task_added(&self, user_task: &UserTask);

    /// Called when a task is removed from the store.
    fn on_task_removed(&self, username: &str, job_id: &JobId);

    /// Called when the store is cleared.
    fn on_store_cleared(&self);
}
