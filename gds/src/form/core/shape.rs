//! Form Shape - Pure Form Appearance
//!
//! FormShape is the **root appearance** of Form in our UserLand Transactional Form Processor.
//! It embodies the **triadic structure** that makes everything **Pure**.
//!
//! ## The Triadic Structure
//!
//! ```
//! FormShape {
//!     shape: Shape,      // Pure form appearance
//!     context: Context,  // Transactional environment
//!     morph: Morph,      // Organic Unity of Shape + Context
//! }
//! ```
//!
//! ## The Three Fundamental Relations
//!
//! These are the **ONLY three relations** we recognize in Semantic Webs:
//! - **X | Y** (Disjunctive) - What belongs? (Membership)
//! - **X → Y** (Implicative) - What follows? (Consequence)
//! - **X & Y** (Conjunctive) - What forms? (Inherence)

use std::collections::HashMap;

/// FormShape - The root appearance of Form
///
/// This is the **Pure Container** that embodies the triadic structure.
/// Everything in our designs must be **Triadic** to be **Pure**.
#[derive(Debug, Clone)]
pub struct FormShape {
    /// The pure form appearance
    pub shape: Shape,
    /// The transactional environment
    pub context: Context,
    /// The organic unity of Shape + Context
    pub morph: Morph,
}

impl FormShape {
    /// Create a new FormShape with the triadic structure
    pub fn new(shape: Shape, context: Context, morph: Morph) -> Self {
        Self { shape, context, morph }
    }

    /// Extract what belongs (X | Y) - Membership
    pub fn membership(&self) -> FieldMembership {
        FieldMembership {
            required_fields: self.shape.required_fields.clone(),
            optional_fields: self.shape.optional_fields.clone(),
            type_constraints: self.shape.type_constraints.clone(),
            validation_rules: self.shape.validation_rules.clone(),
        }
    }

    /// Derive what follows (X → Y) - Consequence
    pub fn consequence(&self) -> ExecutionConsequence {
        ExecutionConsequence {
            dependencies: self.context.dependencies.clone(),
            execution_order: self.context.execution_order.clone(),
            runtime_strategy: self.context.runtime_strategy.clone(),
            conditions: self.context.conditions.clone(),
        }
    }

    /// Recognize what forms (X & Y) - Inherence
    pub fn inherence(&self) -> CodeGenerationInherence {
        CodeGenerationInherence {
            generated_code: self.morph.generated_code.clone(),
            patterns: self.morph.patterns.clone(),
            descriptors: self.morph.descriptors.clone(),
            transformations: self.morph.transformations.clone(),
        }
    }

    /// The triadic cycle - Membership → Consequence → Inherence → Loop
    pub fn cycle(&self) -> TriadicCycle {
        TriadicCycle {
            membership: self.membership(),
            consequence: self.consequence(),
            inherence: self.inherence(),
        }
    }
}

/// Shape - Pure form appearance
///
/// This represents the **pure form** that appears in our UserLand Transactional Form Processor.
#[derive(Debug, Clone)]
pub struct Shape {
    /// Required fields that must be present
    pub required_fields: Vec<String>,
    /// Optional fields that may be present
    pub optional_fields: Vec<String>,
    /// Type constraints for each field
    pub type_constraints: HashMap<String, String>,
    /// Validation rules for each field
    pub validation_rules: HashMap<String, String>,
}

impl Shape {
    /// Create a new Shape
    pub fn new(
        required_fields: Vec<String>,
        optional_fields: Vec<String>,
        type_constraints: HashMap<String, String>,
        validation_rules: HashMap<String, String>,
    ) -> Self {
        Self {
            required_fields,
            optional_fields,
            type_constraints,
            validation_rules,
        }
    }
}

/// Context - Transactional environment
///
/// This represents the **transactional environment** in which the Form operates.
#[derive(Debug, Clone)]
pub struct Context {
    /// Dependencies that must be resolved first
    pub dependencies: Vec<String>,
    /// Execution order for this step
    pub execution_order: Vec<String>,
    /// Runtime strategy to apply
    pub runtime_strategy: String,
    /// Conditions that must be met
    pub conditions: Vec<String>,
}

