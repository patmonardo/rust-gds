//! Community Statistics - Statistical analysis for community detection algorithms
//!
//! **Translation Source**: `org.neo4j.gds.result.CommunityStatistics.java`
//!
//! Provides community size distributions and histogram generation for community
//! detection algorithms (Louvain, Label Propagation, etc.).

use crate::collections::HugeSparseLongArray;
use hdrhistogram::Histogram;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// Empty community marker (communities with size 0)
const EMPTY_COMMUNITY: i64 = 0;

/// Default histogram precision (significant value digits)
const HISTOGRAM_PRECISION: u8 = 5;

/// Community statistics result with count and histogram
#[derive(Debug, Clone)]
pub struct CommunityCountAndHistogram {
    /// Number of distinct communities
    pub component_count: u64,
    /// Histogram of community sizes
    pub histogram: Histogram<u64>,
}

/// Community statistics with optional histogram and timing
#[derive(Debug, Clone)]
pub struct CommunityStats {
    /// Number of distinct communities
    pub component_count: u64,
    /// Optional histogram of community sizes
    pub histogram: Option<Histogram<u64>>,
    /// Time taken to compute statistics (milliseconds)
    pub compute_millis: u64,
    /// Whether computation succeeded
    pub success: bool,
}

impl CommunityStats {
    /// Generate summary map for display
    pub fn summary(&self) -> HashMap<String, f64> {
        if !self.success {
            let mut map = HashMap::new();
            map.insert("error".to_string(), 1.0);
            return map;
        }

        self.histogram
            .as_ref()
            .map(community_summary)
            .unwrap_or_default()
    }
}

/// Instructions for what statistics to compute
#[derive(Debug, Clone, Copy)]
pub struct StatisticsComputationInstructions {
    compute_count_and_distribution: bool,
    compute_count_only: bool,
}

impl StatisticsComputationInstructions {
    /// Compute both count and distribution
    pub fn count_and_distribution() -> Self {
        Self {
            compute_count_and_distribution: true,
            compute_count_only: false,
        }
    }

    /// Compute count only (no histogram)
    pub fn count_only() -> Self {
        Self {
            compute_count_and_distribution: false,
            compute_count_only: true,
        }
    }

    /// Compute nothing
    pub fn none() -> Self {
        Self {
            compute_count_and_distribution: false,
            compute_count_only: false,
        }
    }

    pub fn compute_count_and_distribution(&self) -> bool {
        self.compute_count_and_distribution
    }

    pub fn compute_count_only(&self) -> bool {
        self.compute_count_only
    }
}

/// Compute community sizes from node → community mapping
///
/// **Translation**: `CommunityStatistics.communitySizes()`
///
/// # Arguments
/// * `node_count` - Number of nodes in graph
/// * `community_fn` - Function mapping node_id → community_id
/// * `concurrency` - Number of parallel threads
pub fn community_sizes<F>(
    node_count: u64,
    community_fn: F,
    concurrency: usize,
) -> HugeSparseLongArray
where
    F: Fn(u64) -> i64 + Send + Sync,
{
    let mut builder = HugeSparseLongArray::builder(EMPTY_COMMUNITY);

    if concurrency == 1 {
        // Single-threaded: direct iteration
        for node_id in 0..node_count {
            let community_id = community_fn(node_id);
            builder.add_to(community_id as usize, 1);
        }
    } else {
        // Parallel: partition work across threads
        let chunk_size = node_count.div_ceil(concurrency as u64);

        let local_builders: Vec<_> = (0..concurrency)
            .into_par_iter()
            .map(|chunk_idx| {
                let mut local_builder = HugeSparseLongArray::builder(EMPTY_COMMUNITY);
                let start = chunk_idx as u64 * chunk_size;
                let end = ((chunk_idx + 1) as u64 * chunk_size).min(node_count);

                for node_id in start..end {
                    let community_id = community_fn(node_id);
                    local_builder.add_to(community_id as usize, 1);
                }

                local_builder
            })
            .collect();

        // Merge local builders into main builder
        for local_builder in local_builders {
            let local_array = local_builder.build();
            for index in 0..local_array.capacity() {
                if local_array.contains(index) {
                    let value = local_array.get(index);
                    if value != EMPTY_COMMUNITY {
                        builder.add_to(index, value);
                    }
                }
            }
        }
    }

    builder.build()
}

/// Count number of distinct communities
///
/// **Translation**: `CommunityStatistics.communityCount(nodeCount, ...)`
pub fn community_count<F>(node_count: u64, community_fn: F, concurrency: usize) -> u64
where
    F: Fn(u64) -> i64 + Send + Sync,
{
    let sizes = community_sizes(node_count, community_fn, concurrency);
    community_count_from_sizes(&sizes, concurrency)
}

