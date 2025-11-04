use std::collections::HashMap;
use std::time::Instant;

use super::histogram_utils::community_summary;
use super::statistics_computation_instructions::StatisticsComputationInstructions;

pub fn community_sizes<F>(node_count: usize, community_fn: F) -> HashMap<u64, u64>
where
    F: Fn(usize) -> u64,
{
    let mut sizes: HashMap<u64, u64> = HashMap::new();
    for id in 0..node_count {
        let cid = community_fn(id);
        *sizes.entry(cid).or_insert(0) += 1;
    }
    sizes
}

pub fn community_count_from_sizes(sizes: &HashMap<u64, u64>) -> u64 {
    sizes.values().filter(|&&sz| sz > 0).count() as u64
}

#[derive(Debug, Clone)]
pub struct CommunityStats {
    pub component_count: u64,
    pub histogram: Option<Vec<u64>>, // distribution of community sizes
    pub compute_milliseconds: u128,
    pub success: bool,
}

pub fn community_stats<F, I>(
    node_count: usize,
    community_fn: F,
    instructions: &I,
) -> CommunityStats
where
    F: Fn(usize) -> u64,
    I: StatisticsComputationInstructions,
{
    let start = Instant::now();
    let sizes_map = community_sizes(node_count, community_fn);
    let component_count = community_count_from_sizes(&sizes_map);
    let histogram = if instructions.compute_count_and_distribution() {
        let mut sizes: Vec<u64> = sizes_map.values().copied().collect();
        sizes.sort_unstable();
        // Validate by producing a summary
        let _ = community_summary(&sizes);
        Some(sizes)
    } else { None };
    let elapsed = start.elapsed().as_millis();
    CommunityStats { component_count, histogram, compute_milliseconds: elapsed, success: true }
}


