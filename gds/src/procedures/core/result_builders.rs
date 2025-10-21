//! Result Builders - Algorithm result construction and processing
//!
//! **Translation Source**: `org.neo4j.gds.result.Abstract*ResultBuilder` classes
//! **Key Features**: Result building, statistics integration, histogram generation
//!
//! This module provides result building capabilities for different algorithm types,
//! integrating with our statistics and progress tracking modules.

use crate::procedures::core::statistics::{StatisticalSummary, Histogram, StatisticsEngine, StatisticsConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Base result builder trait
pub trait ResultBuilder<T> {
    /// Build the final result
    fn build(self) -> Result<T, ResultBuilderError>;
    
    /// Add statistics to the result
    fn with_statistics(self, stats: StatisticalSummary) -> Self;
    
    /// Add histogram to the result
    fn with_histogram(self, histogram: Option<Histogram>) -> Self;
    
    /// Add execution metadata
    fn with_metadata(self, metadata: ExecutionMetadata) -> Self;
}

/// Execution metadata for algorithm results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    /// Execution time
    pub execution_time: Duration,
    /// Number of iterations (if applicable)
    pub iterations: Option<u32>,
    /// Convergence status (if applicable)
    pub converged: Option<bool>,
    /// Additional metadata
    pub additional: HashMap<String, String>,
}

impl ExecutionMetadata {
    /// Create new execution metadata
    pub fn new(execution_time: Duration) -> Self {
        Self {
            execution_time,
            iterations: None,
            converged: None,
            additional: HashMap::new(),
        }
    }
    
    /// Add iteration count
    pub fn with_iterations(mut self, iterations: u32) -> Self {
        self.iterations = Some(iterations);
        self
    }
    
    /// Add convergence status
    pub fn with_convergence(mut self, converged: bool) -> Self {
        self.converged = Some(converged);
        self
    }
    
    /// Add additional metadata
    pub fn with_additional(mut self, key: String, value: String) -> Self {
        self.additional.insert(key, value);
        self
    }
}

/// Centrality algorithm result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentralityResult {
    /// Node centrality scores
    pub scores: Vec<f64>,
    /// Statistical summary
    pub statistics: Option<StatisticalSummary>,
    /// Histogram of scores
    pub histogram: Option<Histogram>,
    /// Execution metadata
    pub metadata: ExecutionMetadata,
}

/// Community detection algorithm result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityResult {
    /// Community assignments (node_id -> community_id)
    pub communities: Vec<u32>,
    /// Community sizes
    pub community_sizes: HashMap<u32, usize>,
    /// Number of communities
    pub community_count: u32,
    /// Statistical summary of community sizes
    pub size_statistics: Option<StatisticalSummary>,
    /// Histogram of community sizes
    pub size_histogram: Option<Histogram>,
    /// Execution metadata
    pub metadata: ExecutionMetadata,
}

/// Similarity algorithm result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    /// Similarity scores
    pub scores: Vec<f64>,
    /// Statistical summary
    pub statistics: Option<StatisticalSummary>,
    /// Histogram of scores
    pub histogram: Option<Histogram>,
    /// Execution metadata
    pub metadata: ExecutionMetadata,
}

/// Centrality result builder
pub struct CentralityResultBuilder {
    scores: Vec<f64>,
    statistics: Option<StatisticalSummary>,
    histogram: Option<Histogram>,
    metadata: Option<ExecutionMetadata>,
    compute_statistics: bool,
    compute_histogram: bool,
}

impl CentralityResultBuilder {
    /// Create a new centrality result builder
    pub fn new(scores: Vec<f64>) -> Self {
        Self {
            scores,
            statistics: None,
            histogram: None,
            metadata: None,
            compute_statistics: true,
            compute_histogram: true,
        }
    }
    
    /// Enable or disable statistics computation
    pub fn with_statistics(mut self, compute: bool) -> Self {
        self.compute_statistics = compute;
        self
    }
    
    /// Enable or disable histogram computation
    pub fn with_histogram(mut self, compute: bool) -> Self {
        self.compute_histogram = compute;
        self
    }
    
