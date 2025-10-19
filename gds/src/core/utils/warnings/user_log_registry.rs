//! User log registry for adding warnings to the log store.

use crate::core::utils::progress::Task;
use crate::core::utils::warnings::UserLogStore;

/// Registry for logging user warnings associated with tasks.
///
/// Provides a convenient interface for adding warning messages
/// that are automatically associated with a specific user.
pub struct UserLogRegistry {
    username: String,
    user_log_store: Box<dyn UserLogStore>,
}

impl UserLogRegistry {
    /// Creates a new user log registry.
    ///
    /// # Arguments
    ///
    /// * `username` - The username to associate with all logged warnings
    /// * `user_log_store` - The store to write warnings to
    pub fn new(username: String, user_log_store: Box<dyn UserLogStore>) -> Self {
        Self {
            username,
            user_log_store,
        }
    }

    /// Adds a warning message to the log.
    ///
    /// The warning is associated with the configured username and the provided task.
    ///
    /// # Arguments
    ///
    /// * `task` - The task related to the warning
    /// * `message` - The warning message
    pub fn add_warning_to_log(&self, task: &Task, message: String) {
        self.user_log_store
            .add_user_log_message(&self.username, task, message);
    }

    /// Returns the username associated with this registry.
    pub fn username(&self) -> &str {
        &self.username
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::utils::warnings::EmptyUserLogStore;

    #[test]
    fn test_user_log_registry_creation() {
        let store = Box::new(EmptyUserLogStore::new());
        let registry = UserLogRegistry::new("test_user".to_string(), store);

        assert_eq!(registry.username(), "test_user");
    }

    #[test]
    fn test_add_warning_to_log() {
        let store = Box::new(EmptyUserLogStore::new());
        let registry = UserLogRegistry::new("test_user".to_string(), store);
        let task = Task::new("Test Task".to_string(), 100);

        // Should not panic with empty store
        registry.add_warning_to_log(&task, "Warning message".to_string());
    }
}
