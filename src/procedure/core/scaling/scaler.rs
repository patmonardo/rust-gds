//! Feature scaler implementations for ML pipelines
//!
//! **Translation Source**: `org.neo4j.gds.scaling.*` package
//!
//! Provides feature scaling transformations for node properties before ML algorithms.
//! All scalers follow a common pattern:
//! 1. Parallel statistics computation
//! 2. Reduction to global statistics
//! 3. Scaling transformation using those statistics
//!
//! ## Design Philosophy
//!
//! Instead of translating each Java scaler file separately, we use a **unified approach**:
//! - Single `PropertyStats` aggregator computes all statistics in one pass
//! - Each scaler extracts only the stats it needs
//! - Automatic parallel/serial execution based on concurrency
//! - Zero-value handling when range/std is too small
//!
//! This eliminates ~90% of Java boilerplate while preserving all functionality.

use rayon::prelude::*;
use std::collections::HashMap;

/// Threshold for treating values as zero
pub const CLOSE_TO_ZERO: f64 = 1e-15;

/// Statistics computed during scaler initialization
#[derive(Debug, Clone)]
pub struct ScalerStatistics {
    stats: HashMap<String, Vec<f64>>,
}

impl ScalerStatistics {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn with_stat(mut self, name: &str, value: f64) -> Self {
        self.stats.insert(name.to_string(), vec![value]);
        self
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        self.stats.get(name).and_then(|v| v.first().copied())
    }

    pub fn as_map(&self) -> &HashMap<String, Vec<f64>> {
        &self.stats
    }
}

impl Default for ScalerStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for feature scalers
pub trait Scaler: Send + Sync {
    /// Scale a single property value
    fn scale_property(&self, node_id: u64, property_fn: &dyn Fn(u64) -> f64) -> f64;

    /// Get scaling statistics (min, max, avg, std, etc.)
    fn statistics(&self) -> &ScalerStatistics;

    /// Scaler type name
    fn scaler_type(&self) -> &'static str;
}

/// Parallel statistics computer for property values
struct PropertyStats {
    min: f64,
    max: f64,
    sum: f64,
    squared_sum: f64,
    abs_sum: f64,
    abs_max: f64,
    count: u64,
}

impl PropertyStats {
    fn new() -> Self {
        Self {
            min: f64::MAX,
            max: f64::MIN,
            sum: 0.0,
            squared_sum: 0.0,
            abs_sum: 0.0,
            abs_max: 0.0,
            count: 0,
        }
    }

    fn update(&mut self, value: f64) {
        if value.is_nan() {
            return; // Skip NaN values
        }

        self.count += 1;
        self.sum += value;
        self.squared_sum += value * value;

        let abs_value = value.abs();
        self.abs_sum += abs_value;
        if abs_value > self.abs_max {
            self.abs_max = abs_value;
        }

        if value < self.min {
            self.min = value;
        }
        if value > self.max {
            self.max = value;
        }
    }

    fn merge(mut self, other: PropertyStats) -> Self {
        self.sum += other.sum;
        self.squared_sum += other.squared_sum;
        self.abs_sum += other.abs_sum;
        self.count += other.count;

        if other.min < self.min {
            self.min = other.min;
        }
        if other.max > self.max {
            self.max = other.max;
        }
        if other.abs_max > self.abs_max {
            self.abs_max = other.abs_max;
        }

        self
    }

    fn avg(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum / self.count as f64
        }
    }

    fn std(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            let avg = self.avg();
            let variance = (self.squared_sum - avg * self.sum) / self.count as f64;
            variance.sqrt()
        }
    }
}

/// Compute statistics in parallel over node properties
fn compute_stats<F>(node_count: u64, property_fn: &F, concurrency: usize) -> PropertyStats
where
    F: Fn(u64) -> f64 + Send + Sync,
{
    if concurrency == 1 {
        // Single-threaded path
        let mut stats = PropertyStats::new();
        for node_id in 0..node_count {
            stats.update(property_fn(node_id));
        }
        stats
    } else {
        // Parallel path
        let chunk_size = node_count.div_ceil(concurrency as u64);

        let local_stats: Vec<PropertyStats> = (0..concurrency)
            .into_par_iter()
            .map(|chunk_idx| {
                let start = chunk_idx as u64 * chunk_size;
                let end = ((chunk_idx + 1) as u64 * chunk_size).min(node_count);

                let mut local = PropertyStats::new();
                for node_id in start..end {
                    local.update(property_fn(node_id));
                }
                local
            })
            .collect();

        // Reduce
        local_stats
            .into_iter()
            .reduce(|a, b| a.merge(b))
            .unwrap_or_else(PropertyStats::new)
    }
}

