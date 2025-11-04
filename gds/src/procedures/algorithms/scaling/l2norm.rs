use std::collections::HashMap;

use crate::procedures::algorithms::scaling::traits::{ScalarStats, Scaler, CLOSE_TO_ZERO};

pub struct L2NormScaler {
    values: Vec<f64>,
    len: f64,
    stats: ScalarStats,
}

impl L2NormScaler {
    pub fn from_values(values: Vec<f64>) -> Self {
        let mut squared_sum = 0.0;
        for v in &values { squared_sum += v * v; }
        let len = squared_sum.sqrt();
        let stats = HashMap::new();
        Self { values, len, stats }
    }
}

impl Scaler for L2NormScaler {
    fn scale_property(&self, node_id: usize) -> f64 {
        if self.len < CLOSE_TO_ZERO { 0.0 } else { self.values[node_id] / self.len }
    }

    fn dimension(&self) -> usize { 1 }

    fn statistics(&self) -> &ScalarStats { &self.stats }
}


