//! Base builder infrastructure for algorithm facades
//!
//! Provides common types and patterns used across all facade builders,
//! reducing boilerplate while maintaining consistency.

use std::time::{Duration, Instant};
use super::traits::Result;
use crate::projection::eval::procedure::AlgorithmError;

/// Execution context tracking for algorithm runs
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// When execution started
    pub started_at: Instant,
    /// Number of nodes in the graph
    pub node_count: u64,
    /// Number of edges in the graph
    pub edge_count: u64,
    /// Maximum iterations for iterative algorithms
    pub max_iterations: u32,
}

impl ExecutionContext {
    /// Create a new execution context
    pub fn new(node_count: u64, edge_count: u64) -> Self {
        Self {
            started_at: Instant::now(),
            node_count,
            edge_count,
            max_iterations: 1,
        }
    }

    /// Get elapsed time since start
    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    /// Get elapsed time in milliseconds
    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed().as_millis() as u64
    }

    /// Set maximum iterations for iterative algorithms
    pub fn with_max_iterations(mut self, max_iterations: u32) -> Self {
        self.max_iterations = max_iterations;
        self
    }
}

// ============================================================================
// Mutation and Write Results
// ============================================================================

/// Result of a mutation operation
#[derive(Debug, Clone)]
pub struct MutationResult {
    /// Number of nodes updated
    pub nodes_updated: u64,
    /// Property name created/updated
    pub property_name: String,
    /// Execution time
    pub execution_time: Duration,
}

impl MutationResult {
    /// Create a new mutation result
    pub fn new(nodes_updated: u64, property_name: String, execution_time: Duration) -> Self {
        Self {
            nodes_updated,
            property_name,
            execution_time,
        }
    }

    /// Get execution time in milliseconds
    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time.as_millis() as u64
    }
}

/// Result of a write operation
#[derive(Debug, Clone)]
pub struct WriteResult {
    /// Number of nodes written
    pub nodes_written: u64,
    /// Property name written
    pub property_name: String,
    /// Execution time
    pub execution_time: Duration,
}

impl WriteResult {
    /// Create a new write result
    pub fn new(nodes_written: u64, property_name: String, execution_time: Duration) -> Self {
        Self {
            nodes_written,
            property_name,
            execution_time,
        }
    }

    /// Get execution time in milliseconds
    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time.as_millis() as u64
    }
}

// ============================================================================
// Builder Pattern Utilities
// ============================================================================

/// Common configuration validation
pub struct ConfigValidator;

impl ConfigValidator {
    /// Validate that a value is positive
    pub fn positive(value: f64, field_name: &str) -> Result<()> {
        if value <= 0.0 {
            return Err(AlgorithmError::Execution(
                format!("{} must be positive, got {}", field_name, value)
            ));
        }
        Ok(())
    }

    /// Validate that a value is in range [min, max]
    pub fn in_range(value: f64, min: f64, max: f64, field_name: &str) -> Result<()> {
        if value < min || value > max {
            return Err(AlgorithmError::Execution(
                format!("{} must be in range [{}, {}], got {}", field_name, min, max, value)
            ));
        }
        Ok(())
    }

    /// Validate that an iteration count is reasonable
    pub fn iterations(value: u32, field_name: &str) -> Result<()> {
        if value == 0 || value > 1_000_000 {
            return Err(AlgorithmError::Execution(
                format!("{} must be > 0 and <= 1_000_000, got {}", field_name, value)
            ));
        }
        Ok(())
    }

    /// Validate that a property name is non-empty
    pub fn non_empty_string(value: &str, field_name: &str) -> Result<()> {
        if value.is_empty() {
            return Err(AlgorithmError::Execution(
                format!("{} cannot be empty", field_name)
            ));
        }
        Ok(())
    }
}

// ============================================================================
// Result Aggregation Utilities
// ============================================================================

/// Utilities for computing statistics from result sets
pub struct StatsAggregator;

impl StatsAggregator {
    /// Compute percentile from sorted values
    pub fn percentile(sorted_values: &[f64], p: f64) -> Option<f64> {
        if sorted_values.is_empty() {
            return None;
        }
        if p <= 0.0 || p > 100.0 {
            return None;
        }

        let index = ((p / 100.0) * sorted_values.len() as f64) as usize;
        sorted_values.get(index.min(sorted_values.len() - 1)).copied()
    }

    /// Compute mean from values
    pub fn mean(values: &[f64]) -> Option<f64> {
        if values.is_empty() {
            return None;
        }
        let sum: f64 = values.iter().sum();
        Some(sum / values.len() as f64)
    }

    /// Compute standard deviation from values
    pub fn stddev(values: &[f64]) -> Option<f64> {
        if values.is_empty() {
            return None;
        }
        let mean = Self::mean(values)?;
        let variance = values
            .iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>()
            / values.len() as f64;
        Some(variance.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_context() {
        let ctx = ExecutionContext::new(1000, 5000);
        assert_eq!(ctx.node_count, 1000);
        assert_eq!(ctx.edge_count, 5000);
        std::thread::sleep(Duration::from_millis(10));
        assert!(ctx.elapsed_ms() >= 10);
    }

    #[test]
    fn test_stats_aggregator_mean() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(StatsAggregator::mean(&values), Some(3.0));
    }

    #[test]
    fn test_stats_aggregator_percentile() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let p50 = StatsAggregator::percentile(&values, 50.0);
        assert!(p50.is_some());
    }
}
