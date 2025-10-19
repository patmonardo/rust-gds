#[derive(Debug, Clone)]
pub struct PredictedLink {
    source_id: i64,
    target_id: i64,
    probability: f64,
}

impl PredictedLink {
    pub fn new(source_id: i64, target_id: i64, probability: f64) -> Self {
        Self {
            source_id,
            target_id,
            probability,
        }
    }

    pub fn source_id(&self) -> i64 {
        self.source_id
    }

    pub fn target_id(&self) -> i64 {
        self.target_id
    }

    pub fn probability(&self) -> f64 {
        self.probability
    }
}
