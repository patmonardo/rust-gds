//! Centrality Statistics - Statistical analysis for centrality algorithms
//!
//! **Translation Source**: `org.neo4j.gds.result.CentralityStatistics.java`
//!
//! Provides histogram generation and statistical analysis for centrality
//! algorithm results (PageRank, Betweenness, Degree, etc.).

use hdrhistogram::Histogram;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

/// Default histogram precision (significant value digits)
const HISTOGRAM_PRECISION: u8 = 5;

/// Scaling factor to convert f64 to u64 for histogram (10^5 for 5 decimal places)
const SCALE_FACTOR: f64 = 100_000.0;

/// Centrality statistics result
#[derive(Debug, Clone)]
pub struct CentralityStats {
    /// Optional histogram of centrality values (scaled to u64)
    /// Values are multiplied by SCALE_FACTOR before recording
    pub histogram: Option<Histogram<u64>>,
    /// Time taken to compute statistics (milliseconds)
    pub compute_millis: u64,
    /// Whether computation succeeded
    pub success: bool,
}

impl CentralityStats {
    /// Create successful stats with histogram
    pub fn with_histogram(histogram: Histogram<u64>, compute_millis: u64) -> Self {
        Self {
            histogram: Some(histogram),
            compute_millis,
            success: true,
        }
    }

    /// Create successful stats without histogram
    pub fn without_histogram(compute_millis: u64) -> Self {
        Self {
            histogram: None,
            compute_millis,
            success: true,
        }
    }

    /// Create failed stats (histogram out of bounds)
    pub fn failed(compute_millis: u64) -> Self {
        Self {
            histogram: None,
            compute_millis,
            success: false,
        }
    }

    /// Generate summary map for display
    pub fn summary(&self) -> HashMap<String, f64> {
        if !self.success {
            let mut map = HashMap::new();
            map.insert("error".to_string(), 1.0);
            return map;
        }

        self.histogram
            .as_ref()
            .map(|h| centrality_summary(h))
            .unwrap_or_default()
    }
}

/// Compute centrality statistics
///
/// **Translation**: `CentralityStatistics.centralityStatistics()`
///
/// # Arguments
/// * `node_count` - Number of nodes in the graph
/// * `centrality_fn` - Function mapping node_id → centrality value
/// * `concurrency` - Number of parallel threads
/// * `should_compute` - Whether to compute histogram (false = stats only)
pub fn centrality_statistics<F>(
    node_count: u64,
    centrality_fn: F,
    concurrency: usize,
    should_compute: bool,
) -> CentralityStats
where
    F: Fn(u64) -> f64 + Send + Sync,
{
    let start = Instant::now();

    if !should_compute {
        return CentralityStats::without_histogram(start.elapsed().as_millis() as u64);
    }

    match build_histogram(node_count, &centrality_fn, concurrency) {
        Ok(histogram) => {
            CentralityStats::with_histogram(histogram, start.elapsed().as_millis() as u64)
        }
        Err(_) => CentralityStats::failed(start.elapsed().as_millis() as u64),
    }
}

/// Build histogram of centrality values
///
/// **Translation**: `CentralityStatistics.histogram()`
fn build_histogram<F>(
    node_count: u64,
    centrality_fn: &F,
    concurrency: usize,
) -> Result<Histogram<u64>, String>
where
    F: Fn(u64) -> f64 + Send + Sync,
{
    let out_of_bounds = AtomicBool::new(false);

    if concurrency == 1 {
        // Single-threaded path
        let mut histogram = Histogram::new(HISTOGRAM_PRECISION)
            .map_err(|e| format!("Failed to create histogram: {}", e))?;

        for node_id in 0..node_count {
            let value = centrality_fn(node_id);
            let scaled_value = (value * SCALE_FACTOR) as u64;
            if let Err(_) = histogram.record(scaled_value) {
                return Err("Value out of bounds for histogram".to_string());
            }
        }

        Ok(histogram)
    } else {
        // Parallel path - partition work across threads
        let chunk_size = (node_count + concurrency as u64 - 1) / concurrency as u64;

        let histograms: Result<Vec<_>, _> = (0..concurrency)
            .into_par_iter()
            .map(|chunk_idx| {
                if out_of_bounds.load(Ordering::Relaxed) {
                    return Err("Out of bounds".to_string());
                }

                let start = chunk_idx as u64 * chunk_size;
                let end = ((chunk_idx + 1) as u64 * chunk_size).min(node_count);

                let mut local_histogram = Histogram::new(HISTOGRAM_PRECISION)
                    .map_err(|e| format!("Failed to create histogram: {}", e))?;

                for node_id in start..end {
                    let value = centrality_fn(node_id);
                    let scaled_value = (value * SCALE_FACTOR) as u64;
                    if let Err(_) = local_histogram.record(scaled_value) {
                        out_of_bounds.store(true, Ordering::Relaxed);
                        return Err("Value out of bounds for histogram".to_string());
                    }
                }

                Ok(local_histogram)
            })
            .collect();

        let histograms = histograms?;

        // Merge all histograms
        let mut iter = histograms.into_iter();
        let mut merged = iter.next().unwrap();
        for hist in iter {
            merged
                .add(&hist)
                .map_err(|e| format!("Failed to merge histograms: {}", e))?;
        }

        Ok(merged)
    }
}

