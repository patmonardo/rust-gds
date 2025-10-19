use super::dimensions_map::DimensionsMap;
use crate::projection::{NodeLabel, RelationshipType};
use std::collections::{HashMap, HashSet};

/// Special constants for graph dimensions
pub mod constants {
    pub const ANY_LABEL: i32 = -1;
    pub const ANY_RELATIONSHIP_TYPE: i32 = -1;
    pub const NO_SUCH_LABEL: i32 = -2;
    pub const NO_SUCH_RELATIONSHIP_TYPE: i32 = -2;
    pub const IGNORE: i32 = -4;
}

/// Abstract trait for graph dimensions with complete functionality.
/// Provides both the interface and implementation using trait pattern.
pub trait GraphDimensions {
    // =============================================================================
    // REQUIRED METHODS (Must be implemented)
    // =============================================================================

    /// Total number of nodes in the graph
    fn node_count(&self) -> usize;

    /// Set of node label tokens present in the graph
    fn node_label_tokens(&self) -> Option<&HashSet<i32>>;

    /// Set of relationship type tokens present in the graph
    fn relationship_type_tokens(&self) -> Option<&HashSet<i32>>;

    /// Mapping from tokens to node labels
    fn token_node_label_mapping(&self) -> Option<&HashMap<i32, Vec<NodeLabel>>>;

    /// Mapping from tokens to relationship types
    fn token_relationship_type_mapping(&self) -> Option<&HashMap<i32, Vec<RelationshipType>>>;

    // =============================================================================
    // PROVIDED METHODS WITH DEFAULTS (Can be overridden)
    // =============================================================================

    /// Highest possible node count (defaults to node_count)
    fn highest_possible_node_count(&self) -> usize {
        self.node_count()
    }

    /// Upper bound on relationship count
    fn rel_count_upper_bound(&self) -> usize {
        0
    }

    /// Map of relationship types to their counts
    fn relationship_counts(&self) -> &HashMap<RelationshipType, usize> {
        &EMPTY_REL_COUNTS
    }

    /// Highest relationship ID in the graph
    fn highest_relationship_id(&self) -> usize {
        self.rel_count_upper_bound()
    }

    /// Node property tokens mapping
    fn node_property_tokens(&self) -> &HashMap<String, i32> {
        &EMPTY_PROPERTY_TOKENS
    }

    /// Node property dimensions
    fn node_property_dimensions(&self) -> &DimensionsMap {
        &EMPTY_DIMENSIONS
    }

    /// Relationship property tokens mapping
    fn relationship_property_tokens(&self) -> &HashMap<String, i32> {
        &EMPTY_PROPERTY_TOKENS
    }

    // =============================================================================
    // DERIVED METHODS (Computed from other properties)
    // =============================================================================

    /// Get all available node labels in the graph.
    ///
    /// COMPUTATION:
    /// - Extracts all node labels from token mapping
    /// - Flattens the mapping values into a single collection
    /// - Returns empty set if no token mapping exists
    fn available_node_labels(&self) -> HashSet<NodeLabel> {
        let mapping = match self.token_node_label_mapping() {
            Some(m) => m,
            None => return HashSet::new(),
        };

        let mut labels = HashSet::new();
        for label_list in mapping.values() {
            for label in label_list {
                labels.insert(label.clone());
            }
        }
        labels
    }

    /// Get node labels that apply to all nodes (star mappings).
    ///
    /// STAR MAPPING:
    /// - Uses ANY_LABEL token to represent labels that apply to all nodes
    /// - Common for graphs where some labels are universal
    /// - Returns empty vec if no star mappings exist
    fn star_node_label_mappings(&self) -> Vec<NodeLabel> {
        match self.token_node_label_mapping() {
            Some(mapping) => mapping
                .get(&constants::ANY_LABEL)
                .cloned()
                .unwrap_or_default(),
            None => Vec::new(),
        }
    }

    /// Calculate average degree of nodes in the graph.
    ///
    /// CALCULATION:
    /// - Average degree = total relationships / total nodes
    /// - Returns 0 if graph has no nodes (avoid division by zero)
    /// - Uses upper bound for relationship count
    fn average_degree(&self) -> usize {
        let node_count = self.node_count();
        if node_count == 0 {
            0
        } else {
            self.rel_count_upper_bound() / node_count
        }
    }

    /// Create reverse mapping from relationship types to tokens.
    ///
    /// REVERSE MAPPING:
    /// - Inverts the token → relationship types mapping
    /// - Allows efficient lookup of token by relationship type
    /// - Returns None if no token mapping exists
    fn relationship_type_token_mapping(&self) -> Option<HashMap<RelationshipType, i32>> {
        let token_mapping = self.token_relationship_type_mapping()?;

        let mut reverse_mapping = HashMap::new();
        for (&token, relationship_types) in token_mapping {
            for relationship_type in relationship_types {
                reverse_mapping.insert(relationship_type.clone(), token);
            }
        }

        Some(reverse_mapping)
    }

