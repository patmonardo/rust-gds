//! HyperAspect: A Posteriori Essence (Essential Relation)
//! Part 2b of Objective Logic
//!
//! HyperAspect represents Pure Essence - Pure Relating.
//! After Facts are in Existence (A Posteriori Essence),
//! we have Essential Relation - the purely relational behavior.
//!
//! No "Store" - lifts out of Being entirely.
//! Thing-Property → Appearance as Aspect
//! Presupposing Facticity and Appearance

use super::fact_store::ProjectionLevel;

/// Relation: The fundamental structure of Essential Relation
/// 
/// Represents how two HyperAspects relate to each other.
/// This is pure relational essence - no Being, only Essence.
pub struct Relation {
    pub from: Box<dyn HyperAspect>,
    pub to: Box<dyn HyperAspect>,
    pub relation_type: RelationType,
}

/// RelationType: The type of Essential Relation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationType {
    Composition,  // One contains the other
    Opposition,   // One opposes the other
    Subsumption,  // One subsumes the other
    Sublation,    // One sublates the other
}

/// HyperAspect: A Posteriori Essence (Essential Relation)
/// 
/// Pure Essence - Pure Relating. After Facts are in Existence.
/// Thing-Property → Appearance as Aspect.
/// Presupposing Facticity and Appearance.
///
/// No "Store" - lifts out of Being entirely.
/// This is what a FactStore DOES (its essence), not what it IS.
pub trait HyperAspect: Send + Sync {
    /// Project this aspect to its behavior
    fn project(&self) -> ProjectionBehavior;
    
    /// Relate this aspect to another
    fn relate(&self, other: &dyn HyperAspect) -> Relation;
    
    /// Synthesize this aspect
    fn synthesize(&self) -> Box<dyn HyperAspect>;
    
    /// Analyze this aspect into its components
    fn analyze(&self) -> Vec<Box<dyn HyperAspect>>;
    
    /// Get the dimension of this aspect
    fn dimension(&self) -> ProjectionLevel;
}

/// ProjectionBehavior: The behavioral projection of an aspect
/// 
/// Represents how an aspect projects itself into different dimensions.
#[derive(Debug, Clone)]
pub enum ProjectionBehavior {
    Monadic(MonadicBehavior),
    Dyadic(DyadicBehavior),
    Triadic(TriadicBehavior),
    Tetradic(TetradicBehavior),
    Pentadic(PentadicBehavior),
}

/// MonadicBehavior: Simple Unity behavior
/// 
/// The aspect behaves as a simple unity - no internal structure.
#[derive(Debug, Clone)]
pub struct MonadicBehavior {
    pub unity: String,
}

impl MonadicBehavior {
    pub fn new(unity: String) -> Self {
        Self { unity }
    }
}

/// DyadicBehavior: Reflective relation behavior
/// 
/// The aspect behaves as a reflective relation - no center.
#[derive(Debug, Clone)]
pub struct DyadicBehavior {
    pub left: String,
    pub right: String,
}

impl DyadicBehavior {
    pub fn new(left: String, right: String) -> Self {
        Self { left, right }
    }
}

/// TriadicBehavior: Determinate relation behavior
/// 
/// The aspect behaves as a determinate relation - with center.
#[derive(Debug, Clone)]
pub struct TriadicBehavior {
    pub being: String,
    pub essence: String,  // The center!
    pub concept: String,
}

impl TriadicBehavior {
    pub fn new(being: String, essence: String, concept: String) -> Self {
        Self { being, essence, concept }
    }
}

/// TetradicBehavior: ML Pipeline behavior
/// 
/// The aspect behaves as an ML Pipeline with 4-4-4-4 pattern.
#[derive(Debug, Clone)]
pub struct TetradicBehavior {
    pub dyad_1: DyadicBehavior,
    pub dyad_2: DyadicBehavior,
}

impl TetradicBehavior {
    pub fn new(dyad_1: DyadicBehavior, dyad_2: DyadicBehavior) -> Self {
        Self { dyad_1, dyad_2 }
    }
}

/// PentadicBehavior: Absolute Idea behavior
/// 
/// The aspect behaves as the Absolute Idea.
#[derive(Debug, Clone)]
pub struct PentadicBehavior {
    pub tetrads: Vec<TetradicBehavior>,
}

impl PentadicBehavior {
    pub fn new(tetrads: Vec<TetradicBehavior>) -> Self {
        Self { tetrads }
    }
}

/// MonadicAspect: Simple Unity aspect
/// 
/// Pure Essence as Simple Unity.
pub struct MonadicAspect {
    behavior: MonadicBehavior,
}

impl MonadicAspect {
    pub fn new(behavior: MonadicBehavior) -> Self {
        Self { behavior }
    }
}

impl HyperAspect for MonadicAspect {
    fn project(&self) -> ProjectionBehavior {
        ProjectionBehavior::Monadic(self.behavior.clone())
    }
    
    fn relate(&self, _other: &dyn HyperAspect) -> Relation {
        Relation {
            from: Box::new(MonadicAspect::new(self.behavior.clone())),
            to: Box::new(MonadicAspect::new(self.behavior.clone())),
            relation_type: RelationType::Composition,
        }
    }
    
    fn synthesize(&self) -> Box<dyn HyperAspect> {
        Box::new(MonadicAspect::new(self.behavior.clone()))
    }
    
    fn analyze(&self) -> Vec<Box<dyn HyperAspect>> {
        vec![Box::new(MonadicAspect::new(self.behavior.clone()))]
    }
    
    fn dimension(&self) -> ProjectionLevel {
        ProjectionLevel::Monadic
    }
}

