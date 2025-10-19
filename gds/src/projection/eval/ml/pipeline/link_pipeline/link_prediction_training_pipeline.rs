// Phase 4.1: LinkPredictionTrainingPipeline - Training pipeline for link prediction

use super::{LinkFeatureStep, LinkPredictionSplitConfig};
use crate::projection::eval::ml::pipeline::{ExecutableNodePropertyStep, TrainingPipeline};
use std::collections::HashMap;
use std::marker::PhantomData;

/// Training pipeline for link prediction.
///
/// # Form Evaluation IS Feature Generation! 🌟
///
/// **The Principle Moment**:
/// - **Form Evaluation → Feature Generation**
/// - **Entity → Appearance of Shape** (principle moment of Form qua Contained)
/// - **Form Evaluations are ALWAYS Features**
///
/// This pipeline **evaluates the Form** (Graph) to **generate Features** (Entities)!
///
/// # The Contained Principle
///
/// The LinkPredictionTrainingPipeline is the **Contained** manifestation:
/// - **Container**: TrainingPipeline<F> (abstract)
/// - **Contained**: LinkPredictionTrainingPipeline (concrete with LinkFeatureStep)
/// - **Evaluation**: Form → Features (the principle moment!)
///
/// # Pipeline Architecture
///
/// ```text
/// LinkPredictionTrainingPipeline extends TrainingPipeline<LinkFeatureStep>
///   ├─ nodePropertySteps: Vec<ExecutableNodePropertyStep> (inherited)
///   ├─ featureSteps: Vec<LinkFeatureStep> (inherited, specialized to Link)
///   ├─ splitConfig: LinkPredictionSplitConfig (NEW - link-specific)
///   ├─ trainingType: TrainingType::CLASSIFICATION (inherited)
///   └─ parameterSpace: Vec<TrainerConfig> (inherited)
///
/// Key Differences from Node Pipelines:
///   - Uses LinkFeatureStep instead of NodePropertyStep for features
///   - Has LinkPredictionSplitConfig (with negative sampling config)
///   - Always CLASSIFICATION (binary - link exists/doesn't exist)
/// ```
///
/// # Example
///
/// ```text
/// let mut pipeline = LinkPredictionTrainingPipeline::new();
/// pipeline.add_feature_step(HadamardFeatureStep::new(vec!["embedding".to_string()]));
/// pipeline.add_feature_step(CosineFeatureStep::new(vec!["features".to_string()]));
/// pipeline.set_split_config(LinkPredictionSplitConfig::default());
/// ```
pub struct LinkPredictionTrainingPipeline {
    /// Pipeline type identifier
    pub pipeline_type: &'static str,

    /// Model type identifier
    pub model_type: &'static str,

    /// Node property steps (preprocessing)
    /// TODO: Use actual ExecutableNodePropertyStep type
    node_property_steps: PhantomData<Vec<Box<dyn ExecutableNodePropertyStep>>>,

    /// Link feature steps (feature extraction)
    feature_steps: Vec<Box<dyn LinkFeatureStep>>,

    /// Split configuration (train/test/validation + negative sampling)
    split_config: LinkPredictionSplitConfig,

    /// Training type (always CLASSIFICATION for link prediction)
    /// TODO: Use TrainingType enum
    training_type: String,

    /// Parameter space for hyperparameter search
    /// TODO: Use TrainerConfig type
    parameter_space: PhantomData<()>,
}

impl LinkPredictionTrainingPipeline {
    /// Pipeline type constant
    pub const PIPELINE_TYPE: &'static str = "Link prediction training pipeline";

    /// Model type constant
    pub const MODEL_TYPE: &'static str = "LinkPrediction";

    /// Creates a new LinkPredictionTrainingPipeline.
    ///
    /// # The Form Container!
    ///
    /// This creates the **Container** that will **evaluate Forms** to **generate Features**!
    ///
    /// Defaults:
    /// - Training type: CLASSIFICATION
    /// - Split config: LinkPredictionSplitConfig::default()
    /// - Empty feature steps
    pub fn new() -> Self {
        Self {
            pipeline_type: Self::PIPELINE_TYPE,
            model_type: Self::MODEL_TYPE,
            node_property_steps: PhantomData,
            feature_steps: Vec::new(),
            split_config: LinkPredictionSplitConfig::default(),
            training_type: "CLASSIFICATION".to_string(),
            parameter_space: PhantomData,
        }
    }