    /// Estimate the number of distinct node labels for memory planning.
    ///
    /// ESTIMATION LOGIC:
    /// - Counts unique node labels across all token mappings
    /// - Excludes ALL_NODES label from count (universal label)
    /// - Returns 0 if only ALL_NODES labels exist
    /// - Used for memory estimation in graph algorithms
    fn estimation_node_label_count(&self) -> usize {
        let mapping = match self.token_node_label_mapping() {
            Some(m) => m,
            None => return 0,
        };

        let mut node_labels = HashSet::new();
        for label_list in mapping.values() {
            for label in label_list {
                node_labels.insert(label.clone());
            }
        }

        // Filter out ALL_NODES labels for estimation purposes
        node_labels
            .iter()
            .filter(|label| label != &&NodeLabel::all_nodes())
            .count()
    }

    /// Estimate relationship count for specific relationship types.
    ///
    /// ESTIMATION STRATEGY:
    /// - If requesting specific types (not PROJECT_ALL), try exact counts
    /// - Fall back to upper bound if exact counts unavailable
    /// - Handles wildcard projections efficiently
    fn estimated_rel_count(&self, relationship_type_names: &[String]) -> usize {
        // Handle wildcard projection
        if relationship_type_names.contains(&"*".to_string()) {
            return self.rel_count_upper_bound();
        }

        let rel_counts = self.relationship_counts();
        let requested_types: Vec<_> = relationship_type_names
            .iter()
            .map(RelationshipType::of)
            .collect();

        // Check if we have exact counts for all requested types
        let has_all_counts = requested_types.iter().all(|t| rel_counts.contains_key(t));

        if has_all_counts {
            requested_types
                .iter()
                .filter_map(|t| rel_counts.get(t))
                .sum()
        } else {
            // Fall back to upper bound if exact counts unavailable
            self.rel_count_upper_bound()
        }
    }
}

// Static empty collections to avoid allocations
lazy_static::lazy_static! {
    static ref EMPTY_REL_COUNTS: HashMap<RelationshipType, usize> = HashMap::new();
    static ref EMPTY_PROPERTY_TOKENS: HashMap<String, i32> = HashMap::new();
    static ref EMPTY_DIMENSIONS: DimensionsMap = DimensionsMap::empty();
}

/// Concrete implementation of GraphDimensions with builder pattern.
#[derive(Clone, Debug)]
pub struct ConcreteGraphDimensions {
    node_count_val: usize,
    highest_possible_node_count_val: usize,
    rel_count_upper_bound_val: usize,
    relationship_counts_val: HashMap<RelationshipType, usize>,
    highest_relationship_id_val: usize,
    node_label_tokens_val: Option<HashSet<i32>>,
    relationship_type_tokens_val: Option<HashSet<i32>>,
    token_node_label_mapping_val: Option<HashMap<i32, Vec<NodeLabel>>>,
    token_relationship_type_mapping_val: Option<HashMap<i32, Vec<RelationshipType>>>,
    node_property_tokens_val: HashMap<String, i32>,
    node_property_dimensions_val: DimensionsMap,
    relationship_property_tokens_val: HashMap<String, i32>,
}

impl GraphDimensions for ConcreteGraphDimensions {
    fn node_count(&self) -> usize {
        self.node_count_val
    }

    fn highest_possible_node_count(&self) -> usize {
        self.highest_possible_node_count_val
    }

    fn rel_count_upper_bound(&self) -> usize {
        self.rel_count_upper_bound_val
    }

    fn relationship_counts(&self) -> &HashMap<RelationshipType, usize> {
        &self.relationship_counts_val
    }

    fn highest_relationship_id(&self) -> usize {
        self.highest_relationship_id_val
    }

    fn node_label_tokens(&self) -> Option<&HashSet<i32>> {
        self.node_label_tokens_val.as_ref()
    }

    fn relationship_type_tokens(&self) -> Option<&HashSet<i32>> {
        self.relationship_type_tokens_val.as_ref()
    }

    fn token_node_label_mapping(&self) -> Option<&HashMap<i32, Vec<NodeLabel>>> {
        self.token_node_label_mapping_val.as_ref()
    }

    fn token_relationship_type_mapping(&self) -> Option<&HashMap<i32, Vec<RelationshipType>>> {
        self.token_relationship_type_mapping_val.as_ref()
    }

    fn node_property_tokens(&self) -> &HashMap<String, i32> {
        &self.node_property_tokens_val
    }

    fn node_property_dimensions(&self) -> &DimensionsMap {
        &self.node_property_dimensions_val
    }

    fn relationship_property_tokens(&self) -> &HashMap<String, i32> {
        &self.relationship_property_tokens_val
    }
}

