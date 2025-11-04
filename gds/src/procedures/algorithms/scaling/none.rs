use std::collections::HashMap;

use crate::procedures::algorithms::scaling::traits::{ScalarStats, Scaler};

pub struct NoneScaler {
    stats: ScalarStats,
}

impl NoneScaler {
    pub fn new() -> Self {
        Self { stats: HashMap::new() }
    }
}

impl Scaler for NoneScaler {
    fn scale_property(&self, _node_id: usize) -> f64 {
        0.0
    }

    fn dimension(&self) -> usize { 1 }

    fn statistics(&self) -> &ScalarStats { &self.stats }
}


