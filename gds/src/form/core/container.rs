//! Form Container - Container Management
//!
//! Container manages **FormShapes** and their projection into concrete implementations.
//! It's the **Container-Level** configuration management that handles the **Pure FormShapes**.
//!
//! ## Architecture
//!
//! The Container manages the **triadic cycle**:
//! - **Membership** - What belongs (X | Y)
//! - **Consequence** - What follows (X → Y)
//! - **Inherence** - What forms (X & Y)
//! - **Loop** - The cycle continues

use std::collections::HashMap;
use super::shape::*;

/// Container - Manages FormShapes and their projection
///
/// This is the **Container-Level** configuration management that handles
/// the **Pure FormShapes** and their projection into concrete implementations.
#[derive(Debug)]
pub struct Container {
    /// The name of this container
    pub name: String,
    /// The FormShapes managed by this container
    pub form_shapes: Vec<FormShape>,
    /// The triadic cycles for each FormShape
    pub cycles: HashMap<String, TriadicCycle>,
}

impl Container {
    /// Create a new Container
    pub fn new(name: String) -> Self {
        Self {
            name,
            form_shapes: Vec::new(),
            cycles: HashMap::new(),
        }
    }

    /// Add a FormShape to this container
    pub fn add_form_shape(&mut self, form_shape: FormShape) {
        let cycle = form_shape.cycle();
        let shape_name = format!("FormShape_{}", self.form_shapes.len());
        self.cycles.insert(shape_name.clone(), cycle);
        self.form_shapes.push(form_shape);
    }

    /// Get the number of FormShapes in this container
    pub fn form_shape_count(&self) -> usize {
        self.form_shapes.len()
    }

    /// Find a FormShape by index
    pub fn find_form_shape(&self, index: usize) -> Option<&FormShape> {
        self.form_shapes.get(index)
    }

    /// Get the triadic cycle for a FormShape
    pub fn get_cycle(&self, index: usize) -> Option<&TriadicCycle> {
        let shape_name = format!("FormShape_{}", index);
        self.cycles.get(&shape_name)
    }

    /// Project all FormShapes into their concrete implementations
    pub fn project_all(&self) -> Result<Vec<String>, String> {
        let mut projections = Vec::new();
        
        for (index, form_shape) in self.form_shapes.iter().enumerate() {
            let projection = self.project_form_shape(form_shape, index)?;
            projections.push(projection);
        }
        
        Ok(projections)
    }

    /// Project a single FormShape into its concrete implementation
    fn project_form_shape(&self, _form_shape: &FormShape, index: usize) -> Result<String, String> {
        let cycle = self.get_cycle(index)
            .ok_or_else(|| "Cycle not found".to_string())?;
        
        // Generate concrete implementation using the triadic cycle
        let mut projection = String::new();
        
        // Membership (X | Y) - What belongs
        projection.push_str(&format!("// Membership (X | Y): What belongs\n"));
        projection.push_str(&format!("// Required fields: {:?}\n", cycle.membership.required_fields));
        projection.push_str(&format!("// Optional fields: {:?}\n", cycle.membership.optional_fields));
        
        // Consequence (X → Y) - What follows
        projection.push_str(&format!("// Consequence (X → Y): What follows\n"));
        projection.push_str(&format!("// Dependencies: {:?}\n", cycle.consequence.dependencies));
        projection.push_str(&format!("// Execution order: {:?}\n", cycle.consequence.execution_order));
        projection.push_str(&format!("// Runtime strategy: {}\n", cycle.consequence.runtime_strategy));
        
        // Inherence (X & Y) - What forms
        projection.push_str(&format!("// Inherence (X & Y): What forms\n"));
        projection.push_str(&format!("// Generated code: {:?}\n", cycle.inherence.generated_code));
        projection.push_str(&format!("// Patterns: {:?}\n", cycle.inherence.patterns));
        projection.push_str(&format!("// Descriptors: {:?}\n", cycle.inherence.descriptors));
        
        Ok(projection)
    }

