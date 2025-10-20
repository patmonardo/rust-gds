/// Algorithm Processing Timings - tracks execution phases
/// This is the finalised timings for one algorithm processing.
/// Going for a good old struct here, matching the Java implementation.
#[derive(Debug, Clone)]
pub struct AlgorithmProcessingTimings {
    pub pre_processing_millis: u64,
    pub compute_millis: u64,
    pub side_effect_millis: u64,
}

impl AlgorithmProcessingTimings {
    pub fn new(pre_processing_millis: u64, compute_millis: u64, side_effect_millis: u64) -> Self {
        Self {
            pre_processing_millis,
            compute_millis,
            side_effect_millis,
        }
    }
}

/// Algorithm Processing Timings Builder - gathers timings builder-style
/// This guy gathers timings, builder-stylee.
pub struct AlgorithmProcessingTimingsBuilder {
    // timings
    pre_processing_millis: u64,
    compute_millis: u64,
    side_effect_millis: u64,
}

impl AlgorithmProcessingTimingsBuilder {
    // This is a marker
    const NOT_AVAILABLE: i64 = -1;

    pub fn new() -> Self {
        Self {
            pre_processing_millis: Self::NOT_AVAILABLE as u64,
            compute_millis: Self::NOT_AVAILABLE as u64,
            side_effect_millis: Self::NOT_AVAILABLE as u64,
        }
    }

    pub fn with_pre_processing_millis(&mut self, pre_processing_millis: u64) -> &mut Self {
        self.pre_processing_millis = pre_processing_millis;
        self
    }

    pub fn with_compute_millis(&mut self, compute_millis: u64) -> &mut Self {
        self.compute_millis = compute_millis;
        self
    }

    pub fn with_side_effect_millis(&mut self, side_effect_millis: u64) -> &mut Self {
        self.side_effect_millis = side_effect_millis;
        self
    }

    pub fn build(self) -> AlgorithmProcessingTimings {
        AlgorithmProcessingTimings::new(
            self.pre_processing_millis,
            self.compute_millis,
            self.side_effect_millis,
        )
    }
}

impl Default for AlgorithmProcessingTimingsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