    /// Returns the pipeline type.
    pub fn pipeline_type(&self) -> &str {
        self.pipeline_type
    }

    /// Returns the model type.
    pub fn model_type(&self) -> &str {
        self.model_type
    }

    /// Returns the feature steps.
    pub fn feature_steps(&self) -> &[Box<dyn LinkFeatureStep>] {
        &self.feature_steps
    }

    /// Adds a feature step to the pipeline.
    pub fn add_feature_step(&mut self, step: Box<dyn LinkFeatureStep>) {
        self.feature_steps.push(step);
    }

    /// Returns the split configuration.
    pub fn split_config(&self) -> &LinkPredictionSplitConfig {
        &self.split_config
    }

    /// Sets the split configuration.
    pub fn set_split_config(&mut self, config: LinkPredictionSplitConfig) {
        self.split_config = config;
    }

    /// Returns the feature pipeline description.
    ///
    /// # Form Evaluation Description!
    ///
    /// This describes **how the Form will be evaluated** to **generate Features**!
    ///
    /// Returns map with:
    /// - "nodePropertySteps": Preprocessing steps
    /// - "featureSteps": Feature extraction steps (Form Evaluation!)
    pub fn feature_pipeline_description(
        &self,
    ) -> HashMap<String, Vec<HashMap<String, serde_json::Value>>> {
        let mut description = HashMap::new();

        // Node property steps
        // TODO: Iterate over actual node_property_steps when available
        let node_steps: Vec<HashMap<String, serde_json::Value>> = Vec::new();
        description.insert("nodePropertySteps".to_string(), node_steps);

        // Feature steps (the Form Evaluation steps!)
        let feature_steps_maps: Vec<HashMap<String, serde_json::Value>> = self
            .feature_steps
            .iter()
            .map(|step| step.configuration())
            .collect();
        description.insert("featureSteps".to_string(), feature_steps_maps);

        description
    }

    /// Returns additional pipeline entries.
    ///
    /// Link prediction pipelines include split config (not in node pipelines).
    pub fn additional_entries(&self) -> HashMap<String, serde_json::Value> {
        let mut entries = HashMap::new();
        entries.insert(
            "splitConfig".to_string(),
            serde_json::json!(self.split_config.to_map()),
        );
        entries
    }

    /// Validates the pipeline before execution.
    ///
    /// # Form Evaluation Validation!
    ///
    /// Ensures we have **at least one way to evaluate the Form** (generate Features)!
    pub fn validate_before_execution(&self) -> Result<(), String> {
        if self.feature_steps.is_empty() {
            return Err(
                "Training a Link prediction pipeline requires at least one feature. \
                 You can add features with the procedure `gds.beta.pipeline.linkPrediction.addFeature`."
                    .to_string(),
            );
        }
        Ok(())
    }

    /// Returns tasks grouped by relationship property.
    ///
    /// Some node property steps (like embeddings) may use relationship weights.
    /// This method groups tasks by the relationship property they use.
    ///
    /// # Arguments
    ///
    /// * `model_catalog` - Model catalog for looking up model configs
    /// * `username` - Username for model lookup
    ///
    /// # Returns
    ///
    /// Map of relationship property name → list of task names using that property.
    pub fn tasks_by_relationship_property(
        &self,
        _model_catalog: PhantomData<()>, // TODO: ModelCatalog
        _username: &str,
    ) -> HashMap<String, Vec<String>> {
        // TODO: Implement when ModelCatalog available
        // For each node property step:
        //   - Check if config has RELATIONSHIP_WEIGHT_PROPERTY key
        //   - Or check if config has MODEL_NAME_KEY and look up model
        //   - Group tasks by the relationship property they use
        HashMap::new()
    }

    /// Returns the relationship weight property used by the pipeline (if any).
    ///
    /// Some algorithms (FastRP, Node2Vec) can use relationship weights.
    /// This method extracts the weight property from node property steps.
    pub fn relationship_weight_property(
        &self,
        _model_catalog: PhantomData<()>, // TODO: ModelCatalog
        _username: &str,
    ) -> Option<String> {
        // TODO: Implement when ModelCatalog available
        // Call tasks_by_relationship_property()
        // Return first property if any
        None
    }
}

