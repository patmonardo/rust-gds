//! Collections Metrics Extensions
//!
//! Provides metrics and telemetry capabilities as Collections Extensions for the Collections First approach.
//! This enables performance tracking and OpenTelemetry integration for any Collections implementation.

use crate::collections::traits::Collections;
use crate::config::Extension;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::time::{Duration, Instant};

/// Metrics extension trait for Collections
pub trait MetricsSupport<T> {
    /// Enable metrics collection
    fn enable_metrics(&mut self, config: MetricsConfig) -> Result<(), MetricsError>;
    
    /// Disable metrics collection
    fn disable_metrics(&mut self);
    
    /// Check if metrics are enabled
    fn is_metrics_enabled(&self) -> bool;
    
    /// Get performance metrics
    fn get_metrics(&self) -> Option<PerformanceMetrics>;
    
    /// Get operation counts
    fn get_operation_counts(&self) -> HashMap<String, u64>;
    
    /// Get timing statistics
    fn get_timing_stats(&self) -> HashMap<String, Duration>;
    
    /// Reset metrics
    fn reset_metrics(&mut self);
    
    /// Export metrics to OpenTelemetry
    fn export_metrics(&self) -> Result<(), MetricsError>;
}

/// Metrics configuration
#[derive(Debug, Clone)]
pub struct MetricsConfig {
    pub enable_timing: bool,
    pub enable_counting: bool,
    pub enable_memory_tracking: bool,
    pub enable_opentelemetry: bool,
    pub sample_rate: f64,
    pub export_interval: Duration,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enable_timing: true,
            enable_counting: true,
            enable_memory_tracking: true,
            enable_opentelemetry: false,
            sample_rate: 1.0,
            export_interval: Duration::from_secs(60),
        }
    }
}

/// Performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub operation_counts: HashMap<String, u64>,
    pub timing_stats: HashMap<String, Duration>,
    pub memory_usage: usize,
    pub peak_memory: usize,
    pub last_updated: Instant,
}

/// Metrics-enabled collection wrapper
pub struct MetricsCollection<T, C> 
where
    C: Collections<T>,
{
    inner: C,
    metrics_config: Option<MetricsConfig>,
    is_metrics_enabled: bool,
    metrics: Option<PerformanceMetrics>,
    operation_counts: HashMap<String, u64>,
    timing_stats: HashMap<String, Duration>,
    _phantom: PhantomData<T>,
}

impl<T, C> MetricsCollection<T, C> 
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    pub fn new(inner: C) -> Self {
        Self {
            inner,
            metrics_config: None,
            is_metrics_enabled: false,
            metrics: None,
            operation_counts: HashMap::new(),
            timing_stats: HashMap::new(),
            _phantom: PhantomData,
        }
    }
    
    pub fn with_metrics_config(inner: C, config: MetricsConfig) -> Self {
        Self {
            inner,
            metrics_config: Some(config),
            is_metrics_enabled: false,
            metrics: None,
            operation_counts: HashMap::new(),
            timing_stats: HashMap::new(),
            _phantom: PhantomData,
        }
    }
}

impl<T, C> Collections<T> for MetricsCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn get(&self, index: usize) -> Option<T> {
        self.inner.get(index)
    }
    
    fn set(&mut self, index: usize, value: T) {
        self.inner.set(index, value);
    }
    
    fn len(&self) -> usize {
        self.inner.len()
    }
    
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    
    fn sum(&self) -> Option<T> where T: std::iter::Sum {
        self.inner.sum()
    }
    
    fn min(&self) -> Option<T> where T: Ord {
        self.inner.min()
    }
    
    fn max(&self) -> Option<T> where T: Ord {
        self.inner.max()
    }
    
    fn mean(&self) -> Option<f64> {
        self.inner.mean()
    }
    
    fn std_dev(&self) -> Option<f64> {
        self.inner.std_dev()
    }
    
    fn variance(&self) -> Option<f64> {
        self.inner.variance()
    }
    
    fn median(&self) -> Option<T> where T: Ord {
        self.inner.median()
    }
    
    fn percentile(&self, p: f64) -> Option<T> where T: Ord {
        self.inner.percentile(p)
    }
    
    fn binary_search(&self, key: &T) -> Result<usize, usize> where T: Ord {
        self.inner.binary_search(key)
    }
    
    fn sort(&mut self) where T: Ord {
        self.inner.sort();
    }
    
    fn to_vec(self) -> Vec<T> {
        self.inner.to_vec()
    }
    
    fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }
    
    fn is_null(&self, index: usize) -> bool {
        self.inner.is_null(index)
    }
    
    fn null_count(&self) -> usize {
        self.inner.null_count()
    }
    
    fn default_value(&self) -> T {
        self.inner.default_value()
    }
    
    fn backend(&self) -> crate::config::CollectionsBackend {
        self.inner.backend()
    }
    
    fn features(&self) -> &[crate::config::Extension] {
        &[Extension::Metrics]
    }
    
    fn extensions(&self) -> &[crate::config::Extension] {
        &[Extension::Metrics]
    }
    
    fn value_type(&self) -> crate::types::ValueType {
        self.inner.value_type()
    }
    
    fn with_capacity(_capacity: usize) -> Self where Self: Sized {
        // Implementation for metrics collections
        todo!("Implement with_capacity for MetricsCollection")
    }
    
    fn with_defaults(_count: usize, _default_value: T) -> Self where Self: Sized {
        // Implementation for metrics collections
        todo!("Implement with_defaults for MetricsCollection")
    }
}

