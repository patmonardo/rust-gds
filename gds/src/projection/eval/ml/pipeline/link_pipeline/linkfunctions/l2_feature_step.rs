// Phase 2.3: L2FeatureStep - Euclidean distance between node properties

use super::super::{LinkFeatureAppender, LinkFeatureStep};
use crate::types::graph::Graph;
use std::collections::HashMap;

/// L2 distance (Euclidean distance) link feature.
///
/// Computes squared Euclidean distance between node property vectors:
/// ```text
/// L2(v1, v2) = [(v1[0]-v2[0])², (v1[1]-v2[1])², ..., (v1[n]-v2[n])²]
/// ```
///
/// Note: Returns **squared differences** per dimension, not the final sqrt.
/// Classifier can learn the sqrt relationship if needed.
///
/// # Use Case
///
/// L2 measures **geometric distance** - how far apart are the vectors in space?
/// - Smaller values = nodes are closer (more similar)
/// - Larger values = nodes are farther (less similar)
///
/// Common for spatial embeddings where absolute position matters.
///
/// # Example
///
/// ```text
/// Node A: [3, 4, 0]
/// Node B: [0, 0, 0]
/// L2: [(3-0)², (4-0)², (0-0)²] = [9, 16, 0]
/// Full L2 distance = sqrt(9 + 16 + 0) = 5.0
/// ```
///
/// # Why Squared?
///
/// Returns squared differences per dimension instead of final sqrt because:
/// 1. More efficient (avoids sqrt computation)
/// 2. Preserves ordering (x² < y² ⟺ x < y for positive values)
/// 3. Gives classifier more flexibility (can learn nonlinear combinations)
#[derive(Debug, Clone)]
pub struct L2FeatureStep {
    /// Node properties to compute L2 distance on
    node_properties: Vec<String>,
}

impl L2FeatureStep {
    /// Creates a new L2FeatureStep for the given node properties.
    pub fn new(node_properties: Vec<String>) -> Self {
        Self { node_properties }
    }
}

impl LinkFeatureStep for L2FeatureStep {
    fn link_feature_appender(&self, _graph: &dyn Graph) -> Box<dyn LinkFeatureAppender> {
        // TODO: Implement L2LinkFeatureAppenderFactory
        // For now, return placeholder
        Box::new(L2PlaceholderAppender)
    }

    fn name(&self) -> &str {
        "L2"
    }

    fn configuration(&self) -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert(
            "nodeProperties".to_string(),
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
struct L2PlaceholderAppender;

impl LinkFeatureAppender for L2PlaceholderAppender {
    fn append_features(&self, _source: u64, _target: u64, _features: &mut [f64], _offset: usize) {
        // TODO: Implement L2 computation:
        // For each dimension i:
        //   features[offset + i] = (source[i] - target[i])²
        // Returns vector of squared differences (not summed, not sqrt'd)
    }

    fn dimension(&self) -> usize {
        0 // Placeholder - should match property dimension
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l2_creation() {
        let step = L2FeatureStep::new(vec!["embedding".to_string()]);
        assert_eq!(step.node_properties.len(), 1);
    }

    #[test]
    fn test_l2_name() {
        let step = L2FeatureStep::new(vec!["prop1".to_string()]);
        assert_eq!(step.name(), "L2");
    }

    #[test]
    fn test_l2_configuration() {
        let step = L2FeatureStep::new(vec!["prop1".to_string(), "prop2".to_string()]);

        let config = step.configuration();
        assert!(config.contains_key("nodeProperties"));
    }

    #[test]
    fn test_input_node_properties() {
        let props = vec!["embedding".to_string(), "features".to_string()];
        let step = L2FeatureStep::new(props.clone());

        assert_eq!(step.input_node_properties(), props);
    }

    #[test]
    fn test_multiple_properties() {
        let step = L2FeatureStep::new(vec![
            "pos_x".to_string(),
            "pos_y".to_string(),
            "pos_z".to_string(),
        ]);

        assert_eq!(step.input_node_properties().len(), 3);
        assert_eq!(step.name(), "L2");
    }

    #[test]
    fn test_clone() {
        let step1 = L2FeatureStep::new(vec!["prop".to_string()]);
        let step2 = step1.clone();

        assert_eq!(step1.name(), step2.name());
        assert_eq!(step1.input_node_properties(), step2.input_node_properties());
    }

    #[test]
    fn test_semantic_meaning() {
        // L2 measures geometric distance
        // Smaller squared differences = closer nodes = more similar
        let step = L2FeatureStep::new(vec!["position".to_string()]);
        assert_eq!(step.name(), "L2");
    }
}