//
// === Scaler Implementations ===
//

/// MinMax scaler - normalizes to [0, 1] range
///
/// **Translation**: `MinMax.java`
pub struct MinMaxScaler {
    min: f64,
    range: f64, // max - min
    statistics: ScalerStatistics,
}

impl MinMaxScaler {
    pub fn create<F>(node_count: u64, property_fn: &F, concurrency: usize) -> Box<dyn Scaler>
    where
        F: Fn(u64) -> f64 + Send + Sync,
    {
        let stats = compute_stats(node_count, property_fn, concurrency);
        let range = stats.max - stats.min;

        let statistics = ScalerStatistics::new()
            .with_stat("min", stats.min)
            .with_stat("max", stats.max);

        if range.abs() < CLOSE_TO_ZERO {
            Box::new(ZeroScaler { statistics })
        } else {
            Box::new(Self {
                min: stats.min,
                range,
                statistics,
            })
        }
    }
}

impl Scaler for MinMaxScaler {
    fn scale_property(&self, node_id: u64, property_fn: &dyn Fn(u64) -> f64) -> f64 {
        (property_fn(node_id) - self.min) / self.range
    }

    fn statistics(&self) -> &ScalerStatistics {
        &self.statistics
    }

    fn scaler_type(&self) -> &'static str {
        "minmax"
    }
}

/// StdScore scaler - z-score normalization (mean=0, std=1)
///
/// **Translation**: `StdScore.java`
pub struct StdScoreScaler {
    avg: f64,
    std: f64,
    statistics: ScalerStatistics,
}

impl StdScoreScaler {
    pub fn create<F>(node_count: u64, property_fn: &F, concurrency: usize) -> Box<dyn Scaler>
    where
        F: Fn(u64) -> f64 + Send + Sync,
    {
        let stats = compute_stats(node_count, property_fn, concurrency);
        let avg = stats.avg();
        let std = stats.std();

        let statistics = ScalerStatistics::new()
            .with_stat("avg", avg)
            .with_stat("std", std);

        if std < CLOSE_TO_ZERO {
            Box::new(ZeroScaler { statistics })
        } else {
            Box::new(Self {
                avg,
                std,
                statistics,
            })
        }
    }
}

impl Scaler for StdScoreScaler {
    fn scale_property(&self, node_id: u64, property_fn: &dyn Fn(u64) -> f64) -> f64 {
        (property_fn(node_id) - self.avg) / self.std
    }

    fn statistics(&self) -> &ScalerStatistics {
        &self.statistics
    }

    fn scaler_type(&self) -> &'static str {
        "stdscore"
    }
}

/// Mean scaler - center around mean and normalize by range
///
/// **Translation**: `Mean.java`
pub struct MeanScaler {
    avg: f64,
    range: f64,
    statistics: ScalerStatistics,
}

impl MeanScaler {
    pub fn create<F>(node_count: u64, property_fn: &F, concurrency: usize) -> Box<dyn Scaler>
    where
        F: Fn(u64) -> f64 + Send + Sync,
    {
        let stats = compute_stats(node_count, property_fn, concurrency);
        let avg = stats.avg();
        let range = stats.max - stats.min;

        let statistics = ScalerStatistics::new()
            .with_stat("min", stats.min)
            .with_stat("avg", avg)
            .with_stat("max", stats.max);

        if range.abs() < CLOSE_TO_ZERO {
            Box::new(ZeroScaler { statistics })
        } else {
            Box::new(Self {
                avg,
                range,
                statistics,
            })
        }
    }
}

impl Scaler for MeanScaler {
    fn scale_property(&self, node_id: u64, property_fn: &dyn Fn(u64) -> f64) -> f64 {
        (property_fn(node_id) - self.avg) / self.range
    }