/// Generate summary statistics from histogram
///
/// **Translation**: `HistogramUtils.centralitySummary()`
fn centrality_summary(histogram: &Histogram<u64>) -> HashMap<String, f64> {
    let mut summary = HashMap::new();

    // Unscale all values back to f64
    summary.insert("min".to_string(), histogram.min() as f64 / SCALE_FACTOR);
    summary.insert("max".to_string(), histogram.max() as f64 / SCALE_FACTOR);
    summary.insert("mean".to_string(), histogram.mean() / SCALE_FACTOR);
    summary.insert(
        "p50".to_string(),
        histogram.value_at_quantile(0.50) as f64 / SCALE_FACTOR,
    );
    summary.insert(
        "p75".to_string(),
        histogram.value_at_quantile(0.75) as f64 / SCALE_FACTOR,
    );
    summary.insert(
        "p90".to_string(),
        histogram.value_at_quantile(0.90) as f64 / SCALE_FACTOR,
    );
    summary.insert(
        "p95".to_string(),
        histogram.value_at_quantile(0.95) as f64 / SCALE_FACTOR,
    );
    summary.insert(
        "p99".to_string(),
        histogram.value_at_quantile(0.99) as f64 / SCALE_FACTOR,
    );
    summary.insert(
        "p999".to_string(),
        histogram.value_at_quantile(0.999) as f64 / SCALE_FACTOR,
    );

    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centrality_statistics_single_threaded() {
        let node_count = 100;
        let centrality_fn = |node_id: u64| node_id as f64 / 10.0;

        let stats = centrality_statistics(node_count, centrality_fn, 1, true);

        assert!(stats.success);
        assert!(stats.histogram.is_some());

        let summary = stats.summary();
        assert_eq!(summary.get("min"), Some(&0.0));
        assert!(summary.get("max").is_some());
        assert!(summary.get("mean").is_some());
    }

    #[test]
    fn test_centrality_statistics_parallel() {
        let node_count = 1000;
        let centrality_fn = |node_id: u64| (node_id as f64).sqrt();

        let stats = centrality_statistics(node_count, centrality_fn, 4, true);

        assert!(stats.success);
        assert!(stats.histogram.is_some());

        let summary = stats.summary();
        assert!(summary.contains_key("p50"));
        assert!(summary.contains_key("p99"));
    }

    #[test]
    fn test_centrality_statistics_without_histogram() {
        let node_count = 100;
        let centrality_fn = |node_id: u64| node_id as f64;

        let stats = centrality_statistics(node_count, centrality_fn, 1, false);

        assert!(stats.success);
        assert!(stats.histogram.is_none());
        assert_eq!(stats.summary().len(), 0);
    }

    #[test]
    fn test_summary_with_histogram() {
        let mut histogram = Histogram::new(HISTOGRAM_PRECISION).unwrap();
        for i in 0..100 {
            let scaled = (i as f64 * SCALE_FACTOR) as u64;
            histogram.record(scaled).unwrap();
        }

        let summary = centrality_summary(&histogram);

        // Values should be unscaled back to original range
        assert!((summary.get("min").unwrap() - 0.0).abs() < 0.01);
        assert!((summary.get("max").unwrap() - 99.0).abs() < 0.01);
        assert!(summary.contains_key("mean"));
        assert!(summary.contains_key("p50"));
        assert!(summary.contains_key("p95"));
    }

    #[test]
    fn test_failed_stats() {
        let stats = CentralityStats::failed(123);

        assert!(!stats.success);
        assert!(stats.histogram.is_none());
        assert_eq!(stats.compute_millis, 123);

        let summary = stats.summary();
        assert_eq!(summary.get("error"), Some(&1.0));
    }
}
