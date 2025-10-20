/// Algorithm labels used throughout the Applications system.
/// This provides a unified way to identify and categorize algorithms.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlgorithmLabel {
    // Centrality algorithms
    PageRank,
    ArticleRank,
    EigenVector,
    BetweennessCentrality,
    ClosenessCentrality,
    DegreeCentrality,
    HarmonicCentrality,
    HITS,
    ArticulationPoints,
    Bridges,
    CELF,
    IndirectExposure,
    
    // Community algorithms
    Louvain,
    Leiden,
    LabelPropagation,
    WeaklyConnectedComponents,
    StronglyConnectedComponents,
    TriangleCount,
    LocalClusteringCoefficient,
    
    // Similarity algorithms
    NodeSimilarity,
    FilteredNodeSimilarity,
    KNN,
    FilteredKNN,
    CosineSimilarity,
    JaccardSimilarity,
    OverlapSimilarity,
    
    // Path finding algorithms
    ShortestPath,
    AllPairsShortestPath,
    SingleSourceShortestPath,
    YenKShortestPath,
    
    // Node embedding algorithms
    FastRP,
    GraphSage,
    GraphSageTrain,
    HashGNN,
    Node2Vec,
    
    // Machine learning algorithms
    KGE,
    SplitRelationships,
    LogisticRegression,
    RandomForest,
    GradientBoosting,
    
    // Miscellaneous algorithms
    CollapsePath,
    IndexInverse,
    ScaleProperties,
    ToUndirected,
    KMeans,
    DBSCAN,
}

impl AlgorithmLabel {
    /// Returns the string representation of the algorithm label.
    pub fn as_string(&self) -> &'static str {
        match self {
            // Centrality algorithms
            AlgorithmLabel::PageRank => "PageRank",
            AlgorithmLabel::ArticleRank => "ArticleRank",
            AlgorithmLabel::EigenVector => "EigenVector",
            AlgorithmLabel::BetweennessCentrality => "BetweennessCentrality",
            AlgorithmLabel::ClosenessCentrality => "ClosenessCentrality",
            AlgorithmLabel::DegreeCentrality => "DegreeCentrality",
            AlgorithmLabel::HarmonicCentrality => "HarmonicCentrality",
            AlgorithmLabel::HITS => "HITS",
            AlgorithmLabel::ArticulationPoints => "ArticulationPoints",
            AlgorithmLabel::Bridges => "Bridges",
            AlgorithmLabel::CELF => "CELF",
            AlgorithmLabel::IndirectExposure => "IndirectExposure",
            
            // Community algorithms
            AlgorithmLabel::Louvain => "Louvain",
            AlgorithmLabel::Leiden => "Leiden",
            AlgorithmLabel::LabelPropagation => "LabelPropagation",
            AlgorithmLabel::WeaklyConnectedComponents => "WeaklyConnectedComponents",
            AlgorithmLabel::StronglyConnectedComponents => "StronglyConnectedComponents",
            AlgorithmLabel::TriangleCount => "TriangleCount",
            AlgorithmLabel::LocalClusteringCoefficient => "LocalClusteringCoefficient",
            
            // Similarity algorithms
            AlgorithmLabel::NodeSimilarity => "NodeSimilarity",
            AlgorithmLabel::FilteredNodeSimilarity => "FilteredNodeSimilarity",
            AlgorithmLabel::KNN => "KNN",
            AlgorithmLabel::FilteredKNN => "FilteredKNN",
            AlgorithmLabel::CosineSimilarity => "CosineSimilarity",
            AlgorithmLabel::JaccardSimilarity => "JaccardSimilarity",
            AlgorithmLabel::OverlapSimilarity => "OverlapSimilarity",
            
            // Path finding algorithms
            AlgorithmLabel::ShortestPath => "ShortestPath",
            AlgorithmLabel::AllPairsShortestPath => "AllPairsShortestPath",
            AlgorithmLabel::SingleSourceShortestPath => "SingleSourceShortestPath",
            AlgorithmLabel::YenKShortestPath => "YenKShortestPath",
            
            // Node embedding algorithms
            AlgorithmLabel::FastRP => "FastRP",
            AlgorithmLabel::GraphSage => "GraphSage",
            AlgorithmLabel::GraphSageTrain => "GraphSageTrain",
            AlgorithmLabel::HashGNN => "HashGNN",
            AlgorithmLabel::Node2Vec => "Node2Vec",
            
            // Machine learning algorithms
            AlgorithmLabel::KGE => "KGE",
            AlgorithmLabel::SplitRelationships => "SplitRelationships",
            AlgorithmLabel::LogisticRegression => "LogisticRegression",
            AlgorithmLabel::RandomForest => "RandomForest",
            AlgorithmLabel::GradientBoosting => "GradientBoosting",
            
            // Miscellaneous algorithms
            AlgorithmLabel::CollapsePath => "CollapsePath",
            AlgorithmLabel::IndexInverse => "IndexInverse",
            AlgorithmLabel::ScaleProperties => "ScaleProperties",
            AlgorithmLabel::ToUndirected => "ToUndirected",
            AlgorithmLabel::KMeans => "KMeans",
            AlgorithmLabel::DBSCAN => "DBSCAN",
        }
    }
}

impl std::fmt::Display for AlgorithmLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