/// Builder for GraphDimensions with fluent API and validation.
#[derive(Default)]
pub struct GraphDimensionsBuilder {
    node_count_val: usize,
    highest_possible_node_count_val: Option<usize>,
    rel_count_upper_bound_val: Option<usize>,
    relationship_counts_val: Option<HashMap<RelationshipType, usize>>,
    highest_relationship_id_val: Option<usize>,
    node_label_tokens_val: Option<HashSet<i32>>,
    relationship_type_tokens_val: Option<HashSet<i32>>,
    token_node_label_mapping_val: Option<HashMap<i32, Vec<NodeLabel>>>,
    token_relationship_type_mapping_val: Option<HashMap<i32, Vec<RelationshipType>>>,
    node_property_tokens_val: Option<HashMap<String, i32>>,
    node_property_dimensions_val: Option<DimensionsMap>,
    relationship_property_tokens_val: Option<HashMap<String, i32>>,
}

impl GraphDimensionsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn node_count(mut self, node_count: usize) -> Self {
        self.node_count_val = node_count;
        self
    }

    pub fn highest_possible_node_count(mut self, count: usize) -> Self {
        self.highest_possible_node_count_val = Some(count);
        self
    }

    pub fn rel_count_upper_bound(mut self, count: usize) -> Self {
        self.rel_count_upper_bound_val = Some(count);
        self
    }

    pub fn relationship_counts(mut self, counts: HashMap<RelationshipType, usize>) -> Self {
        self.relationship_counts_val = Some(counts);
        self
    }

    pub fn highest_relationship_id(mut self, id: usize) -> Self {
        self.highest_relationship_id_val = Some(id);
        self
    }

    pub fn node_label_tokens(mut self, tokens: HashSet<i32>) -> Self {
        self.node_label_tokens_val = Some(tokens);
        self
    }

    pub fn relationship_type_tokens(mut self, tokens: HashSet<i32>) -> Self {
        self.relationship_type_tokens_val = Some(tokens);
        self
    }

    pub fn token_node_label_mapping(mut self, mapping: HashMap<i32, Vec<NodeLabel>>) -> Self {
        self.token_node_label_mapping_val = Some(mapping);
        self
    }

    pub fn token_relationship_type_mapping(
        mut self,
        mapping: HashMap<i32, Vec<RelationshipType>>,
    ) -> Self {
        self.token_relationship_type_mapping_val = Some(mapping);
        self
    }

    pub fn node_property_tokens(mut self, tokens: HashMap<String, i32>) -> Self {
        self.node_property_tokens_val = Some(tokens);
        self
    }

    pub fn node_property_dimensions(mut self, dimensions: DimensionsMap) -> Self {
        self.node_property_dimensions_val = Some(dimensions);
        self
    }

    pub fn relationship_property_tokens(mut self, tokens: HashMap<String, i32>) -> Self {
        self.relationship_property_tokens_val = Some(tokens);
        self
    }

    pub fn build(self) -> ConcreteGraphDimensions {
        let node_count = self.node_count_val;
        let rel_count_upper_bound = self.rel_count_upper_bound_val.unwrap_or(0);

        ConcreteGraphDimensions {
            node_count_val: node_count,
            highest_possible_node_count_val: self
                .highest_possible_node_count_val
                .unwrap_or(node_count),
            rel_count_upper_bound_val: rel_count_upper_bound,
            relationship_counts_val: self.relationship_counts_val.unwrap_or_default(),
            highest_relationship_id_val: self
                .highest_relationship_id_val
                .unwrap_or(rel_count_upper_bound),
            node_label_tokens_val: self.node_label_tokens_val,
            relationship_type_tokens_val: self.relationship_type_tokens_val,
            token_node_label_mapping_val: self.token_node_label_mapping_val,
            token_relationship_type_mapping_val: self.token_relationship_type_mapping_val,
            node_property_tokens_val: self.node_property_tokens_val.unwrap_or_default(),
            node_property_dimensions_val: self
                .node_property_dimensions_val
                .unwrap_or_else(DimensionsMap::empty),
            relationship_property_tokens_val: self
                .relationship_property_tokens_val
                .unwrap_or_default(),
        }
    }
}

/// Factory methods for common GraphDimensions patterns
impl ConcreteGraphDimensions {
    /// Create simple GraphDimensions with node and relationship counts.
    pub fn of(node_count: usize, relationship_count: usize) -> Self {
        let mut rel_counts = HashMap::new();
        rel_counts.insert(RelationshipType::all_relationships(), relationship_count);

        GraphDimensionsBuilder::new()
            .node_count(node_count)
            .relationship_counts(rel_counts)
            .rel_count_upper_bound(relationship_count)
            .build()
    }

    /// Create a builder for fluent GraphDimensions construction.
    pub fn builder() -> GraphDimensionsBuilder {
        GraphDimensionsBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_dimensions() {
        let dims = ConcreteGraphDimensions::of(100, 200);
        assert_eq!(dims.node_count(), 100);
        assert_eq!(dims.rel_count_upper_bound(), 200);
    }

    #[test]
    fn test_average_degree() {
        let dims = ConcreteGraphDimensions::of(10, 30);
        assert_eq!(dims.average_degree(), 3);
    }

    #[test]
    fn test_builder() {
        let dims = GraphDimensionsBuilder::new()
            .node_count(50)
            .rel_count_upper_bound(100)
            .build();

        assert_eq!(dims.node_count(), 50);
        assert_eq!(dims.rel_count_upper_bound(), 100);
    }
}
