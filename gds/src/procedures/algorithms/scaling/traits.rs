use std::collections::HashMap;

/// Close-to-zero threshold for numerical stability
pub const CLOSE_TO_ZERO: f64 = 1e-15;

/// Statistics map, following the Java layout: key -> list of one double
pub type ScalarStats = HashMap<String, Vec<f64>>;

/// Generic scaler interface for scalar properties
pub trait Scaler {
    /// Scale a single property's value for the given node id
    fn scale_property(&self, node_id: usize) -> f64;

    /// Dimensionality of the scaler (1 for scalar scalers)
    fn dimension(&self) -> usize;

    /// Optional statistics computed during scaler construction
    fn statistics(&self) -> &ScalarStats;
}

/// Base implementation for scalar (1D) scalers
pub struct ScalarScaler {
    stats: ScalarStats,
}

impl ScalarScaler {
    pub fn new(stats: ScalarStats) -> Self {
        Self { stats }
    }

    pub fn empty() -> Self {
        Self { stats: ScalarStats::new() }
    }
}

impl Scaler for ScalarScaler {
    fn scale_property(&self, _node_id: usize) -> f64 {
        0.0
    }

    fn dimension(&self) -> usize {
        1
    }

    fn statistics(&self) -> &ScalarStats {
        &self.stats
    }
}

/// Array wrapper that exposes multiple ScalarScaler elements as an array scaler
pub struct ArrayScaler {
    elements: Vec<Box<dyn Scaler + Send + Sync>>, // keep Send+Sync to allow future parallelization
    stats_keys: Vec<String>,
}

impl ArrayScaler {
    pub fn new(elements: Vec<Box<dyn Scaler + Send + Sync>>) -> Self {
        // Collect statistics keys from the first element if available
        let stats_keys = if let Some(first) = elements.first() {
            first
                .statistics()
                .keys()
                .cloned()
                .collect::<Vec<String>>()
        } else {
            Vec::new()
        };
        Self { elements, stats_keys }
    }

    /// Fill the result slice with scaled values starting at offset
    pub fn scale_property_into(&self, node_id: usize, result: &mut [f64], offset: usize) {
        for (i, scaler) in self.elements.iter().enumerate() {
            result[offset + i] = scaler.scale_property(node_id);
        }
    }

    /// Merge statistics across elements by zipping the first values per key
    pub fn statistics(&self) -> ScalarStats {
        let mut stats: ScalarStats = ScalarStats::new();
        for key in &self.stats_keys {
            let mut values = Vec::with_capacity(self.elements.len());
            for scaler in &self.elements {
                if let Some(vs) = scaler.statistics().get(key) {
                    if let Some(v) = vs.first() {
                        values.push(*v);
                    }
                }
            }
            stats.insert(key.clone(), values);
        }
        stats
    }
}


