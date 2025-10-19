//! Array property extractor for ML in GDS.
//!
//! Translated from Java GDS ml-core ArrayPropertyExtractor.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::{ArrayFeatureExtractor, FeatureExtractor};

/// Extracts array features from node properties.
pub struct ArrayPropertyExtractor {
    dimension: usize,
    // TODO: Replace placeholder once Graph type is available.
    _graph: (),
    property_key: String,
    // TODO: Replace placeholder once NodePropertyValues are available.
    _node_property_values: (),
}

impl ArrayPropertyExtractor {
    pub(crate) fn _new(dimension: usize, graph: (), property_key: String) -> Self {
        Self {
            dimension,
            _graph: graph,
            property_key,
            _node_property_values: (),
        }
    }

    fn fetch_property_value(&self, node_id: u64) -> Option<Vec<f64>> {
        let _ = node_id;
        let _ = &self._node_property_values;
        todo!("Call NodePropertyValues::double_array_value once available.");
    }

    fn original_node_id(&self, node_id: u64) -> u64 {
        let _ = &self._graph;
        // TODO: Replace with graph.to_original_node_id(node_id) once Graph is available.
        node_id
    }
}

impl FeatureExtractor for ArrayPropertyExtractor {
    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl ArrayFeatureExtractor for ArrayPropertyExtractor {
    fn extract(&self, node_id: u64) -> Vec<f64> {
        let property_value = self.fetch_property_value(node_id).unwrap_or_else(|| {
            panic!(
                "Missing node property for property key `{}` on node with id `{}`. Consider using a default value in the property projection.",
                &self.property_key,
                self.original_node_id(node_id)
            )
        });

        if property_value.len() != self.dimension {
            panic!(
                "The property `{}` contains arrays of differing lengths `{}` and `{}`.",
                &self.property_key,
                property_value.len(),
                self.dimension
            );
        }

        if property_value.iter().any(|val| val.is_nan()) {
            panic!(
                "Node with ID `{}` has invalid feature property value NaN for property `{}`",
                self.original_node_id(node_id),
                &self.property_key
            );
        }

        property_value
    }
}
