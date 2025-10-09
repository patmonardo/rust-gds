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

//! Provider interface for TaskStore instances.
//!
//! Simplified version without Neo4j kernel dependencies.

use crate::core::utils::progress::{TaskStore, TaskStoreHolder};
use std::sync::Arc;

/// Provider trait for TaskStore instances.
///
/// Implementations can provide different strategies for accessing TaskStores,
/// such as from a global registry, dependency injection, or configuration.
pub trait TaskStoreProvider: Send + Sync {
    /// Get TaskStore for a given database name.
    fn get_task_store(&self, database_name: &str) -> Arc<dyn TaskStore>;
}

/// Basic implementation using TaskStoreHolder.
///
/// This is simpler than Neo4j's procedure context integration.
/// It delegates to the global TaskStoreHolder registry.
///
/// # Examples
///
/// ```
/// use rust_gds::core::utils::progress::*;
///
/// let provider = SimpleTaskStoreProvider;
/// let store = provider.get_task_store("neo4j");
///
/// let job_id = JobId::new();
/// let task = Task::new("Process".to_string(), 1000);
/// store.store("alice".to_string(), job_id, task);
/// ```
pub struct SimpleTaskStoreProvider;

impl TaskStoreProvider for SimpleTaskStoreProvider {
    fn get_task_store(&self, database_name: &str) -> Arc<dyn TaskStore> {
        #[allow(deprecated)]
        TaskStoreHolder::get_task_store(database_name)
    }
}

/// Factory for creating TaskStore providers.
///
/// Provides convenient access to standard provider implementations.
pub struct TaskStoreProviders;

impl TaskStoreProviders {
    /// Get the default TaskStore provider.
    ///
    /// Returns a SimpleTaskStoreProvider that uses TaskStoreHolder.
    pub fn default_provider() -> Arc<dyn TaskStoreProvider> {
        Arc::new(SimpleTaskStoreProvider)
    }

    /// Create a provider for a specific database.
    ///
    /// Returns a closure that always returns the same database's store.
    pub fn for_database(database_name: String) -> impl Fn() -> Arc<dyn TaskStore> {
        move || {
            #[allow(deprecated)]
            TaskStoreHolder::get_task_store(&database_name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::{JobId, Task};

    #[test]
    fn test_simple_provider_get_store() {
        let db_name = "test_provider_simple_get";

        #[allow(deprecated)]
        TaskStoreHolder::purge(db_name);

        let provider = SimpleTaskStoreProvider;
        let store = provider.get_task_store(db_name);

        // Should be able to use the store
        let job_id = JobId::new();
        let task = Task::new("Test".to_string(), 100);
        store.store("alice".to_string(), job_id, task);

        assert_eq!(store.task_count(), 1);
    }

    #[test]
    fn test_simple_provider_returns_same_store() {
        let db_name = "test_provider_same_store";

        #[allow(deprecated)]
        TaskStoreHolder::purge(db_name);

        let provider = SimpleTaskStoreProvider;
        let store1 = provider.get_task_store(db_name);
        let store2 = provider.get_task_store(db_name);

        // Should be the same store
        assert!(Arc::ptr_eq(&store1, &store2));
    }

    #[test]
    fn test_simple_provider_different_databases() {
        let db1 = "test_provider_diff_stores_db1";
        let db2 = "test_provider_diff_stores_db2";

        #[allow(deprecated)]
        {
            TaskStoreHolder::purge(db1);
            TaskStoreHolder::purge(db2);
        }

        let provider = SimpleTaskStoreProvider;
        let store1 = provider.get_task_store(db1);
        let store2 = provider.get_task_store(db2);

        // Should be different stores
        assert!(!Arc::ptr_eq(&store1, &store2));
    }

    #[test]
    fn test_default_provider() {
        let db_name = "test_provider_default";

        #[allow(deprecated)]
        TaskStoreHolder::purge(db_name);

        let provider = TaskStoreProviders::default_provider();
        let store = provider.get_task_store(db_name);

        assert_eq!(store.task_count(), 0);
    }

    #[test]
    fn test_for_database_factory() {
        let db_name = "test_provider_for_db_unique";

        #[allow(deprecated)]
        TaskStoreHolder::purge(db_name);

        let get_store = TaskStoreProviders::for_database(db_name.to_string());

        // Call it multiple times
        let store1 = get_store();
        let store2 = get_store();

        // Should return the same store
        assert!(Arc::ptr_eq(&store1, &store2));
    }

    #[test]
    fn test_for_database_different_closures() {
        let db1 = "test_provider_diff_db1";
        let db2 = "test_provider_diff_db2";

        #[allow(deprecated)]
        {
            TaskStoreHolder::purge(db1);
            TaskStoreHolder::purge(db2);
        }

        let get_store1 = TaskStoreProviders::for_database(db1.to_string());
        let get_store2 = TaskStoreProviders::for_database(db2.to_string());

        let store1 = get_store1();
        let store2 = get_store2();

        // Should return different stores
        assert!(!Arc::ptr_eq(&store1, &store2));
    }

    #[test]
    fn test_provider_trait_object() {
        let db_name = "test_provider_trait_obj";

        #[allow(deprecated)]
        TaskStoreHolder::purge(db_name);

        // Test that we can use trait objects
        let provider: Arc<dyn TaskStoreProvider> = Arc::new(SimpleTaskStoreProvider);
        let store = provider.get_task_store(db_name);

        let job_id = JobId::new();
        let task = Task::new("Test".to_string(), 100);
        store.store("alice".to_string(), job_id, task);

        assert_eq!(store.task_count(), 1);
    }

    #[test]
    fn test_concurrent_provider_access() {
        use std::thread;

        let db_name = "test_provider_concurrent_unique_db";

        #[allow(deprecated)]
        TaskStoreHolder::purge(db_name);

        let provider = Arc::new(SimpleTaskStoreProvider);
        let mut handles = vec![];

        // Spawn multiple threads using the same provider
        for i in 0..10 {
            let provider_clone = provider.clone();
            let db_clone = db_name.to_string();
            let handle = thread::spawn(move || {
                let store = provider_clone.get_task_store(&db_clone);
                let job_id = JobId::new();
                let task = Task::new(format!("Task {}", i), 100);
                store.store(format!("user{}", i), job_id, task);
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all tasks were stored
        let store = provider.get_task_store(db_name);
        assert_eq!(store.task_count(), 10);
    }
}
