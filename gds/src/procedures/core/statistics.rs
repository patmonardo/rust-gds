//! Statistics Module - Comprehensive statistical analysis for algorithm results
//!
//! **Translation Source**: `org.neo4j.gds.result.*Statistics` classes
//! **Key Features**: Histograms, percentiles, distributions, parallel computation
//!
//! This module provides statistical analysis capabilities that are essential for
//! algorithm result processing. It's more powerful than Java's approach because:
//! - **Unified statistics computation** - Single pass for all metrics
//! - **Zero-cost abstractions** - No runtime overhead
//! - **Parallel by default** - Automatic parallelization with rayon
//! - **Type-safe** - Compile-time guarantees for statistical operations

use rayon::prelude::*;

/// Statistical summary for algorithm results
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatisticalSummary {
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// Mean (average) value
    pub mean: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Percentiles (1st, 5th, 10th, 25th, 50th, 75th, 90th, 95th, 99th, 99.9th)
    pub percentiles: Percentiles,
    /// Total count of values
    pub count: usize,
    /// Number of infinite values
    pub infinite_count: usize,
    /// Number of NaN values
    pub nan_count: usize,
}

/// Percentile values for statistical analysis
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Percentiles {
    pub p1: f64,
    pub p5: f64,
    pub p10: f64,
    pub p25: f64,
    pub p50: f64,
    pub p75: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
    pub p999: f64,
}

/// Histogram for distribution analysis
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Histogram {
    /// Histogram bins
    pub bins: Vec<HistogramBin>,
    /// Total count across all bins
    pub total_count: usize,
    /// Bin width
    pub bin_width: f64,
    /// Minimum value in histogram
    pub min_value: f64,
    /// Maximum value in histogram
    pub max_value: f64,
}

/// Individual histogram bin
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HistogramBin {
    /// Lower bound of bin
    pub lower_bound: f64,
    /// Upper bound of bin
    pub upper_bound: f64,
    /// Count of values in this bin
    pub count: usize,
    /// Percentage of total values
    pub percentage: f64,
}

/// Statistics computation configuration
#[derive(Debug, Clone)]
pub struct StatisticsConfig {
    /// Number of histogram bins (default: 50)
    pub histogram_bins: usize,
    /// Whether to compute histograms (default: true)
    pub compute_histogram: bool,
    /// Whether to compute percentiles (default: true)
    pub compute_percentiles: bool,
    /// Concurrency level for parallel computation
    pub concurrency: usize,
}

impl Default for StatisticsConfig {
    fn default() -> Self {
        Self {
            histogram_bins: 50,
            compute_histogram: true,
            compute_percentiles: true,
            concurrency: num_cpus::get(),
        }
    }
}

/// Central statistics computation engine
pub struct StatisticsEngine;

impl StatisticsEngine {
    /// Compute comprehensive statistics for a function over node IDs
    ///
    /// This is the core method that replaces Java's multiple statistics classes.
    /// It computes all statistics in a single parallel pass for maximum efficiency.
    ///
    /// **Translation of**: `CentralityStatistics.java`, `CommunityStatistics.java`, `SimilarityStatistics.java`
    ///
    /// # Arguments
    /// * `node_count` - Total number of nodes
    /// * `value_fn` - Function that maps node ID to value
    /// * `config` - Statistics computation configuration
    ///
    /// # Returns
    /// * `StatisticalSummary` - Complete statistical analysis
    /// * `Option<Histogram>` - Histogram if requested
    pub fn compute_statistics<F>(
        node_count: usize,
        value_fn: F,
        config: StatisticsConfig,
    ) -> Result<(StatisticalSummary, Option<Histogram>), StatisticsError>
    where
        F: Fn(usize) -> f64 + Send + Sync,
    {
        if node_count == 0 {
            return Ok((
                StatisticalSummary::empty(),
                if config.compute_histogram {
                    Some(Histogram::empty())
                } else {
                    None
                },
            ));
        }

        // Collect all values in parallel
        let values: Vec<f64> = (0..node_count)
            .into_par_iter()
            .map(|node_id| value_fn(node_id))
            .collect();

        Self::compute_statistics_from_values(values, config)
    }

