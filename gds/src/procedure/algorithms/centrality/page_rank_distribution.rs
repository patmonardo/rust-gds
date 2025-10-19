//! PageRankDistribution - Faithful 1:1 translation from Java GDS
//!
//! Translated from: org.neo4j.gds.algorithms.centrality.PageRankDistribution
//!
//! Holds PageRank statistics and timing information in an immutable container.

use std::collections::HashMap;

/// PageRank distribution result - translated from Java PageRankDistribution
/// 
/// Holds PageRank statistics and timing information in an immutable container.
/// 
/// Java fields:
/// - public final Map<String, Object> centralitySummary;
/// - public final long postProcessingMillis;
pub struct PageRankDistribution {
    /// Centrality summary statistics (mean, std, min, max, etc.)
    /// Translated from: public final Map<String, Object> centralitySummary;
    pub centrality_summary: HashMap<String, String>, // Using String for simplicity, should be Object equivalent
    
    /// Post-processing time in milliseconds
    /// Translated from: public final long postProcessingMillis;
    pub post_processing_millis: u64,
}

impl PageRankDistribution {
    /// Constructor - translated from Java constructor
    /// 
    /// Java constructor:
    /// ```java
    /// PageRankDistribution(Map<String, Object> centralitySummary, long postProcessingMillis) {
    ///     this.centralitySummary = Collections.unmodifiableMap(centralitySummary);
    ///     this.postProcessingMillis = postProcessingMillis;
    /// }
    /// ```
    pub fn new(
        centrality_summary: HashMap<String, String>,
        post_processing_millis: u64,
    ) -> Self {
        Self {
            centrality_summary,
            post_processing_millis,
        }
    }
}
