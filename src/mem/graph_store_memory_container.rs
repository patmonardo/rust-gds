//! Graph store memory container
//!
//! Tracks memory usage per user for stored graphs.

use std::collections::HashMap;
use super::user_entity_memory::UserEntityMemory;

/// Event representing a graph being added
pub struct GraphStoreAddedEvent {
    user: String,
    graph_name: String,
    memory_in_bytes: usize,
}

impl GraphStoreAddedEvent {
    pub fn new(user: String, graph_name: String, memory_in_bytes: usize) -> Self {
        Self {
            user,
            graph_name,
            memory_in_bytes,
        }
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn memory_in_bytes(&self) -> usize {
        self.memory_in_bytes
    }
}

/// Event representing a graph being removed
pub struct GraphStoreRemovedEvent {
    user: String,
    graph_name: String,
}

impl GraphStoreRemovedEvent {
    pub fn new(user: String, graph_name: String) -> Self {
        Self { user, graph_name }
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}

/// Container for tracking graph store memory usage per user
///
/// Thread-safe container that tracks memory consumption for each user's graphs.
#[derive(Debug, Default)]
pub struct GraphStoreMemoryContainer {
    // Map: username -> (graph_name -> memory_bytes)
    graphs_memory: HashMap<String, HashMap<String, usize>>,
    graph_store_reserved_memory_total: usize,
}

impl GraphStoreMemoryContainer {
    /// Creates a new empty container
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a graph and returns the new total reserved memory
    pub fn add_graph(&mut self, user: &str, graph_name: &str, memory_in_bytes: usize) -> usize {
        self.graph_store_reserved_memory_total += memory_in_bytes;

        self.graphs_memory
            .entry(user.to_string())
            .or_default()
            .insert(graph_name.to_string(), memory_in_bytes);

        self.graph_store_reserved_memory_total
    }

    /// Adds a graph using an event and returns the new total reserved memory
    pub fn add_graph_event(&mut self, event: GraphStoreAddedEvent) -> usize {
        self.add_graph(event.user(), event.graph_name(), event.memory_in_bytes())
    }

    /// Removes a graph and returns the new total reserved memory
    pub fn remove_graph(&mut self, user: &str, graph_name: &str) -> usize {
        if let Some(user_graphs) = self.graphs_memory.get_mut(user) {
            if let Some(memory_to_remove) = user_graphs.remove(graph_name) {
                if user_graphs.is_empty() {
                    self.graphs_memory.remove(user);
                }
                self.graph_store_reserved_memory_total -= memory_to_remove;
            }
        }

        self.graph_store_reserved_memory_total
    }

    /// Removes a graph using an event and returns the new total reserved memory
    pub fn remove_graph_event(&mut self, event: GraphStoreRemovedEvent) -> usize {
        self.remove_graph(event.user(), event.graph_name())
    }

    /// Returns the total reserved memory across all users
    pub fn graph_store_reserved_memory(&self) -> usize {
        self.graph_store_reserved_memory_total
    }

    /// Lists all graphs for a specific user
    pub fn list_graphs(&self, user: &str) -> Vec<UserEntityMemory> {
        self.graphs_memory
            .get(user)
            .map(|user_graphs| {
                user_graphs
                    .iter()
                    .map(|(graph_name, &memory_amount)| {
                        UserEntityMemory::create_graph(user, graph_name, memory_amount)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Lists all graphs across all users
    pub fn list_all_graphs(&self) -> Vec<UserEntityMemory> {
        self.graphs_memory
            .keys()
            .flat_map(|user| self.list_graphs(user))
            .collect()
    }

    /// Returns the total memory used by a specific user's graphs
    pub fn memory_of_graphs(&self, user: &str) -> usize {
        self.graphs_memory
            .get(user)
            .map(|user_graphs| user_graphs.values().sum())
            .unwrap_or(0)
    }

    /// Returns all users who have graphs
    pub fn graph_users(&self) -> Vec<String> {
        self.graphs_memory.keys().cloned().collect()
    }

    /// Returns the number of graphs for a specific user
    pub fn graph_count(&self, user: &str) -> usize {
        self.graphs_memory
            .get(user)
            .map(|user_graphs| user_graphs.len())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_graph() {
        let mut container = GraphStoreMemoryContainer::new();

        let total = container.add_graph("alice", "graph1", 1000);
        assert_eq!(total, 1000);
        assert_eq!(container.graph_store_reserved_memory(), 1000);
    }

    #[test]
    fn test_add_multiple_graphs() {
        let mut container = GraphStoreMemoryContainer::new();

        container.add_graph("alice", "graph1", 1000);
        container.add_graph("alice", "graph2", 2000);
        container.add_graph("bob", "graph3", 3000);

        assert_eq!(container.graph_store_reserved_memory(), 6000);
        assert_eq!(container.memory_of_graphs("alice"), 3000);
        assert_eq!(container.memory_of_graphs("bob"), 3000);
    }

    #[test]
    fn test_remove_graph() {
        let mut container = GraphStoreMemoryContainer::new();

        container.add_graph("alice", "graph1", 1000);
        container.add_graph("alice", "graph2", 2000);

        let total = container.remove_graph("alice", "graph1");
        assert_eq!(total, 2000);
        assert_eq!(container.graph_store_reserved_memory(), 2000);
    }

    #[test]
    fn test_remove_nonexistent_graph() {
        let mut container = GraphStoreMemoryContainer::new();

        container.add_graph("alice", "graph1", 1000);

        let total = container.remove_graph("alice", "nonexistent");
        assert_eq!(total, 1000); // No change
    }

    #[test]
    fn test_list_graphs() {
        let mut container = GraphStoreMemoryContainer::new();

        container.add_graph("alice", "graph1", 1000);
        container.add_graph("alice", "graph2", 2000);

        let graphs = container.list_graphs("alice");
        assert_eq!(graphs.len(), 2);
        assert!(graphs.iter().any(|g| g.name() == "graph1"));
        assert!(graphs.iter().any(|g| g.name() == "graph2"));
    }

    #[test]
    fn test_list_all_graphs() {
        let mut container = GraphStoreMemoryContainer::new();

        container.add_graph("alice", "graph1", 1000);
        container.add_graph("bob", "graph2", 2000);

        let all_graphs = container.list_all_graphs();
        assert_eq!(all_graphs.len(), 2);
    }

    #[test]
    fn test_graph_users() {
        let mut container = GraphStoreMemoryContainer::new();

        container.add_graph("alice", "graph1", 1000);
        container.add_graph("bob", "graph2", 2000);

        let users = container.graph_users();
        assert_eq!(users.len(), 2);
        assert!(users.contains(&"alice".to_string()));
        assert!(users.contains(&"bob".to_string()));
    }

    #[test]
    fn test_graph_count() {
        let mut container = GraphStoreMemoryContainer::new();

        container.add_graph("alice", "graph1", 1000);
        container.add_graph("alice", "graph2", 2000);

        assert_eq!(container.graph_count("alice"), 2);
        assert_eq!(container.graph_count("bob"), 0);
    }

    #[test]
    fn test_events() {
        let mut container = GraphStoreMemoryContainer::new();

        let add_event = GraphStoreAddedEvent::new("alice".to_string(), "graph1".to_string(), 1000);
        container.add_graph_event(add_event);

        assert_eq!(container.graph_store_reserved_memory(), 1000);

        let remove_event = GraphStoreRemovedEvent::new("alice".to_string(), "graph1".to_string());
        container.remove_graph_event(remove_event);

        assert_eq!(container.graph_store_reserved_memory(), 0);
    }
}
