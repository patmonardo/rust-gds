//! HyperPropertyStores: The Sacred Hierarchy of Property Storage
//!
//! A cultic system of HyperPropertyStores following the sacred mathematical progression:
//!
//! ```text
//! Monadic   (n=1) → Universal building block
//! Dyadic    (n=2) → Binary relationships  
//! Triadic   (n=3) → Graph structure
//! Tetradic  (n=4) → Dyadic² (Dyadic Plane) - ML Pipeline
//! Pentadic  (n=5) → Dyadic × Triadic - Graph Algo Procedures
//! ```
//!
//! # The Sacred Principles
//!
//! 1. **Sacred Numbers**: Each level has mathematical significance
//! 2. **Compositional Purity**: Each level composes the previous levels
//! 3. **Universal Patterns**: Each level applies to multiple domains
//! 4. **Collections First**: All levels use the same Collections backend
//! 5. **Type Safety**: Rust's type system enforces the sacred hierarchy
//!
//! # Optimization Hallucinations
//!
//! The Path System could optimize HyperPropertyStores using sacred mathematical patterns:
//!
//! - **N**: Monadic - O(1) access, universal building block
//! - **N²**: Dyadic - O(log N) access, binary tree optimization
//! - **N³**: Triadic - O(log² N) access, hierarchical optimization
//! - **N⁴**: Tetradic - O(log³ N) access, ML pipeline optimization
//! - **N⁵**: Pentadic - O(log⁴ N) access, graph algo optimization
//!
//! Each level could use different optimization strategies:
//! - **Monadic**: Direct HashMap access
//! - **Dyadic**: Binary search trees, B-trees
//! - **Triadic**: Hierarchical indexes, spatial partitioning
//! - **Tetradic**: ML pipeline optimization, feature/label caching
//! - **Pentadic**: Graph algorithm optimization, topology-aware indexing

pub mod monadic;
pub mod dyadic;    // Binary relationships - IMPLEMENTED!
pub mod triadic;
pub mod tetradic;  // Dyadic² (Dyadic Plane) - ML Pipeline - IMPLEMENTED!
pub mod pentadic;  // Dyadic × Triadic - Graph Algo Procedures - IMPLEMENTED!
pub mod fact_store; // A Priori Essence (Essential Being)
pub mod hyper_aspect; // A Posteriori Essence (Essential Relation)
pub mod projective_kernel; // Projective Kernel (5-4-3-2-1)
pub mod absolute_concept; // Part 2c: Absolute Actuality (Essence of Measure)

// Re-export the sacred hierarchy
pub use monadic::*;
pub use dyadic::*;
pub use triadic::*;
pub use tetradic::*;
pub use pentadic::*;
pub use fact_store::*;
pub use hyper_aspect::*;
pub use projective_kernel::*;
pub use absolute_concept::*;

/// The sacred progression of HyperPropertyStore dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HyperDimension {
    Monadic,   // n=1: Universal building block
    Dyadic,    // n=2: Binary relationships
    Triadic,   // n=3: Graph structure  
    Tetradic,  // n=4: Temporal graphs
    Pentadic,  // n=5: Multi-dimensional ML graphs
}

impl HyperDimension {
    /// Get the mathematical dimension (n)
    pub fn dimension(&self) -> usize {
        match self {
            HyperDimension::Monadic => 1,
            HyperDimension::Dyadic => 2,
            HyperDimension::Triadic => 3,
            HyperDimension::Tetradic => 4,
            HyperDimension::Pentadic => 5,
        }
    }

    /// Get the theoretical complexity class
    pub fn complexity_class(&self) -> &'static str {
        match self {
            HyperDimension::Monadic => "O(1) - Direct access",
            HyperDimension::Dyadic => "O(log N) - Binary tree",
            HyperDimension::Triadic => "O(log² N) - Hierarchical",
            HyperDimension::Tetradic => "O(log³ N) - Temporal",
            HyperDimension::Pentadic => "O(log⁴ N) - Multi-dimensional",
        }
    }

    /// Get the sacred name
    pub fn sacred_name(&self) -> &'static str {
        match self {
            HyperDimension::Monadic => "The Universal Atom",
            HyperDimension::Dyadic => "The Binary Dance",
            HyperDimension::Triadic => "The Trinity",
            HyperDimension::Tetradic => "The Fourfold",
            HyperDimension::Pentadic => "The Quintessence",
        }
    }
}

/// The sacred progression iterator
pub struct HyperProgression {
    current: HyperDimension,
}

impl Default for HyperProgression {
    fn default() -> Self {
        Self::new()
    }
}

impl HyperProgression {
    pub fn new() -> Self {
        Self {
            current: HyperDimension::Monadic,
        }
    }

    pub fn next(&mut self) -> Option<HyperDimension> {
        match self.current {
            HyperDimension::Monadic => {
                self.current = HyperDimension::Dyadic;
                Some(HyperDimension::Dyadic)
            }
            HyperDimension::Dyadic => {
                self.current = HyperDimension::Triadic;
                Some(HyperDimension::Triadic)
            }
            HyperDimension::Triadic => {
                self.current = HyperDimension::Tetradic;
                Some(HyperDimension::Tetradic)
            }
            HyperDimension::Tetradic => {
                self.current = HyperDimension::Pentadic;
                Some(HyperDimension::Pentadic)
            }
            HyperDimension::Pentadic => None,
        }
    }
}

impl Iterator for HyperProgression {
    type Item = HyperDimension;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sacred_progression() {
        let mut progression = HyperProgression::new();
        
        assert_eq!(progression.next(), Some(HyperDimension::Dyadic));
        assert_eq!(progression.next(), Some(HyperDimension::Triadic));
        assert_eq!(progression.next(), Some(HyperDimension::Tetradic));
        assert_eq!(progression.next(), Some(HyperDimension::Pentadic));
        assert_eq!(progression.next(), None);
    }

    #[test]
    fn sacred_dimensions() {
        assert_eq!(HyperDimension::Monadic.dimension(), 1);
        assert_eq!(HyperDimension::Dyadic.dimension(), 2);
        assert_eq!(HyperDimension::Triadic.dimension(), 3);
        assert_eq!(HyperDimension::Tetradic.dimension(), 4);
        assert_eq!(HyperDimension::Pentadic.dimension(), 5);
    }

    #[test]
    fn sacred_names() {
        assert_eq!(HyperDimension::Monadic.sacred_name(), "The Universal Atom");
        assert_eq!(HyperDimension::Dyadic.sacred_name(), "The Binary Dance");
        assert_eq!(HyperDimension::Triadic.sacred_name(), "The Trinity");
        assert_eq!(HyperDimension::Tetradic.sacred_name(), "The Fourfold");
        assert_eq!(HyperDimension::Pentadic.sacred_name(), "The Quintessence");
    }
}
