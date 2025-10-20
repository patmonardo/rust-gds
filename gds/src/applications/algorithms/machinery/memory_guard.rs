use crate::api::GraphStore;
use crate::applications::algorithms::machinery::{AlgorithmLabel, DimensionTransformer};
use crate::config::base_types::Config;
use crate::mem::MemoryEstimation;
use crate::logging::Log;
use crate::core::GraphDimensions;

/// Memory Guard - memory protection and validation
/// This is just memory guarding. Do not conflate with UI concerns.
pub trait MemoryGuard {
    /// This could be handy for tests
    fn assert_algorithm_can_run<CONFIG: Config>(
        &self,
        username: &str,
        estimation_factory: impl Fn() -> Box<dyn MemoryEstimation>,
        graph_store: &GraphStore,
        configuration: &CONFIG,
        label: AlgorithmLabel,
        dimension_transformer: Box<dyn DimensionTransformer>,
    ) -> Result<(), MemoryGuardError>;
}

/// Memory Guard Error
#[derive(Debug)]
pub enum MemoryGuardError {
    InsufficientMemory { required: u64, available: u64 },
    EstimationNotImplemented,
    Other(String),
}

impl std::fmt::Display for MemoryGuardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryGuardError::InsufficientMemory { required, available } => {
                write!(f, "Memory required ({}) exceeds available memory ({})", required, available)
            }
            MemoryGuardError::EstimationNotImplemented => {
                write!(f, "Memory estimation not implemented")
            }
            MemoryGuardError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for MemoryGuardError {}

/// Default Memory Guard implementation
pub struct DefaultMemoryGuard {
    log: Log,
    graph_dimension_factory: Box<dyn GraphDimensionFactory>,
    use_max_memory_estimation: bool,
    memory_tracker: Box<dyn MemoryTracker>,
}

impl DefaultMemoryGuard {
    pub fn new(
        log: Log,
        graph_dimension_factory: Box<dyn GraphDimensionFactory>,
        use_max_memory_estimation: bool,
        memory_tracker: Box<dyn MemoryTracker>,
    ) -> Self {
        Self {
            log,
            graph_dimension_factory,
            use_max_memory_estimation,
            memory_tracker,
        }
    }

    pub fn create(
        log: Log,
        use_max_memory_estimation: bool,
        memory_tracker: Box<dyn MemoryTracker>,
    ) -> Self {
        let graph_dimension_factory = Box::new(DefaultGraphDimensionFactory::new());
        Self::new(log, graph_dimension_factory, use_max_memory_estimation, memory_tracker)
    }
}

impl MemoryGuard for DefaultMemoryGuard {
    fn assert_algorithm_can_run<CONFIG: Config>(
        &self,
        username: &str,
        estimation_factory: impl Fn() -> Box<dyn MemoryEstimation>,
        graph_store: &GraphStore,
        configuration: &CONFIG,
        label: AlgorithmLabel,
        dimension_transformer: Box<dyn DimensionTransformer>,
    ) -> Result<(), MemoryGuardError> {
        match MemoryRequirement::create(
            estimation_factory,
            graph_store,
            self.graph_dimension_factory.as_ref(),
            dimension_transformer,
            configuration,
            self.use_max_memory_estimation,
        ) {
            Ok(memory_requirement) => {
                let bytes_to_reserve = memory_requirement.required_memory();
                
                if configuration.sudo() {
                    self.memory_tracker.track(username, label.as_string(), configuration.job_id(), bytes_to_reserve);
                    return Ok(());
                }

                match self.memory_tracker.try_to_track(username, label.as_string(), configuration.job_id(), bytes_to_reserve) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(MemoryGuardError::Other(e.to_string())),
                }
            }
            Err(MemoryGuardError::EstimationNotImplemented) => {
                self.log.info(&format!("Memory usage estimate not available for {}, skipping guard", label.as_string()));
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

/// Memory Requirement - represents memory requirements for an algorithm
pub struct MemoryRequirement {
    pub required_memory: u64,
}

impl MemoryRequirement {
    pub fn new(required_memory: u64) -> Self {
        Self { required_memory }
    }

    pub fn create<CONFIG: Config>(
        estimation_factory: impl Fn() -> Box<dyn MemoryEstimation>,
        graph_store: &GraphStore,
        graph_dimension_factory: &dyn GraphDimensionFactory,
        dimension_transformer: Box<dyn DimensionTransformer>,
        configuration: &CONFIG,
        use_max_memory_estimation: bool,
    ) -> Result<Self, MemoryGuardError> {
        let memory_estimation = estimation_factory();
        let graph_dimensions = graph_dimension_factory.create(graph_store, configuration);
        let transformed_graph_dimensions = dimension_transformer.transform(graph_dimensions);
        
        // TODO: Implement actual memory estimation
        // let memory_tree = memory_estimation.estimate(transformed_graph_dimensions, configuration.concurrency());
        // let memory_range = memory_tree.memory_usage();
        // let bytes_required = if use_max_memory_estimation { memory_range.max } else { memory_range.min };
        
        // For now, return a placeholder
        Ok(Self::new(1000))
    }
}

// Placeholder traits
pub trait GraphDimensionFactory {
    fn create<CONFIG: Config>(&self, graph_store: &GraphStore, configuration: &CONFIG) -> Box<dyn GraphDimensions>;
}

pub trait MemoryTracker {
    fn track(&self, username: &str, label: &str, job_id: u64, bytes: u64);
    fn try_to_track(&self, username: &str, label: &str, job_id: u64, bytes: u64) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct DefaultGraphDimensionFactory;

impl DefaultGraphDimensionFactory {
    pub fn new() -> Self {
        Self
    }
}

impl GraphDimensionFactory for DefaultGraphDimensionFactory {
    fn create<CONFIG: Config>(&self, _graph_store: &GraphStore, _configuration: &CONFIG) -> Box<dyn GraphDimensions> {
        // TODO: Implement actual graph dimensions computation
        Box::new(crate::core::GraphDimensionsImpl::new())
    }
}
