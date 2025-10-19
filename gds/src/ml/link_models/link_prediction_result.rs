use super::predicted_link::PredictedLink;
use std::collections::HashMap;

pub trait LinkPredictionResult: Send + Sync {
    /// Stream the predicted links
    fn iter(&self) -> Box<dyn Iterator<Item = PredictedLink> + '_>;

    /// Get sampling statistics
    fn sampling_stats(&self) -> HashMap<String, serde_json::Value>;
}
