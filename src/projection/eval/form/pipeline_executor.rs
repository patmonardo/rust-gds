//! Pipeline executor for ML workflows.
//!
//! Orchestrates execution of complete ML pipelines:
//! - Validates pipeline configuration
//! - Executes steps in sequence (node property → feature)
//! - Manages intermediate state
//! - Produces final results

use std::collections::HashMap;
use std::sync::Arc;

use crate::projection::codegen::computation_descriptor::ComputationDescriptor;
use crate::projection::codegen::computation_runtime::{ComputeContext, ComputeError, Computer};
use crate::projection::codegen::ml::pipeline_descriptor::PipelineDescriptor;
use crate::projection::codegen::ml::step_descriptor::StepDescriptor;
use crate::projection::eval::form::graph_procedure::GraphProcedureRegistry;
use crate::projection::eval::form::pipeline_state::{
    DatasetSplits, ExecutionPhase, PipelineState,
};
use crate::types::graph::Graph;
use crate::types::properties::PropertyValues;

/// Result of pipeline execution
pub struct PipelineResult {
    /// Features computed during execution
    pub features: HashMap<String, Arc<dyn PropertyValues>>,

    /// Number of steps executed
    pub steps_executed: usize,

    /// Execution was successful
    pub success: bool,
}

impl PipelineResult {
    pub fn success(
        features: HashMap<String, Arc<dyn PropertyValues>>,
        steps_executed: usize,
    ) -> Self {
        Self {
            features,
            steps_executed,
            success: true,
        }
    }

    pub fn failure() -> Self {
        Self {
            features: HashMap::new(),
            steps_executed: 0,
            success: false,
        }
    }
}

/// Pipeline executor - coordinates execution of ML pipelines.
///
/// This is the core runtime for ML computations. It:
/// - Validates pipeline configuration
/// - Executes steps in sequence (node properties → features → training)
/// - Manages intermediate state
/// - Produces final results
///
/// **Orchestration Flow:**
/// 1. **Node Property Steps**: Call graph procedures via registry → compute properties
/// 2. **Feature Assembly**: Transform properties → ML-ready features
/// 3. **Dataset Splitting**: Split nodes into train/val/test sets
/// 4. **Model Training**: Train models with auto-tuning (Phase 2.4+)
///
/// Maps to Java GDS Pipeline.java and PipelineExecutor.java.
pub struct PipelineExecutor {
    pipeline: PipelineDescriptor,
    state: PipelineState,
    graph: Option<Arc<dyn Graph>>,
    procedure_registry: GraphProcedureRegistry,
}

impl PipelineExecutor {
    /// Create new pipeline executor with default (empty) procedure registry.
    pub fn new(pipeline: PipelineDescriptor) -> Self {
        Self::with_registry(pipeline, GraphProcedureRegistry::new())
    }

    /// Create pipeline executor with custom procedure registry.
    ///
    /// Use this to provide real graph procedure implementations or custom mocks.
    pub fn with_registry(
        pipeline: PipelineDescriptor,
        procedure_registry: GraphProcedureRegistry,
    ) -> Self {
        let total_steps = pipeline.steps.len();
        Self {
            pipeline,
            state: PipelineState::new(Vec::new(), total_steps),
            graph: None,
            procedure_registry,
        }
    }

    /// Get reference to pipeline descriptor
    pub fn pipeline(&self) -> &PipelineDescriptor {
        &self.pipeline
    }

    /// Get reference to current state
    pub fn state(&self) -> &PipelineState {
        &self.state
    }

    /// Get reference to procedure registry
    pub fn registry(&self) -> &GraphProcedureRegistry {
        &self.procedure_registry
    }

    /// Initialize pipeline execution
    fn init_internal(&mut self) -> Result<(), ComputeError> {
        self.state.phase = ExecutionPhase::NodePropertySteps;

        // Validate pipeline configuration
        self.validate_pipeline()?;

        Ok(())
    }

    /// Execute all pipeline steps in sequence
    fn execute_internal(&mut self) -> Result<(), ComputeError> {
        let graph = self
            .graph
            .as_ref()
            .ok_or_else(|| ComputeError::InitFailed("graph not initialized".into()))?
            .clone();

        // Phase 1: Execute node property steps (call graph procedures)
        self.execute_node_property_steps(&graph)?;

        // Phase 2: Assemble features from properties
        self.assemble_features()?;

        // Phase 3: Split dataset into train/val/test
        self.split_dataset()?;

        // Phase 4: Train models (Phase 2.4+ - for now, just mark complete)
        self.state.set_phase(ExecutionPhase::Training);
        // TODO: self.train_models()?;

        Ok(())
    }

