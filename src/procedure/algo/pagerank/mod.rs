//! PageRank Algorithm - Path Knowledge in Practice
//!
//! PageRank embodies the **Path of Jna dividing itself into Prajna and Jnana**:
//!
//! ```text
//! Jna (Absolute network potential)
//!     ↓ (divides via Dharma)
//! Prajna ↔ Jnana
//! (distributed edge potential) ↔ (aggregated node knowledge)
//!     ↑ (via message-passing Functor)
//! ```
//!
//! ## Architecture
//!
//! - **`spec.rs`**: PageRankAlgorithmSpec (implements AlgorithmSpec trait)
//! - **`storage.rs`**: Storage runtime (reads PropertyValues, manages state across iterations)
//! - **`computation.rs`**: Computation runtime (accumulates scores, propagates messages)
//! - **`executor.rs`** (if needed): Iteration control, convergence detection
//!
//! ## How PageRank Walks the Path
//!
//! Each iteration:
//! 1. **Validator** (apprehends): Recognizes current score form in storage
//! 2. **Projector** (reproduces): Reveals duality (storage scores ↔ message-passing compute)
//! 3. **Functor** (recognition): Converts PropertyValues → computations and back
//! 4. **Execution**: Scores propagate via edges (Prajna → Jnana)
//! 5. **Convergence**: When new scores stabilize, Path's step completes
//!
//! See `doc/adr0020_pagerank_path_knowledge.md` for philosophy.

pub mod computation;
pub mod spec;
pub mod storage;

pub use spec::{PageRankAlgorithmSpec, PageRankComputationResult};
