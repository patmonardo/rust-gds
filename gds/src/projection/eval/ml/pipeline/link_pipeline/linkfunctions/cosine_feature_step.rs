// Phase 2.2: CosineFeatureStep - Angular similarity via cosine distance

use super::super::{LinkFeatureAppender, LinkFeatureStep};
use crate::types::graph::Graph;
use std::collections::HashMap;

/// Cosine similarity link feature.
///
/// Computes cosine similarity between node property vectors:
/// ```text
/// cosine(v1, v2) = dot(v1, v2) / (||v1|| * ||v2||)
///                = Σ(v1[i] * v2[i]) / sqrt(Σ(v1[i]²) * Σ(v2[i]²))
/// ```
///
/// # Use Case
///
/// Cosine measures **angular similarity** - do vectors point in the same direction?
/// - Range: [-1, 1] (or [0, 1] for positive vectors)
/// - 1.0 = same direction (perfectly similar)
/// - 0.0 = orthogonal (no similarity)
/// - -1.0 = opposite direction (perfectly dissimilar)
///
/// Common for embeddings where magnitude doesn't matter, only direction.
///
/// # Example
///
/// ```text
/// Node A: [3, 4, 0]  (||A|| = 5)
/// Node B: [6, 8, 0]  (||B|| = 10)
/// Cosine: (18 + 32) / (5 * 10) = 50/50 = 1.0 (same direction!)
/// ```
///
/// # Implementation Note
///
/// Computes dot product and norms in single pass for efficiency:
/// - Accumulate: dot_product, source_norm², target_norm²
/// - Final: dot_product / sqrt(source_norm² * target_norm²)
#[derive(Debug, Clone)]
pub struct CosineFeatureStep {
    /// Node properties to compute cosine similarity on
    node_property_names: Vec<String>,
}

impl CosineFeatureStep {
    /// Creates a new CosineFeatureStep for the given node properties.
    pub fn new(node_properties: Vec<String>) -> Self {
        Self {
            node_property_names: node_properties,
        }
    }
}

impl LinkFeatureStep for CosineFeatureStep {
    fn link_feature_appender(&self, _graph: &dyn Graph) -> Box<dyn LinkFeatureAppender> {
        // TODO: Implement PartialL2WithNormsComputer for each property type
        // For now, return placeholder
        Box::new(CosinePlaceholderAppender)
    }

    fn name(&self) -> &str {
        "COSINE"
    }

    fn configuration(&self) -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert(
            "nodeProperties".to_string(),
            serde_json::json!(self.node_property_names),
        );
        config
    }

    fn input_node_properties(&self) -> Vec<String> {
        self.node_property_names.clone()
    }

    fn clone_box(&self) -> Box<dyn LinkFeatureStep> {
        Box::new(self.clone())
    }
}

// Placeholder appender for Gamma quality
struct CosinePlaceholderAppender;

impl LinkFeatureAppender for CosinePlaceholderAppender {
    fn append_features(&self, _source: u64, _target: u64, _features: &mut [f64], _offset: usize) {
        // TODO: Implement cosine computation:
        // 1. Accumulate dot_product, source_square_norm, target_square_norm
        // 2. Compute l2_norm = sqrt(source_square_norm * target_square_norm)
        // 3. If l2_norm != 0.0: features[offset] = dot_product / l2_norm
        // 4. Validate not NaN
    }

    fn dimension(&self) -> usize {
        1 // Cosine returns single similarity value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_creation() {
        let step = CosineFeatureStep::new(vec!["embedding".to_string()]);
        assert_eq!(step.node_property_names.len(), 1);
    }

    #[test]
    fn test_cosine_name() {
        let step = CosineFeatureStep::new(vec!["prop1".to_string()]);
        assert_eq!(step.name(), "COSINE");
    }

    #[test]
    fn test_cosine_configuration() {
        let step = CosineFeatureStep::new(vec!["prop1".to_string(), "prop2".to_string()]);

        let config = step.configuration();
        assert!(config.contains_key("nodeProperties"));
    }

    #[test]
    fn test_input_node_properties() {
        let props = vec!["embedding".to_string(), "features".to_string()];
        let step = CosineFeatureStep::new(props.clone());

        assert_eq!(step.input_node_properties(), props);
    }

    #[test]
    fn test_dimension_is_one() {
        // Cosine similarity always returns single scalar value
        let appender = CosinePlaceholderAppender;
        assert_eq!(appender.dimension(), 1);
    }

    #[test]
    fn test_multiple_properties() {
        // Cosine can combine multiple properties
        // (computes overall cosine across concatenated vectors)
        let step = CosineFeatureStep::new(vec!["embedding1".to_string(), "embedding2".to_string()]);

        assert_eq!(step.input_node_properties().len(), 2);
    }

    #[test]
    fn test_clone() {
        let step1 = CosineFeatureStep::new(vec!["prop".to_string()]);
        let step2 = step1.clone();

        assert_eq!(step1.name(), step2.name());
        assert_eq!(step1.input_node_properties(), step2.input_node_properties());
    }
}