    /// Set execution metadata
    pub fn with_metadata(mut self, metadata: ExecutionMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl ResultBuilder<CentralityResult> for CentralityResultBuilder {
    fn build(self) -> Result<CentralityResult, ResultBuilderError> {
        let mut statistics = self.statistics;
        let mut histogram = self.histogram;
        
        // Compute statistics if requested and not already provided
        if self.compute_statistics && statistics.is_none() {
            let config = StatisticsConfig {
                compute_histogram: self.compute_histogram,
                ..Default::default()
            };
            
            let (stats, hist) = StatisticsEngine::compute_statistics_from_values(
                self.scores.clone(),
                config,
            )?;
            
            statistics = Some(stats);
            if self.compute_histogram {
                histogram = hist;
            }
        }
        
        let metadata = self.metadata.unwrap_or_else(|| {
            ExecutionMetadata::new(Duration::from_secs(0))
        });
        
        Ok(CentralityResult {
            scores: self.scores,
            statistics,
            histogram,
            metadata,
        })
    }
    
    fn with_statistics(mut self, stats: StatisticalSummary) -> Self {
        self.statistics = Some(stats);
        self
    }
    
    fn with_histogram(mut self, hist: Option<Histogram>) -> Self {
        self.histogram = hist;
        self
    }
    
    fn with_metadata(mut self, metadata: ExecutionMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Community result builder
pub struct CommunityResultBuilder {
    communities: Vec<u32>,
    compute_statistics: bool,
    compute_histogram: bool,
    metadata: Option<ExecutionMetadata>,
}

impl CommunityResultBuilder {
    /// Create a new community result builder
    pub fn new(communities: Vec<u32>) -> Self {
        Self {
            communities,
            compute_statistics: true,
            compute_histogram: true,
            metadata: None,
        }
    }
    
    /// Enable or disable statistics computation
    pub fn with_statistics(mut self, compute: bool) -> Self {
        self.compute_statistics = compute;
        self
    }
    
    /// Enable or disable histogram computation
    pub fn with_histogram(mut self, compute: bool) -> Self {
        self.compute_histogram = compute;
        self
    }
    
    /// Set execution metadata
    pub fn with_metadata(mut self, metadata: ExecutionMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl ResultBuilder<CommunityResult> for CommunityResultBuilder {
    fn build(self) -> Result<CommunityResult, ResultBuilderError> {
        // Compute community sizes
        let mut community_sizes: HashMap<u32, usize> = HashMap::new();
        for &community_id in &self.communities {
            *community_sizes.entry(community_id).or_insert(0) += 1;
        }
        
        let community_count = community_sizes.len() as u32;
        
        // Compute statistics for community sizes if requested
        let mut size_statistics = None;
        let mut size_histogram = None;
        
        if self.compute_statistics {
            let size_values: Vec<f64> = community_sizes.values().map(|&size| size as f64).collect();
            let config = StatisticsConfig {
                compute_histogram: self.compute_histogram,
                ..Default::default()
            };
            
            let (stats, hist) = StatisticsEngine::compute_statistics_from_values(size_values, config)?;
            size_statistics = Some(stats);
            if self.compute_histogram {
                size_histogram = hist;
            }
        }
        
        let metadata = self.metadata.unwrap_or_else(|| {
            ExecutionMetadata::new(Duration::from_secs(0))
        });
        
        Ok(CommunityResult {
            communities: self.communities,
            community_sizes,
            community_count,
            size_statistics,
            size_histogram,
            metadata,
        })
    }
    
    fn with_statistics(mut self, stats: StatisticalSummary) -> Self {
        // For community results, statistics are computed from community sizes
        // This method is kept for interface compatibility
        self
    }
    
    fn with_histogram(mut self, hist: Option<Histogram>) -> Self {
        // For community results, histogram is computed from community sizes
        // This method is kept for interface compatibility
        self
    }
    
    fn with_metadata(mut self, metadata: ExecutionMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Similarity result builder
pub struct SimilarityResultBuilder {
    scores: Vec<f64>,
    statistics: Option<StatisticalSummary>,
    histogram: Option<Histogram>,
    metadata: Option<ExecutionMetadata>,
    compute_statistics: bool,
    compute_histogram: bool,
}

impl SimilarityResultBuilder {
    /// Create a new similarity result builder
    pub fn new(scores: Vec<f64>) -> Self {
        Self {
            scores,
            statistics: None,
            histogram: None,
            metadata: None,
            compute_statistics: true,
            compute_histogram: true,
        }
    }
    
    /// Enable or disable statistics computation
    pub fn with_statistics(mut self, compute: bool) -> Self {
        self.compute_statistics = compute;
        self
    }
    
    /// Enable or disable histogram computation
    pub fn with_histogram(mut self, compute: bool) -> Self {
        self.compute_histogram = compute;
        self
    }
    
    /// Set execution metadata
    pub fn with_metadata(mut self, metadata: ExecutionMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl ResultBuilder<SimilarityResult> for SimilarityResultBuilder {
    fn build(self) -> Result<SimilarityResult, ResultBuilderError> {
        let mut statistics = self.statistics;
        let mut histogram = self.histogram;
        
        // Compute statistics if requested and not already provided
        if self.compute_statistics && statistics.is_none() {
            let config = StatisticsConfig {
                compute_histogram: self.compute_histogram,
                ..Default::default()
            };
            
            let (stats, hist) = StatisticsEngine::compute_statistics_from_values(
                self.scores.clone(),
                config,
            )?;
            
            statistics = Some(stats);
            if self.compute_histogram {
                histogram = hist;
            }
        }
        
        let metadata = self.metadata.unwrap_or_else(|| {
            ExecutionMetadata::new(Duration::from_secs(0))
        });
        
        Ok(SimilarityResult {
            scores: self.scores,
            statistics,
            histogram,
            metadata,
        })
    }
    
    fn with_statistics(mut self, stats: StatisticalSummary) -> Self {
        self.statistics = Some(stats);
        self
    }
    
    fn with_histogram(mut self, hist: Option<Histogram>) -> Self {
        self.histogram = hist;
        self
    }
    
    fn with_metadata(mut self, metadata: ExecutionMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Result builder error
#[derive(Debug, thiserror::Error)]
pub enum ResultBuilderError {
    #[error("Statistics computation failed: {0}")]
    StatisticsError(#[from] crate::procedures::core::statistics::StatisticsError),
    
    #[error("Invalid result data: {0}")]
    InvalidData(String),
    
    #[error("Builder configuration error: {0}")]
    ConfigurationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centrality_result_builder() {
        let scores = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let metadata = ExecutionMetadata::new(Duration::from_secs(1));
        
        let result = CentralityResultBuilder::new(scores.clone())
            .with_metadata(metadata)
            .build()
            .unwrap();
        
        assert_eq!(result.scores, scores);
        assert!(result.statistics.is_some());
        assert!(result.histogram.is_some());
        assert_eq!(result.metadata.execution_time, Duration::from_secs(1));
    }

    #[test]
    fn test_community_result_builder() {
        let communities = vec![0, 0, 1, 1, 2];
        let metadata = ExecutionMetadata::new(Duration::from_secs(2));
        
        let result = CommunityResultBuilder::new(communities.clone())
            .with_metadata(metadata)
            .build()
            .unwrap();
        
        assert_eq!(result.communities, communities);
        assert_eq!(result.community_count, 3);
        assert_eq!(result.community_sizes.get(&0), Some(&2));
        assert_eq!(result.community_sizes.get(&1), Some(&2));
        assert_eq!(result.community_sizes.get(&2), Some(&1));
        assert!(result.size_statistics.is_some());
        assert!(result.size_histogram.is_some());
    }

    #[test]
    fn test_similarity_result_builder() {
        let scores = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let metadata = ExecutionMetadata::new(Duration::from_secs(3));
        
        let result = SimilarityResultBuilder::new(scores.clone())
            .with_metadata(metadata)
            .build()
            .unwrap();
        
        assert_eq!(result.scores, scores);
        assert!(result.statistics.is_some());
        assert!(result.histogram.is_some());
        assert_eq!(result.metadata.execution_time, Duration::from_secs(3));
    }

    #[test]
    fn test_result_builder_without_statistics() {
        let scores = vec![1.0, 2.0, 3.0];
        
        let result = CentralityResultBuilder::new(scores.clone())
            .with_statistics(false)
            .with_histogram(false)
            .build()
            .unwrap();
        
        assert_eq!(result.scores, scores);
        assert!(result.statistics.is_none());
        assert!(result.histogram.is_none());
    }

    #[test]
    fn test_execution_metadata() {
        let metadata = ExecutionMetadata::new(Duration::from_secs(5))
            .with_iterations(100)
            .with_convergence(true)
            .with_additional("algorithm".to_string(), "pagerank".to_string());
        
        assert_eq!(metadata.execution_time, Duration::from_secs(5));
        assert_eq!(metadata.iterations, Some(100));
        assert_eq!(metadata.converged, Some(true));
        assert_eq!(metadata.additional.get("algorithm"), Some(&"pagerank".to_string()));
    }
}
