use super::property::Property;
use std::collections::HashMap;
use thiserror::Error;

/// Error type for property store operations.
#[derive(Error, Debug, Clone)]
pub enum PropertyStoreError {
    #[error("Property not found: {0}")]
    PropertyNotFound(String),

    #[error("Property already exists: {0}")]
    PropertyAlreadyExists(String),

    #[error("Invalid property key: {0}")]
    InvalidPropertyKey(String),
}

pub type PropertyStoreResult<T> = Result<T, PropertyStoreError>;

/// A store that contains properties indexed by their property key.
///
/// This mirrors the TypeScript PropertyStore interface.
/// Provides default implementations for common map-like operations.
pub trait PropertyStore: Send + Sync {
    type Property: Property;

    /// Returns a reference to the properties map.
    /// This is the only method concrete stores must implement.
    fn properties(&self) -> &HashMap<String, Self::Property>;

    /// Gets a property by key.
    fn get(&self, property_key: &str) -> Option<&Self::Property> {
        self.properties().get(property_key)
    }

    /// Checks if the property store is empty.
    fn is_empty(&self) -> bool {
        self.properties().is_empty()
    }

    /// Returns the number of properties in the store.
    fn len(&self) -> usize {
        self.properties().len()
    }

    /// Returns the set of property keys in this store.
    fn key_set(&self) -> Vec<&str> {
        self.properties().keys().map(|s| s.as_str()).collect()
    }

    /// Checks if the store contains a property with the given key.
    fn contains_key(&self, property_key: &str) -> bool {
        self.properties().contains_key(property_key)
    }
}
