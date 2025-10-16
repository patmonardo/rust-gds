// Phase 4.2: LinkPredictionPredictPipeline - Prediction pipeline for link prediction

use super::LinkFeatureStep;
use crate::projection::eval::ml::pipeline::{ExecutableNodePropertyStep, Pipeline};
use std::collections::HashMap;
use std::marker::PhantomData;

/// Prediction pipeline for link prediction.
///
/// # The Given Form! 🌟
///
/// This is the **Given** (materialized) pipeline used during **prediction** (not training).
/// It contains the **frozen** steps from training.
///
/// **CAR:CDR at Pipeline Level**:
/// - **CAR (Given)**: LinkPredictionPredictPipeline - frozen, immutable steps
/// - **CDR (Process)**: Training created this Given form
/// - **Science**: Training Pipeline (Pure) → Predict Pipeline (Given)
///
/// # The Immutable Container
///
/// Unlike TrainingPipeline (which mutates), this is **immutable**:
/// - No add_feature_step()
/// - No split config (training-only)
/// - No parameter space
/// - Just: nodePropertySteps + featureSteps (frozen!)
///
/// # Example
///
/// ```text
/// // From a training pipeline
/// let predict_pipeline = LinkPredictionPredictPipeline::from_pipeline(&training_pipeline);
///
/// // Or from steps directly
/// let predict_pipeline = LinkPredictionPredictPipeline::from_steps(
///     vec![/* node property steps */],
///     vec![/* feature steps */]
/// );
/// ```
pub struct LinkPredictionPredictPipeline {
    /// Node property steps (preprocessing) - frozen from training
    /// TODO: Use actual ExecutableNodePropertyStep type
    node_property_steps: PhantomData<Vec<Box<dyn ExecutableNodePropertyStep>>>,

    /// Link feature steps (feature extraction) - frozen from training
    feature_steps: Vec<Box<dyn LinkFeatureStep>>,
}

impl LinkPredictionPredictPipeline {
    /// Creates an empty predict pipeline.
    ///
    /// # The Empty Given!
    ///
    /// This is the **minimal Given** - no steps, ready to be filled.
    pub fn empty() -> Self {
        Self {
            node_property_steps: PhantomData,
            feature_steps: Vec::new(),
        }
    }

    /// Creates a new LinkPredictionPredictPipeline from steps.
    ///
    /// # The Given Constructor!
    ///
    /// This creates the **Given** (frozen) pipeline from steps.
    ///
    /// # Arguments
    ///
    /// * `node_property_steps` - Preprocessing steps (placeholder)
    /// * `feature_steps` - Feature extraction steps
    pub fn from_steps(
        _node_property_steps: PhantomData<Vec<Box<dyn ExecutableNodePropertyStep>>>,
        feature_steps: Vec<Box<dyn LinkFeatureStep>>,
    ) -> Self {
        Self {
            node_property_steps: PhantomData,
            feature_steps,
        }
    }

    /// Creates a LinkPredictionPredictPipeline from a training pipeline.
    ///
    /// # The CAR→CDR Transformation!
    ///
    /// This **freezes** the training pipeline into a prediction pipeline:
    /// - Training Pipeline (Pure, mutable) → Predict Pipeline (Given, immutable)
    /// - Copies steps (makes them immutable)
    /// - Drops training-only config (split, parameter space)
    ///
    /// # Arguments
    ///
    /// * `training_pipeline` - The training pipeline to freeze
    pub fn from_training_pipeline(
        training_pipeline: &super::LinkPredictionTrainingPipeline,
    ) -> Self {
        // Copy feature steps from the training pipeline
        let feature_steps = training_pipeline
            .feature_steps()
            .iter()
            .map(|step| step.clone_box())
            .collect();

        Self {
            node_property_steps: PhantomData,
            feature_steps,
        }
    }

    /// Creates a LinkPredictionPredictPipeline from iterators.
    ///
    /// # The Stream Constructor!
    ///
    /// Useful for functional-style pipeline construction.
    ///
    /// # Arguments
    ///
    /// * `node_property_steps` - Iterator of node property steps
    /// * `feature_steps` - Iterator of link feature steps
    pub fn from_iterators(
        _node_property_steps: impl Iterator<Item = PhantomData<Box<dyn ExecutableNodePropertyStep>>>,
        feature_steps: impl Iterator<Item = Box<dyn LinkFeatureStep>>,
    ) -> Self {
        Self {
            node_property_steps: PhantomData,
            feature_steps: feature_steps.collect(),
        }
    }

    /// Returns the node property steps (placeholder).
    pub fn node_property_steps(&self) -> &PhantomData<Vec<Box<dyn ExecutableNodePropertyStep>>> {
        &self.node_property_steps
    }

    /// Returns the feature steps.
    pub fn feature_steps(&self) -> &[Box<dyn LinkFeatureStep>] {
        &self.feature_steps
    }

    /// Converts the pipeline to a map (for serialization).
    ///
    /// # The Given Representation!
    ///
    /// Maps the **Given** (frozen) pipeline to JSON-compatible structure.
    pub fn to_map(&self) -> HashMap<String, Vec<HashMap<String, serde_json::Value>>> {
        let mut map = HashMap::new();

        // Node property steps (placeholder)
        // TODO: Convert actual node property steps
        let node_steps: Vec<HashMap<String, serde_json::Value>> = Vec::new();
        map.insert("nodePropertySteps".to_string(), node_steps);

        // Feature steps
        let feature_steps_maps: Vec<HashMap<String, serde_json::Value>> = self
            .feature_steps
            .iter()
            .map(|step| step.configuration())
            .collect();
        map.insert("featureSteps".to_string(), feature_steps_maps);

        map
    }

