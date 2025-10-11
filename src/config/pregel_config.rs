//! Pregel execution configuration
//!
//! Configuration for Pregel-based BSP (Bulk Synchronous Parallel) computation framework.

use super::base_types::{AlgoBaseConfig, ConcurrencyConfig, Config, IterationsConfig};
use super::validation::{ConfigError, ConfigValidation};

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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default)]
pub enum Partitioning {
    /// Divide vertices into contiguous ID ranges.
    ///
    /// Simple and efficient. Good for graphs with uniform degree distribution.
    #[default]
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
    /// use rust_gds::config::Partitioning;
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

/// Pregel execution configuration
///
/// Configures the Pregel BSP computation framework including:
/// - Iteration limits and convergence criteria
/// - Concurrency and parallelism settings
/// - Execution mode (synchronous vs asynchronous)
/// - Partitioning strategy
/// - Message tracking options
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PregelConfig {
    pub base: AlgoBaseConfig,
    pub max_iterations: usize,
    pub tolerance: Option<f64>,
    pub is_asynchronous: bool,
    pub partitioning: Partitioning,
    pub track_sender: bool,
}

impl Default for PregelConfig {
    fn default() -> Self {
        Self {
            base: AlgoBaseConfig::default(),
            max_iterations: 20,
            tolerance: None,
            is_asynchronous: false,
            partitioning: Partitioning::Range,
            track_sender: false,
        }
    }
}

impl Config for PregelConfig {}

impl ConcurrencyConfig for PregelConfig {
    fn concurrency(&self) -> usize {
        self.base.concurrency
    }
}

impl IterationsConfig for PregelConfig {
    fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn tolerance(&self) -> Option<f64> {
        self.tolerance
    }
}

impl PregelConfig {
    pub fn builder() -> PregelConfigBuilder {
        PregelConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_positive(self.base.concurrency as f64, "concurrency")?;
        ConfigValidation::validate_positive(self.max_iterations as f64, "maxIterations")?;
        
        if let Some(tol) = self.tolerance {
            ConfigValidation::validate_positive(tol, "tolerance")?;
        }
        
        Ok(())
    }

    /// Whether to use ForkJoin execution model.
    ///
    /// When `true`, uses a worker-based execution model with explicit task scheduling.
    /// When `false`, uses simple parallel iteration.
    ///
    /// # Returns
    ///
    /// `true` if AUTO partitioning is selected, `false` otherwise
    pub fn use_fork_join(&self) -> bool {
        matches!(self.partitioning, Partitioning::Auto)
    }
}

/// Builder for Pregel configuration
#[derive(Debug, Default)]
pub struct PregelConfigBuilder {
    concurrency: Option<usize>,
    max_iterations: Option<usize>,
    tolerance: Option<f64>,
    is_asynchronous: Option<bool>,
    partitioning: Option<Partitioning>,
    track_sender: Option<bool>,
}

impl PregelConfigBuilder {
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    pub fn max_iterations(mut self, iterations: usize) -> Self {
        self.max_iterations = Some(iterations);
        self
    }

    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = Some(tolerance);
        self
    }

    pub fn is_asynchronous(mut self, is_async: bool) -> Self {
        self.is_asynchronous = Some(is_async);
        self
    }

    pub fn partitioning(mut self, partitioning: Partitioning) -> Self {
        self.partitioning = Some(partitioning);
        self
    }

    pub fn track_sender(mut self, track: bool) -> Self {
        self.track_sender = Some(track);
        self
    }

    pub fn build(self) -> Result<PregelConfig, ConfigError> {
        let defaults = PregelConfig::default();

        let config = PregelConfig {
            base: AlgoBaseConfig {
                concurrency: self.concurrency.unwrap_or(defaults.base.concurrency),
                node_labels: defaults.base.node_labels,
                relationship_types: defaults.base.relationship_types,
            },
            max_iterations: self.max_iterations.unwrap_or(defaults.max_iterations),
            tolerance: self.tolerance.or(defaults.tolerance),
            is_asynchronous: self.is_asynchronous.unwrap_or(defaults.is_asynchronous),
            partitioning: self.partitioning.unwrap_or(defaults.partitioning),
            track_sender: self.track_sender.unwrap_or(defaults.track_sender),
        };

        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pregel_config_defaults() {
        let config = PregelConfig::default();
        
        assert_eq!(config.max_iterations, 20);
        assert_eq!(config.tolerance, None);
        assert!(!config.is_asynchronous);
        assert_eq!(config.partitioning, Partitioning::Range);
        assert!(!config.track_sender);
        assert!(!config.use_fork_join());
    }

    #[test]
    fn test_pregel_config_builder() {
        let config = PregelConfig::builder()
            .concurrency(8)
            .max_iterations(50)
            .tolerance(0.001)
            .is_asynchronous(true)
            .partitioning(Partitioning::Degree)
            .track_sender(true)
            .build()
            .expect("Valid config");

        assert_eq!(config.concurrency(), 8);
        assert_eq!(config.max_iterations(), 50);
        assert_eq!(config.tolerance(), Some(0.001));
        assert!(config.is_asynchronous);
        assert_eq!(config.partitioning, Partitioning::Degree);
        assert!(config.track_sender);
    }

    #[test]
    fn test_pregel_config_validation() {
        let config = PregelConfig::builder()
            .concurrency(4)
            .max_iterations(10)
            .build();

        assert!(config.is_ok());
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
        let auto_config = PregelConfig::builder()
            .partitioning(Partitioning::Auto)
            .build()
            .expect("Valid config");
        assert!(auto_config.use_fork_join());

        let range_config = PregelConfig::builder()
            .partitioning(Partitioning::Range)
            .build()
            .expect("Valid config");
        assert!(!range_config.use_fork_join());
    }
}
