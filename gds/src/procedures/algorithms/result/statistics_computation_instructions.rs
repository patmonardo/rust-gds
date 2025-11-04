pub trait StatisticsComputationInstructions {
    fn compute_count_only(&self) -> bool;
    fn compute_count_and_distribution(&self) -> bool;
}