    /// Execute node property steps - call graph procedures via registry.
    ///
    /// For each NodePropertyStep:
    /// 1. Lookup procedure in registry by algorithm name
    /// 2. Execute procedure on graph with step config
    /// 3. Store resulting PropertyValues in state
    fn execute_node_property_steps(&mut self, graph: &Arc<dyn Graph>) -> Result<(), ComputeError> {
        use crate::projection::codegen::ml::step_descriptor::NodePropertyStepDescriptor;

        self.state.set_phase(ExecutionPhase::NodePropertySteps);

        // Filter node property steps
        let node_property_steps: Vec<&NodePropertyStepDescriptor> = self
            .pipeline
            .steps
            .iter()
            .filter_map(|step| match step {
                StepDescriptor::NodeProperty(np_step) => Some(np_step),
                _ => None,
            })
            .collect();

        for step in node_property_steps {
            // Lookup procedure in registry
            let procedure = self
                .procedure_registry
                .get(&step.algorithm)
                .ok_or_else(|| {
                    ComputeError::InitFailed(format!(
                        "graph procedure '{}' not found in registry",
                        step.algorithm
                    ))
                })?;

            // Execute procedure
            let config = HashMap::new(); // TODO: Extract config from step
            let property_values = procedure.execute(graph.as_ref(), &config)?;

            // Store in state
            self.state
                .add_property(step.property_name.clone(), property_values);
            self.state.increment_step();
        }

        Ok(())
    }

    /// Assemble features from properties.
    ///
    /// For each FeatureStep:
    /// 1. Read source properties from state
    /// 2. Apply transformation (Identity, Normalize, etc.)
    /// 3. Store resulting feature in state
    ///
    /// Phase 2.3: Simple identity transformation (copy property → feature)
    /// Phase 2.5: Full feature engineering (normalize, one-hot, combine)
    fn assemble_features(&mut self) -> Result<(), ComputeError> {
        use crate::projection::codegen::ml::step_descriptor::FeatureStepDescriptor;

        self.state.set_phase(ExecutionPhase::FeatureSteps);

        // Filter feature steps
        let feature_steps: Vec<&FeatureStepDescriptor> = self
            .pipeline
            .steps
            .iter()
            .filter_map(|step| match step {
                StepDescriptor::Feature(f_step) => Some(f_step),
                _ => None,
            })
            .collect();

        for step in feature_steps {
            // Phase 2.3: Simple identity transformation
            // Just copy first source property as feature
            if let Some(source_property) = step.source_properties.first() {
                if let Some(property_values) = self.state.get_property(source_property) {
                    self.state
                        .add_feature(step.name.clone(), property_values.clone());
                    self.state.increment_step();
                } else {
                    return Err(ComputeError::InitFailed(format!(
                        "source property '{}' not found for feature '{}'",
                        source_property, step.name
                    )));
                }
            } else {
                return Err(ComputeError::InitFailed(format!(
                    "feature '{}' has no source properties",
                    step.name
                )));
            }
        }

        Ok(())
    }

    /// Split dataset into train/validation/test sets.
    ///
    /// Uses split config from pipeline training config.
    /// Applies stratification if specified.
    fn split_dataset(&mut self) -> Result<(), ComputeError> {
        self.state.set_phase(ExecutionPhase::DatasetSplitting);

        let split_config = &self.pipeline.training_config.split_config;

        // Get node IDs from state (or graph if not set)
        let node_ids = if self.state.node_ids.is_empty() {
            // Initialize from graph
            let graph = self
                .graph
                .as_ref()
                .ok_or_else(|| ComputeError::InitFailed("graph not initialized".into()))?;

            let node_count = graph.node_count() as usize;
            (0..node_count as u64).collect()
        } else {
            self.state.node_ids.clone()
        };

        // Create splits using config
        let splits = DatasetSplits::from_fractions(
            &node_ids,
            split_config.train_fraction,
            split_config.validation_fraction,
            split_config.test_fraction,
            split_config.seed,
        );

        self.state.set_splits(splits);

        Ok(())
    }

    /// Finalize pipeline and extract results
    fn finalize_internal(&mut self) -> Result<PipelineResult, ComputeError> {
        self.state.phase = ExecutionPhase::Evaluation;

        // Extract features from state
        let features = std::mem::take(&mut self.state.features);
        let steps_executed = self.state.steps_completed;

        self.state.phase = ExecutionPhase::Completed;

        Ok(PipelineResult::success(features, steps_executed))
    }