    fn statistics(&self) -> &ScalerStatistics {
        &self.statistics
    }

    fn scaler_type(&self) -> &'static str {
        "mean"
    }
}

/// Max scaler - divide by absolute maximum
///
/// **Translation**: `Max.java`
pub struct MaxScaler {
    abs_max: f64,
    statistics: ScalerStatistics,
}

impl MaxScaler {
    pub fn create<F>(node_count: u64, property_fn: &F, concurrency: usize) -> Box<dyn Scaler>
    where
        F: Fn(u64) -> f64 + Send + Sync,
    {
        let stats = compute_stats(node_count, property_fn, concurrency);
        let abs_max = stats.abs_max;

        let statistics = ScalerStatistics::new().with_stat("absMax", abs_max);

        if abs_max < CLOSE_TO_ZERO {
            Box::new(ZeroScaler { statistics })
        } else {
            Box::new(Self {
                abs_max,
                statistics,
            })
        }
    }
}

impl Scaler for MaxScaler {
    fn scale_property(&self, node_id: u64, property_fn: &dyn Fn(u64) -> f64) -> f64 {
        property_fn(node_id) / self.abs_max
    }

    fn statistics(&self) -> &ScalerStatistics {
        &self.statistics
    }

    fn scaler_type(&self) -> &'static str {
        "max"
    }
}

/// Center scaler - subtract mean
///
/// **Translation**: `Center.java`
pub struct CenterScaler {
    avg: f64,
    statistics: ScalerStatistics,
}

impl CenterScaler {
    pub fn create<F>(node_count: u64, property_fn: &F, concurrency: usize) -> Box<dyn Scaler>
    where
        F: Fn(u64) -> f64 + Send + Sync,
    {
        let stats = compute_stats(node_count, property_fn, concurrency);
        let avg = stats.avg();

        let statistics = ScalerStatistics::new().with_stat("avg", avg);

        Box::new(Self { avg, statistics })
    }
}

impl Scaler for CenterScaler {
    fn scale_property(&self, node_id: u64, property_fn: &dyn Fn(u64) -> f64) -> f64 {
        property_fn(node_id) - self.avg
    }

    fn statistics(&self) -> &ScalerStatistics {
        &self.statistics
    }

    fn scaler_type(&self) -> &'static str {
        "center"
    }
}

/// Log scaler - logarithmic transformation
///
/// **Translation**: `LogScaler.java`
pub struct LogScaler {
    offset: f64,
    statistics: ScalerStatistics,
}

impl LogScaler {
    pub fn create(offset: f64) -> Box<dyn Scaler> {
        Box::new(Self {
            offset,
            statistics: ScalerStatistics::new(),
        })
    }
}

impl Scaler for LogScaler {
    fn scale_property(&self, node_id: u64, property_fn: &dyn Fn(u64) -> f64) -> f64 {
        (property_fn(node_id) + self.offset).ln()
    }

    fn statistics(&self) -> &ScalerStatistics {
        &self.statistics
    }

    fn scaler_type(&self) -> &'static str {
        "log"
    }
}

/// None scaler - pass-through (no transformation)
///
/// **Translation**: `NoneScaler.java`
#[derive(Default)]
pub struct NoneScaler {
    statistics: ScalerStatistics,
}

impl NoneScaler {
    pub fn create() -> Box<dyn Scaler> {
        Box::new(Self {
            statistics: ScalerStatistics::new(),
        })
    }
}


impl Scaler for NoneScaler {
    fn scale_property(&self, node_id: u64, property_fn: &dyn Fn(u64) -> f64) -> f64 {
        property_fn(node_id)
    }

    fn statistics(&self) -> &ScalerStatistics {
        &self.statistics
    }

    fn scaler_type(&self) -> &'static str {
        "none"
    }
}

/// Zero scaler - returns 0 for all values (used when range/std is too small)
pub(super) struct ZeroScaler {
    pub(super) statistics: ScalerStatistics,
}

impl Scaler for ZeroScaler {
    fn scale_property(&self, _node_id: u64, _property_fn: &dyn Fn(u64) -> f64) -> f64 {
        0.0
    }

    fn statistics(&self) -> &ScalerStatistics {
        &self.statistics
    }

