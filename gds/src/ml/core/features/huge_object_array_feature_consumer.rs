//! Huge Object Array feature consumer for ML in GDS.
//!
//! Translated from Java GDS ml-core HugeObjectArrayFeatureConsumer.java.
//! This is a literal 1:1 translation following repository translation policy.

use super::FeatureConsumer;

/// Feature consumer backed by a HugeObjectArray of feature vectors.
pub struct HugeObjectArrayFeatureConsumer {
    // TODO: Replace placeholder once HugeObjectArray<double[]> binding is available.
    features: (),
}

impl HugeObjectArrayFeatureConsumer {
    pub fn new(features: ()) -> Self {
        Self { features }
    }
}

impl FeatureConsumer for HugeObjectArrayFeatureConsumer {
    fn accept_scalar(&mut self, node_offset: u64, offset: usize, value: f64) {
        let _ = (node_offset, offset, value);
        let _ = &self.features;
        // TODO: Implement when HugeObjectArray is available.
    }

    fn accept_array(&mut self, node_offset: u64, offset: usize, values: &[f64]) {
        let _ = (node_offset, offset, values);
        let _ = &self.features;
        // TODO: Implement when HugeObjectArray is available.
    }
}