    /// Validates the pipeline before execution.
    ///
    /// # The Given Validation!
    ///
    /// For predict pipelines, validation is minimal (no training-specific checks).
    /// The pipeline is already **Given** (frozen), so it should be valid.
    pub fn validate_before_execution(&self, _graph_store: PhantomData<()>) -> Result<(), String> {
        // Predict pipelines don't have training-specific validation
        // The steps were already validated during training
        Ok(())
    }
}

impl Default for LinkPredictionPredictPipeline {
    fn default() -> Self {
        Self::empty()
    }
}

// TODO: Implement Pipeline trait when associated types are properly configured
// For now, LinkPredictionPredictPipeline provides its own interface#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::ml::pipeline::link_pipeline::linkfunctions::{
        CosineFeatureStep, HadamardFeatureStep,
    };

    #[test]
    fn test_empty_pipeline() {
        let pipeline = LinkPredictionPredictPipeline::empty();
        assert_eq!(pipeline.feature_steps().len(), 0);
    }

    #[test]
    fn test_default_pipeline() {
        let pipeline = LinkPredictionPredictPipeline::default();
        assert_eq!(pipeline.feature_steps().len(), 0);
    }

    #[test]
    fn test_from_steps() {
        let feature_steps = vec![
            Box::new(HadamardFeatureStep::new(vec!["embedding".to_string()]))
                as Box<dyn LinkFeatureStep>,
        ];

        let pipeline = LinkPredictionPredictPipeline::from_steps(PhantomData, feature_steps);

        assert_eq!(pipeline.feature_steps().len(), 1);
        assert_eq!(pipeline.feature_steps()[0].name(), "HADAMARD");
    }

    #[test]
    fn test_from_iterators() {
        let feature_steps = vec![
            Box::new(HadamardFeatureStep::new(vec!["embedding".to_string()]))
                as Box<dyn LinkFeatureStep>,
            Box::new(CosineFeatureStep::new(vec!["features".to_string()]))
                as Box<dyn LinkFeatureStep>,
        ];

        let pipeline = LinkPredictionPredictPipeline::from_iterators(
            std::iter::empty(),
            feature_steps.into_iter(),
        );

        assert_eq!(pipeline.feature_steps().len(), 2);
    }

    #[test]
    fn test_to_map() {
        let feature_steps = vec![
            Box::new(HadamardFeatureStep::new(vec!["embedding".to_string()]))
                as Box<dyn LinkFeatureStep>,
        ];

        let pipeline = LinkPredictionPredictPipeline::from_steps(PhantomData, feature_steps);

        let map = pipeline.to_map();
        assert!(map.contains_key("nodePropertySteps"));
        assert!(map.contains_key("featureSteps"));
        assert_eq!(map.get("featureSteps").unwrap().len(), 1);
    }

    #[test]
    fn test_validate_before_execution() {
        let pipeline = LinkPredictionPredictPipeline::empty();
        let result = pipeline.validate_before_execution(PhantomData);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pipeline_trait_feature_steps() {
        let feature_steps = vec![Box::new(HadamardFeatureStep::new(vec!["prop".to_string()]))
            as Box<dyn LinkFeatureStep>];

        let pipeline = LinkPredictionPredictPipeline::from_steps(PhantomData, feature_steps);

        // Test Pipeline trait method
        let steps: &[Box<dyn LinkFeatureStep>] = Pipeline::feature_steps(&pipeline);
        assert_eq!(steps.len(), 1);
    }

    #[test]
    fn test_given_immutability() {
        // The Given is immutable!
        // LinkPredictionPredictPipeline has no add_feature_step() method
        // Once created, it's frozen

        let feature_steps = vec![
            Box::new(HadamardFeatureStep::new(vec!["embedding".to_string()]))
                as Box<dyn LinkFeatureStep>,
        ];

        let pipeline = LinkPredictionPredictPipeline::from_steps(PhantomData, feature_steps);

        // Can only read, not mutate!
        assert_eq!(pipeline.feature_steps().len(), 1);

        // No pipeline.add_feature_step() exists!
        // This is the Given (frozen) nature!
    }

    #[test]
    fn test_car_cdr_pipeline_transformation() {
        // CAR:CDR at Pipeline Level!
        // Training Pipeline (Pure/Process) → Predict Pipeline (Given/Frozen)

        let feature_steps = vec![
            Box::new(HadamardFeatureStep::new(vec!["embedding".to_string()]))
                as Box<dyn LinkFeatureStep>,
            Box::new(CosineFeatureStep::new(vec!["features".to_string()]))
                as Box<dyn LinkFeatureStep>,
        ];

        // Create the Given (frozen) pipeline
        let predict_pipeline =
            LinkPredictionPredictPipeline::from_steps(PhantomData, feature_steps);

        // The Given is immutable
        assert_eq!(predict_pipeline.feature_steps().len(), 2);

        // CAR: The atomic Given structure
        // CDR: Created from training process
        // Science: Training (Process) → Prediction (Given)

        // The Predict Pipeline is the Given Result of Training! 🌟
    }
}