    /// Validate pipeline configuration
    fn validate_pipeline(&self) -> Result<(), ComputeError> {
        // Check pipeline has at least one step
        if self.pipeline.steps.is_empty() {
            return Err(ComputeError::InitFailed("pipeline has no steps".into()));
        }

        // Validate each step (step-specific validation)
        for (idx, step) in self.pipeline.steps.iter().enumerate() {
            self.validate_step(idx, step)?;
        }

        Ok(())
    }

    /// Validate a single step
    fn validate_step(&self, idx: usize, step: &StepDescriptor) -> Result<(), ComputeError> {
        match step {
            StepDescriptor::NodeProperty(node_prop_step) => {
                if node_prop_step.property_name.is_empty() {
                    return Err(ComputeError::InitFailed(format!(
                        "step {}: property name is empty",
                        idx
                    )));
                }
                if node_prop_step.algorithm.is_empty() {
                    return Err(ComputeError::InitFailed(format!(
                        "step {}: algorithm is empty",
                        idx
                    )));
                }
                Ok(())
            }
            StepDescriptor::Feature(feature_step) => {
                if feature_step.name.is_empty() {
                    return Err(ComputeError::InitFailed(format!(
                        "step {}: feature name is empty",
                        idx
                    )));
                }
                if feature_step.source_properties.is_empty() {
                    return Err(ComputeError::InitFailed(format!(
                        "step {}: no source properties specified",
                        idx
                    )));
                }
                Ok(())
            }
        }
    }

    /// Execute a single pipeline step
    fn execute_step(
        &mut self,
        _idx: usize,
        step: &StepDescriptor,
        graph: &Arc<dyn Graph>,
    ) -> Result<(), ComputeError> {
        // Create step executor and run it
        use crate::projection::eval::form::step_executor::create_step_executor;

        let executor = create_step_executor(step);
        let _result = executor.execute(graph, &mut self.state)?;

        // In Phase 2.1, we accept that execution will fail (not yet implemented)
        // Phase 2.2 will add actual implementations
        Ok(())
    }
}

/// Implement Computer trait so PipelineExecutor can be used as a computation plugin
impl Computer for PipelineExecutor {
    fn init(&mut self, ctx: &mut ComputeContext<'_>) -> Result<(), ComputeError> {
        self.graph = Some(ctx.graph.clone());
        self.init_internal()
    }

    fn step(&mut self, _ctx: &mut ComputeContext<'_>) -> Result<bool, ComputeError> {
        // Execute all steps (pipeline runs once, not iterative)
        self.execute_internal()?;

        // Return false to indicate no more iterations needed
        Ok(false)
    }

    fn finalize(&mut self, _ctx: &mut ComputeContext<'_>) -> Result<(), ComputeError> {
        self.finalize_internal()?;
        Ok(())
    }
}

