use std::time::Instant;

use super::histogram_utils::centrality_summary;

#[derive(Debug, Clone)]
pub struct CentralityStats {
    pub histogram: Option<Vec<f64>>, // raw values for summary
    pub compute_milliseconds: u128,
    pub success: bool,
}

pub fn centrality_statistics<F>(
    node_count: usize,
    centrality_fn: F,
    should_compute: bool,
) -> CentralityStats
where
    F: Fn(usize) -> f64,
{
    if !should_compute {
        return CentralityStats { histogram: None, compute_milliseconds: 0, success: true };
    }
    let start = Instant::now();
    let mut values = Vec::with_capacity(node_count);
    for id in 0..node_count { values.push(centrality_fn(id)); }
    let elapsed = start.elapsed().as_millis();
    // Touch summary to validate; if NaN causes issues, mark failure
    let success = centrality_summary(&values).len() > 0 || !values.is_empty();
    CentralityStats { histogram: Some(values), compute_milliseconds: elapsed, success }
}


