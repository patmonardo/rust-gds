//! Computation runtime traits and small helper surface.
//!
//! Computer/ComputeStep are the minimal runtime contracts that eval! will
//! project ComputationDescriptor into. Keep small and safe so generated code
//! can implement these traits with minimal imports.

use std::fmt;
use std::sync::Arc;

use crate::types::graph::Graph;

/// Errors produced by computation runtime
#[derive(Debug)]
pub enum ComputeError {
    InitFailed(String),
    StepFailed(String),
    FinalizeFailed(String),
    DescriptorMissing(u32),
    Backend(String),
}

impl fmt::Display for ComputeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputeError::InitFailed(s) => write!(f, "init failed: {}", s),
            ComputeError::StepFailed(s) => write!(f, "step failed: {}", s),
            ComputeError::FinalizeFailed(s) => write!(f, "finalize failed: {}", s),
            ComputeError::DescriptorMissing(id) => {
                write!(f, "computation descriptor {} missing", id)
            }
            ComputeError::Backend(s) => write!(f, "backend error: {}", s),
        }
    }
}

impl std::error::Error for ComputeError {}

impl From<crate::types::properties::PropertyValuesError> for ComputeError {
    fn from(error: crate::types::properties::PropertyValuesError) -> Self {
        ComputeError::StepFailed(format!("Property value error: {}", error))
    }
}

// Import descriptors from the descriptors module
use crate::projection::codegen::descriptors::{ComputationDescriptor, PipelineDescriptor};

/// Minimal execution environment passed to Computer and ComputeStep.
///
/// This context carries the essential runtime information:
/// - The graph being operated on
/// - The pipeline descriptor (the Dharma - properties flowing through computation + storage)
/// - The computation descriptor (how this specific computation executes)
/// - Node count for sizing
///
/// Future additions: config, metrics, memory_tracker, progress_tracker
pub struct ComputeContext<'a> {
    pub graph: &'a Arc<dyn Graph>,
    pub pipeline: &'a PipelineDescriptor,
    pub computation: &'a ComputationDescriptor,
    pub node_count: usize,
}

impl<'a> ComputeContext<'a> {
    pub fn new(
        graph: &'a Arc<dyn Graph>,
        pipeline: &'a PipelineDescriptor,
        computation: &'a ComputationDescriptor,
    ) -> Self {
        let node_count = graph.node_count();
        Self {
            graph,
            pipeline,
            computation,
            node_count,
        }
    }
}

/// Messages passed into compute steps (Phase‑0 placeholder).
/// Macro-generated code can define concrete message shapes.
#[derive(Debug, Clone)]
pub struct Messages {
    // keep minimal; generated code may provide typed wrappers
    pub payload_count: usize,
}

impl Messages {
    pub fn empty() -> Self {
        Self { payload_count: 0 }
    }
}

/// The per-step compute operation. A ComputeStep implements the step logic
/// over a single vertex/partition or a global coordination step.
pub trait ComputeStep: Send + Sync {
    /// Execute compute logic for this step. Return Ok(true) to indicate
    /// that further iterations are required; Ok(false) to indicate termination.
    fn compute(
        &self,
        ctx: &mut ComputeContext<'_>,
        messages: &Messages,
    ) -> Result<bool, ComputeError>;
}

