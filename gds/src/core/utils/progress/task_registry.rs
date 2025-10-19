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

//! Registry for managing tasks for a specific user session.
//!
//! Convenient wrapper around TaskStore with bound username and jobId.

use crate::core::utils::progress::{JobId, Task, TaskStore, UserTask};
use std::sync::Arc;

/// Registry for managing tasks for a specific user session.
///
/// This is a convenient wrapper around TaskStore that binds a username and job ID,
/// providing a simplified API for task registration without repeatedly passing these parameters.
///
/// # Examples
///
/// ```
/// use gds::core::utils::progress::*;
/// use std::sync::Arc;
///
/// let store = Arc::new(EmptyTaskStore);
/// let job_id = JobId::new();
/// let registry = TaskRegistry::new("alice".to_string(), store, job_id);
///
/// let task = Task::new("Process data".to_string(), 1000);
/// registry.register_task(task.clone());
/// assert!(registry.contains_task(&task));
/// ```
#[derive(Clone)]
pub struct TaskRegistry {
    username: String,
    task_store: Arc<dyn TaskStore>,
    job_id: JobId,
}

impl TaskRegistry {
    /// Create registry with specific JobId.
    pub fn new(username: String, task_store: Arc<dyn TaskStore>, job_id: JobId) -> Self {
        Self {
            username,
            task_store,
            job_id,
        }
    }

    /// Create registry with auto-generated JobId.
    pub fn with_auto_job_id(username: String, task_store: Arc<dyn TaskStore>) -> Self {
        Self::new(username, task_store, JobId::new())
    }

    /// Register a task for this user session.
    pub fn register_task(&self, task: Task) {
        self.task_store
            .store(self.username.clone(), self.job_id.clone(), task);
    }

    /// Unregister the task for this user session.
    pub fn unregister_task(&self) {
        self.task_store.remove(&self.username, &self.job_id);
    }

    /// Check if the registry contains a specific task.
    /// Uses description comparison since Task is not reference-counted in the store.
    pub fn contains_task(&self, task: &Task) -> bool {
        if let Some(user_task) = self.task_store.query(&self.username, &self.job_id) {
            user_task.task.description == task.description
        } else {
            false
        }
    }

    /// Get the username for this registry.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get the job ID for this registry.
    pub fn job_id(&self) -> &JobId {
        &self.job_id
    }

    /// Get the task store used by this registry.
    pub fn task_store(&self) -> &Arc<dyn TaskStore> {
        &self.task_store
    }

    /// Get the current registered task, if any.
    pub fn current_task(&self) -> Option<UserTask> {
        self.task_store.query(&self.username, &self.job_id)
    }

    /// Check if this registry has a registered task.
    pub fn has_task(&self) -> bool {
        self.current_task().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::EmptyTaskStore;

    #[test]
    fn test_create_registry_with_job_id() {
        let store = Arc::new(EmptyTaskStore);
        let job_id = JobId::new();
        let registry = TaskRegistry::new("alice".to_string(), store, job_id.clone());

        assert_eq!(registry.username(), "alice");
        assert_eq!(registry.job_id(), &job_id);
    }

    #[test]
    fn test_create_registry_with_auto_job_id() {
        let store = Arc::new(EmptyTaskStore);
        let registry = TaskRegistry::with_auto_job_id("bob".to_string(), store);

        assert_eq!(registry.username(), "bob");
        assert!(!registry.job_id().as_string().is_empty());
    }

    #[test]
    fn test_register_and_unregister_with_empty_store() {
        let store = Arc::new(EmptyTaskStore);
        let registry = TaskRegistry::with_auto_job_id("charlie".to_string(), store);

        let task = Task::new("Test task".to_string(), 100);

        // Empty store ignores operations
        registry.register_task(task.clone());
        assert!(!registry.has_task()); // Empty store returns None

        registry.unregister_task();
        assert!(!registry.has_task());
    }

    #[test]
    fn test_contains_task_with_empty_store() {
        let store = Arc::new(EmptyTaskStore);
        let registry = TaskRegistry::with_auto_job_id("dave".to_string(), store);

        let task = Task::new("Test task".to_string(), 100);

        // Empty store always returns false
        assert!(!registry.contains_task(&task));
    }

    #[test]
    fn test_current_task_with_empty_store() {
        let store = Arc::new(EmptyTaskStore);
        let registry = TaskRegistry::with_auto_job_id("eve".to_string(), store);

        assert!(registry.current_task().is_none());
    }

    #[test]
    fn test_task_store_accessor() {
        let store: Arc<dyn TaskStore> = Arc::new(EmptyTaskStore);
        let registry = TaskRegistry::with_auto_job_id("frank".to_string(), store.clone());

        // Verify we can access the store (pointer comparison works with trait objects)
        assert!(Arc::ptr_eq(registry.task_store(), &store));
    }

    #[test]
    fn test_clone_registry() {
        let store = Arc::new(EmptyTaskStore);
        let job_id = JobId::new();
        let registry = TaskRegistry::new("grace".to_string(), store, job_id.clone());

        let cloned = registry.clone();

        assert_eq!(cloned.username(), registry.username());
        assert_eq!(cloned.job_id(), registry.job_id());
        assert!(Arc::ptr_eq(cloned.task_store(), registry.task_store()));
    }
}
