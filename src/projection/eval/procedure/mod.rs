//! Procedure Executor - GDSL Runtime for Algorithm Execution
//!
//! This is the **fixed GDSL Runtime** for procedure execution.
//! Part of the Projection system (`src/projection/eval/`).
//! Involved in NativeFactory codegen directly as the computation runtime.
//!
//! **Architecture**:
//! - **Executor Runtime** (this module) → Fixed GDSL infrastructure
//! - **Algorithm Implementations** (`src/procedure/`) → Extensible content
//!
//! **Translation**: Java GDS Executor package (22 files) → 7 Rust modules
//!
//! ## Complete Procedure Execution Flow
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │  ProcedureExecutor (GDSL Runtime Orchestrator)              │
//! │                                                             │
//! │  1. preprocess_config()    → Enhance with context          │
//! │  2. parse_config()         → Parse & validate JSON         │
//! │  3. validate_before_load() → Config-only validation        │
//! │  4. load_graph()           → Get from catalog              │
//! │  5. validate_after_load()  → Config + graph validation     │
//! │  6. execute_algorithm()    → Run with timing               │
//! │  7. consume_result()       → Transform & validate output   │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Module Structure
//!
//! - **execution_mode** - How to return results (Stream, Stats, Train, Write, Mutate)
//! - **computation_result** - Algorithm output with timing metadata
//! - **execution_context** - Runtime environment (catalog, logging, metrics)
//! - **validation_config** - Two-phase validation system
//! - **algorithm_spec** - Contract between executor and algorithms
//! - **result_consumer** - Result processing helpers
//! - **executor** - Main orchestrator (this brings it all together)
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use rust_gds::projection::eval::procedure::*;
//!
//! // 1. Create execution context
//! let mut context = ExecutionContext::new("user");
//! context.add_graph("my_graph", graph_store);
//!
//! // 2. Create executor
//! let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);
//!
//! // 3. Define algorithm (implements AlgorithmSpec)
//! struct PageRank { /* ... */ }
//! impl AlgorithmSpec for PageRank { /* ... */ }
//!
//! // 4. Execute
//! let algorithm = PageRank::new("my_graph");
//! let config = serde_json::json!({"maxIterations": 20});
//! let result = executor.compute(&algorithm, &config)?;
//! ```

// Module declarations
// NOTE: algorithm_spec moved to codegen/procedure/ (the CONTRACT lives with the macros)
mod computation_result;
mod execution_context;
mod execution_mode;
mod executor;
mod result_consumer;
mod validation_config;

// Re-exports - Public API

// Core types
pub use computation_result::ComputationResult;
pub use execution_context::{ContextError, ExecutionContext, LogLevel, MetricsCollector};
pub use execution_mode::ExecutionMode;

// Validation system
pub use validation_config::{
    AfterLoadValidator,
    BeforeLoadValidator,
    NodeLabelExistsValidator,
    PropertyExistsValidator,
    // Example validators
    RangeValidator,
    RequiredParameterValidator,
    ValidationConfiguration,
    ValidationError,
};

// Algorithm contract (imported from codegen/procedure/)
pub use crate::projection::codegen::procedure::{
    get_optional_param,
    // Helper functions
    get_required_param,
    AlgorithmError,
    AlgorithmSpec,
    ConfigError,
    ConsumerError,
    ProjectionHint,
};

// Result consumption
pub use result_consumer::{
    consume_by_mode,
    mutate_node_property_stats,
    mutate_relationship_stats,
    stats_only,
    // Consumer functions
    stream_results,
    train_model,
    write_node_property_stats,
    write_relationship_stats,
    // Output types
    ConsumerOutput,
    MutateSummary,
    StatsSummary,
    TrainSummary,
    WriteSummary,
};

// Main orchestrator
pub use executor::{ExecutorError, ProcedureExecutor};

// Re-export prelude for convenience
pub mod prelude {
    //! Prelude for common procedure executor imports
    pub use super::{
        AlgorithmSpec, ComputationResult, ExecutionContext, ExecutionMode, ProcedureExecutor,
        ValidationConfiguration,
    };
}
