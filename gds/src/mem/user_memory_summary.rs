//! User memory summary
//!
//! Represents a summary of memory usage for a specific user.

/// Summary of memory usage for a user
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserMemorySummary {
    user: String,
    total_graphs_memory: usize,
    total_tasks_memory: usize,
}

impl UserMemorySummary {
    /// Creates a new user memory summary
    ///
    /// # Examples
    ///
    /// ```
    /// use gds::mem::UserMemorySummary;
    ///
    /// let summary = UserMemorySummary::new("alice".to_string(), 2 * 1024 * 1024, 512 * 1024);
    /// assert_eq!(summary.user(), "alice");
    /// assert_eq!(summary.total_memory(), 2 * 1024 * 1024 + 512 * 1024);
    /// ```
    pub fn new(user: String, total_graphs_memory: usize, total_tasks_memory: usize) -> Self {
        Self {
            user,
            total_graphs_memory,
            total_tasks_memory,
        }
    }

    /// Returns the user
    pub fn user(&self) -> &str {
        &self.user
    }

    /// Returns the total memory consumed by graphs
    pub fn total_graphs_memory(&self) -> usize {
        self.total_graphs_memory
    }

    /// Returns the total memory consumed by tasks
    pub fn total_tasks_memory(&self) -> usize {
        self.total_tasks_memory
    }

    /// Returns the total memory consumed (graphs + tasks)
    pub fn total_memory(&self) -> usize {
        self.total_graphs_memory + self.total_tasks_memory
    }
}

impl std::fmt::Display for UserMemorySummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UserMemorySummary(user={}, totalGraphsMemory={}, totalTasksMemory={}, total={})",
            self.user,
            self.total_graphs_memory,
            self.total_tasks_memory,
            self.total_memory()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let summary = UserMemorySummary::new("alice".to_string(), 2000, 1000);

        assert_eq!(summary.user(), "alice");
        assert_eq!(summary.total_graphs_memory(), 2000);
        assert_eq!(summary.total_tasks_memory(), 1000);
        assert_eq!(summary.total_memory(), 3000);
    }

    #[test]
    fn test_equality() {
        let sum1 = UserMemorySummary::new("alice".to_string(), 2000, 1000);
        let sum2 = UserMemorySummary::new("alice".to_string(), 2000, 1000);
        let sum3 = UserMemorySummary::new("bob".to_string(), 2000, 1000);

        assert_eq!(sum1, sum2);
        assert_ne!(sum1, sum3);
    }

    #[test]
    fn test_display() {
        let summary = UserMemorySummary::new("alice".to_string(), 2048, 1024);
        let display = format!("{}", summary);

        assert!(display.contains("alice"));
        assert!(display.contains("2048"));
        assert!(display.contains("1024"));
        assert!(display.contains("3072")); // total
    }

    #[test]
    fn test_zero_memory() {
        let summary = UserMemorySummary::new("bob".to_string(), 0, 0);
        assert_eq!(summary.total_memory(), 0);
    }
}
