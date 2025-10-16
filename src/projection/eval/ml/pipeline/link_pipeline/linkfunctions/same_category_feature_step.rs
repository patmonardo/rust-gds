// Phase 2.4: SameCategoryStep - Binary categorical equality between node properties

use super::super::{LinkFeatureAppender, LinkFeatureStep};
use crate::types::graph::Graph;
use std::collections::HashMap;

/// SameCategory link feature - binary indicator of categorical equality.
///
/// Returns 1.0 if two nodes have the same value for a property, 0.0 otherwise:
/// ```text
/// SameCategory(node1, node2, property) = {
///     1.0  if node1.property == node2.property
///     0.0  otherwise
/// }
/// ```
///
/// # Use Case
///
/// Categorical features for link prediction - do two nodes share attributes?
/// - Community membership: same community = stronger link likelihood
/// - Type/Class: same type = homophily signal
/// - Label: same label = clustering evidence
///
/// # Multiple Properties
///
/// Returns **one binary feature per property** (not combined).
/// With 3 properties, dimension = 3 (one indicator each).
///
/// # Example
///
/// ```text
/// Node A: { department: "Engineering", country: "USA", role: "Dev" }
/// Node B: { department: "Engineering", country: "UK",  role: "Dev" }
///
/// SameCategory([department, country, role]) = [1.0, 0.0, 1.0]
///                                               ↑    ↑    ↑
///                                              same diff same
/// ```
///
/// # Empirical Concept (Given)
///
/// The **Given Form** is this empirical concept:
/// - **Representation**: The categorical equality check (==)
/// - **Second Representation**: The predicate captures "sameness" itself
/// - **Consciousness**: We represent equality as binary feature (1/0)
///
/// The **Pure Form** (Ideal) deferred:
/// - Type-specific equality predicates (Long, Double, String, etc.)
/// - Property value extraction from Graph
/// - Validation of numeric-only constraint
///
/// This is **Speculative Seeding** - the Given articulates the Pure within!
#[derive(Debug, Clone)]
pub struct SameCategoryStep {
    /// Node properties to check for categorical equality
    node_properties: Vec<String>,
}

impl SameCategoryStep {
    /// Creates a new SameCategoryStep for the given node properties.
    ///
    /// # Arguments
    ///
    /// * `node_properties` - List of property names to check equality on
    ///
    /// Each property will produce one binary feature (dimension = properties.len()).
    pub fn new(node_properties: Vec<String>) -> Self {
        Self { node_properties }
    }
}

impl LinkFeatureStep for SameCategoryStep {
    fn link_feature_appender(&self, _graph: &dyn Graph) -> Box<dyn LinkFeatureAppender> {
        // TODO: Implement sameCategoryPredicate() logic
        // For each property:
        //   - Extract NodePropertyValues from graph
        //   - Match on valueType (Long, Double)
        //   - Create predicate: (source, target) -> source.value == target.value
        //   - Validate numeric-only constraint
        // For now, return placeholder
        Box::new(SameCategoryPlaceholderAppender {
            dimension: self.node_properties.len(),
        })
    }

    fn name(&self) -> &str {
        "SAME_CATEGORY"
    }

    fn configuration(&self) -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        // Note: Java uses "nodeProperty" (singular) but stores list
        config.insert(
            "nodeProperty".to_string(),
            serde_json::json!(self.node_properties),
        );
        config
    }

    fn input_node_properties(&self) -> Vec<String> {
        self.node_properties.clone()
    }

    fn clone_box(&self) -> Box<dyn LinkFeatureStep> {
        Box::new(self.clone())
    }
}

// Placeholder appender for Gamma quality
struct SameCategoryPlaceholderAppender {
    dimension: usize,
}

impl LinkFeatureAppender for SameCategoryPlaceholderAppender {
    fn append_features(&self, _source: u64, _target: u64, _features: &mut [f64], _offset: usize) {
        // TODO: Implement categorical equality check:
        // For each property predicate:
        //   features[offset++] = if predicate(source, target) { 1.0 } else { 0.0 }
        //
        // Predicate logic (from sameCategoryPredicate):
        //   Match property.valueType():
        //     Long   -> source.longValue() == target.longValue()
        //     Double -> source.doubleValue() == target.doubleValue()
        //     Other  -> Error: "only supports numeric properties"
    }

    fn dimension(&self) -> usize {
        self.dimension // One binary feature per property
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_category_creation() {
        let step = SameCategoryStep::new(vec!["department".to_string()]);
        assert_eq!(step.node_properties.len(), 1);
    }

    #[test]
    fn test_same_category_name() {
        let step = SameCategoryStep::new(vec!["category".to_string()]);
        assert_eq!(step.name(), "SAME_CATEGORY");
    }

    #[test]
    fn test_same_category_configuration() {
        let step = SameCategoryStep::new(vec!["department".to_string(), "country".to_string()]);

        let config = step.configuration();
        // Java uses singular "nodeProperty" key
        assert!(config.contains_key("nodeProperty"));
    }

    #[test]
    fn test_input_node_properties() {
        let props = vec!["category".to_string(), "type".to_string()];
        let step = SameCategoryStep::new(props.clone());

        assert_eq!(step.input_node_properties(), props);
    }

    #[test]
    fn test_dimension_matches_property_count() {
        // Each property produces one binary feature
        let step = SameCategoryStep::new(vec![
            "prop1".to_string(),
            "prop2".to_string(),
            "prop3".to_string(),
        ]);

        assert_eq!(step.input_node_properties().len(), 3);
        // Placeholder appender should return dimension = 3
    }

    #[test]
    fn test_clone() {
        let step1 = SameCategoryStep::new(vec!["category".to_string()]);
        let step2 = step1.clone();

        assert_eq!(step1.name(), step2.name());
        assert_eq!(step1.input_node_properties(), step2.input_node_properties());
    }

    #[test]
    fn test_semantic_meaning() {
        // SameCategory checks categorical equality
        // 1.0 = same category (homophily)
        // 0.0 = different category
        let step = SameCategoryStep::new(vec!["community".to_string()]);
        assert_eq!(step.name(), "SAME_CATEGORY");
    }

    #[test]
    fn test_empirical_concept_representation() {
        // The Given Form: categorical equality as binary indicator
        // The Pure Form: type-specific equality predicates (deferred)
        // This is SPECULATIVE SEEDING - articulating the Pure within Given!
        let step = SameCategoryStep::new(vec!["department".to_string(), "role".to_string()]);

        // Container (Given): API surface articulated
        assert_eq!(step.name(), "SAME_CATEGORY");
        assert_eq!(step.input_node_properties().len(), 2);

        // Contained (Pure): Equality predicates deferred with TODOs
        // Second Representation: The concept of "sameness" itself!
    }
}
