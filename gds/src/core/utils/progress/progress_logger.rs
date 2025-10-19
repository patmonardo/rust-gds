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

//! Progress logging abstraction for algorithms and long-running operations.

/// Message factory type - function that produces optional log messages.
pub type MessageFactory = fn() -> Option<String>;

/// No-op message factory that always returns None.
pub const NO_MESSAGE: MessageFactory = || None;

/// Progress logger trait with default implementations for common patterns.
///
/// This trait provides a flexible interface for logging progress during
/// long-running operations. It supports:
/// - Task hierarchy with subtasks
/// - Progress tracking (absolute and incremental)
/// - Various log levels (message, debug, warning, error)
/// - Start/finish lifecycle management
///
/// # Design
///
/// Based on Java GDS ProgressLogger interface, translated to idiomatic Rust:
/// - Abstract methods must be implemented by concrete loggers
/// - Default methods provide convenience patterns (start, finish, subtasks)
/// - MessageFactory is a function pointer for lazy message generation
///
/// # Example
///
/// ```rust,ignore
/// use gds::core::utils::progress::*;
///
/// struct MyLogger { /* ... */ }
///
/// impl ProgressLogger for MyLogger {
///     fn get_task(&self) -> &str { &self.task }
///     fn set_task(&mut self, task: String) { self.task = task; }
///     fn log_progress_with_message(&mut self, progress: i64, msg_factory: MessageFactory) {
///         // Implementation
///     }
///     // ... other required methods
/// }
///
/// let mut logger = MyLogger::new();
/// logger.log_start();
/// logger.log_progress(100);
/// logger.log_finish();
/// ```
pub trait ProgressLogger: Send {
    /// Task separator for hierarchical task names.
    const TASK_SEPARATOR: &'static str = " :: ";

    // ==================== Abstract Methods ====================
    // Must be implemented by concrete types

    /// Get the current task name.
    fn get_task(&self) -> &str;

    /// Set the current task name.
    fn set_task(&mut self, task: String);

    /// Log progress with an optional message.
    ///
    /// # Parameters
    /// - `progress`: Amount of progress to log
    /// - `msg_factory`: Factory function to generate log message (lazy evaluation)
    fn log_progress_with_message(&mut self, progress: i64, msg_factory: MessageFactory);

    /// Log a message at INFO level.
    fn log_message(&mut self, msg: &str);

    /// Log a message at DEBUG level.
    fn log_debug(&mut self, msg: &str);

    /// Log a warning message.
    fn log_warning(&mut self, msg: &str);

    /// Log an error message.
    fn log_error(&mut self, msg: &str);

    /// Log finish percentage (typically 100%).
    fn log_finish_percentage(&mut self);

    /// Reset logger with new task volume.
    ///
    /// Returns the previous task volume.
    fn reset(&mut self, new_task_volume: i64) -> i64;

    /// Release any resources held by the logger.
    fn release(&mut self);

    // ==================== Default Implementations ====================
    // Convenience methods with sensible defaults

    /// Log progress without a message.
    fn log_progress(&mut self) {
        self.log_progress_with_message(1, NO_MESSAGE);
    }

    /// Log specific amount of progress without a message.
    fn log_progress_amount(&mut self, progress: i64) {
        self.log_progress_with_message(progress, NO_MESSAGE);
    }

    /// Log start of task with optional message.
    ///
    /// # Parameters
    /// - `message`: Optional prefix message (empty string for none)
    fn log_start(&mut self, message: &str) {
        let msg = if message.is_empty() {
            "Start".to_string()
        } else {
            format!("{}{}{}", message, Self::TASK_SEPARATOR, "Start")
        };
        self.log_message(&msg);
    }

    /// Log start of task without prefix message.
    fn log_start_default(&mut self) {
        self.log_start("");
    }

    /// Log successful finish with optional message.
    ///
    /// # Parameters
    /// - `message`: Optional prefix message (empty string for none)
    fn log_finish(&mut self, message: &str) {
        let msg = if message.is_empty() {
            "Finished".to_string()
        } else {
            format!("{}{}{}", message, Self::TASK_SEPARATOR, "Finished")
        };
        self.log_message(&msg);
    }

    /// Log successful finish without prefix message.
    fn log_finish_default(&mut self) {
        self.log_finish("");
    }

