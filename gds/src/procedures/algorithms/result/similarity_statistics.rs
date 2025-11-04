use std::time::Instant;

use super::histogram_utils::similarity_summary;

#[derive(Debug, Clone)]
pub struct SimilarityStats {
    pub histogram: Option<Vec<f64>>, // raw similarities for summary
    pub compute_milliseconds: u128,
    pub success: bool,
}

pub fn similarity_stats<I>(values_iter: I, should_compute_distribution: bool) -> SimilarityStats
where
    I: IntoIterator<Item = f64>,
{
    if !should_compute_distribution {
        return SimilarityStats { histogram: None, compute_milliseconds: 0, success: true };
    }
    let start = Instant::now();
    let values: Vec<f64> = values_iter.into_iter().collect();
    let elapsed = start.elapsed().as_millis();
    let success = similarity_summary(&values).len() > 0 || !values.is_empty();
    SimilarityStats { histogram: Some(values), compute_milliseconds: elapsed, success }
}


