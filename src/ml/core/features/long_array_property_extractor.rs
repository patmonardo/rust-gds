//! Long array property extractor for ML in GDS.
//!
//! Translated from Java GDS ml-core LongArrayPropertyExtractor.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::{ArrayFeatureExtractor, FeatureExtractor};

/// Array feature extractor backed by long-valued node properties.
pub struct LongArrayPropertyExtractor {
    dimension: usize,
    // TODO: Replace placeholder once Graph type is available.
    _graph: (),
    property_key: String,
}

impl LongArrayPropertyExtractor {
    pub(crate) fn new(dimension: usize, graph: (), property_key: String) -> Self {
        Self {
            dimension,
            _graph: graph,
            property_key,
        }
    }
}

impl FeatureExtractor for LongArrayPropertyExtractor {
    fn dimension(&self) -> usize {
        self.dimension
    }
}

impl ArrayFeatureExtractor for LongArrayPropertyExtractor {
    fn extract(&self, node_id: u64) -> Vec<f64> {
        // TODO: Implement using EmbeddingUtils once graph and property system are available.
        let _ = node_id;
        let _ = &self.property_key;
        vec![0.0; self.dimension]
    }
}
