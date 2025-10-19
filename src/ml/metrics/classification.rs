use crate::collections::HugeLongArray;
use crate::ml::metrics::{Metric, MetricComparator};

pub const EPSILON: f64 = 1e-8;

pub trait ClassificationMetric: Metric {
    fn compute(&self, targets: &HugeLongArray, predictions: &HugeLongArray) -> f64;
}

#[derive(Debug, Clone)]
pub struct Accuracy {
    original_target: i64,
    internal_target: i64,
}

impl Accuracy {
    pub const NAME: &'static str = "ACCURACY";

    pub fn new(original_target: i64, internal_target: i64) -> Self {
        Self {
            original_target,
            internal_target,
        }
    }
}

impl Metric for Accuracy {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Natural
    }
}

impl ClassificationMetric for Accuracy {
    fn compute(&self, targets: &HugeLongArray, predictions: &HugeLongArray) -> f64 {
        let mut correct = 0;
        let mut total = 0;
        let mut _total_target_class = 0;
        let mut _true_positives = 0;

        for i in 0..targets.size() {
            let target = targets.get(i);
            if target == self.internal_target {
                _total_target_class += 1;
                if predictions.get(i) == target {
                    _true_positives += 1;
                }
            }
            if predictions.get(i) == target {
                correct += 1;
            }
            total += 1;
        }

        if total == 0 {
            return 0.0;
        }
        correct as f64 / total as f64
    }
}

#[derive(Debug, Clone)]
pub struct GlobalAccuracy;

impl Default for GlobalAccuracy {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalAccuracy {
    pub const NAME: &'static str = "ACCURACY";

    pub fn new() -> Self {
        Self
    }
}

impl Metric for GlobalAccuracy {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Natural
    }
}

impl ClassificationMetric for GlobalAccuracy {
    fn compute(&self, targets: &HugeLongArray, predictions: &HugeLongArray) -> f64 {
        let mut accurate_predictions = 0;
        let mut total = 0;

        for i in 0..targets.size() {
            if targets.get(i) == predictions.get(i) {
                accurate_predictions += 1;
            }
            total += 1;
        }

        if total == 0 {
            return 0.0;
        }
        accurate_predictions as f64 / total as f64
    }
}

#[derive(Debug, Clone)]
pub struct F1Score {
    original_target: i64,
    internal_target: i64,
}

impl F1Score {
    pub const NAME: &'static str = "F1";

    pub fn new(original_target: i64, internal_target: i64) -> Self {
        Self {
            original_target,
            internal_target,
        }
    }
}

impl Metric for F1Score {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Natural
    }
}

impl ClassificationMetric for F1Score {
    fn compute(&self, targets: &HugeLongArray, predictions: &HugeLongArray) -> f64 {
        let mut true_positives = 0;
        let mut false_positives = 0;
        let mut false_negatives = 0;

        for i in 0..targets.size() {
            let target = targets.get(i);
            let prediction = predictions.get(i);

            if target == self.internal_target {
                if prediction == target {
                    true_positives += 1;
                } else {
                    false_negatives += 1;
                }
            } else if prediction == self.internal_target {
                false_positives += 1;
            }
        }

        let precision = if true_positives + false_positives == 0 {
            0.0
        } else {
            true_positives as f64 / (true_positives + false_positives) as f64
        };

        let recall = if true_positives + false_negatives == 0 {
            0.0
        } else {
            true_positives as f64 / (true_positives + false_negatives) as f64
        };

        if precision + recall < EPSILON {
            0.0
        } else {
            2.0 * (precision * recall) / (precision + recall)
        }
    }
}
