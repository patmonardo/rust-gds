//! HyperSchema: Dimensional Schema System
//!
//! A dimensional schema system encoding Monadic→Dyadic→Triadic→Tetradic→Pentadic
//! following the sacred mathematical progression as schema metadata.
//!
//! # The Cognitive Moments (Self-Certification in Logic as Science)
//!
//! Each schema level IS a cognitive moment in the methodology of Logic:
//!
//! - **Monadic**: Self-Evidence (immediate presence - "X is")
//! - **Dyadic**: Self-Assertion (the claim - ground ↔ condition)
//! - **Triadic**: Self-Certainty with Objectivity (X AS Y - AS introduces mediating mark)
//! - **Tetradic**: The Path (ignorance → knowledge through self-certification)
//! - **Pentadic**: The Idea (X FROM Y - the self-certified knowing)
//!
//! These cognitive moments from ignorance to knowledge are different levels of Schema.
//! This is Self-Evident - the schema itself encodes the method.
//!
//! ```text
//! Monadic   (n=1) → Universal building block (Self-Evidence)
//! Dyadic    (n=2) → Binary relationships (Self-Assertion: ground ↔ condition)
//! Triadic   (n=3) → Graph structure (Self-Certainty: conceives X AS Y)
//! Tetradic  (n=4) → Dyadic² (The Path from Ignorance to Knowledge)
//! Pentadic  (n=5) → The Idea (The self-certified Knowing: X FROM Y)
//! ```
//!
//! # The Sacred Principles
//!
//! 1. **Sacred Numbers**: Each level has mathematical significance
//! 2. **Compositional Purity**: Each level composes the previous levels
//! 3. **Schema Integration**: HyperSchema mates with Collections via HyperAdapter
//! 4. **Naming Purity**: Identical naming schema down to variable names
//! 5. **Self-Certification**: Each level is a cognitive moment of Logic as Science

pub mod monadic;
pub mod dyadic;
pub mod triadic;
pub mod tetradic;
pub mod pentadic;

// Re-export specific types from each level to avoid ambiguous glob re-exports
pub use monadic::{MonadicSchema, MonadicSchemaId, MonadicHyperSchema, DefaultMonadicSchema};
pub use dyadic::{DyadicSchema, DyadicSchemaId, DyadicReflection, DyadicHyperSchema, DefaultDyadicSchema};
pub use triadic::{TriadicSchema, TriadicSchemaId, CenterMark, TriadicHyperSchema, DefaultTriadicSchema};
pub use tetradic::{TetradicSchema, TetradicSchemaId, TetradicHyperSchema, DefaultTetradicSchema};
pub use pentadic::{PentadicSchema, PentadicSchemaId, Certainty, TriadRef, PentadicHyperSchema, DefaultPentadicSchema};

/// Projection level for HyperSchema
/// Following the sacred mathematical progression
/// 
/// THE EXPONENT: The Metaphysics of the Named Quality
/// 
/// In Algebra, X^n is not a quantity but a Named Quality (the polynomial XXX...).
/// The exponent determines the level of determinateness, not the quantity.
/// 
/// Group Theory Notation: S₀, S₁, S₂, S₃, S₄
/// - S₀ = Monadic (n=0) → Potenz of Simple Unity (Being)
/// - S₁ = Dyadic (n=1) → Potenz of Relationship (ground ↔ condition)
/// - S₂ = Triadic (n=2) → Potenz of Conceiving (X AS Y - introduces Objectivity)
/// - S₃ = Tetradic (n=3) → Potenz of Learning (the path from ignorance to knowledge)
/// - S₄ = Pentadic (n=4) → Potenz of Knowledge (the Idea - X FROM Y)
/// 
/// Each is a Named Quality (Potenz) that determines what the schema IS, not how much there is.
/// 
/// This follows Schelling and Hegel's metaphysics of the Exponent:
/// - The Exponent is a level of determinateness (Potenz in Schelling)
/// - The Coefficient is attached to a Quality
/// - The Exponent determines the Named Quality of the structure
/// - In Schelling's Naturphilosophie: each Potenz is a self-positing stage
/// - Nature's exponents are not applied mathematics, but Nature's own self-organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HyperSchemaLevel {
    Monadic = 1,  // S₁: Named Quality - Simple Unity (Being)
    Dyadic = 2,   // S₂: Named Quality - Reflective Mark (ground ↔ condition)
    Triadic = 3,  // S₃: Named Quality - Determinate Mark (X AS Y - Objectivity)
    Tetradic = 4, // S₄: Named Quality - ML Pipeline (ignorance → knowledge)
    Pentadic = 5, // S₅: Named Quality - Absolute Idea (X FROM Y - Knowing)
}

/// Base HyperSchema trait with dimensional awareness
pub trait HyperSchema: Send + Sync {
    /// Get the projection level of this schema
    fn level(&self) -> HyperSchemaLevel;
    
    /// Get the value type
    fn value_type(&self) -> crate::types::ValueType;
    
    /// Get the key
    fn key(&self) -> &str;
}

