//! LogStore for tracking task messages with capacity limits.

use crate::core::utils::progress::Task;
use std::collections::{BTreeMap, VecDeque};

/// Default capacity for tracking tasks per user.
const DEFAULT_CAPACITY: usize = 100;

/// Stores log messages per task with FIFO eviction when capacity is reached.
///
/// Tasks are ordered by start time and description. When the capacity is reached,
/// the oldest task (with its messages) is removed to make room for new tasks.
///
/// # Capacity Management
///
/// - Tracks up to 100 tasks per user by default (configurable)
/// - Each task can have unlimited messages
/// - FIFO eviction: oldest tasks are removed when capacity is exceeded
pub(crate) struct LogStore {
    /// Maps tasks to their message queues, sorted by task start time + description
    messages: BTreeMap<TaskKey, VecDeque<String>>,
    /// Maximum number of tasks to track
    capacity: usize,
}

/// Key for sorting tasks by start time, then description.
///
/// This ensures consistent ordering even when two tasks have the same start time.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct TaskKey {
    pub(crate) start_time: i64,
    pub(crate) description: String,
}

impl TaskKey {
    fn from_task(task: &Task) -> Self {
        Self {
            start_time: task.start_time(),
            description: task.description().to_string(),
        }
    }
}

impl LogStore {
    /// Creates a new log store with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            messages: BTreeMap::new(),
            capacity,
        }
    }

    /// Creates a new log store with default capacity (100 tasks).
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    /// Adds a log message for a specific task.
    ///
    /// If the task doesn't exist, it's added. If capacity is exceeded,
    /// the oldest task is removed.
    pub fn add_log_message(&mut self, task: &Task, message: String) {
        let key = TaskKey::from_task(task);

        // Get or create message queue for this task
        self.messages
            .entry(key)
            .or_default()
            .push_back(message);

        // Enforce capacity by removing oldest task if needed
        while self.messages.len() > self.capacity {
            self.poll_first_entry();
        }
    }

    /// Removes and returns the first (oldest) entry from the store.
    ///
    /// Returns `None` if the store is empty.
    fn poll_first_entry(&mut self) -> Option<(TaskKey, VecDeque<String>)> {
        // BTreeMap iterator gives us keys in sorted order
        let first_key = self.messages.keys().next()?.clone();
        self.messages.remove_entry(&first_key)
    }

    /// Returns all task-message entries as a vector.
    ///
    /// Entries are ordered by task start time and description.
    pub fn stream(&self) -> Vec<(&TaskKey, &VecDeque<String>)> {
        self.messages.iter().collect()
    }

    /// Returns the number of tasks currently tracked.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Returns whether the store is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

impl Default for LogStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_store_creation() {
        let store = LogStore::new();
        assert_eq!(store.capacity, DEFAULT_CAPACITY);
        assert!(store.is_empty());
    }

    #[test]
    fn test_add_message() {
        let mut store = LogStore::new();
        let task = Task::new("Task 1".to_string(), 100);

        store.add_log_message(&task, "Message 1".to_string());

        assert_eq!(store.len(), 1);
        let entries = store.stream();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].1.len(), 1);
        assert_eq!(entries[0].1[0], "Message 1");
    }

    #[test]
    fn test_multiple_messages_same_task() {
        let mut store = LogStore::new();
        let task = Task::new("Task 1".to_string(), 100);

        store.add_log_message(&task, "Message 1".to_string());
        store.add_log_message(&task, "Message 2".to_string());
        store.add_log_message(&task, "Message 3".to_string());

        assert_eq!(store.len(), 1);
        let entries = store.stream();
        assert_eq!(entries[0].1.len(), 3);
    }

    #[test]
    fn test_multiple_tasks() {
        let mut store = LogStore::new();
        let task1 = Task::new("Task 1".to_string(), 100);
        let task2 = Task::new("Task 2".to_string(), 200);

        store.add_log_message(&task1, "Message 1".to_string());
        store.add_log_message(&task2, "Message 2".to_string());

        assert_eq!(store.len(), 2);
    }

    #[test]
    fn test_capacity_enforcement() {
        let mut store = LogStore::with_capacity(3);

        let task1 = Task::new("Task 1".to_string(), 100);
        let task2 = Task::new("Task 2".to_string(), 200);
        let task3 = Task::new("Task 3".to_string(), 300);
        let task4 = Task::new("Task 4".to_string(), 400);

        store.add_log_message(&task1, "Message 1".to_string());
        store.add_log_message(&task2, "Message 2".to_string());
        store.add_log_message(&task3, "Message 3".to_string());

        assert_eq!(store.len(), 3);

        // Adding 4th task should evict the oldest (task1)
        store.add_log_message(&task4, "Message 4".to_string());

        assert_eq!(store.len(), 3);

        // Verify task1 was removed and task4 is present
        let entries = store.stream();
        let task_names: Vec<&str> = entries
            .iter()
            .map(|(key, _)| key.description.as_str())
            .collect();

        assert!(!task_names.contains(&"Task 1"));
        assert!(task_names.contains(&"Task 4"));
    }

    #[test]
    fn test_task_ordering() {
        let mut store = LogStore::new();

        // Create tasks with different start times
        let task1 = Task::new("ZZZ Task".to_string(), 100);
        std::thread::sleep(std::time::Duration::from_millis(10));
        let task2 = Task::new("AAA Task".to_string(), 200);

        store.add_log_message(&task2, "Message 2".to_string());
        store.add_log_message(&task1, "Message 1".to_string());

        let entries = store.stream();

        // Should be ordered by start time (task1 first), not description
        assert!(entries[0].0.start_time < entries[1].0.start_time);
    }

    #[test]
    fn test_same_start_time_ordered_by_description() {
        let mut store = LogStore::new();

        let task1 = Task::new("BBB Task".to_string(), 100);
        let task2 = Task::new("AAA Task".to_string(), 200);

        // Both tasks have same start time (created immediately)
        store.add_log_message(&task1, "Message 1".to_string());
        store.add_log_message(&task2, "Message 2".to_string());

        let entries = store.stream();

        // When start times are equal, should be ordered by description
        if entries[0].0.start_time == entries[1].0.start_time {
            assert!(entries[0].0.description < entries[1].0.description);
        }
    }
}
