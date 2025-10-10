//! PregelConfig trait - Configuration interface for Pregel algorithms
//!
//! **DEPRECATED**: This file maintains the trait-based API for backward compatibility.
//! New code should use `rust_gds::config::PregelConfig` (struct-based) directly.
//!
//! The trait-based approach is being phased out in favor of the unified config system.

use crate::concurrency::Concurrency;
// Re-export Partitioning from config system (single source of truth)
pub use crate::config::Partitioning;

/// Configuration trait for Pregel algorithms.
///
/// **DEPRECATED**: Use `rust_gds::config::PregelConfig` (struct) instead.
///
/// This trait is maintained for backward compatibility with existing algorithm implementations.
/// It will be removed in a future version once all algorithms migrate to the config system.
///
/// # Migration Guide
///
/// **Old (trait-based)**:
/// ```ignore
/// use rust_gds::pregel::PregelConfig;
///
/// struct PageRankConfig { /* ... */ }
/// impl PregelConfig for PageRankConfig {
///     fn max_iterations(&self) -> usize { 20 }
///     fn concurrency(&self) -> Concurrency { Concurrency::available_cores() }
/// }
/// ```
///
/// **New (struct-based)**:
/// ```ignore
/// use rust_gds::config::{PregelConfig, Partitioning};
///
/// let config = PregelConfig::builder()
///     .max_iterations(20)
///     .concurrency(8)
///     .partitioning(Partitioning::Range)
///     .build()
///     .expect("Valid config");
/// ```
///
/// # Required Configuration
///
/// All Pregel algorithms must specify:
/// - Maximum number of iterations before forced termination
/// - Concurrency level (number of threads)
///
/// # Optional Configuration
///
/// Algorithms can override defaults for:
/// - Asynchronous execution mode
/// - Graph partitioning strategy
/// - Message sender tracking
pub trait PregelConfig: Send + Sync {
    /// Maximum number of supersteps before forcing termination.
    ///
    /// The algorithm will stop after this many iterations even if it hasn't
    /// converged. This prevents infinite loops and provides a safety bound
    /// on execution time.
    ///
    /// # Returns
    ///
    /// Maximum number of iterations (supersteps) to execute
    fn max_iterations(&self) -> usize;

    /// Concurrency level for parallel execution.
    ///
    /// Determines how many threads will be used for parallel vertex computation.
    ///
    /// # Returns
    ///
    /// The number of concurrent threads to use
    fn concurrency(&self) -> Concurrency {
        Concurrency::available_cores()
    }

    /// Whether to run in asynchronous mode.
    ///
    /// - **Synchronous (default)**: All vertices compute in lock-step supersteps.
    ///   Messages sent in superstep N are delivered in superstep N+1.
    ///
    /// - **Asynchronous**: Vertices may compute at different rates. Messages
    ///   may be delivered immediately. Can converge faster but loses determinism.
    ///
    /// # Returns
    ///
    /// `true` for asynchronous mode, `false` for synchronous (default)
    fn is_asynchronous(&self) -> bool {
        false
    }

    /// Graph partitioning strategy.
    ///
    /// Determines how the graph is divided among workers for computation.
    ///
    /// # Returns
    ///
    /// The partitioning strategy to use (default: Range partitioning)
    fn partitioning(&self) -> Partitioning {
        Partitioning::Range
    }

    /// Whether to use ForkJoin execution model.
    ///
    /// When `true`, uses a worker-based execution model with explicit task scheduling.
    /// When `false`, uses simple parallel iteration.
    ///
    /// # Returns
    ///
    /// `true` if AUTO partitioning is selected, `false` otherwise
    fn use_fork_join(&self) -> bool {
        matches!(self.partitioning(), Partitioning::Auto)
    }

    /// Whether to track message senders.
    ///
    /// When enabled, the compute context can identify which node sent each message.
    /// This adds overhead but is needed for some algorithms.
    ///
    /// # Returns
    ///
    /// `true` to track senders, `false` otherwise (default)
    fn track_sender(&self) -> bool {
        false
    }
}

// Implement the trait for the config system struct (bridge old → new)
impl PregelConfig for crate::config::PregelConfig {
    fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn concurrency(&self) -> Concurrency {
        Concurrency::new(self.base.concurrency).unwrap_or_else(|| Concurrency::available_cores())
    }

    fn is_asynchronous(&self) -> bool {
        self.is_asynchronous
    }

    fn partitioning(&self) -> Partitioning {
        self.partitioning
    }

    fn use_fork_join(&self) -> bool {
        crate::config::PregelConfig::use_fork_join(self)
    }

    fn track_sender(&self) -> bool {
        self.track_sender
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple test config
    struct TestConfig {
        max_iters: usize,
        concurrency: Concurrency,
    }

    impl PregelConfig for TestConfig {
        fn max_iterations(&self) -> usize {
            self.max_iters
        }

        fn concurrency(&self) -> Concurrency {
            self.concurrency
        }
    }

    #[test]
    fn test_pregel_config_required_methods() {
        let config = TestConfig {
            max_iters: 20,
            concurrency: Concurrency::new(4).unwrap(),
        };

        assert_eq!(config.max_iterations(), 20);
        assert_eq!(config.concurrency().value(), 4);
    }

    #[test]
    fn test_pregel_config_defaults() {
        let config = TestConfig {
            max_iters: 10,
            concurrency: Concurrency::new(2).unwrap(),
        };

        assert!(!config.is_asynchronous());
        assert_eq!(config.partitioning(), Partitioning::Range);
        assert!(!config.use_fork_join());
        assert!(!config.track_sender());
    }

    #[test]
    fn test_partitioning_parse() {
        assert_eq!(Partitioning::parse("RANGE"), Some(Partitioning::Range));
        assert_eq!(Partitioning::parse("range"), Some(Partitioning::Range));
        assert_eq!(Partitioning::parse("DEGREE"), Some(Partitioning::Degree));
        assert_eq!(Partitioning::parse("degree"), Some(Partitioning::Degree));
        assert_eq!(Partitioning::parse("AUTO"), Some(Partitioning::Auto));
        assert_eq!(Partitioning::parse("auto"), Some(Partitioning::Auto));
        assert_eq!(Partitioning::parse("invalid"), None);
    }

    #[test]
    fn test_partitioning_display() {
        assert_eq!(Partitioning::Range.to_string(), "RANGE");
        assert_eq!(Partitioning::Degree.to_string(), "DEGREE");
        assert_eq!(Partitioning::Auto.to_string(), "AUTO");
    }

    #[test]
    fn test_use_fork_join() {
        struct AutoConfig;
        impl PregelConfig for AutoConfig {
            fn max_iterations(&self) -> usize {
                10
            }
            fn partitioning(&self) -> Partitioning {
                Partitioning::Auto
            }
        }

        let config = AutoConfig;
        assert!(config.use_fork_join());

        let range_config = TestConfig {
            max_iters: 10,
            concurrency: Concurrency::new(4).unwrap(),
        };
        assert!(!range_config.use_fork_join());
    }
}
