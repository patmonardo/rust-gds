use super::{link_prediction_result::LinkPredictionResult, predicted_link::PredictedLink};
use crate::core::utils::queue::BoundedLongLongPriorityQueue;
use std::{collections::HashMap, sync::Arc};

pub struct ExhaustiveLinkPredictionResult {
    prediction_queue: Arc<BoundedLongLongPriorityQueue>,
    links_considered: usize,
}

impl ExhaustiveLinkPredictionResult {
    pub fn new(
        best_predictions: Arc<BoundedLongLongPriorityQueue>,
        links_considered: usize,
    ) -> Self {
        Self {
            prediction_queue: best_predictions,
            links_considered,
        }
    }

    pub fn size(&self) -> usize {
        self.prediction_queue.size()
    }

    pub fn for_each<F>(&self, consumer: F)
    where
        F: FnMut(i64, i64, f64),
    {
        self.prediction_queue.for_each(consumer);
    }
}

impl LinkPredictionResult for ExhaustiveLinkPredictionResult {
    fn iter(&self) -> Box<dyn Iterator<Item = PredictedLink> + '_> {
        let mut links = Vec::with_capacity(self.size());
        self.prediction_queue
            .for_each(|e1, e2, p| links.push(PredictedLink::new(e1, e2, p)));
        Box::new(links.into_iter())
    }

    fn sampling_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();
        stats.insert(
            "strategy".to_string(),
            serde_json::Value::String("exhaustive".to_string()),
        );
        stats.insert(
            "linksConsidered".to_string(),
            serde_json::Value::Number(self.links_considered.into()),
        );
        stats
    }
}
