use std::collections::HashMap;
use std::fmt;

/// Class that holds node property dimensions.
/// It is null-safe and will never return a null value.
#[derive(Clone, Debug, PartialEq)]
pub struct DimensionsMap {
    /// Map of property keys to their optional dimensions
    actual_dimensions: HashMap<String, Option<usize>>,
}

impl DimensionsMap {
    /// Creates a new DimensionsMap.
    pub fn new(actual_dimensions: HashMap<String, Option<usize>>) -> Self {
        DimensionsMap { actual_dimensions }
    }

    /// Returns the dimension for the specified property, or None if no information exists.
    ///
    /// There are two cases when None will be returned:
    ///     1) The property doesn't exist.
    ///     2) The property exists, but dimension information is not known.
    pub fn get(&self, property_key: &str) -> Option<usize> {
        self.actual_dimensions.get(property_key).and_then(|&v| v)
    }

    /// Returns an empty DimensionsMap.
    pub fn empty() -> Self {
        DimensionsMap {
            actual_dimensions: HashMap::new(),
        }
    }
}

impl fmt::Display for DimensionsMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DimensionsMap{{actual_dimensions={:?}}}",
            self.actual_dimensions
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_existing_dimension() {
        let mut map = HashMap::new();
        map.insert("prop1".to_string(), Some(3));
        map.insert("prop2".to_string(), None);

        let dimensions = DimensionsMap::new(map);

        assert_eq!(dimensions.get("prop1"), Some(3));
        assert_eq!(dimensions.get("prop2"), None);
        assert_eq!(dimensions.get("prop3"), None);
    }

    #[test]
    fn test_empty() {
        let dimensions = DimensionsMap::empty();
        assert_eq!(dimensions.get("any_key"), None);
    }
}
