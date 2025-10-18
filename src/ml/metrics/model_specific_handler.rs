use super::Metric;

/// Handler for model-specific metrics during training.
/// Filters metrics to only those marked as model-specific and delegates to a consumer.
///
/// This is a 1:1 translation of ModelSpecificMetricsHandler from Java GDS.
pub struct ModelSpecificMetricsHandler {
    metrics: Vec<String>,
    metric_consumer: Box<dyn Fn(&str, f64) + Send + Sync>,
}

impl ModelSpecificMetricsHandler {
    /// Creates a no-op handler that does nothing
    pub fn noop() -> Self {
        Self {
            metrics: Vec::new(),
            metric_consumer: Box::new(|_, _| {}),
        }
    }

    /// Creates a handler from a list of metrics and a consumer function.
    /// Only metrics marked as model-specific will be included.
    pub fn new<F>(metrics: &[Box<dyn Metric>], consumer: F) -> Self
    where
        F: Fn(&str, f64) + Send + Sync + 'static,
    {
        let filtered_metrics: Vec<String> = metrics
            .iter()
            .filter(|m| m.is_model_specific())
            .map(|m| m.name().to_string())
            .collect();

        Self {
            metrics: filtered_metrics,
            metric_consumer: Box::new(consumer),
        }
    }

    // TODO: Implement for_stats_builder() when we have proper Arc/Mutex patterns
    // The Java version uses BiConsumer which allows mutable access to stats_builder
    // We need a different pattern in Rust (Arc<Mutex<ModelStatsBuilder>> or similar)

    /// Checks if a metric is requested (should be handled)
    pub fn is_requested(&self, metric: &dyn Metric) -> bool {
        self.metrics.contains(&metric.name().to_string())
    }

    /// Handles a metric value, passing it to the consumer
    pub fn handle(&self, metric: &dyn Metric, score: f64) {
        if !self.is_requested(metric) {
            panic!("Should not handle a metric which is not requested");
        }
        (self.metric_consumer)(metric.name(), score);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_handler() {
        let handler = ModelSpecificMetricsHandler::noop();
        assert_eq!(handler.metrics.len(), 0);
    }
}
