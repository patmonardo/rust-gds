//! Organic Unity - The connection between Container and Contained
//!
//! The Container+Contained - the Organic Unity that makes everything work

use crate::triadic::{Container, Contained};

/// The Organic Unity
pub struct OrganicUnity {
    container: Container,
    contained: Contained,
}

impl OrganicUnity {
    /// Create a new Organic Unity
    pub fn new(container: Container, contained: Contained) -> Self {
        Self { container, contained }
    }
    
    /// Project Pure Forms into Appearances
    pub fn project(&self) {
        // Project Container (Pure Forms) into Contained (Appearances)
    }
}