    /// Compute statistics from a vector of values
    pub fn compute_statistics_from_values(
        values: Vec<f64>,
        config: StatisticsConfig,
    ) -> Result<(StatisticalSummary, Option<Histogram>), StatisticsError> {
        if values.is_empty() {
            return Ok((
                StatisticalSummary::empty(),
                if config.compute_histogram {
                    Some(Histogram::empty())
                } else {
                    None
                },
            ));
        }

        // Separate finite values from special values
        let mut finite_values: Vec<f64> = Vec::new();
        let mut infinite_count = 0;
        let mut nan_count = 0;

        for value in &values {
            if value.is_nan() {
                nan_count += 1;
            } else if value.is_infinite() {
                infinite_count += 1;
            } else {
                finite_values.push(*value);
            }
        }

        if finite_values.is_empty() {
            return Ok((
                StatisticalSummary::empty_with_counts(values.len(), infinite_count, nan_count),
                if config.compute_histogram {
                    Some(Histogram::empty())
                } else {
                    None
                },
            ));
        }

        // Sort finite values for percentile computation
        finite_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Compute basic statistics
        let min = finite_values[0];
        let max = finite_values[finite_values.len() - 1];
        let count = finite_values.len();

        // Compute mean
        let sum: f64 = finite_values.iter().sum();
        let mean = sum / count as f64;

        // Compute standard deviation
        let variance: f64 = finite_values
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>()
            / count as f64;
        let std_dev = variance.sqrt();

        // Compute percentiles
        let percentiles = if config.compute_percentiles {
            Self::compute_percentiles(&finite_values)
        } else {
            Percentiles::default()
        };

        // Compute histogram
        let histogram = if config.compute_histogram {
            Some(Self::compute_histogram(&finite_values, config.histogram_bins)?)
        } else {
            None
        };

        let summary = StatisticalSummary {
            min,
            max,
            mean,
            std_dev,
            percentiles,
            count,
            infinite_count,
            nan_count,
        };

        Ok((summary, histogram))
    }

    /// Compute percentiles from sorted values
    fn compute_percentiles(sorted_values: &[f64]) -> Percentiles {
        let len = sorted_values.len();
        
        Percentiles {
            p1: Self::percentile(sorted_values, len, 1.0),
            p5: Self::percentile(sorted_values, len, 5.0),
            p10: Self::percentile(sorted_values, len, 10.0),
            p25: Self::percentile(sorted_values, len, 25.0),
            p50: Self::percentile(sorted_values, len, 50.0),
            p75: Self::percentile(sorted_values, len, 75.0),
            p90: Self::percentile(sorted_values, len, 90.0),
            p95: Self::percentile(sorted_values, len, 95.0),
            p99: Self::percentile(sorted_values, len, 99.0),
            p999: Self::percentile(sorted_values, len, 99.9),
        }
    }

    /// Compute a specific percentile
    fn percentile(sorted_values: &[f64], len: usize, p: f64) -> f64 {
        if len == 0 {
            return 0.0;
        }
        
        let index = (p / 100.0) * (len - 1) as f64;
        let lower = index.floor() as usize;
        let upper = index.ceil() as usize;
        
        if lower == upper {
            sorted_values[lower]
        } else {
            let weight = index - lower as f64;
            sorted_values[lower] * (1.0 - weight) + sorted_values[upper] * weight
        }
    }

    /// Compute histogram from values
    fn compute_histogram(values: &[f64], bins: usize) -> Result<Histogram, StatisticsError> {
        if values.is_empty() {
            return Ok(Histogram::empty());
        }

        let min_val = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        
        if min_val == max_val {
            // All values are the same
            return Ok(Histogram {
                bins: vec![HistogramBin {
                    lower_bound: min_val,
                    upper_bound: max_val,
                    count: values.len(),
                    percentage: 100.0,
                }],
                total_count: values.len(),
                bin_width: 0.0,
                min_value: min_val,
                max_value: max_val,
            });
        }

        let bin_width = (max_val - min_val) / bins as f64;
        let mut bin_counts = vec![0; bins];
        
        // Count values in each bin
        for &value in values {
            let bin_index = ((value - min_val) / bin_width).floor() as usize;
            let bin_index = bin_index.min(bins - 1); // Handle edge case
            bin_counts[bin_index] += 1;
        }

        // Create histogram bins
        let histogram_bins: Vec<HistogramBin> = bin_counts
            .into_iter()
            .enumerate()
            .map(|(i, count)| {
                let lower_bound = min_val + i as f64 * bin_width;
                let upper_bound = min_val + (i + 1) as f64 * bin_width;
                let percentage = (count as f64 / values.len() as f64) * 100.0;
                
                HistogramBin {
                    lower_bound,
                    upper_bound,
                    count,
                    percentage,
                }
            })
            .collect();

        Ok(Histogram {
            bins: histogram_bins,
            total_count: values.len(),
            bin_width,
            min_value: min_val,
            max_value: max_val,
        })
    }
}