/// High-level Computer that owns lifecycle of a computation species.
pub trait Computer: Send + Sync {
    /// Initialize computation (allocate node values, materialize properties).
    fn init(&mut self, ctx: &mut ComputeContext<'_>) -> Result<(), ComputeError>;

    /// Execute one superstep (may call ComputeStep internally).
    /// Return Ok(true) to continue; Ok(false) to stop.
    fn step(&mut self, ctx: &mut ComputeContext<'_>) -> Result<bool, ComputeError>;

    /// Finalize (write back, cleanup).
    fn finalize(&mut self, ctx: &mut ComputeContext<'_>) -> Result<(), ComputeError>;
}

/// Factory function type for instantiating Computers from descriptors
pub type ComputerFactory = fn(u32) -> Result<Box<dyn Computer>, ComputeError>;

lazy_static::lazy_static! {
    static ref COMPUTER_FACTORIES: std::sync::RwLock<std::collections::HashMap<u32, ComputerFactory>> =
        std::sync::RwLock::new(std::collections::HashMap::new());
}

/// Register a computer factory for a given descriptor id.
/// Macro-generated code calls this during module initialization.
pub fn register_computer_factory(descriptor_id: u32, factory: ComputerFactory) -> bool {
    let mut factories = COMPUTER_FACTORIES
        .write()
        .expect("factory registry poisoned");
    factories.insert(descriptor_id, factory).is_none()
}

/// Helper: try build a Computer from a registered ComputationDescriptor.
/// Macro-generated code provides matching factory keyed by descriptor id.
pub fn instantiate_computer_from_descriptor(id: u32) -> Result<Box<dyn Computer>, ComputeError> {
    // First check if descriptor exists
    if let Some(desc) =
        crate::projection::codegen::descriptors::computation::get_computation_descriptor(id)
    {
        // Check if factory registered
        let factories = COMPUTER_FACTORIES
            .read()
            .expect("factory registry poisoned");
        if let Some(factory) = factories.get(&id) {
            factory(id)
        } else {
            Err(ComputeError::InitFailed(format!(
                "no computer factory registered for descriptor '{}'",
                desc.name
            )))
        }
    } else {
        Err(ComputeError::DescriptorMissing(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::codegen::descriptors::computation::{
        clear_computation_registry, register_computation_descriptor, ComputationDescriptor,
        ComputationPattern, ComputationSpecies,
    };
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    // Serialize registry mutations across tests touching global registries
    lazy_static::lazy_static! {
        static ref TEST_REGISTRY_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());
    }

    // Helper to clear computer factory registry (test-only)
    fn clear_factory_registry() {
        let mut factories = super::COMPUTER_FACTORIES
            .write()
            .expect("factory registry poisoned");
        factories.clear();
    }

    struct DummyStep {
        iteration: std::sync::atomic::AtomicUsize,
    }

    impl DummyStep {
        fn new() -> Self {
            Self {
                iteration: std::sync::atomic::AtomicUsize::new(0),
            }
        }
    }

    impl ComputeStep for DummyStep {
        fn compute(
            &self,
            _ctx: &mut ComputeContext<'_>,
            _messages: &Messages,
        ) -> Result<bool, ComputeError> {
            let iter = self
                .iteration
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            // Run for 3 iterations then stop
            Ok(iter < 3)
        }
    }

    struct DummyComputer {
        step: Box<dyn ComputeStep>,
        inited: bool,
    }

    impl DummyComputer {
        fn new() -> Self {
            Self {
                step: Box::new(DummyStep::new()),
                inited: false,
            }
        }
    }

    impl Computer for DummyComputer {
        fn init(&mut self, _ctx: &mut ComputeContext<'_>) -> Result<(), ComputeError> {
            self.inited = true;
            Ok(())
        }

        fn step(&mut self, ctx: &mut ComputeContext<'_>) -> Result<bool, ComputeError> {
            self.step.compute(ctx, &Messages::empty())
        }

        fn finalize(&mut self, _ctx: &mut ComputeContext<'_>) -> Result<(), ComputeError> {
            if !self.inited {
                return Err(ComputeError::FinalizeFailed("not initialized".into()));
            }
            Ok(())
        }
    }

    #[test]
    fn dummy_computer_lifecycle() {
        let config = RandomGraphConfig::default().with_seed(42);
        let graph_store = DefaultGraphStore::random(&config).expect("graph creation failed");
        let graph = graph_store.graph();
        let graph_arc: Arc<dyn Graph> = graph.clone();

        // Create pipeline and computation descriptors
        let pipeline = PipelineDescriptor::test_pipeline("TestPipeline");
        let computation = ComputationDescriptor::new(
            0,
            "test_computation",
            ComputationSpecies::Bsp,
            ComputationPattern::VertexCentric,
        );

        let mut ctx = ComputeContext::new(&graph_arc, &pipeline, &computation);

        let mut computer = DummyComputer::new();

        // Init
        assert!(computer.init(&mut ctx).is_ok());
        assert!(computer.inited);

        // Run 3 iterations
        for i in 0..3 {
            let should_continue = computer.step(&mut ctx).expect("step ok");
            assert!(should_continue, "iteration {} should continue", i);
        }

        // 4th iteration should return false (stop)
        let should_continue = computer.step(&mut ctx).expect("step ok");
        assert!(!should_continue, "iteration 3 should stop");

        // Finalize
        assert!(computer.finalize(&mut ctx).is_ok());
    }

    #[test]
    fn register_and_instantiate_computer() {
        let _guard = TEST_REGISTRY_GUARD.lock().unwrap();
        clear_computation_registry();
        clear_factory_registry();

        // Register descriptor
        let desc = ComputationDescriptor::new(
            99,
            "test_bsp",
            ComputationSpecies::Bsp,
            ComputationPattern::VertexCentric,
        );
        register_computation_descriptor(desc);

        // Register factory
        fn factory(_id: u32) -> Result<Box<dyn Computer>, ComputeError> {
            Ok(Box::new(DummyComputer::new()))
        }

        assert!(register_computer_factory(99, factory));

        // Instantiate
        let computer = instantiate_computer_from_descriptor(99).expect("instantiate ok");

        // Verify lifecycle works
        let config = RandomGraphConfig::default().with_seed(42);
        let graph_store = DefaultGraphStore::random(&config).expect("graph creation failed");
        let graph = graph_store.graph();
        let graph_arc: Arc<dyn Graph> = graph.clone();

        let pipeline = PipelineDescriptor::test_pipeline("TestPipeline");
        let computation = ComputationDescriptor::new(
            99,
            "factory_test",
            ComputationSpecies::Bsp,
            ComputationPattern::VertexCentric,
        );

        let mut ctx = ComputeContext::new(&graph_arc, &pipeline, &computation);

        let mut computer = computer;
        assert!(computer.init(&mut ctx).is_ok());
        assert!(computer.step(&mut ctx).is_ok());
        assert!(computer.finalize(&mut ctx).is_ok());
    }

    #[test]
    fn missing_descriptor_error() {
        let result = instantiate_computer_from_descriptor(9999);
        assert!(result.is_err());
        match result {
            Err(ComputeError::DescriptorMissing(id)) => assert_eq!(id, 9999),
            _ => panic!("expected DescriptorMissing"),
        }
    }

    #[test]
    fn missing_factory_error() {
        let _guard = TEST_REGISTRY_GUARD.lock().unwrap();
        clear_computation_registry();
        clear_factory_registry();

        // Register descriptor but no factory
        let desc = ComputationDescriptor::new(
            88,
            "no_factory",
            ComputationSpecies::Bsp,
            ComputationPattern::VertexCentric,
        );
        register_computation_descriptor(desc);

        let result = instantiate_computer_from_descriptor(88);
        assert!(result.is_err());
        match result {
            Err(ComputeError::InitFailed(msg)) => assert!(msg.contains("no_factory")),
            _ => panic!("expected InitFailed"),
        }
    }
}
