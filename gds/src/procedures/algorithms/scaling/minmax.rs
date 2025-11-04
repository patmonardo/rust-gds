use std::collections::HashMap;

use crate::procedures::algorithms::scaling::traits::{ScalarStats, Scaler, CLOSE_TO_ZERO};

pub struct MinMaxScaler {
    values: Vec<f64>,
    min: f64,
    max_min_diff: f64,
    stats: ScalarStats,
}

impl MinMaxScaler {
    pub fn from_values(values: Vec<f64>) -> Self {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for v in &values { if *v < min { min = *v; } if *v > max { max = *v; } }
        let diff = max - min;
        let mut stats = HashMap::new();
        stats.insert("min".to_string(), vec![min]);
        stats.insert("max".to_string(), vec![max]);
        Self { values, min, max_min_diff: diff, stats }
    }
}

impl Scaler for MinMaxScaler {
    fn scale_property(&self, node_id: usize) -> f64 {
        if self.max_min_diff.abs() < CLOSE_TO_ZERO { 0.0 } else { (self.values[node_id] - self.min) / self.max_min_diff }
    }
    fn dimension(&self) -> usize { 1 }
    fn statistics(&self) -> &ScalarStats { &self.stats }
}


