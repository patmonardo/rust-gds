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

//! Application-level service for managing TaskStores.
//!
//! This class should hold all TaskStores for the application.
//! Therefore, it should be a singleton. You instantiate it once as part of assembling the application.
//! TaskStores are tied to databases and live for the lifetime of a database.

use crate::core::utils::progress::{EmptyTaskStore, TaskStore, TaskStoreHolder};
use std::sync::Arc;

/// Application-level service for managing TaskStores.
///
/// This service provides a centralized interface for accessing TaskStores
/// across the application. It can be configured to enable or disable
/// progress tracking globally.
///
/// # Design Notes
///
/// This is intended to be a singleton - instantiate once during application startup.
/// TaskStores are tied to databases and persist for the database lifetime.
///
/// # Examples
///
/// ```
/// use rust_gds::core::utils::progress::*;
///
/// // Create service with progress tracking enabled
/// let service = TaskStoreService::new(true);
///
/// // Get store for a database
/// let store = service.get_task_store("neo4j");
///
/// // Use the store
/// let job_id = JobId::new();
/// let task = Task::new("Process".to_string(), 1000);
/// store.store("alice".to_string(), job_id, task);
///
/// // Clean up when done
/// service.purge_database("neo4j");
/// ```
pub struct TaskStoreService {
    progress_tracking_enabled: bool,
}

impl TaskStoreService {
    /// Create a new TaskStoreService.
    ///
    /// # Arguments
    ///
    /// * `progress_tracking_enabled` - Whether progress tracking is enabled globally
    pub fn new(progress_tracking_enabled: bool) -> Self {
        Self {
            progress_tracking_enabled,
        }
    }

    /// Get TaskStore for the given database.
    ///
    /// Returns EmptyTaskStore if progress tracking is disabled,
    /// otherwise returns the actual store from TaskStoreHolder.
    pub fn get_task_store(&self, database_name: &str) -> Arc<dyn TaskStore> {
        if !self.progress_tracking_enabled {
            Arc::new(EmptyTaskStore)
        } else {
            #[allow(deprecated)]
            TaskStoreHolder::get_task_store(database_name)
        }
    }

    /// Check if progress tracking is enabled.
    pub fn is_progress_tracking_enabled(&self) -> bool {
        self.progress_tracking_enabled
    }

    /// Get all database names that have TaskStores.
    ///
    /// Returns an empty vector if progress tracking is disabled.
    pub fn database_names(&self) -> Vec<String> {
        if !self.progress_tracking_enabled {
            Vec::new()
        } else {
            #[allow(deprecated)]
            TaskStoreHolder::database_names()
        }
    }

    /// Get total number of databases with TaskStores.
    ///
    /// Returns 0 if progress tracking is disabled.
    pub fn database_count(&self) -> usize {
        if !self.progress_tracking_enabled {
            0
        } else {
            #[allow(deprecated)]
            TaskStoreHolder::size()
        }
    }

    /// Clean up TaskStore for a specific database.
    ///
    /// No-op if progress tracking is disabled.
    pub fn purge_database(&self, database_name: &str) {
        if self.progress_tracking_enabled {
            #[allow(deprecated)]
            TaskStoreHolder::purge(database_name);
        }
    }

    /// Clean up all TaskStores (useful for testing/shutdown).
    ///
    /// No-op if progress tracking is disabled.
    pub fn purge_all(&self) {
        if self.progress_tracking_enabled {
            #[allow(deprecated)]
            TaskStoreHolder::clear();
        }
    }
}

