//! Empty user log store that discards all messages.

use crate::core::utils::progress::Task;
use crate::core::utils::warnings::{UserLogEntry, UserLogStore};

/// A no-op implementation of UserLogStore that discards all messages.
///
/// This is used for testing and scenarios where logging is disabled.
/// All operations are no-ops and queries return empty results.
#[derive(Debug, Clone, Copy)]
pub struct EmptyUserLogStore;

impl EmptyUserLogStore {
    /// Creates a new empty user log store instance.
    pub const fn new() -> Self {
        Self
    }

    /// Returns a static reference to a shared empty store instance.
    pub fn instance() -> &'static Self {
        &EMPTY_USER_LOG_STORE_INSTANCE
    }
}

impl Default for EmptyUserLogStore {
    fn default() -> Self {
        Self::new()
    }
}

impl UserLogStore for EmptyUserLogStore {
    fn add_user_log_message(&self, _username: &str, _task: &Task, _message: String) {
        // No-op: discard the message
    }

    fn query(&self, _username: &str) -> Vec<UserLogEntry> {
        // Return empty vector
        Vec::new()
    }
}

/// Singleton instance of EmptyUserLogStore.
pub static EMPTY_USER_LOG_STORE_INSTANCE: EmptyUserLogStore = EmptyUserLogStore;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_store_discards_messages() {
        let store = EmptyUserLogStore::new();
        let task = Task::new("Test".to_string(), 100);

        store.add_user_log_message("user1", &task, "message".to_string());

        let entries = store.query("user1");
        assert!(entries.is_empty());
    }

    #[test]
    fn test_empty_store_instance() {
        let store = EmptyUserLogStore::instance();
        let task = Task::new("Test".to_string(), 100);

        store.add_user_log_message("user1", &task, "message".to_string());

        let entries = store.query("user1");
        assert!(entries.is_empty());
    }
}
