//! Pipeline descriptor for ML workflows.
//!
//! Represents a complete ML pipeline with steps, configuration, and metadata.
//! Maps to Java's `org.neo4j.gds.ml.pipeline.Pipeline` and `TrainingPipeline`.
//!
//! **FormDB Design**: Pipelines are the primary abstraction for ML Knowledge Apps.
//! They describe complete workflows: data → features → trained models.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::step::StepDescriptor; // Updated: step_descriptor.rs → step.rs

/// Descriptor for a complete ML pipeline.
///
/// **Pipeline-Centric Design**: This is the ML workflow orchestrator.
/// FormDB is an ML Knowledge Apps platform - pipelines are how users define ML workflows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineDescriptor {
    /// Unique pipeline name/identifier
    pub name: String,

    /// Pipeline type (classification, regression, link prediction)
    pub pipeline_type: PipelineType,

    /// Ordered list of pipeline steps (node property + feature steps)
    pub steps: Vec<StepDescriptor>,

    /// Training configuration (models, splits, metrics)
    pub training_config: TrainingConfig,

    /// Pipeline configuration (auto-tuning, validation, etc.)
    pub config: PipelineConfig,

    /// Pipeline metadata (creation time, user, etc.)
    pub metadata: PipelineMetadata,
}

/// Pipeline type determines what the model predicts.
///
/// Maps to Java's pipeline types: NodeClassificationTrainingPipeline, NodeRegressionTrainingPipeline, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipelineType {
    /// Node classification - predict node class
    NodeClassification {
        /// Target property containing class labels
        target_property: String,
    },

    /// Node regression - predict node value
    NodeRegression {
        /// Target property containing continuous values
        target_property: String,
    },

    /// Link prediction - predict if link exists
    LinkPrediction {
        source_node_label: String,
        target_node_label: String,
    },
}

/// Training configuration - defines models and evaluation.
///
/// Maps to Java's `TrainingPipeline.trainingParameterSpace` and related config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Model configurations to try (hyperparameter space)
    pub model_candidates: Vec<ModelCandidate>,

    /// Dataset split configuration
    pub split_config: SplitConfig,

    /// Validation metric (for model selection)
    pub validation_metric: ValidationMetric,
}

/// Model candidate configuration.
///
/// Each candidate represents a model type with specific hyperparameters.
/// Auto-tuning will train all candidates and select the best based on validation metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCandidate {
    /// Model type
    pub model_type: ModelType,

    /// Model-specific hyperparameters
    pub params: HashMap<String, serde_json::Value>,
}

/// Model type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModelType {
    DecisionTreeClassifier,
    DecisionTreeRegressor,
    LogisticRegression,
    LinearRegression,
    RandomForest,
}

/// Dataset split configuration.
///
/// Maps to Java's split configuration in training pipelines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitConfig {
    /// Train fraction (e.g., 0.7)
    pub train_fraction: f64,

    /// Validation fraction (e.g., 0.15)
    pub validation_fraction: f64,

    /// Test fraction (e.g., 0.15)
    pub test_fraction: f64,

    /// Random seed (reproducibility)
    pub seed: u64,

    /// Stratify by target (classification)
    pub stratify: bool,
}

/// Validation metric for model selection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidationMetric {
    // Classification metrics
    Accuracy,
    F1,
    Precision,
    Recall,
    AUCROC,

    // Regression metrics
    RMSE,
    MAE,
    R2,
}

/// Configuration for the overall pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Auto-tuning configuration (hyperparameter search)
    pub auto_tuning: Option<AutoTuningConfig>,

    /// Validation split configuration (deprecated - use training_config.split_config)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<ValidationConfig>,
}

/// Auto-tuning configuration (hyperparameter search).
///
/// Maps to Java's `org.neo4j.gds.ml.pipeline.AutoTuningConfig`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoTuningConfig {
    /// Maximum number of trials
    pub max_trials: usize,

    /// Search strategy
    pub search_strategy: SearchStrategy,
}

/// Search strategy for hyperparameter tuning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchStrategy {
    /// Try all combinations
    GridSearch,

    /// Random sampling
    RandomSearch { iterations: usize },
}

/// Validation split configuration (deprecated - use TrainingConfig).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Validation fraction (0.0 to 1.0)
    pub validation_fraction: f64,
}

/// Pipeline metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineMetadata {
    /// Creation timestamp (RFC 3339 format)
    pub created_at: String,

    /// Creator username
    pub created_by: String,

    /// Optional description
    pub description: Option<String>,
}

// Defaults

impl Default for SplitConfig {
    fn default() -> Self {
        Self {
            train_fraction: 0.7,
            validation_fraction: 0.15,
            test_fraction: 0.15,
            seed: 42,
            stratify: true,
        }
    }
}

impl Default for AutoTuningConfig {
    fn default() -> Self {
        Self {
            max_trials: 10,
            search_strategy: SearchStrategy::GridSearch,
        }
    }
}

