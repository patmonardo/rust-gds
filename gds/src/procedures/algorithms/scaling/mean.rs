use std::collections::HashMap;

use crate::procedures::algorithms::scaling::traits::{ScalarStats, Scaler, CLOSE_TO_ZERO};

pub struct MeanScaler {
    values: Vec<f64>,
    avg: f64,
    max_min_diff: f64,
    stats: ScalarStats,
}

impl MeanScaler {
    pub fn from_values(values: Vec<f64>) -> Self {
        let n = values.len() as f64;
        let sum: f64 = values.iter().copied().sum();
        let avg = if n > 0.0 { sum / n } else { 0.0 };
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for v in &values { if *v < min { min = *v; } if *v > max { max = *v; } }
        let diff = max - min;
        let mut stats = HashMap::new();
        stats.insert("min".to_string(), vec![min]);
        stats.insert("avg".to_string(), vec![avg]);
        stats.insert("max".to_string(), vec![max]);
        Self { values, avg, max_min_diff: diff, stats }
    }
}

impl Scaler for MeanScaler {
    fn scale_property(&self, node_id: usize) -> f64 {
        if self.max_min_diff.abs() < CLOSE_TO_ZERO { 0.0 } else { (self.values[node_id] - self.avg) / self.max_min_diff }
    }
    fn dimension(&self) -> usize { 1 }
    fn statistics(&self) -> &ScalarStats { &self.stats }
}


