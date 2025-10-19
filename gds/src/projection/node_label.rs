use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

/// Represents a node label in a graph.
///
/// NodeLabel is used to identify and classify nodes. It supports
/// efficient interning through a static cache, ensuring that labels
/// with the same name share the same underlying instance.
#[derive(Clone)]
pub struct NodeLabel {
    name: Arc<String>,
}

// Custom Serialize - just write the string
impl Serialize for NodeLabel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.name.as_str().serialize(serializer)
    }
}

// Custom Deserialize - read string and re-intern
impl<'de> Deserialize<'de> for NodeLabel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        Ok(NodeLabel::of(name))
    }
}

impl NodeLabel {
    /// Represents all node labels.
    pub const ALL_NODES_NAME: &'static str = "__ALL__";

    /// Creates a new NodeLabel with the given name.
    ///
    /// Note: Prefer using `NodeLabel::of()` for interned instances.
    fn new(name: String) -> Self {
        NodeLabel {
            name: Arc::new(name),
        }
    }

    /// Returns the special ALL_NODES label that matches all nodes.
    pub fn all_nodes() -> Self {
        Self::of(Self::ALL_NODES_NAME)
    }

    /// Factory method to create or retrieve an interned NodeLabel.
    ///
    /// This method ensures that NodeLabels with the same name share
    /// the same underlying data, improving memory efficiency and
    /// enabling fast equality comparisons.
    ///
    /// # Arguments
    /// * `name` - The label name
    ///
    /// # Returns
    /// An interned NodeLabel instance
    pub fn of(name: impl Into<String>) -> Self {
        lazy_static::lazy_static! {
            static ref INSTANCES: RwLock<HashMap<String, NodeLabel>> = RwLock::new(HashMap::new());
        }

        let name_string = name.into();

        // Try read lock first for common case
        {
            // Use unwrap_or_else to recover from a poisoned lock instead of panicking.
            let instances = INSTANCES.read().unwrap_or_else(|e| e.into_inner());
            if let Some(label) = instances.get(&name_string) {
                return label.clone();
            }
        }

        // Need to create new instance
        let mut instances = INSTANCES.write().unwrap_or_else(|e| e.into_inner());
        // Check again in case another thread created it
        if let Some(label) = instances.get(&name_string) {
            return label.clone();
        }

        let label = NodeLabel::new(name_string.clone());
        instances.insert(name_string, label.clone());
        label
    }

    /// Creates a collection of NodeLabels from strings.
    ///
    /// # Arguments
    /// * `labels` - Iterator of label names
    ///
    /// # Returns
    /// Vector of NodeLabels
    pub fn list_of<I, S>(labels: I) -> Vec<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        labels.into_iter().map(NodeLabel::of).collect()
    }

    /// Returns the name of this label.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns whether this is the ALL_NODES label.
    pub fn is_all_nodes(&self) -> bool {
        self.name.as_str() == Self::ALL_NODES_NAME
    }

    /// Returns the label that projects all nodes.
    pub fn project_all(&self) -> Self {
        Self::all_nodes()
    }
}

impl fmt::Display for NodeLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Debug for NodeLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeLabel({})", self.name)
    }
}

impl PartialEq for NodeLabel {
    fn eq(&self, other: &Self) -> bool {
        // Fast pointer comparison for interned strings
        Arc::ptr_eq(&self.name, &other.name) || self.name == other.name
    }
}

impl Eq for NodeLabel {}

impl Hash for NodeLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialOrd for NodeLabel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeLabel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_label_creation() {
        let label1 = NodeLabel::of("Person");
        let label2 = NodeLabel::of("Person");

        assert_eq!(label1, label2);
        assert_eq!(label1.name(), "Person");
    }

    #[test]
    fn test_all_nodes() {
        let all_nodes = NodeLabel::all_nodes();
        assert_eq!(all_nodes.name(), NodeLabel::ALL_NODES_NAME);
        assert!(all_nodes.is_all_nodes());
    }

    #[test]
    fn test_list_of() {
        let labels = NodeLabel::list_of(vec!["Person", "Company", "City"]);
        assert_eq!(labels.len(), 3);
        assert_eq!(labels[0].name(), "Person");
        assert_eq!(labels[1].name(), "Company");
        assert_eq!(labels[2].name(), "City");
    }

    #[test]
    fn test_interning() {
        let label1 = NodeLabel::of("Test");
        let label2 = NodeLabel::of("Test");

        // Should be the same instance due to interning
        assert!(Arc::ptr_eq(&label1.name, &label2.name));
    }

    #[test]
    fn test_display() {
        let label = NodeLabel::of("Person");
        assert_eq!(format!("{}", label), "Person");
    }

    #[test]
    fn test_ordering() {
        let label1 = NodeLabel::of("A");
        let label2 = NodeLabel::of("B");
        let label3 = NodeLabel::of("C");

        assert!(label1 < label2);
        assert!(label2 < label3);
        assert!(label1 < label3);
    }

    #[test]
    fn test_serde_roundtrip() {
        let label = NodeLabel::of("Person");
        let json = serde_json::to_string(&label).unwrap();
        let deserialized: NodeLabel = serde_json::from_str(&json).unwrap();

        assert_eq!(label, deserialized);
        // After deserialization, should be re-interned
        assert!(Arc::ptr_eq(&label.name, &deserialized.name));
    }
}
