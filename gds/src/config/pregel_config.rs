//! Pregel execution configuration
//!
//! Macro-generated demo of PregelConfig using the in-repo `define_config!` macro.

// The demo macro is defined in `projection::codegen::config::define_config.rs` and exported
// as `define_config!`. This file invokes that macro at crate level to produce
// a struct, builder, Default, and validation bridge.

use crate::define_config;

define_config!(
    pub struct PregelConfig {
        validate = |cfg: &PregelConfig| {
            crate::config::validation::ConfigValidation::validate_positive(cfg.base.concurrency as f64, "concurrency")?;
            crate::config::validation::ConfigValidation::validate_positive(cfg.max_iterations as f64, "maxIterations")?;
            if let Some(tol) = cfg.tolerance {
                crate::config::validation::ConfigValidation::validate_positive(tol, "tolerance")?;
            }
            Ok(())
        },
        base: crate::config::base_types::AlgoBaseConfig = crate::config::base_types::AlgoBaseConfig::default(),
        max_iterations: usize = 20,
        tolerance: Option<f64> = None,
        is_asynchronous: bool = false,
        partitioning: crate::core::utils::partition::Partitioning = crate::core::utils::partition::Partitioning::Range,
        track_sender: bool = false,
    }
);

impl crate::config::IterationsConfig for PregelConfig {
    fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn tolerance(&self) -> Option<f64> {
        self.tolerance
    }
}

impl crate::config::ConcurrencyConfig for PregelConfig {
    fn concurrency(&self) -> usize {
        self.base.concurrency
    }
}

/// Common interface for Pregel runtime configuration.
pub trait PregelRuntimeConfig:
    crate::config::IterationsConfig + crate::config::ConcurrencyConfig + Clone + Send + Sync
{
    fn is_asynchronous(&self) -> bool;
    fn partitioning(&self) -> crate::core::utils::partition::Partitioning;
    fn track_sender(&self) -> bool;

    fn use_fork_join(&self) -> bool {
        matches!(
            self.partitioning(),
            crate::core::utils::partition::Partitioning::Auto
        )
    }
}

impl PregelRuntimeConfig for PregelConfig {
    fn is_asynchronous(&self) -> bool {
        self.is_asynchronous
    }

    fn partitioning(&self) -> crate::core::utils::partition::Partitioning {
        self.partitioning
    }

    fn track_sender(&self) -> bool {
        self.track_sender
    }
}
