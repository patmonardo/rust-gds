//! Projection System - The core of the GDSL Runtime
//!
//! This module implements the Projection System that generates Empirical Forms
//! through the Container-Contained Organic Unity.

use crate::triadic::{Container, Contained};
use crate::organic_unity::OrganicUnity;

/// The ProjectionFactory - generates forms from names
///
/// This is the core of the Projection System that maps names to forms,
/// implementing the Nama-Rupa Unity (Name-Form Unity).
pub struct ProjectionFactory {
    container: Container,
    contained: Contained,
}

impl ProjectionFactory {
    /// Create a new ProjectionFactory
    pub fn new() -> Self {
        Self {
            container: Container,
            contained: Contained,
        }
    }
    
    /// Project a name into a form
    ///
    /// This implements the Nama-Rupa Unity where names are projected
    /// into forms through the Container-Contained Organic Unity.
    pub fn project(&self, _name: &str) -> OrganicUnity {
        // Project the name into a form through the Organic Unity
        OrganicUnity::new(self.container.clone(), self.contained.clone())
    }
}

/// The Eval/Form system - Pure Form Processor
///
/// This implements the Pure Form Processor that is the heart of the
/// Projection System's Eval/Form system.
pub struct EvalForm {
    processor: PureFormProcessor,
}

impl EvalForm {
    /// Create a new EvalForm
    pub fn new() -> Self {
        Self {
            processor: PureFormProcessor::new(),
        }
    }
    
    /// Evaluate a form
    pub fn eval(&self, form: &OrganicUnity) {
        // Evaluate the form through the Pure Form Processor
        self.processor.process(form);
    }
}

/// The Pure Form Processor
///
/// This is the Container that processes Pure Forms and projects them
/// into Appearances (Contained) through the Organic Unity.
pub struct PureFormProcessor {
    // The processor state
}

impl PureFormProcessor {
    /// Create a new Pure Form Processor
    pub fn new() -> Self {
        Self {}
    }
    
    /// Process a form through the Organic Unity
    pub fn process(&self, form: &OrganicUnity) {
        // Process the form through the Container-Contained Organic Unity
        form.project();
    }
}