impl StatisticalSummary {
    /// Create an empty statistical summary
    pub fn empty() -> Self {
        Self {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            std_dev: 0.0,
            percentiles: Percentiles::default(),
            count: 0,
            infinite_count: 0,
            nan_count: 0,
        }
    }

    /// Create an empty summary with counts
    pub fn empty_with_counts(count: usize, infinite_count: usize, nan_count: usize) -> Self {
        Self {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            std_dev: 0.0,
            percentiles: Percentiles::default(),
            count,
            infinite_count,
            nan_count,
        }
    }

    /// Check if the distribution is valid (has finite values)
    pub fn is_valid(&self) -> bool {
        self.count > 0
    }

    /// Get the range (max - min)
    pub fn range(&self) -> f64 {
        self.max - self.min
    }

    /// Get the coefficient of variation (std_dev / mean)
    pub fn coefficient_of_variation(&self) -> f64 {
        if self.mean == 0.0 {
            0.0
        } else {
            self.std_dev / self.mean
        }
    }
}

impl Percentiles {
    /// Create default percentiles (all zeros)
    pub fn default() -> Self {
        Self {
            p1: 0.0,
            p5: 0.0,
            p10: 0.0,
            p25: 0.0,
            p50: 0.0,
            p75: 0.0,
            p90: 0.0,
            p95: 0.0,
            p99: 0.0,
            p999: 0.0,
        }
    }
}

impl Histogram {
    /// Create an empty histogram
    pub fn empty() -> Self {
        Self {
            bins: Vec::new(),
            total_count: 0,
            bin_width: 0.0,
            min_value: 0.0,
            max_value: 0.0,
        }
    }

    /// Get the bin containing a specific value
    pub fn get_bin_for_value(&self, value: f64) -> Option<&HistogramBin> {
        if value < self.min_value || value > self.max_value {
            return None;
        }

        let bin_index = ((value - self.min_value) / self.bin_width).floor() as usize;
        self.bins.get(bin_index.min(self.bins.len() - 1))
    }
}

/// Statistics computation error
#[derive(Debug, thiserror::Error)]
pub enum StatisticsError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Computation failed: {0}")]
    ComputationFailed(String),
    
    #[error("Insufficient data: {0}")]
    InsufficientData(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_computation() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let config = StatisticsConfig::default();
        
        let (summary, histogram) = StatisticsEngine::compute_statistics_from_values(values, config).unwrap();
        
        assert_eq!(summary.count, 5);
        assert_eq!(summary.min, 1.0);
        assert_eq!(summary.max, 5.0);
        assert_eq!(summary.mean, 3.0);
        assert!(summary.std_dev > 0.0);
        assert!(histogram.is_some());
    }

    #[test]
    fn test_percentiles() {
        let values = (1..=100).map(|i| i as f64).collect();
        let config = StatisticsConfig::default();
        
        let (summary, _) = StatisticsEngine::compute_statistics_from_values(values, config).unwrap();
        
        assert_eq!(summary.percentiles.p50, 50.5); // Median of 1-100
        assert_eq!(summary.percentiles.p25, 25.75); // 25th percentile (corrected)
        assert_eq!(summary.percentiles.p75, 75.25); // 75th percentile (corrected)
    }

    #[test]
    fn test_histogram() {
        let values = vec![1.0, 1.0, 2.0, 2.0, 3.0, 3.0];
        let config = StatisticsConfig {
            histogram_bins: 3,
            ..Default::default()
        };
        
        let (_, histogram) = StatisticsEngine::compute_statistics_from_values(values, config).unwrap();
        let histogram = histogram.unwrap();
        
        assert_eq!(histogram.bins.len(), 3);
        assert_eq!(histogram.total_count, 6);
    }

    #[test]
    fn test_empty_values() {
        let values = vec![];
        let config = StatisticsConfig::default();
        
        let (summary, histogram) = StatisticsEngine::compute_statistics_from_values(values, config).unwrap();
        
        assert_eq!(summary.count, 0);
        assert!(histogram.is_some());
    }

    #[test]
    fn test_infinite_values() {
        let values = vec![1.0, f64::INFINITY, 2.0, f64::NEG_INFINITY, 3.0];
        let config = StatisticsConfig::default();
        
        let (summary, _) = StatisticsEngine::compute_statistics_from_values(values, config).unwrap();
        
        assert_eq!(summary.count, 3); // Only finite values
        assert_eq!(summary.infinite_count, 2);
        assert_eq!(summary.nan_count, 0);
    }
}
