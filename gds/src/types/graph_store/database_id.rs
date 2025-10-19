//! DatabaseId - Unique identifier for a database.

use std::fmt;

/// Unique identifier for a database.
///
/// DatabaseId wraps a String to provide semantic meaning
/// for database identifiers in the system.
#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct DatabaseId {
    value: String,
}

impl DatabaseId {
    /// Creates a new DatabaseId.
    ///
    pub fn new<S: AsRef<str>>(value: S) -> Self {
        Self {
            value: value.as_ref().to_string(),
        }
    }

    /// Returns the database ID value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the database ID value (alias).
    pub fn database_name(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for DatabaseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<String> for DatabaseId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for DatabaseId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl AsRef<str> for DatabaseId {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_id_creation() {
        let db_id = DatabaseId::new("test-db");
        assert_eq!(db_id.value(), "test-db");
        assert_eq!(db_id.database_name(), "test-db");
    }

    #[test]
    fn test_database_id_display() {
        let db_id = DatabaseId::new("my-db");
        assert_eq!(format!("{}", db_id), "my-db");
    }

    #[test]
    fn test_database_id_from_string() {
        let db_id: DatabaseId = "test".to_string().into();
        assert_eq!(db_id.value(), "test");
    }

    #[test]
    fn test_database_id_from_str() {
        let db_id: DatabaseId = "test".into();
        assert_eq!(db_id.value(), "test");
    }

    #[test]
    fn test_database_id_equality() {
        let id1 = DatabaseId::new("db");
        let id2 = DatabaseId::new("db");
        let id3 = DatabaseId::new("other");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}
