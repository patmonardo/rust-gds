//! Traversal State for Dijkstra Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.paths.dijkstra.TraversalState`
//!
//! This module defines the traversal state enum that controls the execution flow
//! of the Dijkstra Algorithmic Virtual Machine.

/// Traversal state for controlling Dijkstra algorithm execution
///
/// Translation of: `TraversalState.java` (lines 22-26)
/// This enum represents the execution state of the Algorithmic Virtual Machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraversalState {
    /// Emit a path result and stop the algorithm
    EmitAndStop,
    
    /// Emit a path result and continue the algorithm
    EmitAndContinue,
    
    /// Continue the algorithm without emitting a result
    Continue,
}

impl TraversalState {
    /// Check if the algorithm should emit a result
    pub fn should_emit(&self) -> bool {
        matches!(self, TraversalState::EmitAndStop | TraversalState::EmitAndContinue)
    }
    
    /// Check if the algorithm should stop
    pub fn should_stop(&self) -> bool {
        matches!(self, TraversalState::EmitAndStop)
    }
    
    /// Check if the algorithm should continue
    pub fn should_continue(&self) -> bool {
        matches!(self, TraversalState::EmitAndContinue | TraversalState::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traversal_state_should_emit() {
        assert!(TraversalState::EmitAndStop.should_emit());
        assert!(TraversalState::EmitAndContinue.should_emit());
        assert!(!TraversalState::Continue.should_emit());
    }

    #[test]
    fn test_traversal_state_should_stop() {
        assert!(TraversalState::EmitAndStop.should_stop());
        assert!(!TraversalState::EmitAndContinue.should_stop());
        assert!(!TraversalState::Continue.should_stop());
    }

    #[test]
    fn test_traversal_state_should_continue() {
        assert!(!TraversalState::EmitAndStop.should_continue());
        assert!(TraversalState::EmitAndContinue.should_continue());
        assert!(TraversalState::Continue.should_continue());
    }
}