/// DyadicAspect: Reflective relation aspect
/// 
/// Pure Essence as Reflective Relation (no center).
pub struct DyadicAspect {
    behavior: DyadicBehavior,
}

impl DyadicAspect {
    pub fn new(behavior: DyadicBehavior) -> Self {
        Self { behavior }
    }
}

impl HyperAspect for DyadicAspect {
    fn project(&self) -> ProjectionBehavior {
        ProjectionBehavior::Dyadic(self.behavior.clone())
    }
    
    fn relate(&self, _other: &dyn HyperAspect) -> Relation {
        Relation {
            from: Box::new(DyadicAspect::new(self.behavior.clone())),
            to: Box::new(DyadicAspect::new(self.behavior.clone())),
            relation_type: RelationType::Composition,
        }
    }
    
    fn synthesize(&self) -> Box<dyn HyperAspect> {
        Box::new(DyadicAspect::new(self.behavior.clone()))
    }
    
    fn analyze(&self) -> Vec<Box<dyn HyperAspect>> {
        vec![Box::new(DyadicAspect::new(self.behavior.clone()))]
    }
    
    fn dimension(&self) -> ProjectionLevel {
        ProjectionLevel::Dyadic
    }
}

/// TriadicAspect: Determinate relation aspect
/// 
/// Pure Essence as Determinate Relation (with center).
pub struct TriadicAspect {
    behavior: TriadicBehavior,
}

impl TriadicAspect {
    pub fn new(behavior: TriadicBehavior) -> Self {
        Self { behavior }
    }
}

impl HyperAspect for TriadicAspect {
    fn project(&self) -> ProjectionBehavior {
        ProjectionBehavior::Triadic(self.behavior.clone())
    }
    
    fn relate(&self, _other: &dyn HyperAspect) -> Relation {
        Relation {
            from: Box::new(TriadicAspect::new(self.behavior.clone())),
            to: Box::new(TriadicAspect::new(self.behavior.clone())),
            relation_type: RelationType::Sublation,
        }
    }
    
    fn synthesize(&self) -> Box<dyn HyperAspect> {
        Box::new(TriadicAspect::new(self.behavior.clone()))
    }
    
    fn analyze(&self) -> Vec<Box<dyn HyperAspect>> {
        vec![Box::new(TriadicAspect::new(self.behavior.clone()))]
    }
    
    fn dimension(&self) -> ProjectionLevel {
        ProjectionLevel::Triadic
    }
}

/// TetradicAspect: ML Pipeline aspect
/// 
/// Pure Essence as ML Pipeline (4-4-4-4 pattern).
pub struct TetradicAspect {
    behavior: TetradicBehavior,
}

impl TetradicAspect {
    pub fn new(behavior: TetradicBehavior) -> Self {
        Self { behavior }
    }
}

impl HyperAspect for TetradicAspect {
    fn project(&self) -> ProjectionBehavior {
        ProjectionBehavior::Tetradic(self.behavior.clone())
    }
    
    fn relate(&self, _other: &dyn HyperAspect) -> Relation {
        Relation {
            from: Box::new(TetradicAspect::new(self.behavior.clone())),
            to: Box::new(TetradicAspect::new(self.behavior.clone())),
            relation_type: RelationType::Composition,
        }
    }
    
    fn synthesize(&self) -> Box<dyn HyperAspect> {
        Box::new(TetradicAspect::new(self.behavior.clone()))
    }
    
    fn analyze(&self) -> Vec<Box<dyn HyperAspect>> {
        vec![Box::new(TetradicAspect::new(self.behavior.clone()))]
    }
    
    fn dimension(&self) -> ProjectionLevel {
        ProjectionLevel::Tetradic
    }
}

/// PentadicAspect: Absolute Idea aspect
/// 
/// Pure Essence as Absolute Idea.
pub struct PentadicAspect {
    behavior: PentadicBehavior,
}

impl PentadicAspect {
    pub fn new(behavior: PentadicBehavior) -> Self {
        Self { behavior }
    }
}

impl HyperAspect for PentadicAspect {
    fn project(&self) -> ProjectionBehavior {
        ProjectionBehavior::Pentadic(self.behavior.clone())
    }
    
    fn relate(&self, _other: &dyn HyperAspect) -> Relation {
        Relation {
            from: Box::new(PentadicAspect::new(self.behavior.clone())),
            to: Box::new(PentadicAspect::new(self.behavior.clone())),
            relation_type: RelationType::Sublation,
        }
    }
    
    fn synthesize(&self) -> Box<dyn HyperAspect> {
        Box::new(PentadicAspect::new(self.behavior.clone()))
    }
    
    fn analyze(&self) -> Vec<Box<dyn HyperAspect>> {
        vec![Box::new(PentadicAspect::new(self.behavior.clone()))]
    }
    
    fn dimension(&self) -> ProjectionLevel {
        ProjectionLevel::Pentadic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monadic_aspect() {
        let behavior = MonadicBehavior::new("Unity".to_string());
        let aspect = MonadicAspect::new(behavior);
        assert_eq!(aspect.dimension(), ProjectionLevel::Monadic);
    }

    #[test]
    fn test_dyadic_aspect() {
        let behavior = DyadicBehavior::new("Left".to_string(), "Right".to_string());
        let aspect = DyadicAspect::new(behavior);
        assert_eq!(aspect.dimension(), ProjectionLevel::Dyadic);
    }

    #[test]
    fn test_triadic_aspect() {
        let behavior = TriadicBehavior::new(
            "Being".to_string(),
            "Essence".to_string(),
            "Concept".to_string(),
        );
        let aspect = TriadicAspect::new(behavior);
        assert_eq!(aspect.dimension(), ProjectionLevel::Triadic);
    }
}

