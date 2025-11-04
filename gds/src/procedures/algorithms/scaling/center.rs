use std::collections::HashMap;

use crate::procedures::algorithms::scaling::traits::{ScalarStats, Scaler};

pub struct CenterScaler {
    values: Vec<f64>,
    avg: f64,
    stats: ScalarStats,
}

impl CenterScaler {
    pub fn from_values(values: Vec<f64>) -> Self {
        let n = values.len() as f64;
        let sum: f64 = values.iter().copied().sum();
        let avg = if n > 0.0 { sum / n } else { 0.0 };
        let mut stats = HashMap::new();
        stats.insert("avg".to_string(), vec![avg]);
        Self { values, avg, stats }
    }
}

impl Scaler for CenterScaler {
    fn scale_property(&self, node_id: usize) -> f64 { self.values[node_id] - self.avg }
    fn dimension(&self) -> usize { 1 }
    fn statistics(&self) -> &ScalarStats { &self.stats }
}