/// Count communities from pre-computed sizes
///
/// **Translation**: `CommunityStatistics.communityCount(communitySizes, ...)`
pub fn community_count_from_sizes(
    community_sizes: &HugeSparseLongArray,
    concurrency: usize,
) -> u64 {
    let capacity = community_sizes.capacity();

    if concurrency == 1 {
        let mut count = 0u64;
        for community_id in 0..capacity {
            if community_sizes.get(community_id) != EMPTY_COMMUNITY {
                count += 1;
            }
        }
        count
    } else {
        // Parallel counting
        let chunk_size = capacity.div_ceil(concurrency);
        let count = AtomicU64::new(0);

        (0..concurrency).into_par_iter().for_each(|chunk_idx| {
            let start = chunk_idx * chunk_size;
            let end = ((chunk_idx + 1) * chunk_size).min(capacity);
            let mut local_count = 0u64;

            for community_id in start..end {
                if community_sizes.get(community_id) != EMPTY_COMMUNITY {
                    local_count += 1;
                }
            }

            count.fetch_add(local_count, Ordering::Relaxed);
        });

        count.load(Ordering::Relaxed)
    }
}

/// Compute community count and histogram
///
/// **Translation**: `CommunityStatistics.communityCountAndHistogram(nodeCount, ...)`
pub fn community_count_and_histogram<F>(
    node_count: u64,
    community_fn: F,
    concurrency: usize,
) -> Result<CommunityCountAndHistogram, String>
where
    F: Fn(u64) -> i64 + Send + Sync,
{
    let sizes = community_sizes(node_count, community_fn, concurrency);
    community_count_and_histogram_from_sizes(&sizes, concurrency)
}

/// Compute community count and histogram from pre-computed sizes
///
/// **Translation**: `CommunityStatistics.communityCountAndHistogram(communitySizes, ...)`
pub fn community_count_and_histogram_from_sizes(
    community_sizes: &HugeSparseLongArray,
    concurrency: usize,
) -> Result<CommunityCountAndHistogram, String> {
    let mut histogram;
    let mut community_count = 0u64;

    if concurrency == 1 {
        // Single-threaded path
        histogram =
            Histogram::new(HISTOGRAM_PRECISION).map_err(|e| format!("Histogram error: {}", e))?;

        let capacity = community_sizes.capacity();
        for community_id in 0..capacity {
            let community_size = community_sizes.get(community_id);
            if community_size != EMPTY_COMMUNITY {
                community_count += 1;
                histogram
                    .record(community_size as u64)
                    .map_err(|e| format!("Histogram record error: {}", e))?;
            }
        }
    } else {
        // Parallel path
        let capacity = community_sizes.capacity();
        let chunk_size = capacity.div_ceil(concurrency);

        let results: Result<Vec<_>, String> = (0..concurrency)
            .into_par_iter()
            .map(|chunk_idx| {
                let start = chunk_idx * chunk_size;
                let end = ((chunk_idx + 1) * chunk_size).min(capacity);

                let mut local_histogram = Histogram::new(HISTOGRAM_PRECISION)
                    .map_err(|e| format!("Histogram error: {}", e))?;
                let mut local_count = 0u64;

                for community_id in start..end {
                    let community_size = community_sizes.get(community_id);
                    if community_size != EMPTY_COMMUNITY {
                        local_count += 1;
                        local_histogram
                            .record(community_size as u64)
                            .map_err(|e| format!("Histogram record error: {}", e))?;
                    }
                }

                Ok((local_count, local_histogram))
            })
            .collect();

        let task_results = results?;

        // Find maximum value for final histogram
        let mut highest_trackable_value = 2u64; // Must be >= 2
        for (count, hist) in &task_results {
            community_count += count;
            if hist.max() > highest_trackable_value {
                highest_trackable_value = hist.max();
            }
        }

        // Create final histogram with appropriate range
        histogram = Histogram::new_with_max(highest_trackable_value, HISTOGRAM_PRECISION)
            .map_err(|e| format!("Histogram creation error: {}", e))?;

        // Merge task histograms
        for (_count, task_hist) in task_results {
            histogram
                .add(&task_hist)
                .map_err(|e| format!("Histogram merge error: {}", e))?;
        }
    }

    Ok(CommunityCountAndHistogram {
        component_count: community_count,
        histogram,
    })
}

/// Compute complete community statistics with optional histogram
///
/// **Translation**: `CommunityStatistics.communityStats()`
pub fn community_stats<F>(
    node_count: u64,
    community_fn: F,
    concurrency: usize,
    instructions: StatisticsComputationInstructions,
) -> CommunityStats
where
    F: Fn(u64) -> i64 + Send + Sync,
{
    let mut component_count = 0u64;
    let mut histogram = None;
    let start = Instant::now();

    let result: Result<(), String> = (|| {
        if instructions.compute_count_and_distribution() {
            let stats = community_count_and_histogram(node_count, &community_fn, concurrency)?;
            component_count = stats.component_count;
            histogram = Some(stats.histogram);
        } else if instructions.compute_count_only() {
            component_count = community_count(node_count, &community_fn, concurrency);
        }
        Ok(())
    })();

    let compute_millis = start.elapsed().as_millis() as u64;

    match result {
        Ok(_) => CommunityStats {
            component_count,
            histogram,
            compute_millis,
            success: true,
        },
        Err(e) => {
            // Check if error is histogram bounds issue
            let is_bounds_error = e.to_string().contains("out of bounds");
            CommunityStats {
                component_count: if is_bounds_error { 0 } else { component_count },
                histogram: None,
                compute_millis,
                success: false,
            }
        }
    }
}

