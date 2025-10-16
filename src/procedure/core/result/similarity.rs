//! Similarity Statistics - Statistical analysis for similarity algorithms
//!
//! **Translation Source**: `org.neo4j.gds.result.SimilarityStatistics.java`
//!
//! Provides histogram generation and statistical analysis for similarity
//! algorithm results (Node Similarity, K-Nearest Neighbors, etc.).

use hdrhistogram::Histogram;
use std::collections::HashMap;
use std::time::Instant;

/// Default histogram precision (significant value digits)
const HISTOGRAM_PRECISION: u8 = 5;

/// Scaling factor to convert f64 to u64 for histogram (10^5 for 5 decimal places)
const SCALE_FACTOR: f64 = 100_000.0;

/// Similarity histogram result
#[derive(Debug, Clone)]
pub struct SimilarityHistogram {
    /// Optional histogram of similarity scores
    pub histogram: Option<Histogram<u64>>,
    /// Whether computation succeeded
    pub success: bool,
}

/// Similarity statistics with histogram and timing
#[derive(Debug, Clone)]
pub struct SimilarityStats {
    /// Optional histogram of similarity scores
    pub histogram: Option<Histogram<u64>>,
    /// Time taken to compute statistics (milliseconds)
    pub compute_millis: u64,
    /// Whether computation succeeded
    pub success: bool,
}

impl SimilarityStats {
    /// Generate summary map for display
    pub fn summary(&self) -> HashMap<String, f64> {
        if !self.success {
            let mut map = HashMap::new();
            map.insert("error".to_string(), 1.0);
            return map;
        }

        self.histogram
            .as_ref()
            .map(similarity_summary)
            .unwrap_or_default()
    }
}

/// Compute similarity statistics from relationship iterator
///
/// **Translation**: `SimilarityStatistics.similarityStats()`
///
/// # Arguments
/// * `relationship_fn` - Function that yields (source, target, similarity_score) tuples
/// * `should_compute` - Whether to compute histogram (false = stats only)
///
/// # Type Parameters
/// * `F` - Iterator-producing function
/// * `I` - Iterator over (u64, u64, f64) tuples
pub fn similarity_stats<F, I>(relationship_fn: F, should_compute: bool) -> SimilarityStats
where
    F: FnOnce() -> I,
    I: Iterator<Item = (u64, u64, f64)>,
{
    if !should_compute {
        return SimilarityStats {
            histogram: None,
            compute_millis: 0,
            success: true,
        };
    }

    let start = Instant::now();
    let histogram_result = compute_histogram(relationship_fn);
    let compute_millis = start.elapsed().as_millis() as u64;

    SimilarityStats {
        histogram: histogram_result.histogram,
        compute_millis,
        success: histogram_result.success,
    }
}

/// Compute histogram from relationship properties
///
/// **Translation**: `SimilarityStatistics.computeHistogram()`
///
/// # Arguments
/// * `relationship_fn` - Function that yields (source, target, similarity_score) tuples
///
/// # Type Parameters
/// * `F` - Iterator-producing function
/// * `I` - Iterator over (u64, u64, f64) tuples
pub fn compute_histogram<F, I>(relationship_fn: F) -> SimilarityHistogram
where
    F: FnOnce() -> I,
    I: Iterator<Item = (u64, u64, f64)>,
{
    let mut histogram = match Histogram::new(HISTOGRAM_PRECISION) {
        Ok(h) => h,
        Err(_) => {
            return SimilarityHistogram {
                histogram: None,
                success: false,
            }
        }
    };

    for (_source, _target, similarity) in relationship_fn() {
        // Skip NaN values (like Java's Double.NaN default)
        if similarity.is_nan() {
            continue;
        }

        let scaled_value = (similarity * SCALE_FACTOR) as u64;
        if let Err(e) = histogram.record(scaled_value) {
            // Check if it's a bounds error
            if e.to_string().contains("out of bounds") {
                return SimilarityHistogram {
                    histogram: Some(histogram),
                    success: false,
                };
            }
            // Other errors also result in failure
            return SimilarityHistogram {
                histogram: None,
                success: false,
            };
        }
    }

    SimilarityHistogram {
        histogram: Some(histogram),
        success: true,
    }
}

/// Generate summary statistics from histogram
///
/// **Translation**: `HistogramUtils.similaritySummary()`
fn similarity_summary(histogram: &Histogram<u64>) -> HashMap<String, f64> {
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
    fn test_compute_histogram_success() {
        let similarities = vec![
            (0, 1, 0.5),
            (0, 2, 0.75),
            (1, 2, 0.9),
            (1, 3, 0.85),
            (2, 3, 0.95),
        ];

        let result = compute_histogram(|| similarities.clone().into_iter());

        assert!(result.success);
        assert!(result.histogram.is_some());

        let histogram = result.histogram.unwrap();
        assert_eq!(histogram.len(), 5);
    }

    #[test]
    fn test_compute_histogram_with_nan() {
        let similarities = vec![
            (0, 1, 0.5),
            (0, 2, f64::NAN), // Should be skipped
            (1, 2, 0.9),
        ];

        let result = compute_histogram(|| similarities.clone().into_iter());

        assert!(result.success);
        let histogram = result.histogram.unwrap();
        assert_eq!(histogram.len(), 2); // Only 2 non-NaN values
    }

    #[test]
    fn test_similarity_stats_with_histogram() {
        let similarities = vec![(0, 1, 0.5), (0, 2, 0.75), (1, 2, 0.9)];

        let stats = similarity_stats(|| similarities.into_iter(), true);

        assert!(stats.success);
        assert!(stats.histogram.is_some());
        assert!(stats.compute_millis < 100); // Should be fast
    }

    #[test]
    fn test_similarity_stats_without_histogram() {
        let similarities = vec![(0, 1, 0.5), (0, 2, 0.75)];

        let stats = similarity_stats(|| similarities.into_iter(), false);

        assert!(stats.success);
        assert!(stats.histogram.is_none());
        assert_eq!(stats.compute_millis, 0);
    }

    #[test]
    fn test_similarity_summary() {
        let mut histogram = Histogram::new(HISTOGRAM_PRECISION).unwrap();

        // Record some scaled similarity values (using SCALE_FACTOR)
        for value in [0.050, 0.060, 0.070, 0.080, 0.090] {
            let scaled = (value * SCALE_FACTOR) as u64;
            histogram.record(scaled).unwrap();
        }

        let summary = similarity_summary(&histogram);

        assert!((summary.get("min").unwrap() - 0.050).abs() < 0.001);
        assert!((summary.get("max").unwrap() - 0.090).abs() < 0.001);
        assert!(summary.contains_key("mean"));
        assert!(summary.contains_key("p50"));
        assert!(summary.contains_key("p95"));
    }

    #[test]
    fn test_stats_summary_on_failure() {
        let stats = SimilarityStats {
            histogram: None,
            compute_millis: 0,
            success: false,
        };

        let summary = stats.summary();
        assert_eq!(summary.get("error"), Some(&1.0));
    }

    #[test]
    fn test_empty_relationships() {
        let similarities: Vec<(u64, u64, f64)> = vec![];

        let result = compute_histogram(|| similarities.into_iter());

        assert!(result.success);
        assert!(result.histogram.is_some());

        let histogram = result.histogram.unwrap();
        assert_eq!(histogram.len(), 0);
    }
}