impl Default for TaskStoreService {
    /// Create service with progress tracking enabled by default.
    fn default() -> Self {
        Self::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::progress::{JobId, Task};

    #[test]
    fn test_create_service_enabled() {
        let service = TaskStoreService::new(true);
        assert!(service.is_progress_tracking_enabled());
    }

    #[test]
    fn test_create_service_disabled() {
        let service = TaskStoreService::new(false);
        assert!(!service.is_progress_tracking_enabled());
    }

    #[test]
    fn test_default_service() {
        let service = TaskStoreService::default();
        assert!(service.is_progress_tracking_enabled());
    }

    #[test]
    fn test_get_task_store_enabled() {
        let db_name = "test_service_enabled";

        #[allow(deprecated)]
        TaskStoreHolder::purge(db_name);

        let service = TaskStoreService::new(true);
        let store = service.get_task_store(db_name);

        // Should be a real store
        let job_id = JobId::new();
        let task = Task::new("Test".to_string(), 100);
        store.store("alice".to_string(), job_id.clone(), task);

        assert_eq!(store.task_count(), 1);

        // Should be able to query it back
        let result = store.query("alice", &job_id);
        assert!(result.is_some());
    }

    #[test]
    fn test_get_task_store_disabled() {
        let service = TaskStoreService::new(false);
        let store = service.get_task_store("test_db");

        // Should be EmptyTaskStore
        let job_id = JobId::new();
        let task = Task::new("Test".to_string(), 100);
        store.store("alice".to_string(), job_id.clone(), task);

        // EmptyTaskStore ignores everything
        assert_eq!(store.task_count(), 0);
        assert!(store.query("alice", &job_id).is_none());
    }

    #[test]
    fn test_database_names_enabled() {
        let db1 = "test_service_names_db1";
        let db2 = "test_service_names_db2";

        #[allow(deprecated)]
        {
            TaskStoreHolder::purge(db1);
            TaskStoreHolder::purge(db2);
        }

        let service = TaskStoreService::new(true);
        service.get_task_store(db1);
        service.get_task_store(db2);

        let names = service.database_names();
        assert!(names.contains(&db1.to_string()));
        assert!(names.contains(&db2.to_string()));
    }

    #[test]
    fn test_database_names_disabled() {
        let service = TaskStoreService::new(false);
        service.get_task_store("db1");
        service.get_task_store("db2");

        // Should return empty even though we called get_task_store
        let names = service.database_names();
        assert_eq!(names.len(), 0);
    }

    #[test]
    fn test_database_count_enabled() {
        let db1 = "test_service_count_db1";
        let db2 = "test_service_count_db2";
        let db3 = "test_service_count_db3";

        #[allow(deprecated)]
        {
            TaskStoreHolder::purge(db1);
            TaskStoreHolder::purge(db2);
            TaskStoreHolder::purge(db3);
        }

        let service = TaskStoreService::new(true);
        service.get_task_store(db1);
        service.get_task_store(db2);
        service.get_task_store(db3);

        let names = service.database_names();
        assert!(names.contains(&db1.to_string()));
        assert!(names.contains(&db2.to_string()));
        assert!(names.contains(&db3.to_string()));
    }

    #[test]
    fn test_database_count_disabled() {
        let service = TaskStoreService::new(false);
        service.get_task_store("db1");
        service.get_task_store("db2");

        assert_eq!(service.database_count(), 0);
    }

    #[test]
    fn test_purge_database_enabled() {
        let db1 = "test_service_purge_db1";
        let db2 = "test_service_purge_db2";

        #[allow(deprecated)]
        {
            TaskStoreHolder::purge(db1);
            TaskStoreHolder::purge(db2);
        }

        let service = TaskStoreService::new(true);
        service.get_task_store(db1);
        service.get_task_store(db2);

        let names_before = service.database_names();
        assert!(names_before.contains(&db1.to_string()));
        assert!(names_before.contains(&db2.to_string()));

        service.purge_database(db1);

        let names_after = service.database_names();
        assert!(!names_after.contains(&db1.to_string()));
        assert!(names_after.contains(&db2.to_string()));
    }

    #[test]
    fn test_purge_database_disabled() {
        let service = TaskStoreService::new(false);
        service.get_task_store("db1");

        // Should be no-op
        service.purge_database("db1");
        assert_eq!(service.database_count(), 0);
    }

    #[test]
    fn test_purge_all_enabled() {
        let db1 = "test_service_purge_all_db1";
        let db2 = "test_service_purge_all_db2";
        let db3 = "test_service_purge_all_db3";

        #[allow(deprecated)]
        {
            TaskStoreHolder::purge(db1);
            TaskStoreHolder::purge(db2);
            TaskStoreHolder::purge(db3);
        }

        let service = TaskStoreService::new(true);
        service.get_task_store(db1);
        service.get_task_store(db2);
        service.get_task_store(db3);

        let names_before = service.database_names();
        assert!(names_before.contains(&db1.to_string()));
        assert!(names_before.contains(&db2.to_string()));
        assert!(names_before.contains(&db3.to_string()));

        service.purge_all();

        let names_after = service.database_names();
        assert!(!names_after.contains(&db1.to_string()));
        assert!(!names_after.contains(&db2.to_string()));
        assert!(!names_after.contains(&db3.to_string()));
    }

    #[test]
    fn test_purge_all_disabled() {
        let service = TaskStoreService::new(false);
        service.get_task_store("db1");

        // Should be no-op
        service.purge_all();
        assert_eq!(service.database_count(), 0);
    }

    #[test]
    fn test_same_store_returned() {
        let db_name = "test_service_same_store";

        #[allow(deprecated)]
        TaskStoreHolder::purge(db_name);

        let service = TaskStoreService::new(true);
        let store1 = service.get_task_store(db_name);
        let store2 = service.get_task_store(db_name);

        // Should be the same store
        assert!(Arc::ptr_eq(&store1, &store2));
    }

    #[test]
    fn test_disabled_returns_different_empty_stores() {
        let service = TaskStoreService::new(false);
        let store1 = service.get_task_store("test_db");
        let store2 = service.get_task_store("test_db");

        // EmptyTaskStore is created fresh each time (not cached)
        assert!(!Arc::ptr_eq(&store1, &store2));
    }

    // This test uses global TaskStoreHolder, so must not run in parallel with other tests
    #[test]
    #[ignore] // Run with --ignored to test concurrency without cross-test interference
    fn test_concurrent_access_enabled() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::thread;

        // Use unique name with timestamp to avoid cross-test interference
        let db_name = format!(
            "test_service_concurrent_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );

        #[allow(deprecated)]
        TaskStoreHolder::purge(&db_name);

        let service = Arc::new(TaskStoreService::new(true));
        let stored_count = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        // Pre-create the store to ensure it exists before threads start
        let main_store = service.get_task_store(&db_name);

        // Spawn multiple threads
        for i in 0..10 {
            let service_clone = service.clone();
            let db_clone = db_name.clone();
            let counter = Arc::clone(&stored_count);
            let main_store_clone = Arc::clone(&main_store);
            let handle = thread::spawn(move || {
                let store = service_clone.get_task_store(&db_clone);
                // Verify we got the same store instance
                assert!(
                    Arc::ptr_eq(&store, &main_store_clone),
                    "Thread {} got different store instance!",
                    i
                );
                let job_id = JobId::new();
                let task = Task::new(format!("Task {}", i), 100);
                store.store(format!("user{}", i), job_id, task);
                counter.fetch_add(1, Ordering::SeqCst);
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all threads completed their stores
        assert_eq!(
            stored_count.load(Ordering::SeqCst),
            10,
            "Not all threads completed"
        );

        // Small delay to ensure all writes are visible
        std::thread::sleep(std::time::Duration::from_millis(50));

        // Verify the store has all tasks
        let store = service.get_task_store(&db_name);
        let actual_count = store.task_count();

        // Debug: Check if we're getting the same store
        let store2 = service.get_task_store(&db_name);
        assert!(
            Arc::ptr_eq(&store, &store2),
            "get_task_store returned different instances!"
        );

        assert_eq!(
            actual_count, 10,
            "Expected 10 tasks in store, found {}. Database: {}",
            actual_count, db_name
        );

        // Clean up
        #[allow(deprecated)]
        TaskStoreHolder::purge(&db_name);
    }

    #[test]
    fn test_toggle_behavior() {
        let db_name = "test_service_toggle";

        #[allow(deprecated)]
        TaskStoreHolder::purge(db_name);

        // Start enabled
        let service = TaskStoreService::new(true);
        let store = service.get_task_store(db_name);

        let job_id = JobId::new();
        let task = Task::new("Test".to_string(), 100);
        store.store("alice".to_string(), job_id, task);

        assert_eq!(store.task_count(), 1);

        // Create disabled service (simulating restart with different config)
        let service_disabled = TaskStoreService::new(false);
        let store_disabled = service_disabled.get_task_store(db_name);

        // Should be empty store (new instance)
        assert_eq!(store_disabled.task_count(), 0);
    }
}
