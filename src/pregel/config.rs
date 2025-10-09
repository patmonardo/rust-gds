//! PregelConfig trait - Configuration for Pregel algorithms
//!
//! Defines the configuration interface that all Pregel algorithms must implement.

use crate::concurrency::Concurrency;

/// Configuration trait for Pregel algorithms.
///
/// This trait combines several configuration concerns:
/// - Iteration limits
/// - Concurrency settings
/// - Execution mode (synchronous vs asynchronous)
/// - Partitioning strategy
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
///
/// # Example
///
/// ```
/// use rust_gds::pregel::PregelConfig;
/// use rust_gds::concurrency::Concurrency;
///
/// struct PageRankConfig {
///     max_iterations: usize,
///     concurrency: Concurrency,
///     damping_factor: f64,
/// }
///
/// impl PregelConfig for PageRankConfig {
///     fn max_iterations(&self) -> usize {
///         self.max_iterations
///     }
///     
///     fn concurrency(&self) -> Concurrency {
///         self.concurrency
///     }
/// }
/// ```
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

/// Graph partitioning strategies for Pregel computation.
///
/// Determines how vertices are distributed across parallel workers.
///
/// # Strategies
///
/// - **Range**: Vertices divided into contiguous ranges (e.g., [0-999], [1000-1999])
/// - **Degree**: Vertices grouped by degree to balance computation load
/// - **Auto**: Framework chooses the best strategy based on graph properties
///
/// # Performance Impact
///
/// - **Range** is simplest and has lowest overhead
/// - **Degree** can provide better load balancing for skewed degree distributions
/// - **Auto** adds analysis overhead but may optimize for specific graph shapes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Partitioning {
    /// Divide vertices into contiguous ID ranges.
    ///
    /// Simple and efficient. Good for graphs with uniform degree distribution.
    Range,

    /// Group vertices by degree to balance computation.
    ///
    /// Better for power-law graphs where a few vertices have very high degree.
    Degree,

    /// Let the framework choose the best strategy.
    ///
    /// Analyzes graph properties to select optimal partitioning.
    Auto,
}

impl Partitioning {
    /// Parse a partitioning strategy from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - String representation: "RANGE", "DEGREE", or "AUTO" (case-insensitive)
    ///
    /// # Returns
    ///
    /// The parsed `Partitioning` value, or `None` if invalid
    ///
    /// # Example
    ///
    /// ```
    /// use rust_gds::pregel::Partitioning;
    ///
    /// assert_eq!(Partitioning::parse("RANGE"), Some(Partitioning::Range));
    /// assert_eq!(Partitioning::parse("degree"), Some(Partitioning::Degree));
    /// assert_eq!(Partitioning::parse("invalid"), None);
    /// ```
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "RANGE" => Some(Partitioning::Range),
            "DEGREE" => Some(Partitioning::Degree),
            "AUTO" => Some(Partitioning::Auto),
            _ => None,
        }
    }

    /// Convert partitioning strategy to string representation.
    ///
    /// # Returns
    ///
    /// Uppercase string: "RANGE", "DEGREE", or "AUTO"
    pub fn to_string_upper(&self) -> &'static str {
        match self {
            Partitioning::Range => "RANGE",
            Partitioning::Degree => "DEGREE",
            Partitioning::Auto => "AUTO",
        }
    }
}

impl std::fmt::Display for Partitioning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_upper())
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
