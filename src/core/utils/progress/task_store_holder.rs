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

//! Global registry for TaskStore instances per database.
//!
//! # Warning
//!
//! This is a **deprecated** temporary workaround. Eliminate as soon as possible.
//!
//! Translation of Java TaskStoreHolder - maintains per-database task stores
//! in a JVM-wide (now process-wide) singleton map.

use crate::core::utils::progress::{PerDatabaseTaskStore, TaskStore};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Global registry for TaskStore instances per database.
///
/// # Deprecation Warning
///
/// This is a temporary workaround for maintaining global state during migration.
/// It should be eliminated as soon as the codebase is refactored to pass TaskStore
/// instances explicitly through dependency injection.
///
/// # Thread Safety
///
/// This implementation is thread-safe using RwLock for the global registry.
///
/// # Examples
///
/// ```
/// use rust_gds::core::utils::progress::*;
///
/// // Get or create store for a database
/// let store = TaskStoreHolder::get_task_store("neo4j");
///
/// // Use the store
/// let job_id = JobId::new();
/// let task = Task::new("Process".to_string(), 1000);
/// store.store("alice".to_string(), job_id, task);
///
/// // Clean up when done
/// TaskStoreHolder::purge("neo4j");
/// ```
#[deprecated(
    since = "0.1.0",
    note = "This is a temporary workaround. Use dependency injection instead."
)]
pub struct TaskStoreHolder;

lazy_static::lazy_static! {
    static ref TASK_STORES: RwLock<HashMap<String, Arc<dyn TaskStore>>> = RwLock::new(HashMap::new());
}

impl TaskStoreHolder {
    /// Get or create a TaskStore for the given database.
    ///
    /// Database names are normalized to lowercase for consistency.
    ///
    /// # Thread Safety
    ///
    /// This method is thread-safe. Multiple threads can call it simultaneously.
    /// If the store doesn't exist, only one thread will create it.
    pub fn get_task_store(database_name: &str) -> Arc<dyn TaskStore> {
        let normalized = Self::to_lowercase(database_name);

        // Fast path: try read lock first
        {
            let stores = TASK_STORES.read().unwrap();
            if let Some(store) = stores.get(&normalized) {
                return store.clone();
            }
        }

        // Slow path: need to create store
        let mut stores = TASK_STORES.write().unwrap();

        // Double-check in case another thread created it
        if let Some(store) = stores.get(&normalized) {
            return store.clone();
        }

        // Create new store
        let store: Arc<dyn TaskStore> = Arc::new(PerDatabaseTaskStore::new());
        stores.insert(normalized.clone(), store.clone());
        store
    }

    /// Remove the TaskStore for the given database.
    ///
    /// This clears all tasks for that database from the global registry.
    pub fn purge(database_name: &str) {
        let normalized = Self::to_lowercase(database_name);
        let mut stores = TASK_STORES.write().unwrap();
        stores.remove(&normalized);
    }

    /// Clear all TaskStores.
    ///
    /// This is useful for testing or application shutdown.
    pub fn clear() {
        let mut stores = TASK_STORES.write().unwrap();
        stores.clear();
    }

    /// Get all registered database names.
    pub fn database_names() -> Vec<String> {
        let stores = TASK_STORES.read().unwrap();
        stores.keys().cloned().collect()
    }

    /// Get the number of registered databases.
    pub fn size() -> usize {
        let stores = TASK_STORES.read().unwrap();
        stores.len()
    }

