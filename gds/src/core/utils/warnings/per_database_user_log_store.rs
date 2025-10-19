//! Per-database user log store implementation.

use crate::core::utils::progress::Task;
use crate::core::utils::warnings::{log_store::LogStore, UserLogEntry, UserLogStore};
use std::collections::HashMap;
use std::sync::RwLock;

/// User log store that maintains separate log stores per user.
///
/// This implementation is thread-safe and maintains separate
/// LogStore instances for each username, allowing concurrent
/// access from multiple users.
pub struct PerDatabaseUserLogStore {
    /// Map of username to their log store
    log_stores: RwLock<HashMap<String, LogStore>>,
}

impl PerDatabaseUserLogStore {
    /// Creates a new per-database user log store.
    pub fn new() -> Self {
        Self {
            log_stores: RwLock::new(HashMap::new()),
        }
    }

    /// Gets or creates a log store for the specified user.
    #[allow(dead_code)]
    fn get_user_log_store(&self, username: &str) -> LogStore {
        // First try read lock to see if it exists
        {
            let stores = self.log_stores.read().unwrap();
            if let Some(_store) = stores.get(username) {
                // Clone the store for independent mutation
                return LogStore::with_capacity(100); // Will be replaced below
            }
        }

        // Need write lock to create new store
        let mut stores = self.log_stores.write().unwrap();
        // Double-check after acquiring write lock
        stores.entry(username.to_string()).or_default();

        // Return a new LogStore - note: this is a limitation of our simple implementation
        // In a real system, we'd want to return a reference or use Arc
        LogStore::new()
    }

    /// Adds a log message, handling the store access internally.
    fn add_log_message_internal(&self, username: &str, task: &Task, message: String) {
        let mut stores = self.log_stores.write().unwrap();
        let log_store = stores.entry(username.to_string()).or_default();
        log_store.add_log_message(task, message);
    }

    /// Queries log entries, handling the store access internally.
    fn query_internal(&self, username: &str) -> Vec<UserLogEntry> {
        let stores = self.log_stores.read().unwrap();

        if let Some(log_store) = stores.get(username) {
            // Convert log store entries to user log entries
            log_store
                .stream()
                .into_iter()
                .flat_map(|(task_key, messages)| {
                    // Recreate task from key for UserLogEntry
                    // Note: We only have start_time and description from the key
                    messages.iter().map(move |message| {
                        let task = Task::new(task_key.description.clone(), 0);
                        UserLogEntry::new(&task, message.clone())
                    })
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for PerDatabaseUserLogStore {
    fn default() -> Self {
        Self::new()
    }
}

impl UserLogStore for PerDatabaseUserLogStore {
    fn add_user_log_message(&self, username: &str, task: &Task, message: String) {
        self.add_log_message_internal(username, task, message);
    }

    fn query(&self, username: &str) -> Vec<UserLogEntry> {
        self.query_internal(username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_per_database_store_creation() {
        let store = PerDatabaseUserLogStore::new();
        let entries = store.query("user1");
        assert!(entries.is_empty());
    }

    #[test]
    fn test_add_message_for_user() {
        let store = PerDatabaseUserLogStore::new();
        let task = Task::new("Task 1".to_string(), 100);

        store.add_user_log_message("user1", &task, "Message 1".to_string());

        let entries = store.query("user1");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].message(), "Message 1");
    }

    #[test]
    fn test_separate_users() {
        let store = PerDatabaseUserLogStore::new();
        let task1 = Task::new("Task 1".to_string(), 100);
        let task2 = Task::new("Task 2".to_string(), 200);

        store.add_user_log_message("user1", &task1, "User 1 message".to_string());
        store.add_user_log_message("user2", &task2, "User 2 message".to_string());

        let entries1 = store.query("user1");
        let entries2 = store.query("user2");

        assert_eq!(entries1.len(), 1);
        assert_eq!(entries2.len(), 1);
        assert_eq!(entries1[0].message(), "User 1 message");
        assert_eq!(entries2[0].message(), "User 2 message");
    }

    #[test]
    fn test_multiple_messages_per_user() {
        let store = PerDatabaseUserLogStore::new();
        let task = Task::new("Task 1".to_string(), 100);

        store.add_user_log_message("user1", &task, "Message 1".to_string());
        store.add_user_log_message("user1", &task, "Message 2".to_string());
        store.add_user_log_message("user1", &task, "Message 3".to_string());

        let entries = store.query("user1");
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn test_query_nonexistent_user() {
        let store = PerDatabaseUserLogStore::new();
        let entries = store.query("nonexistent");
        assert!(entries.is_empty());
    }

    #[test]
    fn test_multiple_tasks_per_user() {
        let store = PerDatabaseUserLogStore::new();
        let task1 = Task::new("Task 1".to_string(), 100);
        let task2 = Task::new("Task 2".to_string(), 200);

        store.add_user_log_message("user1", &task1, "Task 1 message".to_string());
        store.add_user_log_message("user1", &task2, "Task 2 message".to_string());

        let entries = store.query("user1");
        assert_eq!(entries.len(), 2);
    }
}
