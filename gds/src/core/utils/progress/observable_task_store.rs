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

//! Abstract TaskStore that handles observer pattern for task lifecycle events.
//!
//! Concrete implementations only need to handle storage mechanics by implementing
//! the abstract methods. This base handles all listener management and notifications.

use crate::core::utils::progress::{JobId, Task, TaskStoreListener, UserTask};
use std::sync::{Arc, RwLock};

/// Abstract TaskStore that handles observer pattern for task lifecycle events.
///
/// This base class provides complete listener management and notification logic.
/// Concrete implementations only need to implement the storage mechanics:
/// - `store_user_task()` - Store a task and return the UserTask
/// - `remove_user_task()` - Remove a task and return it if it existed
/// - Query methods from TaskStore trait
///
/// # Example
///
/// ```rust,ignore
/// use rust_gds::core::utils::progress::*;
///
/// pub struct MyTaskStore {
///     base: ObservableTaskStore,
///     // ... your storage fields
/// }
///
/// impl MyTaskStore {
///     pub fn new() -> Self {
///         Self {
///             base: ObservableTaskStore::new(),
///             // ... initialize storage
///         }
///     }
/// }
///
/// impl TaskStore for MyTaskStore {
///     fn store(&self, username: String, job_id: JobId, task: Task) {
///         self.base.store_with_notification(username, job_id, task, |u, j, t| {
///             // Your storage logic here
///             let user_task = UserTask::new(u, j, t);
///             // Store it
///             user_task
///         });
///     }
///     // ... implement other methods
/// }
/// ```
pub struct ObservableTaskStore {
    listeners: RwLock<Vec<Arc<dyn TaskStoreListener>>>,
}

impl ObservableTaskStore {
    /// Create a new ObservableTaskStore with no listeners.
    pub fn new() -> Self {
        Self {
            listeners: RwLock::new(Vec::new()),
        }
    }

    /// Create a new ObservableTaskStore with initial listeners.
    pub fn with_listeners(listeners: Vec<Arc<dyn TaskStoreListener>>) -> Self {
        Self {
            listeners: RwLock::new(listeners),
        }
    }

    /// Store a task and notify listeners.
    ///
    /// The `store_fn` should perform the actual storage and return the UserTask.
    pub fn store_with_notification<F>(
        &self,
        username: String,
        job_id: JobId,
        task: Task,
        store_fn: F,
    ) -> UserTask
    where
        F: FnOnce(String, JobId, Task) -> UserTask,
    {
        let user_task = store_fn(username, job_id, task);
        self.notify_task_added(&user_task);
        user_task
    }

    /// Remove a task and notify listeners if it existed.
    ///
    /// The `remove_fn` should perform the actual removal and return the UserTask if it existed.
    pub fn remove_with_notification<F>(&self, username: &str, job_id: &JobId, remove_fn: F)
    where
        F: FnOnce(&str, &JobId) -> Option<UserTask>,
    {
        if let Some(_user_task) = remove_fn(username, job_id) {
            self.notify_task_removed(username, job_id);
        }
    }

    /// Notify listeners that the store has been cleared.
    pub fn notify_store_cleared(&self) {
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener.on_store_cleared();
        }
    }

    /// Add a listener for task store events.
    pub fn add_listener(&self, listener: Arc<dyn TaskStoreListener>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.push(listener);
    }

    /// Remove a listener.
    pub fn remove_listener(&self, listener: &Arc<dyn TaskStoreListener>) {
        let mut listeners = self.listeners.write().unwrap();
        listeners.retain(|l| !Arc::ptr_eq(l, listener));
    }

    /// Get the number of registered listeners.
    pub fn listener_count(&self) -> usize {
        self.listeners.read().unwrap().len()
    }

    /// Notify all listeners that a task was added.
    fn notify_task_added(&self, user_task: &UserTask) {
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener.on_task_added(user_task);
        }
    }

    /// Notify all listeners that a task was removed.
    fn notify_task_removed(&self, username: &str, job_id: &JobId) {
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener.on_task_removed(username, job_id);
        }
    }
}

