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

/// Type-erased feature extractor wrapper.
///
/// **Rust Pattern**: Enum dispatch instead of Java's instanceof.
/// This is more idiomatic Rust and avoids dynamic casting.
pub enum AnyFeatureExtractor {
    Scalar(Box<dyn ScalarFeatureExtractor>),
    Array(Box<dyn ArrayFeatureExtractor>),
}

impl FeatureExtractor for AnyFeatureExtractor {
    fn dimension(&self) -> usize {
        match self {
            AnyFeatureExtractor::Scalar(_) => 1,
            AnyFeatureExtractor::Array(extractor) => extractor.dimension(),
        }
    }
}

impl AnyFeatureExtractor {
    /// Extract features into a consumer (replaces Java's instanceof dispatch).
    pub fn extract_into(
        &self,
        node_id: u64,
        node_offset: u64,
        offset: usize,
        consumer: &mut dyn FeatureConsumer,
    ) {
        match self {
            AnyFeatureExtractor::Scalar(extractor) => {
                let value = extractor.extract(node_id);
                consumer.accept_scalar(node_offset, offset, value);
            }
            AnyFeatureExtractor::Array(extractor) => {
                let values = extractor.extract(node_id);
                consumer.accept_array(node_offset, offset, &values);
            }
        }
    }
}

/// Extract features for a single node using a list of extractors.
///
/// Corresponds to: `FeatureExtraction.extract(long nodeId, long nodeOffset,
///                  List<FeatureExtractor> extractors, FeatureConsumer consumer)`
pub fn extract(
    node_id: u64,
    node_offset: u64,
    extractors: &[AnyFeatureExtractor],
    consumer: &mut dyn FeatureConsumer,
) {
    let mut offset = 0;
    for extractor in extractors {
        extractor.extract_into(node_id, node_offset, offset, consumer);
        offset += extractor.dimension();
    }
}

// TODO: Translate these methods once dependencies are available:
// pub fn extract_batch(batch: &Batch, extractors: &[AnyFeatureExtractor]) -> Constant<Matrix>
// pub fn extract_graph(graph: &Graph, extractors: &[AnyFeatureExtractor], features: &mut HugeObjectArray<Vec<f64>>) -> &mut HugeObjectArray<Vec<f64>>

/// Calculate total feature count from a collection of extractors.
///
/// Corresponds to: `FeatureExtraction.featureCount(Collection<FeatureExtractor> extractors)`
pub fn feature_count(extractors: &[AnyFeatureExtractor]) -> usize {
    extractors.iter().map(|e| e.dimension()).sum()
}

// TODO: Translate these utility methods once Graph and property system are available:
// pub fn feature_count_from_graph(graph: &Graph, feature_properties: &[String]) -> usize
// pub fn property_extractors(graph: &Graph, feature_properties: &[String]) -> Vec<AnyFeatureExtractor>
// pub fn property_extractors_with_init(graph: &Graph, feature_properties: &[String], init_node_id: u64) -> Vec<AnyFeatureExtractor>
// pub fn feature_count_with_bias(graph: &Graph, feature_properties: &[String]) -> usize
// pub fn memory_usage_in_bytes(number_of_features: usize) -> usize

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

    struct MockArrayExtractor {
        dim: usize,
    }
    impl FeatureExtractor for MockArrayExtractor {
        fn dimension(&self) -> usize {
            self.dim
        }
    }
    impl ArrayFeatureExtractor for MockArrayExtractor {
        fn extract(&self, _node_id: u64) -> Vec<f64> {
            vec![1.0; self.dim]
        }
    }

    #[test]
    fn test_feature_count() {
        let extractors = vec![
            AnyFeatureExtractor::Scalar(Box::new(MockScalarExtractor)),
            AnyFeatureExtractor::Scalar(Box::new(MockScalarExtractor)),
        ];
        assert_eq!(feature_count(&extractors), 2);
    }

    #[test]
    fn test_feature_count_mixed() {
        let extractors = vec![
            AnyFeatureExtractor::Scalar(Box::new(MockScalarExtractor)),
            AnyFeatureExtractor::Array(Box::new(MockArrayExtractor { dim: 3 })),
            AnyFeatureExtractor::Scalar(Box::new(MockScalarExtractor)),
        ];
        assert_eq!(feature_count(&extractors), 5); // 1 + 3 + 1
    }

    #[test]
    fn test_extract() {
        struct TestConsumer {
            scalars: Vec<(u64, usize, f64)>,
            arrays: Vec<(u64, usize, Vec<f64>)>,
        }
        impl FeatureConsumer for TestConsumer {
            fn accept_scalar(&mut self, node_offset: u64, offset: usize, value: f64) {
                self.scalars.push((node_offset, offset, value));
            }
            fn accept_array(&mut self, node_offset: u64, offset: usize, values: &[f64]) {
                self.arrays.push((node_offset, offset, values.to_vec()));
            }
        }

        let extractors = vec![
            AnyFeatureExtractor::Scalar(Box::new(MockScalarExtractor)),
            AnyFeatureExtractor::Array(Box::new(MockArrayExtractor { dim: 2 })),
        ];

        let mut consumer = TestConsumer {
            scalars: Vec::new(),
            arrays: Vec::new(),
        };

        extract(100, 0, &extractors, &mut consumer);

        assert_eq!(consumer.scalars.len(), 1);
        assert_eq!(consumer.scalars[0], (0, 0, 42.0));

        assert_eq!(consumer.arrays.len(), 1);
        assert_eq!(consumer.arrays[0].0, 0); // node_offset
        assert_eq!(consumer.arrays[0].1, 1); // offset (after scalar)
        assert_eq!(consumer.arrays[0].2, vec![1.0, 1.0]);
    }
}
