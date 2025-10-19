use super::element_projection::ElementProjection;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

/// Base trait for collections of element projections.
///
/// Manages a collection of projections indexed by element identifiers
/// (node labels or relationship types).
pub trait AbstractProjections<I, P>
where
    I: Clone + Eq + Hash + Debug,
    P: Debug,
{
    /// Returns the underlying projections map.
    fn projections(&self) -> &HashMap<I, P>;

    /// Returns all property keys used in any projection.
    fn all_properties(&self) -> HashSet<String>
    where
        P: AsRef<dyn ElementProjection>,
    {
        let mut property_keys = HashSet::new();

        for projection in self.projections().values() {
            let proj_ref = projection.as_ref();
            for mapping in proj_ref.properties().mappings() {
                property_keys.insert(mapping.property_key().to_string());
            }
        }

        property_keys
    }

    /// Returns all projections as a collection.
    fn all_projections(&self) -> Vec<&P>
    where
        I: 'static,
        P: 'static,
    {
        self.projections().values().collect()
    }

    /// Checks if the projections include an identifier.
    ///
    /// # Arguments
    /// * `identifier` - The identifier to check for
    fn contains_key(&self, identifier: &I) -> bool {
        self.projections().contains_key(identifier)
    }

    /// Returns the projection for an identifier.
    ///
    /// # Arguments
    /// * `identifier` - The identifier to get the projection for
    fn get(&self, identifier: &I) -> Option<&P>
    where
        I: 'static,
        P: 'static,
    {
        self.projections().get(identifier)
    }

    /// Returns the number of projections.
    fn size(&self) -> usize {
        self.projections().len()
    }

    /// Checks if there are no projections.
    fn is_empty(&self) -> bool {
        self.projections().is_empty()
    }

    /// Returns all identifiers in this collection.
    fn identifiers(&self) -> Vec<&I>
    where
        I: 'static,
        P: 'static,
    {
        self.projections().keys().collect()
    }
}

/// Concrete implementation of AbstractProjections.
///
/// Provides a HashMap-backed storage for element projections.
#[derive(Debug, Clone)]
pub struct Projections<I, P>
where
    I: Clone + Eq + Hash + Debug,
    P: Debug,
{
    projections: HashMap<I, P>,
}

impl<I, P> Projections<I, P>
where
    I: Clone + Eq + Hash + Debug,
    P: Debug,
{
    /// Creates a new empty Projections.
    pub fn empty() -> Self {
        Projections {
            projections: HashMap::new(),
        }
    }

    /// Creates a new Projections from a map.
    pub fn new(projections: HashMap<I, P>) -> Self {
        Projections { projections }
    }

    /// Creates a builder for Projections.
    pub fn builder() -> ProjectionsBuilder<I, P> {
        ProjectionsBuilder::new()
    }

    /// Adds a projection for an identifier.
    pub fn insert(&mut self, identifier: I, projection: P) {
        self.projections.insert(identifier, projection);
    }

    /// Removes a projection for an identifier.
    pub fn remove(&mut self, identifier: &I) -> Option<P> {
        self.projections.remove(identifier)
    }
}

impl<I, P> AbstractProjections<I, P> for Projections<I, P>
where
    I: Clone + Eq + Hash + Debug,
    P: Debug,
{
    fn projections(&self) -> &HashMap<I, P> {
        &self.projections
    }
}

impl<I, P> Default for Projections<I, P>
where
    I: Clone + Eq + Hash + Debug,
    P: Debug,
{
    fn default() -> Self {
        Self::empty()
    }
}

/// Builder for Projections with fluent API.
#[derive(Debug)]
pub struct ProjectionsBuilder<I, P>
where
    I: Clone + Eq + Hash + Debug,
    P: Debug,
{
    projections: HashMap<I, P>,
}

impl<I, P> ProjectionsBuilder<I, P>
where
    I: Clone + Eq + Hash + Debug,
    P: Debug,
{
    /// Creates a new empty builder.
    pub fn new() -> Self {
        ProjectionsBuilder {
            projections: HashMap::new(),
        }
    }

    /// Adds a projection for an identifier.
    pub fn add(mut self, identifier: I, projection: P) -> Self {
        self.projections.insert(identifier, projection);
        self
    }

    /// Adds a projection for an identifier (mutable reference).
    pub fn add_ref(&mut self, identifier: I, projection: P) -> &mut Self {
        self.projections.insert(identifier, projection);
        self
    }

    /// Adds all projections from a map.
    pub fn add_all(mut self, projections: HashMap<I, P>) -> Self {
        self.projections.extend(projections);
        self
    }

    /// Adds all projections from an iterator.
    pub fn add_iter(mut self, projections: impl IntoIterator<Item = (I, P)>) -> Self {
        self.projections.extend(projections);
        self
    }

    /// Builds the Projections.
    pub fn build(self) -> Projections<I, P> {
        Projections {
            projections: self.projections,
        }
    }
}

impl<I, P> Default for ProjectionsBuilder<I, P>
where
    I: Clone + Eq + Hash + Debug,
    P: Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock identifier for testing
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct TestId(String);

    // Mock projection for testing
    #[derive(Debug, Clone)]
    struct TestProjection {
        name: String,
    }

    #[test]
    fn test_projections_empty() {
        let projections: Projections<TestId, TestProjection> = Projections::empty();
        assert!(projections.is_empty());
        assert_eq!(projections.size(), 0);
    }

    #[test]
    fn test_projections_builder() {
        let projections = Projections::builder()
            .add(
                TestId("id1".to_string()),
                TestProjection {
                    name: "proj1".to_string(),
                },
            )
            .add(
                TestId("id2".to_string()),
                TestProjection {
                    name: "proj2".to_string(),
                },
            )
            .build();

        assert_eq!(projections.size(), 2);
        assert!(projections.contains_key(&TestId("id1".to_string())));
        assert!(projections.contains_key(&TestId("id2".to_string())));
    }

    #[test]
    fn test_projections_get() {
        let mut projections = Projections::empty();
        projections.insert(
            TestId("id1".to_string()),
            TestProjection {
                name: "proj1".to_string(),
            },
        );

        let proj = projections.get(&TestId("id1".to_string()));
        assert!(proj.is_some());
        assert_eq!(proj.unwrap().name, "proj1");
    }

    #[test]
    fn test_projections_remove() {
        let mut projections = Projections::empty();
        projections.insert(
            TestId("id1".to_string()),
            TestProjection {
                name: "proj1".to_string(),
            },
        );

        assert_eq!(projections.size(), 1);

        let removed = projections.remove(&TestId("id1".to_string()));
        assert!(removed.is_some());
        assert_eq!(projections.size(), 0);
    }

    #[test]
    fn test_projections_identifiers() {
        let projections = Projections::builder()
            .add(
                TestId("id1".to_string()),
                TestProjection {
                    name: "proj1".to_string(),
                },
            )
            .add(
                TestId("id2".to_string()),
                TestProjection {
                    name: "proj2".to_string(),
                },
            )
            .build();

        let ids = projections.identifiers();
        assert_eq!(ids.len(), 2);
    }
}
