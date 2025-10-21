//! Form Morph - Organic Unity
//!
//! Morph represents the **Organic Unity** of Shape and Context.
//! It's the **synthesis** that forms from the **thesis** (Shape) and **antithesis** (Context).
//!
//! ## The Organic Unity
//!
//! ```
//! Morph = Organic Unity of Shape + Context
//!        = Synthesis of Thesis + Antithesis
//!        = X & Y (Conjunctive relation)
//! ```
//!
//! This is where **Philosophic Logic** goes where **Formal Logic** does not.
//! They lack the idea of **Organic Unity** as **Thesis-Antithesis-Synthesis**.

use std::collections::HashMap;
use super::shape::*;

/// Morph - Organic Unity of Shape + Context
///
/// This represents the **organic unity** that forms from the synthesis
/// of Shape (thesis) and Context (antithesis).
#[derive(Debug, Clone)]
pub struct Morph {
    /// Generated code structures
    pub generated_code: Vec<String>,
    /// Recognized patterns
    pub patterns: Vec<String>,
    /// Synthesized descriptors
    pub descriptors: Vec<String>,
    /// Transformations applied
    pub transformations: Vec<String>,
    /// The organic unity metadata
    pub unity_metadata: HashMap<String, String>,
}

impl Morph {
    /// Create a new Morph
    pub fn new(
        generated_code: Vec<String>,
        patterns: Vec<String>,
        descriptors: Vec<String>,
        transformations: Vec<String>,
    ) -> Self {
        Self {
            generated_code,
            patterns,
            descriptors,
            transformations,
            unity_metadata: HashMap::new(),
        }
    }

    /// Create a Morph from Shape and Context (Organic Unity)
    pub fn from_shape_and_context(shape: &Shape, context: &Context) -> Self {
        let mut generated_code = Vec::new();
        let mut patterns = Vec::new();
        let mut descriptors = Vec::new();
        let mut transformations = Vec::new();
        let mut unity_metadata = HashMap::new();

        // Synthesis: Combine Shape and Context into Organic Unity
        generated_code.push(format!("// Generated from Shape: {:?}", shape.required_fields));
        generated_code.push(format!("// Generated from Context: {}", context.runtime_strategy));
        
        patterns.push("triadic_pattern".to_string());
        patterns.push("organic_unity_pattern".to_string());
        
        descriptors.push("synthesized_descriptor".to_string());
        
        transformations.push("shape_to_morph".to_string());
        transformations.push("context_to_morph".to_string());
        
        unity_metadata.insert("thesis".to_string(), "shape".to_string());
        unity_metadata.insert("antithesis".to_string(), "context".to_string());
        unity_metadata.insert("synthesis".to_string(), "morph".to_string());

        Self {
            generated_code,
            patterns,
            descriptors,
            transformations,
            unity_metadata,
        }
    }

    /// Get the organic unity metadata
    pub fn get_unity_metadata(&self) -> &HashMap<String, String> {
        &self.unity_metadata
    }

    /// Check if this Morph represents organic unity
    pub fn is_organic_unity(&self) -> bool {
        self.unity_metadata.contains_key("thesis") &&
        self.unity_metadata.contains_key("antithesis") &&
        self.unity_metadata.contains_key("synthesis")
    }

    /// Get the thesis (Shape) from the organic unity
    pub fn get_thesis(&self) -> Option<&String> {
        self.unity_metadata.get("thesis")
    }

    /// Get the antithesis (Context) from the organic unity
    pub fn get_antithesis(&self) -> Option<&String> {
        self.unity_metadata.get("antithesis")
    }

    /// Get the synthesis (Morph) from the organic unity
    pub fn get_synthesis(&self) -> Option<&String> {
        self.unity_metadata.get("synthesis")
    }
}

