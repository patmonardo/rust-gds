// Phase 2.1: HadamardFeatureStep - Element-wise multiplication of node properties

use super::super::{LinkFeatureAppender, LinkFeatureStep};
use crate::types::graph::Graph;
use std::collections::HashMap;
use std::marker::PhantomData;

// TODO: Replace with real types
pub type NodePropertyValues = PhantomData<()>;
pub type UnionLinkFeatureAppender = PhantomData<()>;

/// Hadamard product (element-wise multiplication) link feature.
///
/// For node property vectors v1 and v2, computes:
/// ```text
/// hadamard(v1, v2) = [v1[0]*v2[0], v1[1]*v2[1], ..., v1[n]*v2[n]]
/// ```
///
/// # Use Case
///
/// Hadamard captures **feature interaction** - which dimensions correlate between nodes?
/// Most common link feature in practice. Works with:
/// - Scalar properties (long, double)
/// - Array properties (long[], float[], double[])
///
/// # Example
///
/// ```text
/// Node A: embedding = [0.5, 0.8, 0.2]
/// Node B: embedding = [0.3, 0.9, 0.1]
/// Hadamard: [0.15, 0.72, 0.02]
/// ```
///
/// High values in Hadamard indicate dimensions where both nodes have high values.
#[derive(Debug, Clone)]
pub struct HadamardFeatureStep {
    /// Node properties to compute Hadamard product on
    node_properties: Vec<String>,
}

impl HadamardFeatureStep {
    /// Creates a new HadamardFeatureStep for the given node properties.
    pub fn new(node_properties: Vec<String>) -> Self {
        Self { node_properties }
    }
}

impl LinkFeatureStep for HadamardFeatureStep {
    fn link_feature_appender(&self, _graph: &dyn Graph) -> Box<dyn LinkFeatureAppender> {
        // TODO: Implement HadamardFeatureAppenderFactory
        // For now, return placeholder
        Box::new(HadamardPlaceholderAppender)
    }

    fn name(&self) -> &str {
        "HADAMARD"
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
struct HadamardPlaceholderAppender;

impl LinkFeatureAppender for HadamardPlaceholderAppender {
    fn append_features(&self, _source: u64, _target: u64, _features: &mut [f64], _offset: usize) {
        // TODO: Implement Hadamard computation
    }

    fn dimension(&self) -> usize {
        0 // Placeholder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hadamard_creation() {
        let step = HadamardFeatureStep::new(vec!["embedding".to_string()]);
        assert_eq!(step.node_properties.len(), 1);
    }

    #[test]
    fn test_hadamard_name() {
        let step = HadamardFeatureStep::new(vec!["prop1".to_string()]);
        assert_eq!(step.name(), "HADAMARD");
    }

    #[test]
    fn test_hadamard_configuration() {
        let step = HadamardFeatureStep::new(vec!["prop1".to_string(), "prop2".to_string()]);

        let config = step.configuration();
        assert!(config.contains_key("nodeProperties"));
    }

    #[test]
    fn test_input_node_properties() {
        let props = vec!["embedding".to_string(), "features".to_string()];
        let step = HadamardFeatureStep::new(props.clone());

        assert_eq!(step.input_node_properties(), props);
    }

    #[test]
    fn test_multiple_properties() {
        let step = HadamardFeatureStep::new(vec![
            "embedding1".to_string(),
            "embedding2".to_string(),
            "embedding3".to_string(),
        ]);

        assert_eq!(step.input_node_properties().len(), 3);
        assert_eq!(step.name(), "HADAMARD");
    }

    #[test]
    fn test_clone() {
        let step1 = HadamardFeatureStep::new(vec!["prop".to_string()]);
        let step2 = step1.clone();

        assert_eq!(step1.name(), step2.name());
        assert_eq!(step1.input_node_properties(), step2.input_node_properties());
    }
}
