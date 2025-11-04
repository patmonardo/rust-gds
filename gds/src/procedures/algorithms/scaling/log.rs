use std::collections::HashMap;

use crate::procedures::algorithms::scaling::traits::{ScalarStats, Scaler};

pub struct LogScaler {
    values: Vec<f64>,
    offset: f64,
    stats: ScalarStats,
}

impl LogScaler {
    pub fn from_values(values: Vec<f64>, offset: f64) -> Self {
        let stats = HashMap::new();
        Self { values, offset, stats }
    }
}

impl Scaler for LogScaler {
    fn scale_property(&self, node_id: usize) -> f64 { (self.values[node_id] + self.offset).ln() }
    fn dimension(&self) -> usize { 1 }
    fn statistics(&self) -> &ScalarStats { &self.stats }
}