impl<T, C> MetricsSupport<T> for MetricsCollection<T, C>
where
    C: Collections<T>,
    T: Clone + Send + Sync,
{
    fn enable_metrics(&mut self, config: MetricsConfig) -> Result<(), MetricsError> {
        self.metrics_config = Some(config);
        self.is_metrics_enabled = true;
        
        // Initialize metrics
        self.metrics = Some(PerformanceMetrics {
            operation_counts: HashMap::new(),
            timing_stats: HashMap::new(),
            memory_usage: 0,
            peak_memory: 0,
            last_updated: Instant::now(),
        });
        
        Ok(())
    }
    
    fn disable_metrics(&mut self) {
        self.metrics_config = None;
        self.is_metrics_enabled = false;
        self.metrics = None;
    }
    
    fn is_metrics_enabled(&self) -> bool {
        self.is_metrics_enabled
    }
    
    fn get_metrics(&self) -> Option<PerformanceMetrics> {
        self.metrics.clone()
    }
    
    fn get_operation_counts(&self) -> HashMap<String, u64> {
        self.operation_counts.clone()
    }
    
    fn get_timing_stats(&self) -> HashMap<String, Duration> {
        self.timing_stats.clone()
    }
    
    fn reset_metrics(&mut self) {
        self.operation_counts.clear();
        self.timing_stats.clear();
        if let Some(ref mut metrics) = self.metrics {
            metrics.operation_counts.clear();
            metrics.timing_stats.clear();
            metrics.memory_usage = 0;
            metrics.peak_memory = 0;
            metrics.last_updated = Instant::now();
        }
    }
    
    fn export_metrics(&self) -> Result<(), MetricsError> {
        if !self.is_metrics_enabled {
            return Err(MetricsError::MetricsNotEnabled);
        }
        
        // Export to OpenTelemetry (placeholder implementation)
        if let Some(config) = &self.metrics_config {
            if config.enable_opentelemetry {
                // TODO: Implement actual OpenTelemetry export
                println!("Exporting metrics to OpenTelemetry...");
            }
        }
        
        Ok(())
    }
}

/// Metrics error types
#[derive(Debug, thiserror::Error)]
pub enum MetricsError {
    #[error("Metrics initialization failed: {0}")]
    InitFailed(String),
    #[error("Metrics collection failed: {0}")]
    CollectionFailed(String),
    #[error("Metrics export failed: {0}")]
    ExportFailed(String),
    #[error("Metrics not enabled")]
    MetricsNotEnabled,
    #[error("OpenTelemetry integration failed: {0}")]
    OpenTelemetryFailed(String),
}

/// Metrics utilities
pub struct MetricsUtils;

impl MetricsUtils {
    /// Create default metrics configuration
    pub fn default_config() -> MetricsConfig {
        MetricsConfig::default()
    }
    
    /// Create metrics configuration for production
    pub fn production_config() -> MetricsConfig {
        MetricsConfig {
            enable_timing: true,
            enable_counting: true,
            enable_memory_tracking: true,
            enable_opentelemetry: true,
            sample_rate: 0.1, // 10% sampling for production
            export_interval: Duration::from_secs(30),
        }
    }
    
    /// Create metrics configuration for development
    pub fn development_config() -> MetricsConfig {
        MetricsConfig {
            enable_timing: true,
            enable_counting: true,
            enable_memory_tracking: true,
            enable_opentelemetry: false,
            sample_rate: 1.0, // 100% sampling for development
            export_interval: Duration::from_secs(10),
        }
    }
    
    /// Format metrics for human reading
    pub fn format_metrics(metrics: &PerformanceMetrics) -> String {
        let mut output = String::new();
        output.push_str("=== Collections Performance Metrics ===\n");
        
        output.push_str("Operation Counts:\n");
        for (operation, count) in &metrics.operation_counts {
            output.push_str(&format!("  {}: {}\n", operation, count));
        }
        
        output.push_str("Timing Statistics:\n");
        for (operation, duration) in &metrics.timing_stats {
            output.push_str(&format!("  {}: {:?}\n", operation, duration));
        }
        
        output.push_str(&format!("Memory Usage: {} bytes\n", metrics.memory_usage));
        output.push_str(&format!("Peak Memory: {} bytes\n", metrics.peak_memory));
        
        output
    }
}
