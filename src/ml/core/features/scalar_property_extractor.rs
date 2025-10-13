//! Scalar property extractor for ML in GDS.
//!
//! Translated from Java GDS ml-core ScalarPropertyExtractor.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::{FeatureExtractor, ScalarFeatureExtractor};
// TODO: Uncomment when Graph and NodePropertyValues are available
// use crate::types::graph::Graph;
// use crate::types::properties::NodePropertyValues;

/// Extracts scalar features from node properties.
///
/// This corresponds to the ScalarPropertyExtractor class in Java GDS.
/// Package-private constructor in Java - use through FeatureExtraction.propertyExtractors().
pub struct ScalarPropertyExtractor {
    // TODO: Uncomment when dependencies available
    // graph: Graph,
    // property_key: String,
    // node_property_values: NodePropertyValues,
    _placeholder: (),
}

impl ScalarPropertyExtractor {
    /// Create a new scalar property extractor.
    ///
    /// Package-private in Java (no pub visibility modifier).
    /// Note: This is a placeholder until Graph type is available.
    #[allow(dead_code)]
    pub(crate) fn new(_graph: (), _property_key: String) -> Self {
        // TODO: Implement when Graph type available
        // let node_property_values = graph.node_properties(&property_key);
        Self {
            // graph,
            // property_key,
            // node_property_values,
            _placeholder: (),
        }
    }
}

impl FeatureExtractor for ScalarPropertyExtractor {
    fn dimension(&self) -> usize {
        1
    }
}

impl ScalarFeatureExtractor for ScalarPropertyExtractor {
    fn extract(&self, _node_id: u64) -> f64 {
        // TODO: Implement when NodePropertyValues available
        // let property_value = self.node_property_values.double_value(node_id);
        // if property_value.is_nan() {
        //     panic!(
        //         "Node with ID `{}` has invalid feature property value `NaN` for property `{}`",
        //         self.graph.to_original_node_id(node_id),
        //         self.property_key
        //     );
        // }
        // property_value

        // Placeholder
        0.0
    }
}

// TODO: Add tests when Graph mock is available
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        // Placeholder test until dependencies are ready
        let extractor = ScalarPropertyExtractor::new((), "test".to_string());
        assert_eq!(FeatureExtractor::dimension(&extractor), 1);
    }
}
