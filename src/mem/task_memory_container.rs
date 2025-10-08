//! Task memory container
//!
//! Tracks memory usage per user for running tasks.

use std::collections::HashMap;
use super::user_entity_memory::UserEntityMemory;

/// Represents a user task with memory tracking
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserTask {
    username: String,
    job_id: String,
}

impl UserTask {
    pub fn new(username: String, job_id: String) -> Self {
        Self { username, job_id }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn job_id(&self) -> &str {
        &self.job_id
    }
}

/// Information about a task's memory usage
#[derive(Debug, Clone)]
struct TaskInfo {
    task_name: String,
    memory_amount: usize,
}

/// Container for tracking task memory usage per user
///
/// Thread-safe container that tracks memory consumption for each user's running tasks.
#[derive(Debug, Default)]
pub struct TaskMemoryContainer {
    // Map: username -> (job_id -> TaskInfo)
    memory_in_use: HashMap<String, HashMap<String, TaskInfo>>,
    allocated_memory: usize,
}

impl TaskMemoryContainer {
    /// Creates a new empty container
    pub fn new() -> Self {
        Self::default()
    }

    /// Reserves memory for a task
    pub fn reserve(
        &mut self,
        username: &str,
        task_name: &str,
        job_id: &str,
        memory_amount: usize,
    ) {
        self.allocated_memory += memory_amount;

        let task_info = TaskInfo {
            task_name: task_name.to_string(),
            memory_amount,
        };

        self.memory_in_use
            .entry(username.to_string())
            .or_default()
            .insert(job_id.to_string(), task_info);
    }

    /// Removes a task and returns the updated allocated memory
    ///
    /// Note: Returns total allocated memory if task is not found (mimicking Java behavior)
    pub fn remove_task(&mut self, task: &UserTask) -> usize {
        if let Some(user_tasks) = self.memory_in_use.get_mut(task.username()) {
            if let Some(task_info) = user_tasks.remove(task.job_id()) {
                if user_tasks.is_empty() {
                    self.memory_in_use.remove(task.username());
                }
                self.allocated_memory -= task_info.memory_amount;
            }
        }

        self.allocated_memory
    }

    /// Returns the total task reserved memory
    pub fn task_reserved_memory(&self) -> usize {
        self.allocated_memory
    }

    /// Lists all tasks for a specific user
    pub fn list_tasks(&self, user: &str) -> Vec<UserEntityMemory> {
        self.memory_in_use
            .get(user)
            .map(|user_tasks| {
                user_tasks
                    .iter()
                    .map(|(job_id, task_info)| {
                        UserEntityMemory::create_task(
                            user,
                            &task_info.task_name,
                            job_id,
                            task_info.memory_amount,
                        )
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Lists all tasks across all users
    pub fn list_all_tasks(&self) -> Vec<UserEntityMemory> {
        self.memory_in_use
            .keys()
            .flat_map(|user| self.list_tasks(user))
            .collect()
    }

    /// Returns the total memory used by a specific user's tasks
    pub fn memory_of_tasks(&self, user: &str) -> usize {
        self.memory_in_use
            .get(user)
            .map(|user_tasks| {
                user_tasks
                    .values()
                    .map(|task_info| task_info.memory_amount)
                    .sum()
            })
            .unwrap_or(0)
    }

    /// Returns all users who have tasks
    pub fn task_users(&self) -> Vec<String> {
        self.memory_in_use.keys().cloned().collect()
    }

    /// Returns the number of tasks for a specific user
    pub fn task_count(&self, user: &str) -> usize {
        self.memory_in_use
            .get(user)
            .map(|user_tasks| user_tasks.len())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reserve_task() {
        let mut container = TaskMemoryContainer::new();

        container.reserve("alice", "pagerank", "job-1", 1000);
        assert_eq!(container.task_reserved_memory(), 1000);
    }

    #[test]
    fn test_reserve_multiple_tasks() {
        let mut container = TaskMemoryContainer::new();

        container.reserve("alice", "pagerank", "job-1", 1000);
        container.reserve("alice", "louvain", "job-2", 2000);
        container.reserve("bob", "betweenness", "job-3", 3000);

        assert_eq!(container.task_reserved_memory(), 6000);
        assert_eq!(container.memory_of_tasks("alice"), 3000);
        assert_eq!(container.memory_of_tasks("bob"), 3000);
    }

    #[test]
    fn test_remove_task() {
        let mut container = TaskMemoryContainer::new();

        container.reserve("alice", "pagerank", "job-1", 1000);
        container.reserve("alice", "louvain", "job-2", 2000);

        let task = UserTask::new("alice".to_string(), "job-1".to_string());
        let total = container.remove_task(&task);

        assert_eq!(total, 2000);
        assert_eq!(container.task_reserved_memory(), 2000);
    }

    #[test]
    fn test_remove_nonexistent_task() {
        let mut container = TaskMemoryContainer::new();

        container.reserve("alice", "pagerank", "job-1", 1000);

        let task = UserTask::new("alice".to_string(), "nonexistent".to_string());
        let total = container.remove_task(&task);

        assert_eq!(total, 1000); // No change
    }

    #[test]
    fn test_list_tasks() {
        let mut container = TaskMemoryContainer::new();

        container.reserve("alice", "pagerank", "job-1", 1000);
        container.reserve("alice", "louvain", "job-2", 2000);

        let tasks = container.list_tasks("alice");
        assert_eq!(tasks.len(), 2);
        assert!(tasks.iter().any(|t| t.name() == "pagerank"));
        assert!(tasks.iter().any(|t| t.name() == "louvain"));
    }

    #[test]
    fn test_list_all_tasks() {
        let mut container = TaskMemoryContainer::new();

        container.reserve("alice", "pagerank", "job-1", 1000);
        container.reserve("bob", "louvain", "job-2", 2000);

        let all_tasks = container.list_all_tasks();
        assert_eq!(all_tasks.len(), 2);
    }

    #[test]
    fn test_task_users() {
        let mut container = TaskMemoryContainer::new();

        container.reserve("alice", "pagerank", "job-1", 1000);
        container.reserve("bob", "louvain", "job-2", 2000);

        let users = container.task_users();
        assert_eq!(users.len(), 2);
        assert!(users.contains(&"alice".to_string()));
        assert!(users.contains(&"bob".to_string()));
    }

    #[test]
    fn test_task_count() {
        let mut container = TaskMemoryContainer::new();

        container.reserve("alice", "pagerank", "job-1", 1000);
        container.reserve("alice", "louvain", "job-2", 2000);

        assert_eq!(container.task_count("alice"), 2);
        assert_eq!(container.task_count("bob"), 0);
    }
}