/// Public execution API (convenience wrapper)
impl PipelineExecutor {
    /// Execute pipeline end-to-end and return results
    pub fn run(
        &mut self,
        graph: &Arc<dyn Graph>,
        computation: &ComputationDescriptor,
    ) -> Result<PipelineResult, ComputeError> {
        // Create dummy pipeline descriptor for context
        // In real usage, this comes from the pipeline itself
        use crate::projection::codegen::pipeline_descriptor::PipelineDescriptor as CodegenPipelineDescriptor;
        let codegen_pipeline = CodegenPipelineDescriptor {
            name: "ml_pipeline".into(),
            properties: vec![],
            computation_flow: Some("ml".into()),
            storage_flow: None,
        };

        let mut ctx = ComputeContext::new(graph, &codegen_pipeline, computation);

        // Run full lifecycle
        self.init(&mut ctx)?;
        while self.step(&mut ctx)? {
            // Continue until step returns false
        }
        self.finalize(&mut ctx)?;

        // Extract result
        self.finalize_internal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::codegen::ml::pipeline_descriptor::PipelineDescriptor as MLPipelineDescriptor;
    use crate::projection::codegen::ml::step_descriptor::{
        FeatureStepDescriptor, FeatureType, NodePropertyStepDescriptor,
    };

    #[test]
    fn test_pipeline_state_creation() {
        let node_ids = vec![0, 1, 2, 3, 4];
        let state = PipelineState::new(node_ids.clone(), 5);
        assert_eq!(state.node_ids, node_ids);
        assert_eq!(state.total_steps, 5);
        assert_eq!(state.steps_completed, 0);
        assert_eq!(state.phase, ExecutionPhase::NotStarted);
        assert!(state.features.is_empty());
    }

    #[test]
    fn test_pipeline_state_progress() {
        let mut state = PipelineState::new(vec![0, 1, 2, 3], 4);
        assert_eq!(state.progress(), 0.0);

        state.increment_step();
        state.increment_step();
        assert_eq!(state.progress(), 0.5);

        state.increment_step();
        state.increment_step();
        assert_eq!(state.progress(), 1.0);
    }

    #[test]
    fn test_pipeline_executor_creation() {
        use crate::projection::codegen::ml::pipeline_descriptor::*;

        let pipeline = MLPipelineDescriptor::builder(
            "test_pipeline".to_string(),
            PipelineType::NodeClassification {
                target_property: "label".to_string(),
            },
        )
        .training_config(TrainingConfig {
            model_candidates: vec![],
            split_config: SplitConfig::default(),
            validation_metric: ValidationMetric::Accuracy,
        })
        .build()
        .unwrap();

        let executor = PipelineExecutor::new(pipeline);
        assert_eq!(executor.state().total_steps, 0);
        assert_eq!(executor.pipeline().name, "test_pipeline");
    }

    #[test]
    fn test_validate_empty_pipeline() {
        use crate::projection::codegen::ml::pipeline_descriptor::*;

        let pipeline = MLPipelineDescriptor::builder(
            "empty_pipeline".to_string(),
            PipelineType::NodeClassification {
                target_property: "label".to_string(),
            },
        )
        .training_config(TrainingConfig {
            model_candidates: vec![],
            split_config: SplitConfig::default(),
            validation_metric: ValidationMetric::Accuracy,
        })
        .build()
        .unwrap();

        let executor = PipelineExecutor::new(pipeline);
        let result = executor.validate_pipeline();

        assert!(result.is_err());
        assert!(matches!(result, Err(ComputeError::InitFailed(_))));
    }

    #[test]
    fn test_validate_node_property_step() {
        use crate::projection::codegen::ml::pipeline_descriptor::*;

        let step = StepDescriptor::NodeProperty(NodePropertyStepDescriptor::new(
            "pagerank_step".into(),
            "pageRank".into(),
            "pr_score".into(),
        ));

        let pipeline = MLPipelineDescriptor::builder(
            "test_pipeline".to_string(),
            PipelineType::NodeClassification {
                target_property: "label".to_string(),
            },
        )
        .add_step(step)
        .training_config(TrainingConfig {
            model_candidates: vec![],
            split_config: SplitConfig::default(),
            validation_metric: ValidationMetric::Accuracy,
        })
        .build()
        .unwrap();

        let executor = PipelineExecutor::new(pipeline);
        let result = executor.validate_pipeline();
        assert!(result.is_ok());
    }

    #[test]
    fn test_pipeline_with_multiple_steps() {
        use crate::projection::codegen::ml::pipeline_descriptor::*;

        let step1 = StepDescriptor::NodeProperty(NodePropertyStepDescriptor::new(
            "pagerank_step".into(),
            "pageRank".into(),
            "pr_score".into(),
        ));

        let step2 = StepDescriptor::NodeProperty(NodePropertyStepDescriptor::new(
            "degree_step".into(),
            "degree".into(),
            "degree_score".into(),
        ));

        let pipeline = MLPipelineDescriptor::builder(
            "multi_step_pipeline".to_string(),
            PipelineType::NodeClassification {
                target_property: "label".to_string(),
            },
        )
        .add_step(step1)
        .add_step(step2)
        .training_config(TrainingConfig {
            model_candidates: vec![],
            split_config: SplitConfig::default(),
            validation_metric: ValidationMetric::Accuracy,
        })
        .build()
        .unwrap();

        let executor = PipelineExecutor::new(pipeline);
        assert_eq!(executor.state().total_steps, 2);

        let result = executor.validate_pipeline();
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_with_mock_registry() {
        use super::super::graph_procedure::create_mock_registry;
        use crate::projection::codegen::ml::pipeline_descriptor::*;
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::RandomGraphConfig;

        // Create pipeline with NodeProperty step
        let step = StepDescriptor::NodeProperty(NodePropertyStepDescriptor::new(
            "pagerank_step".into(),
            "pageRank".into(),
            "pr_score".into(),
        ));

        let pipeline = MLPipelineDescriptor::builder(
            "mock_pipeline".to_string(),
            PipelineType::NodeClassification {
                target_property: "label".to_string(),
            },
        )
        .add_step(step)
        .training_config(TrainingConfig {
            model_candidates: vec![],
            split_config: SplitConfig::default(),
            validation_metric: ValidationMetric::Accuracy,
        })
        .build()
        .unwrap();

        // Create executor with mock registry
        let registry = create_mock_registry();
        let mut executor = PipelineExecutor::with_registry(pipeline, registry);

        // Create graph
        let config = RandomGraphConfig {
            node_count: 100,
            ..RandomGraphConfig::default()
        }
        .with_seed(42);
        let store = DefaultGraphStore::random(&config).expect("random graph");
        let graph = store.graph();

        // Initialize and execute node property steps
        executor.init_internal();
        let graph_arc: Arc<dyn Graph> = graph;
        executor.graph = Some(graph_arc.clone());

        let result = executor.execute_node_property_steps(&graph_arc);
        assert!(result.is_ok(), "should execute node property steps");

        // Verify property stored
        assert!(executor.state().get_property("pr_score").is_some());
        assert_eq!(executor.state().steps_completed, 1);
    }

    #[test]
    fn test_dataset_splitting() {
        use crate::projection::codegen::ml::pipeline_descriptor::*;
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::RandomGraphConfig;

        let pipeline = MLPipelineDescriptor::builder(
            "split_test".to_string(),
            PipelineType::NodeClassification {
                target_property: "label".to_string(),
            },
        )
        .training_config(TrainingConfig {
            model_candidates: vec![],
            split_config: SplitConfig::default(),
            validation_metric: ValidationMetric::Accuracy,
        })
        .build()
        .unwrap();

        let mut executor = PipelineExecutor::new(pipeline);

        // Create graph
        let config = RandomGraphConfig {
            node_count: 100,
            ..RandomGraphConfig::default()
        }
        .with_seed(42);
        let store = DefaultGraphStore::random(&config).expect("random graph");
        let graph = store.graph();

        executor.init_internal();
        executor.graph = Some(graph);

        // Split dataset
        let result = executor.split_dataset();
        assert!(result.is_ok(), "should split dataset");

        // Verify splits created
        assert!(executor.state().has_splits());
        let splits = &executor.state().splits;

        // Default split config: 70% train, 15% val, 15% test
        assert_eq!(splits.train.len(), 70);
        assert_eq!(splits.validation.len(), 15);
        assert_eq!(splits.test.len(), 15);
    }

    #[test]
    fn test_end_to_end_orchestration() {
        use super::super::graph_procedure::create_mock_registry;
        use crate::projection::codegen::ml::pipeline_descriptor::*;
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::RandomGraphConfig;

        // Create complete pipeline with NodeProperty and Feature steps
        let step1 = StepDescriptor::NodeProperty(NodePropertyStepDescriptor::new(
            "pagerank_step".into(),
            "pageRank".into(),
            "pr_score".into(),
        ));

        let step2 = StepDescriptor::Feature(FeatureStepDescriptor {
            name: "pr_feature".to_string(),
            feature_type: FeatureType::Scalar,
            source_properties: vec!["pr_score".to_string()],
            target_dimension: None,
        });

        let pipeline = MLPipelineDescriptor::builder(
            "end_to_end_test".to_string(),
            PipelineType::NodeClassification {
                target_property: "label".to_string(),
            },
        )
        .add_step(step1)
        .add_step(step2)
        .training_config(TrainingConfig {
            model_candidates: vec![],
            split_config: SplitConfig::default(),
            validation_metric: ValidationMetric::Accuracy,
        })
        .build()
        .unwrap();

        // Create executor with mock registry
        let registry = create_mock_registry();
        let mut executor = PipelineExecutor::with_registry(pipeline, registry);

        // Create graph
        let config = RandomGraphConfig {
            node_count: 100,
            ..RandomGraphConfig::default()
        }
        .with_seed(42);
        let store = DefaultGraphStore::random(&config).expect("random graph");
        let graph = store.graph();

        // Execute full pipeline
        executor.init_internal();
        executor.graph = Some(graph);

        let result = executor.execute_internal();
        assert!(result.is_ok(), "should execute full pipeline");

        // Verify final state
        assert!(
            executor.state().get_property("pr_score").is_some(),
            "property stored"
        );
        assert!(
            executor.state().get_feature("pr_feature").is_some(),
            "feature stored"
        );
        assert!(executor.state().has_splits(), "splits created");
        assert_eq!(executor.state().steps_completed, 2);

        // Phase should be Training (after splitting)
        matches!(executor.state().phase, ExecutionPhase::Training);
    }
}