    /// Log failed finish with optional message.
    ///
    /// # Parameters
    /// - `message`: Optional prefix message (empty string for none)
    fn log_finish_with_failure(&mut self, message: &str) {
        let msg = if message.is_empty() {
            "Failed".to_string()
        } else {
            format!("{}{}{}", message, Self::TASK_SEPARATOR, "Failed")
        };
        self.log_message(&msg);
    }

    /// Log failed finish without prefix message.
    fn log_finish_with_failure_default(&mut self) {
        self.log_finish_with_failure("");
    }

    /// Log failed finish of a subtask.
    ///
    /// # Parameters
    /// - `subtask_name`: Name of the failed subtask
    fn log_finish_subtask_with_failure(&mut self, subtask_name: &str) {
        let current_task = self.get_task().to_string();

        // Navigate back up the task hierarchy
        if let Some(parent_pos) = current_task.rfind(Self::TASK_SEPARATOR) {
            let parent_task = &current_task[..parent_pos];
            self.set_task(parent_task.to_string());
        }

        let msg = format!("{}{}{}", subtask_name, Self::TASK_SEPARATOR, "Failed");
        self.log_message(&msg);
    }

    /// Start a subtask.
    ///
    /// Appends the subtask name to the current task and logs start.
    ///
    /// # Parameters
    /// - `subtask_name`: Name of the subtask to start
    fn start_subtask(&mut self, subtask_name: &str) {
        let current_task = self.get_task().to_string();
        let new_task = format!("{}{}{}", current_task, Self::TASK_SEPARATOR, subtask_name);
        self.set_task(new_task);
        self.log_start_default();
    }

