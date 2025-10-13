//! Feature extraction utilities for ML in GDS.
//!
//! Translated from Java GDS ml-core FeatureExtraction.java.
//! This is a literal 1:1 translation following repository translation policy.
//!
//! Responsible for extracting features into abstract consumers (FeatureConsumer).
//! Also contains logic for looping on graphs and batches and writing into
//! Matrices and HugeObjectArrays.

use super::{ArrayFeatureExtractor, FeatureConsumer, FeatureExtractor, ScalarFeatureExtractor};
// Note: These imports will need to be completed once we translate the full ml/core module
// use crate::ml::core::batch::Batch;
// use crate::ml::core::functions::Constant;
// use crate::ml::core::tensor::Matrix;
// use crate::types::graph::Graph;
// use crate::collections::HugeObjectArray;

/// Extract features for a single node using a list of extractors.
///
/// Corresponds to: `FeatureExtraction.extract(long nodeId, long nodeOffset,
///                  List<FeatureExtractor> extractors, FeatureConsumer consumer)`
pub fn extract(
    node_id: u64,
    node_offset: u64,
    extractors: &[&dyn FeatureExtractor],
    consumer: &mut dyn FeatureConsumer,
) {
    let mut offset = 0;
    for extractor in extractors {
        // Type dispatch based on trait - Rust equivalent of instanceof
        if let Some(scalar_extractor) = downcast_to_scalar(*extractor) {
            consumer.accept_scalar(node_offset, offset, scalar_extractor.extract(node_id));
        } else if let Some(array_extractor) = downcast_to_array(*extractor) {
            consumer.accept_array(node_offset, offset, &array_extractor.extract(node_id));
        } else {
            panic!("Only ScalarFeatureExtractor and ArrayFeatureExtractor are handled");
        }
        offset += extractor.dimension();
    }
}

// TODO: Translate these methods once dependencies are available:
// pub fn extract_batch(batch: &Batch, extractors: &[&dyn FeatureExtractor]) -> Constant<Matrix>
// pub fn extract_graph(graph: &Graph, extractors: &[&dyn FeatureExtractor], features: &mut HugeObjectArray<Vec<f64>>) -> &mut HugeObjectArray<Vec<f64>>

/// Calculate total feature count from a collection of extractors.
///
/// Corresponds to: `FeatureExtraction.featureCount(Collection<FeatureExtractor> extractors)`
pub fn feature_count(extractors: &[&dyn FeatureExtractor]) -> usize {
    extractors.iter().map(|e| e.dimension()).sum()
}

// TODO: Translate these utility methods once Graph and property system are available:
// pub fn feature_count_from_graph(graph: &Graph, feature_properties: &[String]) -> usize
// pub fn property_extractors(graph: &Graph, feature_properties: &[String]) -> Vec<Box<dyn FeatureExtractor>>
// pub fn property_extractors_with_init(graph: &Graph, feature_properties: &[String], init_node_id: u64) -> Vec<Box<dyn FeatureExtractor>>
// pub fn feature_count_with_bias(graph: &Graph, feature_properties: &[String]) -> usize
// pub fn memory_usage_in_bytes(number_of_features: usize) -> usize

/// Downcast FeatureExtractor to ScalarFeatureExtractor (Rust equivalent of instanceof).
fn downcast_to_scalar(_extractor: &dyn FeatureExtractor) -> Option<&dyn ScalarFeatureExtractor> {
    // This will be implemented properly once we have Any trait on FeatureExtractor
    // For now, this is a placeholder for the instanceof pattern
    None
}

/// Downcast FeatureExtractor to ArrayFeatureExtractor (Rust equivalent of instanceof).
fn downcast_to_array(_extractor: &dyn FeatureExtractor) -> Option<&dyn ArrayFeatureExtractor> {
    // This will be implemented properly once we have Any trait on FeatureExtractor
    // For now, this is a placeholder for the instanceof pattern
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockScalarExtractor;
    impl FeatureExtractor for MockScalarExtractor {
        fn dimension(&self) -> usize {
            1
        }
    }
    impl ScalarFeatureExtractor for MockScalarExtractor {
        fn extract(&self, _node_id: u64) -> f64 {
            42.0
        }
    }

    #[test]
    fn test_feature_count() {
        let extractors: Vec<&dyn FeatureExtractor> =
            vec![&MockScalarExtractor, &MockScalarExtractor];
        assert_eq!(feature_count(&extractors), 2);
    }
}