impl Context {
    /// Create a new Context
    pub fn new(
        dependencies: Vec<String>,
        execution_order: Vec<String>,
        runtime_strategy: String,
        conditions: Vec<String>,
    ) -> Self {
        Self {
            dependencies,
            execution_order,
            runtime_strategy,
            conditions,
        }
    }
}

/// Morph - Organic Unity of Shape + Context
///
/// This represents the **organic unity** of Shape and Context.
/// It's the **synthesis** that forms from the **thesis** (Shape) and **antithesis** (Context).
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
        }
    }
}

/// FieldMembership - What belongs (X | Y)
///
/// This represents the **disjunctive relation** - what belongs to this form.
#[derive(Debug, Clone)]
pub struct FieldMembership {
    /// Required fields that must be present
    pub required_fields: Vec<String>,
    /// Optional fields that may be present
    pub optional_fields: Vec<String>,
    /// Type constraints for each field
    pub type_constraints: HashMap<String, String>,
    /// Validation rules for each field
    pub validation_rules: HashMap<String, String>,
}

/// ExecutionConsequence - What follows (X → Y)
///
/// This represents the **implicative relation** - what follows from this form.
#[derive(Debug, Clone)]
pub struct ExecutionConsequence {
    /// Dependencies that must be resolved first
    pub dependencies: Vec<String>,
    /// Execution order for this step
    pub execution_order: Vec<String>,
    /// Runtime strategy to apply
    pub runtime_strategy: String,
    /// Conditions that must be met
    pub conditions: Vec<String>,
}

/// CodeGenerationInherence - What forms (X & Y)
///
/// This represents the **conjunctive relation** - what forms inhere in this form.
#[derive(Debug, Clone)]
pub struct CodeGenerationInherence {
    /// Generated code structures
    pub generated_code: Vec<String>,
    /// Recognized patterns
    pub patterns: Vec<String>,
    /// Synthesized descriptors
    pub descriptors: Vec<String>,
    /// Transformations applied
    pub transformations: Vec<String>,
}

/// TriadicCycle - The complete cycle
///
/// This represents the **complete triadic cycle**:
/// Membership → Consequence → Inherence → Loop
#[derive(Debug, Clone)]
pub struct TriadicCycle {
    /// What belongs (X | Y)
    pub membership: FieldMembership,
    /// What follows (X → Y)
    pub consequence: ExecutionConsequence,
    /// What forms (X & Y)
    pub inherence: CodeGenerationInherence,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_shape_creation() {
        let shape = Shape::new(
            vec!["id".to_string()],
            vec!["name".to_string()],
            HashMap::new(),
            HashMap::new(),
        );

        let context = Context::new(
            vec!["dependency1".to_string()],
            vec!["step1".to_string(), "step2".to_string()],
            "test_strategy".to_string(),
            vec!["condition1".to_string()],
        );

        let morph = Morph::new(
            vec!["generated_code".to_string()],
            vec!["pattern1".to_string()],
            vec!["descriptor1".to_string()],
            vec!["transform1".to_string()],
        );

        let form_shape = FormShape::new(shape, context, morph);
        
        // Test the triadic cycle
        let cycle = form_shape.cycle();
        assert_eq!(cycle.membership.required_fields.len(), 1);
        assert_eq!(cycle.consequence.dependencies.len(), 1);
        assert_eq!(cycle.inherence.generated_code.len(), 1);
    }

    #[test]
    fn test_triadic_relations() {
        let shape = Shape::new(
            vec!["field1".to_string()],
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
        
        // Test X | Y (Membership)
        let membership = form_shape.membership();
        assert_eq!(membership.required_fields, vec!["field1"]);
        
        // Test X → Y (Consequence)
        let consequence = form_shape.consequence();
        assert_eq!(consequence.runtime_strategy, "strategy");
        
        // Test X & Y (Inherence)
        let inherence = form_shape.inherence();
        assert_eq!(inherence.generated_code.len(), 0);
    }
}
