//! Procedure Contract - AlgorithmSpec Trait
//!
//! This module contains THE CONTRACT that all algorithms must implement
//! to work with the ProcedureExecutor (GDSL Runtime).
//!
//! **Moved from**: `src/projection/eval/procedure/algorithm_spec.rs`
//! **Why moved**: Macros in `codegen/macros/procedure/` generate AlgorithmSpec impls
//!
//! ## Architecture
//!
//! ```text
//! codegen/procedure/
//! └── algorithm_spec.rs  ← THE CONTRACT (this file)
//!
//! codegen/macros/procedure/
//! ├── algorithm.rs       ← Generates AlgorithmSpec impls
//! └── config.rs          ← Generates config structs
//!
//! eval/procedure/
//! └── executor.rs        ← Executes AlgorithmSpec impls
//!
//! procedure/
//! ├── pagerank.rs        ← impl AlgorithmSpec for PageRank
//! └── louvain.rs         ← impl AlgorithmSpec for Louvain
//! ```
//!
//! ## The Pattern
//!
//! 1. **Contract** (here): `pub trait AlgorithmSpec { ... }`
//! 2. **Generators** (macros/procedure/): Generate `impl AlgorithmSpec for X`
//! 3. **Executor** (eval/procedure/): `executor.compute::<A: AlgorithmSpec>(...)`
//! 4. **Implementations** (procedure/): Concrete PageRank, Louvain, etc.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use rust_gds::projection::codegen::procedure::AlgorithmSpec;
//!
//! struct PageRankAlgorithm {
//!     graph_name: String,
//!     config: PageRankConfig,
//! }
//!
//! impl AlgorithmSpec for PageRankAlgorithm {
//!     type Output = Vec<(NodeId, f64)>;
//!     
//!     fn name(&self) -> &str { "pagerank" }
//!     fn graph_name(&self) -> &str { &self.graph_name }
//!     // ... implement other methods
//! }
//! ```

pub mod algorithm_spec;

// Re-export the contract and all supporting types
pub use algorithm_spec::{
    get_optional_param, get_required_param, AlgorithmError, AlgorithmSpec, ConfigError,
    ConsumerError, ProjectionHint,
};
