//! SCC Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.scc.Scc`
//!
//! This module implements the computation runtime for SCC algorithm - the "Subtle pole" for ephemeral computation.

use crate::collections::{HugeLongArray, HugeObjectArray, BitSet};
use crate::core::utils::paged::HugeLongArrayStack;
use crate::types::prelude::GraphStore;
use crate::types::properties::relationship::traits::RelationshipIterator;
use crate::types::properties::relationship::RelationshipProperties;
use crate::core::utils::progress::ProgressTracker;
use crate::termination::TerminationFlag;
use super::spec::SccResult;

/// SCC computation result
#[derive(Debug, Clone)]
pub struct SccComputationResult {
    /// Component ID for each node
    pub components: Vec<u64>,
    /// Number of strongly connected components found
    pub component_count: usize,
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
}

impl SccComputationResult {
    /// Create a new SCC computation result
    pub fn new(components: Vec<u64>, component_count: usize, computation_time_ms: u64) -> Self {
        Self {
            components,
            component_count,
            computation_time_ms,
        }
    }
}

/// Stack event for iterative DFS
///
/// Translation of: The todo stack logic in `Scc.computePerNode()` (lines 89-110)
#[derive(Debug, Clone, Default)]
pub struct StackEvent {
    node_id: u64,
    is_node_visit: bool, // true for node visit, false for edge visit
}

impl StackEvent {
    /// Create a node visit event
    pub fn node_visit(node_id: u64) -> Self {
        Self {
            node_id,
            is_node_visit: true,
        }
    }
    
    /// Create an edge visit event
    pub fn edge_visit(node_id: u64) -> Self {
        Self {
            node_id,
            is_node_visit: false,
        }
    }
    
    /// Get the node ID
    pub fn node_id(&self) -> u64 {
        self.node_id
    }
    
    /// Check if this is a node visit
    pub fn is_node_visit(&self) -> bool {
        self.is_node_visit
    }
}

/// SCC computation runtime for ephemeral computation
///
/// Translation of: `org.neo4j.gds.scc.Scc` data structures and algorithm logic
pub struct SccComputationRuntime {
    /// Boundaries stack for tracking component boundaries
    boundaries: HugeLongArrayStack,
    /// Connected components result
    connected_components: HugeLongArray,
    /// Index array for DFS numbering
    index: HugeLongArray,
    /// DFS stack
    stack: HugeLongArrayStack,
    /// Todo stack for iterative DFS
    todo: HugeObjectArray<StackEvent>,
    /// Visited nodes
    visited: BitSet,
    /// Progress tracker
    progress_tracker: Option<ProgressTracker>,
    /// Termination flag
    termination_flag: Option<TerminationFlag>,
}

const UNORDERED: i64 = -1;

impl SccComputationRuntime {
    /// Create a new SCC computation runtime
    pub fn new() -> Self {
        Self {
            boundaries: HugeLongArrayStack::new(0),
            connected_components: HugeLongArray::new(0),
            index: HugeLongArray::new(0),
            stack: HugeLongArrayStack::new(0),
            todo: HugeObjectArray::new(0),
            visited: BitSet::new(0),
            progress_tracker: None,
            termination_flag: None,
        }
    }
    /// Initialize the computation runtime
    ///
    /// Translation of: `Scc` constructor (lines 47-65)
    pub fn initialize(
        &mut self,
        node_count: usize,
        progress_tracker: ProgressTracker,
        termination_flag: TerminationFlag,
    ) {
        self.boundaries = HugeLongArrayStack::new(node_count);
        self.connected_components = HugeLongArray::new(node_count);
        self.index = HugeLongArray::new(node_count);
        self.stack = HugeLongArrayStack::new(node_count);
        self.todo = HugeObjectArray::new(node_count * 2); // Can be as high as relationship count
        self.visited = BitSet::new(node_count);
        
        self.progress_tracker = Some(progress_tracker);
        self.termination_flag = Some(termination_flag);
        
        // Initialize arrays
        self.index.set_all(|_| UNORDERED);
        self.connected_components.set_all(|_| UNORDERED);
    }
    
    /// Check if a node is unordered (not yet processed)
    pub fn is_node_unordered(&self, node_id: usize) -> bool {
        self.index.get(node_id) == UNORDERED
    }
    
