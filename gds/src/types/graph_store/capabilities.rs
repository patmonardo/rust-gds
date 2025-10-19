//! Capabilities - Graph storage backend capabilities.

use std::collections::HashSet;

/// Capabilities that a graph storage backend can support.
#[derive(Clone, Debug)]
pub struct Capabilities {
    features: HashSet<String>,
}

impl Capabilities {
    // Creates a new Capabilities instance.
    pub fn new() -> Self {
        Self {
            features: HashSet::new(),
        }
    }

    // Creates a Capabilities instance with specified features.
    pub fn with_features<I, S>(features: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            features: features.into_iter().map(|s| s.into()).collect(),
        }
    }

    /// Adds a feature to the capabilities.
    ///
    /// # Arguments
    /// * `feature` - The feature name to add
    pub fn add_feature<S: Into<String>>(&mut self, feature: S) {
        self.features.insert(feature.into());
    }

    /// Checks if a feature is supported.
    ///
    /// # Arguments
    /// * `feature` - The feature name to check
    ///
    /// # Returns
    /// `true` if the feature is supported, `false` otherwise
    pub fn supports(&self, feature: &str) -> bool {
        self.features.contains(feature)
    }

    /// Checks if the backend can write data.
    pub fn can_write(&self) -> bool {
        self.supports("write")
    }

    /// Checks if the backend can delete data.
    pub fn can_delete(&self) -> bool {
        self.supports("delete")
    }

    /// Checks if the backend supports transient graphs.
    pub fn supports_transient(&self) -> bool {
        self.supports("transient")
    }

    /// Returns all supported features.
    pub fn features(&self) -> Vec<&str> {
        self.features.iter().map(|s| s.as_str()).collect()
    }
}

impl Default for Capabilities {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capabilities_new() {
        let caps = Capabilities::new();
        assert!(!caps.supports("write"));
        assert!(!caps.can_write());
    }

    #[test]
    fn test_capabilities_with_features() {
        let caps = Capabilities::with_features(vec!["write", "delete"]);
        assert!(caps.supports("write"));
        assert!(caps.supports("delete"));
        assert!(!caps.supports("transient"));
    }

    #[test]
    fn test_add_feature() {
        let mut caps = Capabilities::new();
        assert!(!caps.supports("write"));

        caps.add_feature("write");
        assert!(caps.supports("write"));
    }

    #[test]
    fn test_can_write() {
        let mut caps = Capabilities::new();
        assert!(!caps.can_write());

        caps.add_feature("write");
        assert!(caps.can_write());
    }

    #[test]
    fn test_can_delete() {
        let mut caps = Capabilities::new();
        assert!(!caps.can_delete());

        caps.add_feature("delete");
        assert!(caps.can_delete());
    }

    #[test]
    fn test_supports_transient() {
        let mut caps = Capabilities::new();
        assert!(!caps.supports_transient());

        caps.add_feature("transient");
        assert!(caps.supports_transient());
    }

    #[test]
    fn test_features() {
        let caps = Capabilities::with_features(vec!["write", "delete", "transient"]);
        let mut features = caps.features();
        features.sort();

        assert_eq!(features, vec!["delete", "transient", "write"]);
    }
}