    fn scaler_type(&self) -> &'static str {
        "zero"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minmax_scaler() {
        // Values: [0, 25, 50, 75, 100]
        let property_fn = |node_id: u64| (node_id * 25) as f64;

        let scaler = MinMaxScaler::create(5, &property_fn, 1);

        assert_eq!(scaler.scale_property(0, &property_fn), 0.0);
        assert_eq!(scaler.scale_property(2, &property_fn), 0.5);
        assert_eq!(scaler.scale_property(4, &property_fn), 1.0);

        assert_eq!(scaler.statistics().get("min"), Some(0.0));
        assert_eq!(scaler.statistics().get("max"), Some(100.0));
    }

    #[test]
    fn test_stdscore_scaler() {
        // Values: [1.0, 2.0, 3.0, 4.0, 5.0] -> mean=3.0, std=sqrt(2)
        let property_fn = |node_id: u64| (node_id + 1) as f64;

        let scaler = StdScoreScaler::create(5, &property_fn, 1);

        // Mean should be 3.0
        assert!((scaler.scale_property(2, &property_fn) - 0.0).abs() < 0.01);

        assert_eq!(scaler.statistics().get("avg"), Some(3.0));
        assert!(scaler.statistics().get("std").is_some());
    }

    #[test]
    fn test_mean_scaler() {
        let property_fn = |node_id: u64| (node_id * 10) as f64;

        let scaler = MeanScaler::create(5, &property_fn, 1);

        // Should center around mean and normalize by range
        let scaled = scaler.scale_property(2, &property_fn);
        assert!(scaled >= -1.0 && scaled <= 1.0);
    }

    #[test]
    fn test_max_scaler() {
        let property_fn = |node_id: u64| (node_id * 10) as f64;

        let scaler = MaxScaler::create(5, &property_fn, 1);

        // Max is 40, so scale_property(4) should be 1.0
        assert_eq!(scaler.scale_property(4, &property_fn), 1.0);
        assert_eq!(scaler.scale_property(2, &property_fn), 0.5);
    }

    #[test]
    fn test_center_scaler() {
        let property_fn = |node_id: u64| (node_id + 1) as f64;

        let scaler = CenterScaler::create(5, &property_fn, 1);

        // Mean is 3.0, so center(3) should be 0.0
        assert_eq!(scaler.scale_property(2, &property_fn), 0.0);
    }

    #[test]
    fn test_log_scaler() {
        let property_fn = |node_id: u64| (node_id + 1) as f64;

        let scaler = LogScaler::create(0.0);

        assert!((scaler.scale_property(0, &property_fn) - 0.0).abs() < 0.01); // ln(1) = 0
        assert!((scaler.scale_property(1, &property_fn) - 0.693).abs() < 0.01); // ln(2) ≈ 0.693
    }

    #[test]
    fn test_none_scaler() {
        let property_fn = |node_id: u64| (node_id * 42) as f64;

        let scaler = NoneScaler::create();

        assert_eq!(scaler.scale_property(0, &property_fn), 0.0);
        assert_eq!(scaler.scale_property(1, &property_fn), 42.0);
        assert_eq!(scaler.scale_property(5, &property_fn), 210.0);
    }

    #[test]
    fn test_parallel_stats_computation() {
        let property_fn = |node_id: u64| (node_id * 10) as f64;

        let scaler_single = MinMaxScaler::create(100, &property_fn, 1);
        let scaler_parallel = MinMaxScaler::create(100, &property_fn, 4);

        // Should produce same results
        assert_eq!(
            scaler_single.statistics().get("min"),
            scaler_parallel.statistics().get("min")
        );
        assert_eq!(
            scaler_single.statistics().get("max"),
            scaler_parallel.statistics().get("max")
        );
    }

    #[test]
    fn test_zero_range_becomes_zero_scaler() {
        // All values are 42.0
        let property_fn = |_node_id: u64| 42.0;

        let scaler = MinMaxScaler::create(10, &property_fn, 1);

        // Should return 0 for all values (zero range)
        assert_eq!(scaler.scale_property(0, &property_fn), 0.0);
        assert_eq!(scaler.scale_property(5, &property_fn), 0.0);
    }
}
