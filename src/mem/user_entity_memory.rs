//! User entity memory record
//!
//! Represents memory usage associated with a user-defined entity (graph or task).

/// Memory usage associated with a user entity
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserEntityMemory {
    user: String,
    name: String,
    entity: String,
    memory_in_bytes: usize,
}

impl UserEntityMemory {
    /// Creates a new user entity memory record
    pub fn new(user: String, name: String, entity: String, memory_in_bytes: usize) -> Self {
        Self {
            user,
            name,
            entity,
            memory_in_bytes,
        }
    }

    /// Creates a user entity memory record for a graph
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::mem::UserEntityMemory;
    ///
    /// let graph_mem = UserEntityMemory::create_graph("alice", "my-graph", 1024 * 1024);
    /// assert_eq!(graph_mem.user(), "alice");
    /// assert_eq!(graph_mem.name(), "my-graph");
    /// ```
    pub fn create_graph(user: &str, name: &str, memory_in_bytes: usize) -> Self {
        Self {
            user: user.to_string(),
            name: name.to_string(),
            entity: "graph".to_string(),
            memory_in_bytes,
        }
    }

    /// Creates a user entity memory record for a task
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_gds::mem::UserEntityMemory;
    ///
    /// let task_mem = UserEntityMemory::create_task("bob", "pagerank", "job-123", 512 * 1024);
    /// assert_eq!(task_mem.user(), "bob");
    /// assert_eq!(task_mem.name(), "pagerank");
    /// ```
    pub fn create_task(user: &str, name: &str, job_id: &str, memory_in_bytes: usize) -> Self {
        Self {
            user: user.to_string(),
            name: name.to_string(),
            entity: job_id.to_string(),
            memory_in_bytes,
        }
    }

    /// Returns the user
    pub fn user(&self) -> &str {
        &self.user
    }

    /// Returns the name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the entity identifier
    pub fn entity(&self) -> &str {
        &self.entity
    }

    /// Returns the memory in bytes
    pub fn memory_in_bytes(&self) -> usize {
        self.memory_in_bytes
    }
}

impl std::fmt::Display for UserEntityMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UserEntityMemory(user={}, name={}, entity={}, memoryInBytes={})",
            self.user, self.name, self.entity, self.memory_in_bytes
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_graph() {
        let mem = UserEntityMemory::create_graph("alice", "my-graph", 1024);
        
        assert_eq!(mem.user(), "alice");
        assert_eq!(mem.name(), "my-graph");
        assert_eq!(mem.entity(), "graph");
        assert_eq!(mem.memory_in_bytes(), 1024);
    }

    #[test]
    fn test_create_task() {
        let mem = UserEntityMemory::create_task("bob", "pagerank", "job-123", 2048);
        
        assert_eq!(mem.user(), "bob");
        assert_eq!(mem.name(), "pagerank");
        assert_eq!(mem.entity(), "job-123");
        assert_eq!(mem.memory_in_bytes(), 2048);
    }

    #[test]
    fn test_equality() {
        let mem1 = UserEntityMemory::create_graph("alice", "graph1", 1000);
        let mem2 = UserEntityMemory::create_graph("alice", "graph1", 1000);
        let mem3 = UserEntityMemory::create_graph("alice", "graph2", 1000);

        assert_eq!(mem1, mem2);
        assert_ne!(mem1, mem3);
    }

    #[test]
    fn test_display() {
        let mem = UserEntityMemory::create_graph("alice", "my-graph", 1024);
        let display = format!("{}", mem);
        
        assert!(display.contains("alice"));
        assert!(display.contains("my-graph"));
        assert!(display.contains("1024"));
    }
}
