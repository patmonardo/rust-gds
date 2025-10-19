//! Empirical Forms - Everything is Empirical Forms
//!
//! The GDSL Runtime generates Empirical Forms through Triadic-Pentadic structures

/// An Empirical Form
pub trait EmpiricalForm {
    /// Generate the form
    fn generate(&self);
}

/// Computation as Empirical Form
pub struct ComputationForm;

impl EmpiricalForm for ComputationForm {
    fn generate(&self) {
        // Generate computation form
    }
}

/// Storage as Empirical Form
pub struct StorageForm;

impl EmpiricalForm for StorageForm {
    fn generate(&self) {
        // Generate storage form
    }
}