    /// Finish a subtask.
    ///
    /// Logs finish and navigates back up the task hierarchy.
    ///
    /// # Parameters
    /// - `_subtask_name`: Name of the subtask to finish (unused, kept for API consistency)
    fn finish_subtask(&mut self, _subtask_name: &str) {
        self.log_finish_default();

        let current_task = self.get_task().to_string();

        // Navigate back up the task hierarchy
        if let Some(parent_pos) = current_task.rfind(Self::TASK_SEPARATOR) {
            let parent_task = &current_task[..parent_pos];
            self.set_task(parent_task.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// Test logger that captures log messages for verification.
    struct TestLogger {
        task: String,
        messages: Arc<Mutex<Vec<String>>>,
        progress_calls: Arc<Mutex<Vec<i64>>>,
    }

    impl TestLogger {
        fn new() -> Self {
            Self {
                task: String::new(),
                messages: Arc::new(Mutex::new(Vec::new())),
                progress_calls: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn get_messages(&self) -> Vec<String> {
            self.messages.lock().unwrap().clone()
        }

        fn get_progress_calls(&self) -> Vec<i64> {
            self.progress_calls.lock().unwrap().clone()
        }
    }

    impl ProgressLogger for TestLogger {
        fn get_task(&self) -> &str {
            &self.task
        }

        fn set_task(&mut self, task: String) {
            self.task = task;
        }

        fn log_progress_with_message(&mut self, progress: i64, _msg_factory: MessageFactory) {
            self.progress_calls.lock().unwrap().push(progress);
        }

        fn log_message(&mut self, msg: &str) {
            self.messages
                .lock()
                .unwrap()
                .push(format!("[INFO] {}", msg));
        }

        fn log_debug(&mut self, msg: &str) {
            self.messages
                .lock()
                .unwrap()
                .push(format!("[DEBUG] {}", msg));
        }

        fn log_warning(&mut self, msg: &str) {
            self.messages
                .lock()
                .unwrap()
                .push(format!("[WARN] {}", msg));
        }

        fn log_error(&mut self, msg: &str) {
            self.messages
                .lock()
                .unwrap()
                .push(format!("[ERROR] {}", msg));
        }

        fn log_finish_percentage(&mut self) {
            self.messages
                .lock()
                .unwrap()
                .push("[INFO] 100%".to_string());
        }

        fn reset(&mut self, _new_task_volume: i64) -> i64 {
            let old_volume = 1000; // Dummy value
            old_volume
        }

        fn release(&mut self) {
            // No-op for test logger
        }
    }

    #[test]
    fn test_get_set_task() {
        let mut logger = TestLogger::new();
        logger.set_task("Test Task".to_string());
        assert_eq!(logger.get_task(), "Test Task");
    }

    #[test]
    fn test_log_progress_default() {
        let mut logger = TestLogger::new();
        logger.log_progress();
        assert_eq!(logger.get_progress_calls(), vec![1]);
    }

    #[test]
    fn test_log_progress_amount() {
        let mut logger = TestLogger::new();
        logger.log_progress_amount(42);
        assert_eq!(logger.get_progress_calls(), vec![42]);
    }

    #[test]
    fn test_log_start_default() {
        let mut logger = TestLogger::new();
        logger.log_start_default();
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[INFO] Start");
    }

    #[test]
    fn test_log_start_with_message() {
        let mut logger = TestLogger::new();
        logger.log_start("Setup");
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[INFO] Setup :: Start");
    }

    #[test]
    fn test_log_finish_default() {
        let mut logger = TestLogger::new();
        logger.log_finish_default();
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[INFO] Finished");
    }

    #[test]
    fn test_log_finish_with_message() {
        let mut logger = TestLogger::new();
        logger.log_finish("Algorithm");
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[INFO] Algorithm :: Finished");
    }

    #[test]
    fn test_log_finish_with_failure() {
        let mut logger = TestLogger::new();
        logger.log_finish_with_failure("Algorithm");
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[INFO] Algorithm :: Failed");
    }

    #[test]
    fn test_log_message() {
        let mut logger = TestLogger::new();
        logger.log_message("Processing nodes");
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[INFO] Processing nodes");
    }

    #[test]
    fn test_log_debug() {
        let mut logger = TestLogger::new();
        logger.log_debug("Debug info");
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[DEBUG] Debug info");
    }

    #[test]
    fn test_log_warning() {
        let mut logger = TestLogger::new();
        logger.log_warning("Warning message");
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[WARN] Warning message");
    }

    #[test]
    fn test_log_error() {
        let mut logger = TestLogger::new();
        logger.log_error("Error occurred");
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[ERROR] Error occurred");
    }

    #[test]
    fn test_start_subtask() {
        let mut logger = TestLogger::new();
        logger.set_task("Main Task".to_string());
        logger.start_subtask("SubTask");

        assert_eq!(logger.get_task(), "Main Task :: SubTask");
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[INFO] Start");
    }

    #[test]
    fn test_finish_subtask() {
        let mut logger = TestLogger::new();
        logger.set_task("Main Task :: SubTask".to_string());
        logger.finish_subtask("SubTask");

        assert_eq!(logger.get_task(), "Main Task");
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[INFO] Finished");
    }

    #[test]
    fn test_nested_subtasks() {
        let mut logger = TestLogger::new();
        logger.set_task("Algorithm".to_string());

        // Start first subtask
        logger.start_subtask("Phase1");
        assert_eq!(logger.get_task(), "Algorithm :: Phase1");

        // Start nested subtask
        logger.start_subtask("Step1");
        assert_eq!(logger.get_task(), "Algorithm :: Phase1 :: Step1");

        // Finish nested subtask
        logger.finish_subtask("Step1");
        assert_eq!(logger.get_task(), "Algorithm :: Phase1");

        // Finish first subtask
        logger.finish_subtask("Phase1");
        assert_eq!(logger.get_task(), "Algorithm");
    }

    #[test]
    fn test_log_finish_subtask_with_failure() {
        let mut logger = TestLogger::new();
        logger.set_task("Main Task :: SubTask".to_string());
        logger.log_finish_subtask_with_failure("SubTask");

        assert_eq!(logger.get_task(), "Main Task");
        let messages = logger.get_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "[INFO] SubTask :: Failed");
    }

    #[test]
    fn test_task_access() {
        let mut logger = TestLogger::new();
        logger.set_task("Test Task".to_string());
        assert_eq!(logger.get_task(), "Test Task");
    }

    #[test]
    fn test_task_separator_constant() {
        assert_eq!(TestLogger::TASK_SEPARATOR, " :: ");
    }

    #[test]
    fn test_no_message_factory() {
        let result = NO_MESSAGE();
        assert_eq!(result, None);
    }
}
