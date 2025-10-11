use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

/// Identifies a relationship type in a graph.
///
/// RelationshipType is used to classify and identify relationships between nodes.
/// It supports efficient interning through a static cache, ensuring that types
/// with the same name share the same underlying instance.
#[derive(Clone)]
pub struct RelationshipType {
    name: Arc<String>,
}

// Custom Serialize - just write the string
impl Serialize for RelationshipType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.name.as_str().serialize(serializer)
    }
}

// Custom Deserialize - read string and re-intern
impl<'de> Deserialize<'de> for RelationshipType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        Ok(RelationshipType::of(name))
    }
}

impl RelationshipType {
    /// Relationship type that matches all relationships.
    pub const ALL_RELATIONSHIPS_NAME: &'static str = "__ALL__";

    /// Creates a new RelationshipType with the given name.
    ///
    /// Note: Prefer using `RelationshipType::of()` for interned instances.
    fn new(name: String) -> Self {
        RelationshipType {
            name: Arc::new(name),
        }
    }

    /// Returns the special ALL_RELATIONSHIPS type that matches all relationships.
    pub fn all_relationships() -> Self {
        Self::of(Self::ALL_RELATIONSHIPS_NAME)
    }

    /// Factory method to create or retrieve an interned RelationshipType.
    ///
    /// This method ensures that RelationshipTypes with the same name share
    /// the same underlying data, improving memory efficiency and
    /// enabling fast equality comparisons.
    ///
    /// # Arguments
    /// * `name` - The relationship type name
    ///
    /// # Returns
    /// An interned RelationshipType instance
    pub fn of(name: impl Into<String>) -> Self {
        lazy_static::lazy_static! {
            static ref INSTANCES: RwLock<HashMap<String, RelationshipType>> = RwLock::new(HashMap::new());
        }

        let name_string = name.into();

        // Try read lock first for common case
        {
            // Use unwrap_or_else to recover from a poisoned lock instead of panicking.
            let instances = INSTANCES.read().unwrap_or_else(|e| e.into_inner());
            if let Some(rel_type) = instances.get(&name_string) {
                return rel_type.clone();
            }
        }

        // Need to create new instance
        let mut instances = INSTANCES.write().unwrap_or_else(|e| e.into_inner());
        // Check again in case another thread created it
        if let Some(rel_type) = instances.get(&name_string) {
            return rel_type.clone();
        }

        let rel_type = RelationshipType::new(name_string.clone());
        instances.insert(name_string, rel_type.clone());
        rel_type
    }

    /// Creates a collection of RelationshipTypes from strings.
    ///
    /// # Arguments
    /// * `types` - Iterator of relationship type names
    ///
    /// # Returns
    /// Vector of RelationshipTypes
    pub fn list_of<I, S>(types: I) -> Vec<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        types.into_iter().map(RelationshipType::of).collect()
    }

    /// Returns the name of this relationship type.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns whether this is the ALL_RELATIONSHIPS type.
    pub fn is_all_relationships(&self) -> bool {
        self.name.as_str() == Self::ALL_RELATIONSHIPS_NAME
    }

    /// Returns the relationship type that projects all relationships.
    pub fn project_all(&self) -> Self {
        Self::all_relationships()
    }
}

impl fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Debug for RelationshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RelationshipType({})", self.name)
    }
}

impl PartialEq for RelationshipType {
    fn eq(&self, other: &Self) -> bool {
        // Fast pointer comparison for interned strings
        Arc::ptr_eq(&self.name, &other.name) || self.name == other.name
    }
}

impl Eq for RelationshipType {}

impl Hash for RelationshipType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialOrd for RelationshipType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RelationshipType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_type_creation() {
        let rel_type1 = RelationshipType::of("KNOWS");
        let rel_type2 = RelationshipType::of("KNOWS");

        assert_eq!(rel_type1, rel_type2);
        assert_eq!(rel_type1.name(), "KNOWS");
    }

    #[test]
    fn test_all_relationships() {
        let all_rels = RelationshipType::all_relationships();
        assert_eq!(all_rels.name(), RelationshipType::ALL_RELATIONSHIPS_NAME);
        assert!(all_rels.is_all_relationships());
    }

    #[test]
    fn test_list_of() {
        let types = RelationshipType::list_of(vec!["KNOWS", "LIKES", "FOLLOWS"]);
        assert_eq!(types.len(), 3);
        assert_eq!(types[0].name(), "KNOWS");
        assert_eq!(types[1].name(), "LIKES");
        assert_eq!(types[2].name(), "FOLLOWS");
    }

    #[test]
    fn test_interning() {
        let rel_type1 = RelationshipType::of("TEST");
        let rel_type2 = RelationshipType::of("TEST");

        // Should be the same instance due to interning
        assert!(Arc::ptr_eq(&rel_type1.name, &rel_type2.name));
    }

    #[test]
    fn test_display() {
        let rel_type = RelationshipType::of("KNOWS");
        assert_eq!(format!("{}", rel_type), "KNOWS");
    }

    #[test]
    fn test_ordering() {
        let rel_type1 = RelationshipType::of("A");
        let rel_type2 = RelationshipType::of("B");
        let rel_type3 = RelationshipType::of("C");

        assert!(rel_type1 < rel_type2);
        assert!(rel_type2 < rel_type3);
        assert!(rel_type1 < rel_type3);
    }

    #[test]
    fn test_project_all() {
        let rel_type = RelationshipType::of("KNOWS");
        let all = rel_type.project_all();
        assert!(all.is_all_relationships());
    }

    #[test]
    fn test_serde_roundtrip() {
        let rel_type = RelationshipType::of("KNOWS");
        let json = serde_json::to_string(&rel_type).unwrap();
        let deserialized: RelationshipType = serde_json::from_str(&json).unwrap();

        assert_eq!(rel_type, deserialized);
        // After deserialization, should be re-interned
        assert!(Arc::ptr_eq(&rel_type.name, &deserialized.name));
    }
}