    /// Execute the triadic cycle for all FormShapes
    pub fn execute_cycle(&self) -> Result<Vec<String>, String> {
        let mut results = Vec::new();
        
        for (index, _form_shape) in self.form_shapes.iter().enumerate() {
            let cycle = self.get_cycle(index)
                .ok_or_else(|| "Cycle not found".to_string())?;
            
            // Execute the triadic cycle
            let result = self.execute_triadic_cycle(cycle)?;
            results.push(result);
        }
        
        Ok(results)
    }

    /// Execute a single triadic cycle
    fn execute_triadic_cycle(&self, cycle: &TriadicCycle) -> Result<String, String> {
        let mut result = String::new();
        
        // Execute Membership (X | Y)
        result.push_str(&format!("Executing Membership (X | Y): {:?}\n", cycle.membership.required_fields));
        
        // Execute Consequence (X → Y)
        result.push_str(&format!("Executing Consequence (X → Y): {:?}\n", cycle.consequence.execution_order));
        
        // Execute Inherence (X & Y)
        result.push_str(&format!("Executing Inherence (X & Y): {:?}\n", cycle.inherence.generated_code));
        
        // Loop continues
        result.push_str("Loop continues...\n");
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_creation() {
        let container = Container::new("test_container".to_string());
        assert_eq!(container.name, "test_container");
        assert_eq!(container.form_shape_count(), 0);
    }

    #[test]
    fn test_container_add_form_shape() {
        let mut container = Container::new("test_container".to_string());
        
        let shape = Shape::new(
            vec!["id".to_string()],
            vec![],
            HashMap::new(),
            HashMap::new(),
        );

        let context = Context::new(
            vec![],
            vec![],
            "strategy".to_string(),
            vec![],
        );

        let morph = Morph::new(
            vec![],
            vec![],
            vec![],
            vec![],
        );

        let form_shape = FormShape::new(shape, context, morph);
        container.add_form_shape(form_shape);
        
        assert_eq!(container.form_shape_count(), 1);
        assert!(container.get_cycle(0).is_some());
    }

    #[test]
    fn test_container_project_all() {
        let mut container = Container::new("test_container".to_string());
        
        let shape = Shape::new(
            vec!["id".to_string()],
            vec![],
            HashMap::new(),
            HashMap::new(),
        );

        let context = Context::new(
            vec!["dep1".to_string()],
            vec!["step1".to_string()],
            "strategy".to_string(),
            vec![],
        );

        let morph = Morph::new(
            vec!["code1".to_string()],
            vec![],
            vec![],
            vec![],
        );

        let form_shape = FormShape::new(shape, context, morph);
        container.add_form_shape(form_shape);
        
        let projections = container.project_all().unwrap();
        assert_eq!(projections.len(), 1);
        assert!(projections[0].contains("Membership (X | Y)"));
        assert!(projections[0].contains("Consequence (X → Y)"));
        assert!(projections[0].contains("Inherence (X & Y)"));
    }

    #[test]
    fn test_container_execute_cycle() {
        let mut container = Container::new("test_container".to_string());
        
        let shape = Shape::new(
            vec!["id".to_string()],
            vec![],
            HashMap::new(),
            HashMap::new(),
        );

        let context = Context::new(
            vec![],
            vec!["step1".to_string()],
            "strategy".to_string(),
            vec![],
        );

        let morph = Morph::new(
            vec!["code1".to_string()],
            vec![],
            vec![],
            vec![],
        );

        let form_shape = FormShape::new(shape, context, morph);
        container.add_form_shape(form_shape);
        
        let results = container.execute_cycle().unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("Executing Membership"));
        assert!(results[0].contains("Executing Consequence"));
        assert!(results[0].contains("Executing Inherence"));
        assert!(results[0].contains("Loop continues"));
    }
}
