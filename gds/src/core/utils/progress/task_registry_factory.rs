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

//! Factory interface for creating TaskRegistry instances.

use crate::core::utils::progress::{JobId, TaskRegistry, TaskStore};
use std::sync::Arc;

/// Factory trait for creating TaskRegistry instances.
///
/// Implementations can provide different strategies for registry creation,
/// such as validation, empty registries, or custom configurations.
pub trait TaskRegistryFactory: Send + Sync {
    /// Create a new TaskRegistry instance with the given JobId.
    fn new_instance(&self, job_id: JobId) -> TaskRegistry;
}

/// Static factory methods for common TaskRegistryFactory implementations.
pub struct TaskRegistryFactories;

impl TaskRegistryFactories {
    /// Create a local TaskRegistryFactory for a specific user and TaskStore.
    /// Validates against duplicate job IDs.
    pub fn local(username: String, task_store: Arc<dyn TaskStore>) -> Arc<dyn TaskRegistryFactory> {
        Arc::new(LocalTaskRegistryFactory::new(username, task_store))
    }

    /// Create an empty TaskRegistryFactory that creates no-op registries.
    pub fn empty() -> Arc<dyn TaskRegistryFactory> {
        Arc::new(EmptyTaskRegistryFactory)
    }
}

/// Empty TaskRegistryFactory that creates no-op TaskRegistry instances.
///
/// This is a zero-sized type that creates registries with EmptyTaskStore,
/// effectively creating registries that do nothing.
///
/// # Examples
///
/// ```
/// use rust_gds::core::utils::progress::*;
///
/// let factory = EmptyTaskRegistryFactory;
/// let job_id = JobId::new();
/// let registry = factory.new_instance(job_id);
///
/// // Registry operations are no-ops with EmptyTaskStore
/// let task = Task::new("Test".to_string(), 100);
/// registry.register_task(task);
/// assert!(!registry.has_task()); // Empty store returns None
/// ```
pub struct EmptyTaskRegistryFactory;

impl TaskRegistryFactory for EmptyTaskRegistryFactory {
    fn new_instance(&self, job_id: JobId) -> TaskRegistry {
        use crate::core::utils::progress::EmptyTaskStore;
        TaskRegistry::new(String::new(), Arc::new(EmptyTaskStore), job_id)
    }
}

/// Local implementation of TaskRegistryFactory with duplicate job validation.
///
/// Ensures no duplicate jobs are running for the same user by checking
/// the TaskStore before creating a new registry.
///
/// # Examples
///
/// ```
/// use rust_gds::core::utils::progress::*;
/// use std::sync::Arc;
///
/// let store = Arc::new(EmptyTaskStore);
/// let factory = LocalTaskRegistryFactory::new("alice".to_string(), store);
///
/// let job_id = JobId::new();
/// let registry = factory.new_instance(job_id);
///
/// assert_eq!(registry.username(), "alice");
/// ```
#[derive(Clone)]
pub struct LocalTaskRegistryFactory {
    username: String,
    task_store: Arc<dyn TaskStore>,
}

impl LocalTaskRegistryFactory {
    /// Create a new LocalTaskRegistryFactory.
    pub fn new(username: String, task_store: Arc<dyn TaskStore>) -> Self {
        Self {
            username,
            task_store,
        }
    }

    /// Get the username this factory creates registries for.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get the TaskStore this factory uses.
    pub fn task_store(&self) -> &Arc<dyn TaskStore> {
        &self.task_store
    }
}

impl TaskRegistryFactory for LocalTaskRegistryFactory {
    fn new_instance(&self, job_id: JobId) -> TaskRegistry {
        // Check if there's already a job running with this jobId
        let existing_task = self.task_store.query(&self.username, &job_id);

        if existing_task.is_some() {
            panic!(
                "A task with job ID {} is already running for user {}",
                job_id.as_string(),
                self.username
            );
        }

        TaskRegistry::new(self.username.clone(), self.task_store.clone(), job_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::{EmptyTaskStore, Task};

    #[test]
    fn test_empty_factory_creates_empty_registry() {
        let factory = EmptyTaskRegistryFactory;
        let job_id = JobId::new();
        let registry = factory.new_instance(job_id);

        assert_eq!(registry.username(), "");
        assert!(!registry.has_task());
    }

    #[test]
    fn test_empty_factory_via_factories() {
        let factory = TaskRegistryFactories::empty();
        let job_id = JobId::new();
        let registry = factory.new_instance(job_id);

        assert!(!registry.has_task());
    }

    #[test]
    fn test_local_factory_creates_registry() {
        let store = Arc::new(EmptyTaskStore);
        let factory = LocalTaskRegistryFactory::new("alice".to_string(), store);

        let job_id = JobId::new();
        let registry = factory.new_instance(job_id.clone());

        assert_eq!(registry.username(), "alice");
        assert_eq!(registry.job_id(), &job_id);
    }

    #[test]
    fn test_local_factory_via_factories() {
        let store = Arc::new(EmptyTaskStore);
        let factory = TaskRegistryFactories::local("bob".to_string(), store);

        let job_id = JobId::new();
        let registry = factory.new_instance(job_id);

        assert_eq!(registry.username(), "bob");
    }

    #[test]
    fn test_local_factory_accessors() {
        let store: Arc<dyn TaskStore> = Arc::new(EmptyTaskStore);
        let factory = LocalTaskRegistryFactory::new("charlie".to_string(), store.clone());

        assert_eq!(factory.username(), "charlie");
        assert!(Arc::ptr_eq(factory.task_store(), &store));
    }

    #[test]
    fn test_local_factory_clone() {
        let store = Arc::new(EmptyTaskStore);
        let factory = LocalTaskRegistryFactory::new("dave".to_string(), store);

        let cloned = factory.clone();

        assert_eq!(cloned.username(), factory.username());
        assert!(Arc::ptr_eq(cloned.task_store(), factory.task_store()));
    }

    #[test]
    fn test_local_factory_no_duplicate_with_empty_store() {
        // EmptyTaskStore always returns None, so no duplicate detection
        let store = Arc::new(EmptyTaskStore);
        let factory = LocalTaskRegistryFactory::new("eve".to_string(), store);

        let job_id = JobId::new();

        // First registry
        let _registry1 = factory.new_instance(job_id.clone());

        // Second registry with same job_id - should work because EmptyTaskStore returns None
        let _registry2 = factory.new_instance(job_id);
    }

    #[test]
    fn test_empty_factory_operations_are_noop() {
        let factory = EmptyTaskRegistryFactory;
        let registry = factory.new_instance(JobId::new());

        let task = Task::new("Test task".to_string(), 100);
        registry.register_task(task.clone());

        // Empty store means no task is actually stored
        assert!(!registry.has_task());
        assert!(!registry.contains_task(&task));
        assert!(registry.current_task().is_none());
    }
}