impl Default for LinkPredictionTrainingPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::ml::pipeline::link_pipeline::linkfunctions::{
        CosineFeatureStep, HadamardFeatureStep,
    };

    #[test]
    fn test_pipeline_creation() {
        let pipeline = LinkPredictionTrainingPipeline::new();
        assert_eq!(
            pipeline.pipeline_type(),
            "Link prediction training pipeline"
        );
        assert_eq!(pipeline.model_type(), "LinkPrediction");
    }

    #[test]
    fn test_pipeline_constants() {
        assert_eq!(
            LinkPredictionTrainingPipeline::PIPELINE_TYPE,
            "Link prediction training pipeline"
        );
        assert_eq!(LinkPredictionTrainingPipeline::MODEL_TYPE, "LinkPrediction");
    }

    #[test]
    fn test_add_feature_step() {
        let mut pipeline = LinkPredictionTrainingPipeline::new();
        pipeline.add_feature_step(Box::new(HadamardFeatureStep::new(vec!["prop".to_string()])));

        assert_eq!(pipeline.feature_steps().len(), 1);
        assert_eq!(pipeline.feature_steps()[0].name(), "HADAMARD");
    }

    #[test]
    fn test_multiple_feature_steps() {
        let mut pipeline = LinkPredictionTrainingPipeline::new();
        pipeline.add_feature_step(Box::new(HadamardFeatureStep::new(vec![
            "embedding".to_string()
        ])));
        pipeline.add_feature_step(Box::new(CosineFeatureStep::new(vec![
            "features".to_string()
        ])));

        assert_eq!(pipeline.feature_steps().len(), 2);
    }

    #[test]
    fn test_split_config() {
        let pipeline = LinkPredictionTrainingPipeline::new();
        let _config = pipeline.split_config();
        // Default split config should exist
    }

    #[test]
    fn test_set_split_config() {
        let mut pipeline = LinkPredictionTrainingPipeline::new();
        let custom_config = LinkPredictionSplitConfig::default();
        pipeline.set_split_config(custom_config);
        // Config should be set
    }

    #[test]
    fn test_validation_empty_features() {
        let pipeline = LinkPredictionTrainingPipeline::new();
        let result = pipeline.validate_before_execution();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("at least one feature"));
    }

    #[test]
    fn test_validation_with_features() {
        let mut pipeline = LinkPredictionTrainingPipeline::new();
        pipeline.add_feature_step(Box::new(HadamardFeatureStep::new(vec!["prop".to_string()])));

        let result = pipeline.validate_before_execution();
        assert!(result.is_ok());
    }

    #[test]
    fn test_feature_pipeline_description() {
        let mut pipeline = LinkPredictionTrainingPipeline::new();
        pipeline.add_feature_step(Box::new(HadamardFeatureStep::new(vec![
            "embedding".to_string()
        ])));

        let description = pipeline.feature_pipeline_description();
        assert!(description.contains_key("nodePropertySteps"));
        assert!(description.contains_key("featureSteps"));
        assert_eq!(description.get("featureSteps").unwrap().len(), 1);
    }

    #[test]
    fn test_additional_entries() {
        let pipeline = LinkPredictionTrainingPipeline::new();
        let entries = pipeline.additional_entries();

        assert!(entries.contains_key("splitConfig"));
    }

    #[test]
    fn test_form_evaluation_is_feature_generation() {
        // Form Evaluation IS Feature Generation!
        // Entity is the Appearance of Shape (principle moment of Form qua Contained)

        let mut pipeline = LinkPredictionTrainingPipeline::new();

        // Add Form Evaluation steps (Feature Generation!)
        pipeline.add_feature_step(Box::new(HadamardFeatureStep::new(vec![
            "embedding".to_string()
        ])));
        pipeline.add_feature_step(Box::new(CosineFeatureStep::new(vec![
            "features".to_string()
        ])));

        // The pipeline contains Form Evaluations
        assert_eq!(pipeline.feature_steps().len(), 2);

        // Each step evaluates the Form to generate Features
        assert_eq!(pipeline.feature_steps()[0].name(), "HADAMARD");
        assert_eq!(pipeline.feature_steps()[1].name(), "COSINE");

        // This is the Contained principle:
        // - Container: TrainingPipeline<F> (abstract)
        // - Contained: LinkPredictionTrainingPipeline (concrete)
        // - Evaluation: Form → Features (the principle moment!)

        // Form Evaluations are ALWAYS Features! 🌟
    }
}