impl PipelineMetadata {
    pub fn new(created_by: String) -> Self {
        use chrono::Utc;
        Self {
            created_at: Utc::now().to_rfc3339(),
            created_by,
            description: None,
        }
    }
}

// Builder pattern for ergonomic pipeline construction

impl PipelineDescriptor {
    /// Create a builder for constructing pipelines.
    pub fn builder(name: String, pipeline_type: PipelineType) -> PipelineDescriptorBuilder {
        PipelineDescriptorBuilder::new(name, pipeline_type)
    }

    /// Create a minimal test pipeline for unit testing.
    ///
    /// **Test-Only**: This is a simplified constructor for unit tests in runtime modules.
    /// Production code should use the builder pattern.
    #[cfg(test)]
    pub fn test_pipeline(name: &str) -> Self {
        Self {
            name: name.to_string(),
            pipeline_type: PipelineType::NodeClassification {
                target_property: "test_target".to_string(),
            },
            steps: Vec::new(),
            training_config: TrainingConfig {
                model_candidates: Vec::new(),
                split_config: SplitConfig {
                    train_fraction: 0.7,
                    validation_fraction: 0.15,
                    test_fraction: 0.15,
                    seed: 42,
                    stratify: false,
                },
                validation_metric: ValidationMetric::Accuracy,
            },
            config: PipelineConfig {
                auto_tuning: None,
                validation: None,
            },
            metadata: PipelineMetadata::new("test".to_string()),
        }
    }
}

/// Builder for constructing ML pipelines.
pub struct PipelineDescriptorBuilder {
    name: String,
    pipeline_type: PipelineType,
    steps: Vec<StepDescriptor>,
    training_config: Option<TrainingConfig>,
    pipeline_config: Option<PipelineConfig>,
    metadata: Option<PipelineMetadata>,
}

impl PipelineDescriptorBuilder {
    pub fn new(name: String, pipeline_type: PipelineType) -> Self {
        Self {
            name,
            pipeline_type,
            steps: Vec::new(),
            training_config: None,
            pipeline_config: None,
            metadata: None,
        }
    }

    pub fn add_step(mut self, step: StepDescriptor) -> Self {
        self.steps.push(step);
        self
    }

    pub fn training_config(mut self, config: TrainingConfig) -> Self {
        self.training_config = Some(config);
        self
    }

    pub fn pipeline_config(mut self, config: PipelineConfig) -> Self {
        self.pipeline_config = Some(config);
        self
    }

    pub fn metadata(mut self, metadata: PipelineMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn build(self) -> Result<PipelineDescriptor, String> {
        let training_config = self
            .training_config
            .ok_or_else(|| "Training config is required".to_string())?;

        let pipeline_config = self.pipeline_config.unwrap_or_default();

        let metadata = self
            .metadata
            .unwrap_or_else(|| PipelineMetadata::new("unknown".to_string()));

        Ok(PipelineDescriptor {
            name: self.name,
            pipeline_type: self.pipeline_type,
            steps: self.steps,
            training_config,
            config: pipeline_config,
            metadata,
        })
    }
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            auto_tuning: Some(AutoTuningConfig::default()),
            validation: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_descriptor_builder() {
        let pipeline = PipelineDescriptor::builder(
            "test_pipeline".to_string(),
            PipelineType::NodeClassification {
                target_property: "label".to_string(),
            },
        )
        .training_config(TrainingConfig {
            model_candidates: vec![ModelCandidate {
                model_type: ModelType::DecisionTreeClassifier,
                params: HashMap::new(),
            }],
            split_config: SplitConfig::default(),
            validation_metric: ValidationMetric::Accuracy,
        })
        .build()
        .unwrap();

        assert_eq!(pipeline.name, "test_pipeline");
        assert_eq!(pipeline.training_config.model_candidates.len(), 1);
        assert_eq!(
            pipeline.training_config.validation_metric,
            ValidationMetric::Accuracy
        );
    }

    #[test]
    fn test_pipeline_serialization() {
        let pipeline = PipelineDescriptor::builder(
            "test".to_string(),
            PipelineType::NodeRegression {
                target_property: "value".to_string(),
            },
        )
        .training_config(TrainingConfig {
            model_candidates: vec![],
            split_config: SplitConfig::default(),
            validation_metric: ValidationMetric::RMSE,
        })
        .build()
        .unwrap();

        // Serialize to JSON
        let json = serde_json::to_string(&pipeline).unwrap();

        // Deserialize from JSON
        let deserialized: PipelineDescriptor = serde_json::from_str(&json).unwrap();

        assert_eq!(pipeline.name, deserialized.name);
        assert_eq!(pipeline.pipeline_type, deserialized.pipeline_type);
    }

    #[test]
    fn test_split_config_default() {
        let config = SplitConfig::default();
        assert_eq!(config.train_fraction, 0.7);
        assert_eq!(config.validation_fraction, 0.15);
        assert_eq!(config.test_fraction, 0.15);
        assert!(config.stratify);
    }
}
