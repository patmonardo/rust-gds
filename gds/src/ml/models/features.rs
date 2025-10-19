//! Features module - 1:1 translation of Features.java and FeaturesFactory.java
//!
//! The Features trait is the core interface for accessing feature data.
//! Moved from NodeId-based API to index-based API to match Java.

use crate::ml::core::tensor::Vector;

/// Features trait - 1:1 with Features.java
/// This trait is implemented in base.rs as well, keeping this for re-export
pub use super::base::Features;

/// Dense in-memory feature storage
#[derive(Clone, Debug)]
pub struct DenseFeatures {
    data: Vec<Vec<f64>>,
}

impl DenseFeatures {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        Self { data }
    }

    pub fn from_vectors(vectors: Vec<Vector>) -> Self {
        let data = vectors.into_iter().map(|v| v.to_vec()).collect();
        Self { data }
    }
}

impl Features for DenseFeatures {
    fn size(&self) -> usize {
        self.data.len()
    }

    fn get(&self, id: usize) -> &[f64] {
        &self.data[id]
    }
}

/// Lazy feature extraction from graph properties
/// TODO: Implement lazy extraction following FeaturesFactory.extractLazyFeatures()
pub struct LazyFeatures {
    // Will hold graph reference and property extractors
    size: usize,
    feature_dimension: usize,
}

impl Features for LazyFeatures {
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, _id: usize) -> &[f64] {
        // TODO: Implement lazy extraction
        unimplemented!("Lazy feature extraction not yet implemented")
    }

    fn feature_dimension(&self) -> usize {
        self.feature_dimension
    }
}

/// Features factory - 1:1 with FeaturesFactory.java
pub struct FeaturesFactory;

impl FeaturesFactory {
    /// Wrap a HugeObjectArray of feature vectors
    /// 1:1 with wrap(HugeObjectArray<double[]>) in Java
    pub fn wrap_array(features: Vec<Vec<f64>>) -> Box<dyn Features> {
        Box::new(DenseFeatures::new(features))
    }

    /// Wrap a single feature vector
    /// 1:1 with wrap(double[]) in Java
    pub fn wrap_single(features: Vec<f64>) -> Box<dyn Features> {
        Box::new(DenseFeatures::new(vec![features]))
    }

    /// Wrap a list of feature vectors
    /// 1:1 with wrap(List<double[]>) in Java
    pub fn wrap_list(features: Vec<Vec<f64>>) -> Box<dyn Features> {
        Box::new(DenseFeatures::new(features))
    }

    // TODO: Add extractLazyFeatures and extractEagerFeatures methods
    // These require graph property extraction infrastructure
}
