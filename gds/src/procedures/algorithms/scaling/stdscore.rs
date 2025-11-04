use std::collections::HashMap;

use crate::procedures::algorithms::scaling::traits::{ScalarStats, Scaler, CLOSE_TO_ZERO};

pub struct StdScoreScaler {
    values: Vec<f64>,
    avg: f64,
    std: f64,
    stats: ScalarStats,
}

impl StdScoreScaler {
    pub fn from_values(values: Vec<f64>) -> Self {
        let n = values.len() as f64;
        let sum: f64 = values.iter().copied().sum();
        let avg = if n > 0.0 { sum / n } else { 0.0 };
        let mut squared_sum = 0.0;
        for v in &values { squared_sum += *v * *v; }
        let variance = if n > 0.0 { (squared_sum - avg * sum) / n } else { 0.0 };
        let std = variance.sqrt();
        let mut stats = HashMap::new();
        stats.insert("avg".to_string(), vec![avg]);
        stats.insert("std".to_string(), vec![std]);
        Self { values, avg, std, stats }
    }
}

impl Scaler for StdScoreScaler {
    fn scale_property(&self, node_id: usize) -> f64 {
        if self.std < CLOSE_TO_ZERO { 0.0 } else { (self.values[node_id] - self.avg) / self.std }
    }
    fn dimension(&self) -> usize { 1 }
    fn statistics(&self) -> &ScalarStats { &self.stats }
}


