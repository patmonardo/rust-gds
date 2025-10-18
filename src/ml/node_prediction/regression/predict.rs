//! Node regression prediction
//! 1:1 translation of NodeRegressionPredict.java

use crate::{
    collections::HugeDoubleArray,
    ml::models::{Features, Regressor},
};
use std::sync::Arc;

/// Performs regression prediction on nodes
/// 1:1 with NodeRegressionPredict.java
pub struct NodeRegressionPredict {
    regressor: Arc<dyn Regressor>,
    features: Arc<dyn Features>,
}

impl NodeRegressionPredict {
    /// Creates a new regression predictor
    pub fn new(regressor: Arc<dyn Regressor>, features: Arc<dyn Features>) -> Self {
        Self {
            regressor,
            features,
        }
    }

    /// Computes predictions for all nodes
    /// 1:1 with compute() in Java
    pub fn compute(&self) -> HugeDoubleArray {
        let mut predicted_targets = HugeDoubleArray::new(self.features.size());

        // Sequential prediction (parallel version with infrastructure comes later)
        for id in 0..self.features.size() {
            let feature_vec = self.features.get(id);
            let prediction = self.regressor.predict(feature_vec);
            predicted_targets.set(id, prediction);
        }

        predicted_targets
    }
}
