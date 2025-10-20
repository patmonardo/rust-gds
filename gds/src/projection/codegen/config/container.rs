//! Container - Container-Level Configuration Management
//!
//! This module implements the Container-Level configuration management
//! that handles the Pure FormShapes and their projection into concrete
//! implementations.

use crate::projection::codegen::config::form_shape::FormShape;

/// A Container manages Pure FormShapes and their projection
#[derive(Debug)]
pub struct Container {
    /// The name of this container
    pub name: String,
    /// The FormShapes managed by this container
    pub form_shapes: Vec<FormShape>,
}

impl Container {
    /// Create a new Container
    pub fn new(name: String) -> Self {
        Self {
            name,
            form_shapes: Vec::new(),
        }
    }

    /// Add a FormShape to this container
    pub fn add_form_shape(&mut self, form_shape: FormShape) {
        self.form_shapes.push(form_shape);
    }

    /// Get the number of FormShapes in this container
    pub fn form_shape_count(&self) -> usize {
        self.form_shapes.len()
    }

    /// Find a FormShape by name
    pub fn find_form_shape(&self, name: &str) -> Option<&FormShape> {
        self.form_shapes.iter().find(|fs| fs.name == name)
    }

    /// Project all FormShapes into their concrete implementations
    pub fn project_all(&self) -> Result<Vec<String>, String> {
        let mut projections = Vec::new();
        
        for form_shape in &self.form_shapes {
            let projection = self.project_form_shape(form_shape)?;
            projections.push(projection);
        }
        
        Ok(projections)
    }

    /// Project a single FormShape into its concrete implementation
    fn project_form_shape(&self, form_shape: &FormShape) -> Result<String, String> {
        // TODO: Implement actual projection logic
        // This would generate the actual Rust code for the FormShape
        Ok(format!("// Projected FormShape: {}", form_shape.name))
    }
}
