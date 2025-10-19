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

//! TaskStore implementation for a specific database.
//!
//! Each database gets its own isolated task storage with thread-safe concurrent access.

use crate::core::utils::progress::{
    JobId, ObservableTaskStore, Task, TaskStore, TaskStoreListener, UserTask,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// TaskStore implementation for a specific database.
///
/// Each database gets its own isolated task storage. Tasks are stored in a
/// nested map: username -> job_id -> UserTask.
///
/// # Thread Safety
///
/// This implementation is thread-safe using RwLock for concurrent access.
/// Multiple readers can query simultaneously, while writes are exclusive.
///
/// # Example
///
/// ```
/// use gds::core::utils::progress::*;
///
/// let store = PerDatabaseTaskStore::new();
///
/// let job_id = JobId::new();
/// let task = Task::new("Process data".to_string(), 1000);
///
/// store.store("alice".to_string(), job_id.clone(), task);
///
/// assert_eq!(store.task_count(), 1);
/// assert!(!store.is_empty());
///
/// let tasks = store.query_by_username("alice");
/// assert_eq!(tasks.len(), 1);
/// ```
pub struct PerDatabaseTaskStore {
    tasks: RwLock<HashMap<String, HashMap<JobId, UserTask>>>,
    observable: ObservableTaskStore,
}

impl PerDatabaseTaskStore {
    /// Create a new empty PerDatabaseTaskStore.
    pub fn new() -> Self {
        Self {
            tasks: RwLock::new(HashMap::new()),
            observable: ObservableTaskStore::new(),
        }
    }

    /// Create a new PerDatabaseTaskStore with initial listeners.
    pub fn with_listeners(listeners: Vec<Arc<dyn TaskStoreListener>>) -> Self {
        Self {
            tasks: RwLock::new(HashMap::new()),
            observable: ObservableTaskStore::with_listeners(listeners),
        }
    }

    /// Clear all tasks from the store.
    ///
    /// This removes all tasks for all users and notifies listeners.
    pub fn clear(&self) {
        let mut tasks = self.tasks.write().unwrap();
        tasks.clear();
        self.observable.notify_store_cleared();
    }

    /// Generate unique key for username + job_id combination.
    #[allow(dead_code)] // Reserved for future use
    fn generate_key(username: &str, job_id: &JobId) -> String {
        format!("{}:{}", username, job_id.as_string())
    }
}

impl Default for PerDatabaseTaskStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskStore for PerDatabaseTaskStore {
    fn store(&self, username: String, job_id: JobId, task: Task) {
        let user_task = self.observable.store_with_notification(
            username.clone(),
            job_id.clone(),
            task,
            |username, job_id, task| {
                let user_task = UserTask::new(username.clone(), job_id.clone(), task);

                // Store in nested map
                let mut tasks = self.tasks.write().unwrap();
                let user_tasks = tasks.entry(username).or_default();
                user_tasks.insert(job_id, user_task.clone());

                user_task
            },
        );

        // Make sure we're actually storing it (user_task is used)
        let _ = user_task;
    }

    fn remove(&self, username: &str, job_id: &JobId) {
        self.observable
            .remove_with_notification(username, job_id, |username, job_id| {
                let mut tasks = self.tasks.write().unwrap();

                if let Some(user_tasks) = tasks.get_mut(username) {
                    let removed = user_tasks.remove(job_id);

                    // Clean up empty user entry
                    if user_tasks.is_empty() {
                        tasks.remove(username);
                    }

                    removed
                } else {
                    None
                }
            });
    }

    fn query_all(&self) -> Vec<UserTask> {
        let tasks = self.tasks.read().unwrap();
        tasks
            .values()
            .flat_map(|user_tasks| user_tasks.values().cloned())
            .collect()
    }

    fn query_by_job_id(&self, job_id: &JobId) -> Vec<UserTask> {
        let tasks = self.tasks.read().unwrap();
        tasks
            .values()
            .filter_map(|user_tasks| user_tasks.get(job_id).cloned())
            .collect()
    }

    fn query_by_username(&self, username: &str) -> Vec<UserTask> {
        let tasks = self.tasks.read().unwrap();
        tasks
            .get(username)
            .map(|user_tasks| user_tasks.values().cloned().collect())
            .unwrap_or_default()
    }

    fn query(&self, username: &str, job_id: &JobId) -> Option<UserTask> {
        let tasks = self.tasks.read().unwrap();
        tasks
            .get(username)
            .and_then(|user_tasks| user_tasks.get(job_id).cloned())
    }

    fn is_empty(&self) -> bool {
        let tasks = self.tasks.read().unwrap();
        tasks.is_empty()
    }

    fn task_count(&self) -> usize {
        let tasks = self.tasks.read().unwrap();
        tasks.values().map(|user_tasks| user_tasks.len()).sum()
    }

    fn add_listener(&self, listener: Box<dyn TaskStoreListener>) {
        self.observable.add_listener(Arc::from(listener));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // Test listener that tracks calls
    struct TestListener {
        added: Mutex<Vec<String>>,
        removed: Mutex<Vec<String>>,
        cleared_count: Mutex<usize>,
    }

    impl TestListener {
        fn new() -> Arc<Self> {
            Arc::new(Self {
                added: Mutex::new(Vec::new()),
                removed: Mutex::new(Vec::new()),
                cleared_count: Mutex::new(0),
            })
        }

        fn added_count(&self) -> usize {
            self.added.lock().unwrap().len()
        }

        fn removed_count(&self) -> usize {
            self.removed.lock().unwrap().len()
        }

        fn cleared_count(&self) -> usize {
            *self.cleared_count.lock().unwrap()
        }
    }

    impl TaskStoreListener for TestListener {
        fn on_task_added(&self, user_task: &UserTask) {
            self.added.lock().unwrap().push(user_task.username.clone());
        }

        fn on_task_removed(&self, username: &str, _job_id: &JobId) {
            self.removed.lock().unwrap().push(username.to_string());
        }

        fn on_store_cleared(&self) {
            *self.cleared_count.lock().unwrap() += 1;
        }
    }

    #[test]
    fn test_create_empty_store() {
        let store = PerDatabaseTaskStore::new();
        assert!(store.is_empty());
        assert_eq!(store.task_count(), 0);
    }

    #[test]
    fn test_store_and_query() {
        let store = PerDatabaseTaskStore::new();
        let job_id = JobId::new();
        let task = Task::new("Test task".to_string(), 100);

        store.store("alice".to_string(), job_id.clone(), task.clone());

        assert_eq!(store.task_count(), 1);
        assert!(!store.is_empty());

        let result = store.query("alice", &job_id);
        assert!(result.is_some());

        let user_task = result.unwrap();
        assert_eq!(user_task.username, "alice");
        assert_eq!(user_task.task.description, "Test task");
    }

    #[test]
    fn test_store_multiple_users() {
        let store = PerDatabaseTaskStore::new();
        let job_id1 = JobId::new();
        let job_id2 = JobId::new();
        let task1 = Task::new("Task 1".to_string(), 100);
        let task2 = Task::new("Task 2".to_string(), 200);

        store.store("alice".to_string(), job_id1.clone(), task1);
        store.store("bob".to_string(), job_id2.clone(), task2);

        assert_eq!(store.task_count(), 2);

        let alice_tasks = store.query_by_username("alice");
        assert_eq!(alice_tasks.len(), 1);

        let bob_tasks = store.query_by_username("bob");
        assert_eq!(bob_tasks.len(), 1);
    }

    #[test]
    fn test_store_multiple_jobs_same_user() {
        let store = PerDatabaseTaskStore::new();
        let job_id1 = JobId::new();
        let job_id2 = JobId::new();
        let task1 = Task::new("Task 1".to_string(), 100);
        let task2 = Task::new("Task 2".to_string(), 200);

        store.store("alice".to_string(), job_id1, task1);
        store.store("alice".to_string(), job_id2, task2);

        assert_eq!(store.task_count(), 2);

        let tasks = store.query_by_username("alice");
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn test_remove_task() {
        let store = PerDatabaseTaskStore::new();
        let job_id = JobId::new();
        let task = Task::new("Test task".to_string(), 100);

        store.store("alice".to_string(), job_id.clone(), task);
        assert_eq!(store.task_count(), 1);

        store.remove("alice", &job_id);
        assert_eq!(store.task_count(), 0);
        assert!(store.is_empty());
    }

    #[test]
    fn test_remove_nonexistent_task() {
        let store = PerDatabaseTaskStore::new();
        let job_id = JobId::new();

        // Should not panic
        store.remove("alice", &job_id);
        assert!(store.is_empty());
    }

    #[test]
    fn test_query_all() {
        let store = PerDatabaseTaskStore::new();
        let job_id1 = JobId::new();
        let job_id2 = JobId::new();
        let task1 = Task::new("Task 1".to_string(), 100);
        let task2 = Task::new("Task 2".to_string(), 200);

        store.store("alice".to_string(), job_id1, task1);
        store.store("bob".to_string(), job_id2, task2);

        let all_tasks = store.query_all();
        assert_eq!(all_tasks.len(), 2);
    }

    #[test]
    fn test_query_by_job_id() {
        let store = PerDatabaseTaskStore::new();
        let job_id = JobId::new();
        let task = Task::new("Test task".to_string(), 100);

        store.store("alice".to_string(), job_id.clone(), task.clone());
        store.store("bob".to_string(), job_id.clone(), task);

        let tasks = store.query_by_job_id(&job_id);
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn test_query_by_username() {
        let store = PerDatabaseTaskStore::new();
        let job_id1 = JobId::new();
        let job_id2 = JobId::new();
        let task = Task::new("Test task".to_string(), 100);

        store.store("alice".to_string(), job_id1, task.clone());
        store.store("alice".to_string(), job_id2, task);

        let tasks = store.query_by_username("alice");
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn test_query_nonexistent_user() {
        let store = PerDatabaseTaskStore::new();
        let tasks = store.query_by_username("nonexistent");
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_clear() {
        let store = PerDatabaseTaskStore::new();
        let job_id = JobId::new();
        let task = Task::new("Test task".to_string(), 100);

        store.store("alice".to_string(), job_id, task);
        assert_eq!(store.task_count(), 1);

        store.clear();
        assert!(store.is_empty());
        assert_eq!(store.task_count(), 0);
    }

    #[test]
    fn test_listener_on_store() {
        let listener = TestListener::new();
        let store = PerDatabaseTaskStore::with_listeners(vec![listener.clone()]);

        let job_id = JobId::new();
        let task = Task::new("Test task".to_string(), 100);

        store.store("alice".to_string(), job_id, task);

        assert_eq!(listener.added_count(), 1);
    }

    #[test]
    fn test_listener_on_remove() {
        let listener = TestListener::new();
        let store = PerDatabaseTaskStore::with_listeners(vec![listener.clone()]);

        let job_id = JobId::new();
        let task = Task::new("Test task".to_string(), 100);

        store.store("alice".to_string(), job_id.clone(), task);
        store.remove("alice", &job_id);

        assert_eq!(listener.removed_count(), 1);
    }

    #[test]
    fn test_listener_on_clear() {
        let listener = TestListener::new();
        let store = PerDatabaseTaskStore::with_listeners(vec![listener.clone()]);

        let job_id = JobId::new();
        let task = Task::new("Test task".to_string(), 100);

        store.store("alice".to_string(), job_id, task);
        store.clear();

        assert_eq!(listener.cleared_count(), 1);
    }

    #[test]
    fn test_default_implementation() {
        let store = PerDatabaseTaskStore::default();
        assert!(store.is_empty());
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;

        let store = Arc::new(PerDatabaseTaskStore::new());
        let mut handles = vec![];

        // Spawn multiple threads to store tasks
        for i in 0..10 {
            let store_clone = store.clone();
            let handle = thread::spawn(move || {
                let job_id = JobId::new();
                let task = Task::new(format!("Task {}", i), 100);
                store_clone.store(format!("user{}", i), job_id, task);
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(store.task_count(), 10);
    }
}