    /// Compute SCC for a single node using iterative DFS
    ///
    /// Translation of: `Scc.computePerNode()` (lines 80-110)
    pub fn compute_per_node<G: GraphStore + RelationshipIterator + RelationshipProperties>(
        &mut self,
        node_id: usize,
        component_id: usize,
        graph_store: &G,
    ) -> Result<(), String> {
        // Push node visit to todo stack
        self.todo.set(0, StackEvent::node_visit(node_id as u64));
        let mut todo_index = 0;
        
        while todo_index >= 0 {
            if let Some(termination_flag) = &self.termination_flag {
                if !termination_flag.running() {
                    return Err("Algorithm terminated by user".to_string());
                }
            }
            
            let event = self.todo.get(todo_index).clone();
            todo_index -= 1;
            
            if event.is_node_visit() {
                self.distinguish_node_visit_type(event.node_id() as usize, component_id, graph_store)?;
            } else {
                self.visit_edge(event.node_id() as usize, component_id)?;
            }
        }
        
        Ok(())
    }
    
    /// Distinguish between first and last visit to a node
    ///
    /// Translation of: `Scc.distinguishNodeVisitType()` (lines 112-118)
    fn distinguish_node_visit_type<G: GraphStore + RelationshipIterator + RelationshipProperties>(
        &mut self,
        node_id: usize,
        component_id: usize,
        graph_store: &G,
    ) -> Result<(), String> {
        if self.index.get(node_id) != UNORDERED {
            // Last visit
            self.post_visit_node(node_id, component_id)?;
        } else {
            // First visit
            self.visit_node(node_id, component_id, graph_store)?;
        }
        Ok(())
    }
    
    /// Visit a node for the first time
    ///
    /// Translation of: `Scc.visitNode()` (lines 120-130)
    fn visit_node<G: GraphStore + RelationshipIterator + RelationshipProperties>(
        &mut self,
        node_id: usize,
        component_id: usize,
        graph_store: &G,
    ) -> Result<(), String> {
        let stack_size = self.stack.size();
        self.index.set(node_id, stack_size as i64);
        self.stack.push(node_id as i64);
        self.boundaries.push(stack_size as i64);
        
        // Push node visit to todo stack
        self.todo.set(0, StackEvent::node_visit(node_id as u64));
        
        // Process all outgoing relationships
        let relationships = graph_store.stream_relationships(node_id as u64, graph_store.default_property_value());
        for relationship in relationships {
            let target_id = relationship.target_id();
            self.todo.set(0, StackEvent::edge_visit(target_id));
        }
        
        Ok(())
    }
    
    /// Visit an edge
    ///
    /// Translation of: `Scc.visitEdge()` (lines 132-140)
    fn visit_edge(&mut self, node_id: usize, component_id: usize) -> Result<(), String> {
        if self.index.get(node_id) == UNORDERED {
            // Organize a first visit to node_id
            self.todo.set(0, StackEvent::node_visit(node_id as u64));
        } else if !self.visited.get(node_id) {
            // Skip nodes already in a component
            while self.index.get(node_id) < self.boundaries.peek() {
                self.boundaries.pop();
            }
        }
        Ok(())
    }
    
    /// Post-visit a node
    ///
    /// Translation of: `Scc.postVisitNode()` (lines 142-153)
    fn post_visit_node(&mut self, node_id: usize, component_id: usize) -> Result<(), String> {
        if self.boundaries.peek() == self.index.get(node_id) {
            self.boundaries.pop();
            let mut element;
            loop {
                element = self.stack.pop();
                self.connected_components.set(element as usize, component_id as i64);
                self.visited.set(element as usize);
                if element == node_id as i64 {
                    break;
                }
            }
        }
        
        if let Some(_progress_tracker) = &self.progress_tracker {
            // TODO: Implement progress logging when ProgressTracker is fully implemented
        }
        
        Ok(())
    }
    
    /// Finalize the computation result
    pub fn finalize_result(&self, computation_time_ms: u64) -> SccComputationResult {
        let components: Vec<u64> = (0..self.connected_components.size())
            .map(|i| self.connected_components.get(i) as u64)
            .collect();
        
        let component_count = components.iter().max().map_or(0, |&max| max as usize + 1);
        
        SccComputationResult::new(components, component_count, computation_time_ms)
    }
}