impl Default for ObservableTaskStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // Test listener that tracks calls
    struct TestListener {
        added_count: Mutex<usize>,
        removed_count: Mutex<usize>,
        cleared_count: Mutex<usize>,
    }

    impl TestListener {
        fn new() -> Arc<Self> {
            Arc::new(Self {
                added_count: Mutex::new(0),
                removed_count: Mutex::new(0),
                cleared_count: Mutex::new(0),
            })
        }

        fn get_added_count(&self) -> usize {
            *self.added_count.lock().unwrap()
        }

        fn get_removed_count(&self) -> usize {
            *self.removed_count.lock().unwrap()
        }

        fn get_cleared_count(&self) -> usize {
            *self.cleared_count.lock().unwrap()
        }
    }

    impl TaskStoreListener for TestListener {
        fn on_task_added(&self, _user_task: &UserTask) {
            let mut count = self.added_count.lock().unwrap();
            *count += 1;
        }

        fn on_task_removed(&self, _username: &str, _job_id: &JobId) {
            let mut count = self.removed_count.lock().unwrap();
            *count += 1;
        }

        fn on_store_cleared(&self) {
            let mut count = self.cleared_count.lock().unwrap();
            *count += 1;
        }
    }

    #[test]
    fn test_create_empty_store() {
        let store = ObservableTaskStore::new();
        assert_eq!(store.listener_count(), 0);
    }

    #[test]
    fn test_create_with_listeners() {
        let listener = TestListener::new();
        let store = ObservableTaskStore::with_listeners(vec![listener.clone()]);
        assert_eq!(store.listener_count(), 1);
    }

    #[test]
    fn test_add_listener() {
        let store = ObservableTaskStore::new();
        let listener = TestListener::new();

        store.add_listener(listener.clone());
        assert_eq!(store.listener_count(), 1);
    }

    #[test]
    fn test_remove_listener() {
        let store = ObservableTaskStore::new();
        let listener = TestListener::new();
        let listener_trait: Arc<dyn TaskStoreListener> = listener.clone();

        store.add_listener(listener.clone());
        assert_eq!(store.listener_count(), 1);

        store.remove_listener(&listener_trait);
        assert_eq!(store.listener_count(), 0);
    }

    #[test]
    fn test_store_with_notification() {
        let store = ObservableTaskStore::new();
        let listener = TestListener::new();
        store.add_listener(listener.clone());

        let task = Task::new("Test".to_string(), 100);
        let job_id = JobId::new();

        store.store_with_notification(
            "alice".to_string(),
            job_id.clone(),
            task.clone(),
            |username, job_id, task| UserTask::new(username, job_id, task),
        );

        assert_eq!(listener.get_added_count(), 1);
        assert_eq!(listener.get_removed_count(), 0);
    }

    #[test]
    fn test_remove_with_notification_existing() {
        let store = ObservableTaskStore::new();
        let listener = TestListener::new();
        store.add_listener(listener.clone());

        let task = Task::new("Test".to_string(), 100);
        let job_id = JobId::new();

        // Simulate removal of existing task
        store.remove_with_notification("alice", &job_id, |username, job_id| {
            Some(UserTask::new(
                username.to_string(),
                job_id.clone(),
                task.clone(),
            ))
        });

        assert_eq!(listener.get_removed_count(), 1);
    }

    #[test]
    fn test_remove_with_notification_nonexistent() {
        let store = ObservableTaskStore::new();
        let listener = TestListener::new();
        store.add_listener(listener.clone());

        let job_id = JobId::new();

        // Simulate removal of non-existent task
        store.remove_with_notification("alice", &job_id, |_username, _job_id| None);

        assert_eq!(listener.get_removed_count(), 0);
    }

    #[test]
    fn test_notify_store_cleared() {
        let store = ObservableTaskStore::new();
        let listener = TestListener::new();
        store.add_listener(listener.clone());

        store.notify_store_cleared();

        assert_eq!(listener.get_cleared_count(), 1);
    }

    #[test]
    fn test_multiple_listeners() {
        let store = ObservableTaskStore::new();
        let listener1 = TestListener::new();
        let listener2 = TestListener::new();

        store.add_listener(listener1.clone());
        store.add_listener(listener2.clone());

        let task = Task::new("Test".to_string(), 100);
        let job_id = JobId::new();

        store.store_with_notification(
            "alice".to_string(),
            job_id,
            task,
            |username, job_id, task| UserTask::new(username, job_id, task),
        );

        assert_eq!(listener1.get_added_count(), 1);
        assert_eq!(listener2.get_added_count(), 1);
    }

    #[test]
    fn test_default_implementation() {
        let store = ObservableTaskStore::default();
        assert_eq!(store.listener_count(), 0);
    }
}
