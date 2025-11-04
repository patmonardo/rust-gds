use std::collections::HashMap;

use crate::procedures::algorithms::scaling::traits::{ScalarStats, Scaler, CLOSE_TO_ZERO};

pub struct L1NormScaler {
    values: Vec<f64>,
    l1: f64,
    stats: ScalarStats,
}

impl L1NormScaler {
    pub fn from_values(values: Vec<f64>) -> Self {
        let mut l1 = 0.0;
        for v in &values { l1 += v.abs(); }
        let stats = HashMap::new();
        Self { values, l1, stats }
    }
}

impl Scaler for L1NormScaler {
    fn scale_property(&self, node_id: usize) -> f64 {
        if self.l1 < CLOSE_TO_ZERO { 0.0 } else { self.values[node_id] / self.l1 }
    }

    fn dimension(&self) -> usize { 1 }

    fn statistics(&self) -> &ScalarStats { &self.stats }
}


