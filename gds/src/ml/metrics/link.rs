use crate::ml::metrics::{Metric, MetricComparator};
use derive_builder::Builder;

#[derive(Debug, Clone)]
pub struct SignedProbabilities {
    probabilities: Vec<f64>,
    positive_count: usize,
    negative_count: usize,
}

impl SignedProbabilities {
    pub const ALMOST_ZERO: f64 = 1e-100;

    pub fn new(mut probabilities: Vec<f64>, positive_count: usize, negative_count: usize) -> Self {
        probabilities.sort_by(|a, b| {
            a.abs()
                .partial_cmp(&b.abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Self {
            probabilities,
            positive_count,
            negative_count,
        }
    }

    pub fn positive_count(&self) -> usize {
        self.positive_count
    }

    pub fn negative_count(&self) -> usize {
        self.negative_count
    }

    pub fn probabilities(&self) -> &[f64] {
        &self.probabilities
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LinkMetric {
    AUCPR,
}

impl LinkMetric {
    pub fn compute(
        &self,
        signed_probabilities: &SignedProbabilities,
        negative_class_weight: f64,
    ) -> f64 {
        match self {
            LinkMetric::AUCPR => self.compute_aucpr(signed_probabilities, negative_class_weight),
        }
    }

    fn compute_aucpr(
        &self,
        signed_probabilities: &SignedProbabilities,
        negative_class_weight: f64,
    ) -> f64 {
        let positive_count = signed_probabilities.positive_count();
        let negative_count = signed_probabilities.negative_count();

        if positive_count == 0 {
            return 0.0;
        }

        let mut true_positives = 0.0;
        let mut false_positives = 0.0;
        let mut area = 0.0;
        let mut prev_recall = 0.0;
        let mut prev_precision = 1.0;

        for &prob in signed_probabilities.probabilities() {
            if prob.abs() < SignedProbabilities::ALMOST_ZERO {
                continue;
            }

            if prob > 0.0 {
                true_positives += 1.0;
            } else {
                false_positives += negative_class_weight;
            }

            let recall = true_positives / positive_count as f64;
            let precision = if true_positives + false_positives < SignedProbabilities::ALMOST_ZERO {
                1.0
            } else {
                true_positives / (true_positives + false_positives)
            };

            area += (recall - prev_recall) * (precision + prev_precision) / 2.0;
            prev_recall = recall;
            prev_precision = precision;
        }

        area
    }
}

impl Metric for LinkMetric {
    fn name(&self) -> &str {
        match self {
            LinkMetric::AUCPR => "AUCPR",
        }
    }

    fn comparator(&self) -> MetricComparator {
        MetricComparator::Natural
    }
}

pub struct SignedProbabilitiesBuilder {
    probabilities: Vec<f64>,
    positive_count: usize,
    negative_count: usize,
    concurrency: usize,
}

impl SignedProbabilitiesBuilder {
    pub fn new(concurrency: usize) -> Self {
        Self {
            probabilities: Vec::new(),
            positive_count: 0,
            negative_count: 0,
            concurrency,
        }
    }

    pub fn add_probability(&mut self, prob: f64, is_positive: bool) {
        self.probabilities
            .push(if is_positive { prob } else { -prob });
        if is_positive {
            self.positive_count += 1;
        } else {
            self.negative_count += 1;
        }
    }

    pub fn build(self) -> SignedProbabilities {
        SignedProbabilities::new(self.probabilities, self.positive_count, self.negative_count)
    }
}