/// Generate summary statistics from histogram
///
/// **Translation**: `HistogramUtils.communitySummary()`
fn community_summary(histogram: &Histogram<u64>) -> HashMap<String, f64> {
    let mut summary = HashMap::new();

    summary.insert("min".to_string(), histogram.min() as f64);
    summary.insert("max".to_string(), histogram.max() as f64);
    summary.insert("mean".to_string(), histogram.mean());
    summary.insert("p50".to_string(), histogram.value_at_quantile(0.50) as f64);
    summary.insert("p75".to_string(), histogram.value_at_quantile(0.75) as f64);
    summary.insert("p90".to_string(), histogram.value_at_quantile(0.90) as f64);
    summary.insert("p95".to_string(), histogram.value_at_quantile(0.95) as f64);
    summary.insert("p99".to_string(), histogram.value_at_quantile(0.99) as f64);
    summary.insert(
        "p999".to_string(),
        histogram.value_at_quantile(0.999) as f64,
    );

    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_community_sizes_single_threaded() {
        let node_count = 100;
        // 4 communities: 0-24→0, 25-49→1, 50-74→2, 75-99→3
        let community_fn = |node_id: u64| (node_id / 25) as i64;

        let sizes = community_sizes(node_count, community_fn, 1);

        assert_eq!(sizes.get(0), 25);
        assert_eq!(sizes.get(1), 25);
        assert_eq!(sizes.get(2), 25);
        assert_eq!(sizes.get(3), 25);
        assert_eq!(sizes.get(4), EMPTY_COMMUNITY);
    }

    #[test]
    fn test_community_sizes_parallel() {
        let node_count = 100;
        let community_fn = |node_id: u64| (node_id / 25) as i64;

        let sizes = community_sizes(node_count, community_fn, 4);

        assert_eq!(sizes.get(0), 25);
        assert_eq!(sizes.get(1), 25);
        assert_eq!(sizes.get(2), 25);
        assert_eq!(sizes.get(3), 25);
    }

    #[test]
    fn test_community_count() {
        let node_count = 100;
        let community_fn = |node_id: u64| (node_id / 25) as i64;

        let count = community_count(node_count, community_fn, 1);
        assert_eq!(count, 4);

        let count_parallel = community_count(node_count, community_fn, 4);
        assert_eq!(count_parallel, 4);
    }

    #[test]
    fn test_community_count_and_histogram() {
        let node_count = 100;
        let community_fn = |node_id: u64| (node_id / 25) as i64;

        let result = community_count_and_histogram(node_count, community_fn, 1).unwrap();

        assert_eq!(result.component_count, 4);
        assert_eq!(result.histogram.min(), 25);
        assert_eq!(result.histogram.max(), 25);
    }

    #[test]
    fn test_community_stats_with_distribution() {
        let node_count = 100;
        let community_fn = |node_id: u64| (node_id / 25) as i64;

        let stats = community_stats(
            node_count,
            community_fn,
            1,
            StatisticsComputationInstructions::count_and_distribution(),
        );

        assert!(stats.success);
        assert_eq!(stats.component_count, 4);
        assert!(stats.histogram.is_some());

        let summary = stats.summary();
        assert_eq!(summary.get("min"), Some(&25.0));
        assert_eq!(summary.get("max"), Some(&25.0));
    }

    #[test]
    fn test_community_stats_count_only() {
        let node_count = 100;
        let community_fn = |node_id: u64| (node_id / 25) as i64;

        let stats = community_stats(
            node_count,
            community_fn,
            1,
            StatisticsComputationInstructions::count_only(),
        );

        assert!(stats.success);
        assert_eq!(stats.component_count, 4);
        assert!(stats.histogram.is_none());
        assert_eq!(stats.summary().len(), 0);
    }

    #[test]
    fn test_community_summary() {
        let mut histogram = Histogram::new(HISTOGRAM_PRECISION).unwrap();
        histogram.record(10).unwrap();
        histogram.record(20).unwrap();
        histogram.record(30).unwrap();

        let summary = community_summary(&histogram);

        assert_eq!(summary.get("min"), Some(&10.0));
        assert_eq!(summary.get("max"), Some(&30.0));
        assert!(summary.contains_key("mean"));
        assert!(summary.contains_key("p50"));
        assert!(summary.contains_key("p95"));
    }
}