    /// Normalize database name to lowercase.
    fn to_lowercase(s: &str) -> String {
        s.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::{JobId, Task};

    #[test]
    fn test_get_task_store_creates_new() {
        let db_name = "test_db_creates_new";
        TaskStoreHolder::purge(db_name);

        let store = TaskStoreHolder::get_task_store(db_name);
        let size_after = TaskStoreHolder::size();

        // Verify it's a working store
        let job_id = JobId::new();
        let task = Task::new("Test".to_string(), 100);
        store.store("alice".to_string(), job_id.clone(), task);

        assert_eq!(store.task_count(), 1);
        assert!(size_after >= 1); // At least this database exists
    }

    #[test]
    fn test_get_task_store_returns_same_instance() {
        let db_name = "test_db_same_instance";
        TaskStoreHolder::purge(db_name);

        let store1 = TaskStoreHolder::get_task_store(db_name);
        let store2 = TaskStoreHolder::get_task_store(db_name);

        // Should be the same Arc instance
        assert!(Arc::ptr_eq(&store1, &store2));
    }

    #[test]
    fn test_database_name_normalization() {
        let db_name_base = "test_normalization_db";
        TaskStoreHolder::purge(db_name_base);

        let store1 = TaskStoreHolder::get_task_store("test_normalization_DB");
        let store2 = TaskStoreHolder::get_task_store("test_normalization_db");
        let store3 = TaskStoreHolder::get_task_store("TEST_NORMALIZATION_DB");

        // All should return the same store
        assert!(Arc::ptr_eq(&store1, &store2));
        assert!(Arc::ptr_eq(&store2, &store3));
    }

    #[test]
    fn test_multiple_databases() {
        let db1 = "test_multi_db1";
        let db2 = "test_multi_db2";
        let db3 = "test_multi_db3";

        TaskStoreHolder::purge(db1);
        TaskStoreHolder::purge(db2);
        TaskStoreHolder::purge(db3);

        let store1 = TaskStoreHolder::get_task_store(db1);
        let store2 = TaskStoreHolder::get_task_store(db2);
        let store3 = TaskStoreHolder::get_task_store(db3);

        let names = TaskStoreHolder::database_names();
        assert!(names.contains(&db1.to_string()));
        assert!(names.contains(&db2.to_string()));
        assert!(names.contains(&db3.to_string()));

        // Verify they are different stores
        assert!(!Arc::ptr_eq(&store1, &store2));
        assert!(!Arc::ptr_eq(&store2, &store3));
    }

    #[test]
    fn test_purge_removes_database() {
        let db1 = "test_purge_db1";
        let db2 = "test_purge_db2";

        TaskStoreHolder::purge(db1);
        TaskStoreHolder::purge(db2);

        TaskStoreHolder::get_task_store(db1);
        TaskStoreHolder::get_task_store(db2);

        let names_before = TaskStoreHolder::database_names();
        assert!(names_before.contains(&db1.to_string()));
        assert!(names_before.contains(&db2.to_string()));

        TaskStoreHolder::purge(db1);

        let names_after = TaskStoreHolder::database_names();
        assert!(!names_after.contains(&db1.to_string()));
        assert!(names_after.contains(&db2.to_string()));

        // db2 should still exist
        let store2 = TaskStoreHolder::get_task_store(db2);
        assert_eq!(store2.task_count(), 0);
    }

    #[test]
    fn test_purge_nonexistent_database() {
        // Should not panic
        TaskStoreHolder::purge("nonexistent_totally_unique_db_xyz");
    }

    #[test]
    fn test_database_names() {
        let db1 = "test_names_alpha";
        let db2 = "test_names_beta";
        let db3 = "test_names_gamma";

        TaskStoreHolder::purge(db1);
        TaskStoreHolder::purge(db2);
        TaskStoreHolder::purge(db3);

        TaskStoreHolder::get_task_store(db1);
        TaskStoreHolder::get_task_store(db2);
        TaskStoreHolder::get_task_store(db3);

        let names = TaskStoreHolder::database_names();

        assert!(names.contains(&db1.to_string()));
        assert!(names.contains(&db2.to_string()));
        assert!(names.contains(&db3.to_string()));
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;

        let db_name = "test_concurrent_unique_db";
        TaskStoreHolder::purge(db_name);

        let mut handles = vec![];

        // Spawn multiple threads trying to get the same store
        for i in 0..10 {
            let db_clone = db_name.to_string();
            let handle = thread::spawn(move || {
                let store = TaskStoreHolder::get_task_store(&db_clone);
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

        // Should have all 10 tasks
        let store = TaskStoreHolder::get_task_store(db_name);
        assert_eq!(store.task_count(), 10);
    }

    #[test]
    fn test_store_isolation() {
        let db1 = "test_isolation_db1";
        let db2 = "test_isolation_db2";

        TaskStoreHolder::purge(db1);
        TaskStoreHolder::purge(db2);

        let store1 = TaskStoreHolder::get_task_store(db1);
        let store2 = TaskStoreHolder::get_task_store(db2);

        // Add task to store1
        let job_id1 = JobId::new();
        let task1 = Task::new("Task 1".to_string(), 100);
        store1.store("alice".to_string(), job_id1, task1);

        // Add task to store2
        let job_id2 = JobId::new();
        let task2 = Task::new("Task 2".to_string(), 200);
        store2.store("bob".to_string(), job_id2, task2);

        // Verify isolation
        assert_eq!(store1.task_count(), 1);
        assert_eq!(store2.task_count(), 1);

        let alice_tasks = store1.query_by_username("alice");
        assert_eq!(alice_tasks.len(), 1);

        let bob_tasks = store2.query_by_username("bob");
        assert_eq!(bob_tasks.len(), 1);

        // Cross queries should be empty
        assert_eq!(store1.query_by_username("bob").len(), 0);
        assert_eq!(store2.query_by_username("alice").len(), 0);
    }
}
