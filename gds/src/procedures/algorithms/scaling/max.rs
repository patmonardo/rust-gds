use std::collections::HashMap;

use crate::procedures::algorithms::scaling::traits::{ScalarStats, Scaler, CLOSE_TO_ZERO};

pub struct MaxScaler {
    values: Vec<f64>,
    abs_max: f64,
    stats: ScalarStats,
}

impl MaxScaler {
    pub fn from_values(values: Vec<f64>) -> Self {
        let mut abs_max = 0.0;
        for v in &values { let a = v.abs(); if a > abs_max { abs_max = a; } }
        let mut stats = HashMap::new();
        stats.insert("absMax".to_string(), vec![abs_max]);
        Self { values, abs_max, stats }
    }
}

impl Scaler for MaxScaler {
    fn scale_property(&self, node_id: usize) -> f64 {
        if self.abs_max < CLOSE_TO_ZERO { 0.0 } else { self.values[node_id] / self.abs_max }
    }
    fn dimension(&self) -> usize { 1 }
    fn statistics(&self) -> &ScalarStats { &self.stats }
}