/// OrganicUnity - The complete organic unity
///
/// This represents the **complete organic unity** of Shape and Context:
/// Thesis (Shape) + Antithesis (Context) = Synthesis (Morph)
#[derive(Debug, Clone)]
pub struct OrganicUnity {
    /// The thesis (Shape)
    pub thesis: Shape,
    /// The antithesis (Context)
    pub antithesis: Context,
    /// The synthesis (Morph)
    pub synthesis: Morph,
}

impl OrganicUnity {
    /// Create a new OrganicUnity
    pub fn new(thesis: Shape, antithesis: Context, synthesis: Morph) -> Self {
        Self {
            thesis,
            antithesis,
            synthesis,
        }
    }

    /// Create an OrganicUnity from Shape and Context
    pub fn from_shape_and_context(shape: Shape, context: Context) -> Self {
        let morph = Morph::from_shape_and_context(&shape, &context);
        Self::new(shape, context, morph)
    }

    /// Get the complete triadic cycle
    pub fn get_triadic_cycle(&self) -> TriadicCycle {
        TriadicCycle {
            membership: FieldMembership {
                required_fields: self.thesis.required_fields.clone(),
                optional_fields: self.thesis.optional_fields.clone(),
                type_constraints: self.thesis.type_constraints.clone(),
                validation_rules: self.thesis.validation_rules.clone(),
            },
            consequence: ExecutionConsequence {
                dependencies: self.antithesis.dependencies.clone(),
                execution_order: self.antithesis.execution_order.clone(),
                runtime_strategy: self.antithesis.runtime_strategy.clone(),
                conditions: self.antithesis.conditions.clone(),
            },
            inherence: CodeGenerationInherence {
                generated_code: self.synthesis.generated_code.clone(),
                patterns: self.synthesis.patterns.clone(),
                descriptors: self.synthesis.descriptors.clone(),
                transformations: self.synthesis.transformations.clone(),
            },
        }
    }

    /// Execute the organic unity cycle
    pub fn execute_cycle(&self) -> String {
        let mut result = String::new();
        
        // Execute Thesis (Shape) - X | Y
        result.push_str(&format!("Executing Thesis (Shape): {:?}\n", self.thesis.required_fields));
        
        // Execute Antithesis (Context) - X → Y
        result.push_str(&format!("Executing Antithesis (Context): {:?}\n", self.antithesis.execution_order));
        
        // Execute Synthesis (Morph) - X & Y
        result.push_str(&format!("Executing Synthesis (Morph): {:?}\n", self.synthesis.generated_code));
        
        // Organic Unity achieved
        result.push_str("Organic Unity achieved!\n");
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morph_creation() {
        let morph = Morph::new(
            vec!["code1".to_string()],
            vec!["pattern1".to_string()],
            vec!["descriptor1".to_string()],
            vec!["transform1".to_string()],
        );
        
        assert_eq!(morph.generated_code.len(), 1);
        assert_eq!(morph.patterns.len(), 1);
        assert_eq!(morph.descriptors.len(), 1);
        assert_eq!(morph.transformations.len(), 1);
    }

    #[test]
    fn test_morph_from_shape_and_context() {
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

        let morph = Morph::from_shape_and_context(&shape, &context);
        
        assert!(morph.is_organic_unity());
        assert_eq!(morph.get_thesis(), Some(&"shape".to_string()));
        assert_eq!(morph.get_antithesis(), Some(&"context".to_string()));
        assert_eq!(morph.get_synthesis(), Some(&"morph".to_string()));
    }

    #[test]
    fn test_organic_unity_creation() {
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

        let organic_unity = OrganicUnity::from_shape_and_context(shape, context);
        
        assert!(organic_unity.synthesis.is_organic_unity());
    }

    #[test]
    fn test_organic_unity_execute_cycle() {
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

        let organic_unity = OrganicUnity::from_shape_and_context(shape, context);
        let result = organic_unity.execute_cycle();
        
        assert!(result.contains("Executing Thesis"));
        assert!(result.contains("Executing Antithesis"));
        assert!(result.contains("Executing Synthesis"));
        assert!(result.contains("Organic Unity achieved"));
    }
}
