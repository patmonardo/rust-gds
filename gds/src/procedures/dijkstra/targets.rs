//! Targets System for Dijkstra Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.paths.dijkstra.Targets`
//!
//! This module implements the polymorphic target system that acts as the "instruction set"
//! for the Dijkstra Algorithmic Virtual Machine. Different target implementations control
//! the algorithm's behavior for single-target, many-targets, and all-targets modes.

use super::traversal_state::TraversalState;
use std::collections::HashSet;

/// Trait for target implementations that control Dijkstra algorithm behavior
///
/// Translation of: `Targets.java` interface (lines 24-39)
/// This trait acts as the "instruction set" for the Algorithmic Virtual Machine
pub trait Targets {
    /// Apply the target logic to determine traversal state
    ///
    /// Translation of: `apply(long nodeId)` method (line 26)
    /// This is the "instruction execution" of the VM
    fn apply(&mut self, node_id: u32) -> TraversalState;
}

/// Single target implementation - stops when target is reached
///
/// Translation of: `SingleTarget.java` (lines 25-37)
pub struct SingleTarget {
    target_node: u32,
}

impl SingleTarget {
    /// Create a new single target
    ///
    /// Translation of: Constructor (lines 29-31)
    pub fn new(target_node: u32) -> Self {
        Self { target_node }
    }
}

impl Targets for SingleTarget {
    /// Stop when target is reached
    ///
    /// Translation of: `apply()` method (lines 34-36)
    fn apply(&mut self, node_id: u32) -> TraversalState {
        if node_id == self.target_node {
            TraversalState::EmitAndStop
        } else {
            TraversalState::Continue
        }
    }
}

/// Many targets implementation - continues until all targets are found
///
/// Translation of: `ManyTargets.java` (lines 26-50)
pub struct ManyTargets {
    target_nodes: HashSet<u32>,
    remaining_count: usize,
}

impl ManyTargets {
    /// Create a new many targets implementation
    ///
    /// Translation of: Constructor (lines 30-39)
    pub fn new(target_nodes: Vec<u32>) -> Self {
        let target_set: HashSet<u32> = target_nodes.into_iter().collect();
        let remaining_count = target_set.len();
        
        Self {
            target_nodes: target_set,
            remaining_count,
        }
    }
}

impl Targets for ManyTargets {
    /// Continue until all targets are found
    ///
    /// Translation of: `apply()` method (lines 41-49)
    fn apply(&mut self, node_id: u32) -> TraversalState {
        if self.target_nodes.contains(&node_id) {
            self.remaining_count -= 1;
            
            if self.remaining_count == 0 {
                TraversalState::EmitAndStop
            } else {
                TraversalState::EmitAndContinue
            }
        } else {
            TraversalState::Continue
        }
    }
}

/// All targets implementation - continues for all nodes
///
/// Translation of: `AllTargets.java` (lines 22-28)
pub struct AllTargets;

impl AllTargets {
    /// Create a new all targets implementation
    pub fn new() -> Self {
        Self
    }
}

impl Targets for AllTargets {
    /// Always continue and emit results
    ///
    /// Translation of: `apply()` method (lines 25-27)
    fn apply(&mut self, _node_id: u32) -> TraversalState {
        TraversalState::EmitAndContinue
    }
}

/// Factory function for creating appropriate target implementation
///
/// Translation of: `Targets.of()` static method (lines 28-38)
pub fn create_targets(target_nodes: Vec<u32>) -> Box<dyn Targets> {
    if target_nodes.is_empty() {
        Box::new(AllTargets::new())
    } else if target_nodes.len() == 1 {
        Box::new(SingleTarget::new(target_nodes[0]))
    } else {
        Box::new(ManyTargets::new(target_nodes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_target() {
        let mut target = SingleTarget::new(5);
        
        assert_eq!(target.apply(3), TraversalState::Continue);
        assert_eq!(target.apply(5), TraversalState::EmitAndStop);
        assert_eq!(target.apply(7), TraversalState::Continue);
    }

    #[test]
    fn test_many_targets() {
        let mut target = ManyTargets::new(vec![3, 5, 7]);
        
        assert_eq!(target.apply(1), TraversalState::Continue);
        assert_eq!(target.apply(3), TraversalState::EmitAndContinue);
        assert_eq!(target.apply(5), TraversalState::EmitAndContinue);
        assert_eq!(target.apply(7), TraversalState::EmitAndStop);
        assert_eq!(target.apply(9), TraversalState::Continue);
    }

    #[test]
    fn test_all_targets() {
        let mut target = AllTargets::new();
        
        assert_eq!(target.apply(1), TraversalState::EmitAndContinue);
        assert_eq!(target.apply(5), TraversalState::EmitAndContinue);
        assert_eq!(target.apply(10), TraversalState::EmitAndContinue);
    }

    #[test]
    fn test_create_targets_factory() {
        // Empty list -> AllTargets
        let mut targets = create_targets(vec![]);
        assert_eq!(targets.apply(5), TraversalState::EmitAndContinue);
        
        // Single target -> SingleTarget
        let mut targets = create_targets(vec![5]);
        assert_eq!(targets.apply(3), TraversalState::Continue);
        assert_eq!(targets.apply(5), TraversalState::EmitAndStop);
        
        // Multiple targets -> ManyTargets
        let mut targets = create_targets(vec![3, 5, 7]);
        assert_eq!(targets.apply(1), TraversalState::Continue);
        assert_eq!(targets.apply(3), TraversalState::EmitAndContinue);
        assert_eq!(targets.apply(5), TraversalState::EmitAndContinue);
        assert_eq!(targets.apply(7), TraversalState::EmitAndStop);
    }
}
