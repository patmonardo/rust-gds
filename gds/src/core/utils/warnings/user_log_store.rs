//! User log store interface for tracking user messages per task.

use crate::core::utils::progress::Task;
use crate::core::utils::warnings::UserLogEntry;

/// Interface for storing and querying user log messages.
///
/// Stores log messages on a per-user, per-task basis and provides
/// querying capabilities to retrieve logged entries.
pub trait UserLogStore: Send + Sync {
    /// Adds a log message for a specific user and task.
    ///
    /// # Arguments
    ///
    /// * `username` - The username associated with the log message
    /// * `task` - The task related to the message
    /// * `message` - The log message content
    fn add_user_log_message(&self, username: &str, task: &Task, message: String);

    /// Queries all log entries for a specific user.
    ///
    /// Returns a vector of all log entries associated with the user,
    /// ordered by task start time.
    ///
    /// # Arguments
    ///
    /// * `username` - The username to query logs for
    fn query(&self, username: &str) -> Vec<UserLogEntry>;
}
