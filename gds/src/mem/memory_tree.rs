//! Memory tree for hierarchical memory descriptions
//!
//! Provides tree-shaped descriptions of memory usage for complex data structures.

use super::memory_range::MemoryRange;
use std::fmt;

/// A tree-shaped description of an object that has resources residing in memory
///
/// Memory trees allow hierarchical representation of memory usage, making it
/// easier to understand where memory is being consumed in complex algorithms.
#[derive(Debug, Clone)]
pub struct MemoryTree {
    description: String,
    memory_usage: MemoryRange,
    components: Vec<MemoryTree>,
}

impl MemoryTree {
    /// Creates a new memory tree
    pub fn new(
        description: String,
        memory_usage: MemoryRange,
        components: Vec<MemoryTree>,
    ) -> Self {
        Self {
            description,
            memory_usage,
            components,
        }
    }

    /// Creates a leaf node (no children)
    pub fn leaf(description: String, memory_usage: MemoryRange) -> Self {
        Self {
            description,
            memory_usage,
            components: Vec::new(),
        }
    }

    /// Creates an empty/null memory tree
    pub fn empty() -> Self {
        Self {
            description: String::new(),
            memory_usage: MemoryRange::empty(),
            components: Vec::new(),
        }
    }

    /// Returns the description for this component
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the resident memory of this component
    pub fn memory_usage(&self) -> &MemoryRange {
        &self.memory_usage
    }

    /// Returns nested resources of this component
    pub fn components(&self) -> &[MemoryTree] {
        &self.components
    }

    /// Finds the resident memory component if present
    pub fn resident_memory(&self) -> Option<&MemoryTree> {
        self.components
            .iter()
            .find(|c| c.description == "residentMemory")
    }

    /// Renders the memory tree as a human-readable string
    pub fn render(&self) -> String {
        let mut buffer = Vec::new();
        self.render_internal(&mut buffer, 0);
        buffer.join("\n")
    }

    /// Renders the memory tree as a structured map
    pub fn render_map(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "name".to_string(),
            serde_json::Value::String(self.description.clone()),
        );
        map.insert(
            "memoryUsage".to_string(),
            serde_json::Value::String(format!("{}", self.memory_usage)),
        );

        if !self.components.is_empty() {
            let components: Vec<_> = self.components.iter().map(|c| c.render_map()).collect();
            map.insert(
                "components".to_string(),
                serde_json::Value::Array(components),
            );
        }

        serde_json::Value::Object(map)
    }

    fn render_internal(&self, buffer: &mut Vec<String>, depth: usize) {
        // Add indentation
        for _ in 1..depth {
            buffer.push("  ".to_string());
        }

        if depth > 0 {
            buffer.push("├─ ".to_string());
        }

        buffer.push(format!("{}: {}", self.description, self.memory_usage));

        // Recursively render components
        for component in &self.components {
            component.render_internal(buffer, depth + 1);
        }
    }
}

impl fmt::Display for MemoryTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// A value class pairing a memory tree with graph dimensions
#[derive(Debug, Clone)]
pub struct MemoryTreeWithDimensions {
    memory_tree: MemoryTree,
    node_count: usize,
    relationship_count: usize,
}

impl MemoryTreeWithDimensions {
    /// Creates a new pairing of memory tree and dimensions
    pub fn new(memory_tree: MemoryTree, node_count: usize, relationship_count: usize) -> Self {
        Self {
            memory_tree,
            node_count,
            relationship_count,
        }
    }

    /// Returns the memory tree
    pub fn memory_tree(&self) -> &MemoryTree {
        &self.memory_tree
    }

    /// Returns the node count
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    /// Returns the relationship count
    pub fn relationship_count(&self) -> usize {
        self.relationship_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_tree() {
        let tree = MemoryTree::leaf("NodeArray".to_string(), MemoryRange::of(1024));

        assert_eq!(tree.description(), "NodeArray");
        assert_eq!(tree.memory_usage().min(), 1024);
        assert_eq!(tree.components().len(), 0);
    }

    #[test]
    fn test_composite_tree() {
        let child1 = MemoryTree::leaf("Nodes".to_string(), MemoryRange::of(1000));
        let child2 = MemoryTree::leaf("Relationships".to_string(), MemoryRange::of(2000));

        let parent = MemoryTree::new(
            "Graph".to_string(),
            MemoryRange::of(3000),
            vec![child1, child2],
        );

        assert_eq!(parent.components().len(), 2);
        assert_eq!(parent.memory_usage().min(), 3000);
    }

    #[test]
    fn test_render() {
        let tree = MemoryTree::new(
            "Total".to_string(),
            MemoryRange::of(3000),
            vec![
                MemoryTree::leaf("Part1".to_string(), MemoryRange::of(1000)),
                MemoryTree::leaf("Part2".to_string(), MemoryRange::of(2000)),
            ],
        );

        let rendered = tree.render();
        assert!(rendered.contains("Total"));
        assert!(rendered.contains("Part1"));
        assert!(rendered.contains("Part2"));
    }

    #[test]
    fn test_empty() {
        let tree = MemoryTree::empty();
        assert!(tree.description().is_empty());
        assert!(tree.memory_usage().is_empty());
    }
}
