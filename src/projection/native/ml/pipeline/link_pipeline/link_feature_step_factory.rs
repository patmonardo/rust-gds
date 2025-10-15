// LinkFeatureStepFactory - Factory enum for creating LinkFeatureStep instances

use super::linkfunctions::{
    CosineFeatureStep, HadamardFeatureStep, L2FeatureStep, LinkFeatureStepConfiguration,
    SameCategoryStep,
};
use super::LinkFeatureStep;
use serde_json::Value;
use std::fmt;

/// Factory enum for creating LinkFeatureStep instances from configuration.
///
/// # CAR:CDR - The Atom of Science! 🎯
///
/// This is the **CAR** - the atomic factory that gives us the Given!
/// - **CAR**: The factory case (HADAMARD, COSINE, L2, SAME_CATEGORY)
/// - **CDR**: The reconstruction (create() method builds the instance)
///
/// **Science is Given and Reconstruction of Given**:
/// - Given: The enum variant identifies the type
/// - Reconstruction: The create() method reconstructs the instance
///
/// # Pattern: Enum Factory
///
/// Java uses enum with abstract method:
/// ```java
/// enum LinkFeatureStepFactory {
///     HADAMARD { LinkFeatureStep create(Config c) { ... } },
///     COSINE { LinkFeatureStep create(Config c) { ... } },
///     ...
/// }
/// ```
///
/// Rust uses enum with match:
/// ```rust
/// enum LinkFeatureStepFactory {
///     Hadamard, Cosine, L2, SameCategory
/// }
/// impl LinkFeatureStepFactory {
///     fn create(&self, config) -> LinkFeatureStep { match self { ... } }
/// }
/// ```
///
/// # Usage
///
/// ```text
/// // From string name
/// let factory = LinkFeatureStepFactory::parse("HADAMARD")?;
/// let step = factory.create(config);
///
/// // Direct creation
/// let step = LinkFeatureStepFactory::create("COSINE", config)?;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkFeatureStepFactory {
    /// Hadamard product (element-wise multiplication)
    Hadamard,
    /// Cosine similarity (angular distance)
    Cosine,
    /// L2 distance (Euclidean distance squared)
    L2,
    /// Same category (categorical equality)
    SameCategory,
}

impl LinkFeatureStepFactory {
    /// All valid factory values as strings (uppercase).
    pub const VALUES: &'static [&'static str] = &["HADAMARD", "COSINE", "L2", "SAME_CATEGORY"];

    /// Parse factory from string (case-insensitive).
    ///
    /// # The CAR - The Given Atom!
    ///
    /// This is the **Given** - we receive a string and extract the atomic type.
    ///
    /// # Arguments
    ///
    /// * `input` - Factory name (case-insensitive)
    ///
    /// # Returns
    ///
    /// Factory variant or error if unknown.
    pub fn parse(input: &str) -> Result<Self, String> {
        let input_upper = input.to_uppercase();

        match input_upper.as_str() {
            "HADAMARD" => Ok(Self::Hadamard),
            "COSINE" => Ok(Self::Cosine),
            "L2" => Ok(Self::L2),
            "SAME_CATEGORY" | "SAMECATEGORY" => Ok(Self::SameCategory),
            _ => Err(format!(
                "LinkFeatureStep `{}` is not supported. Must be one of: {}.",
                input,
                Self::VALUES.join(", ")
            )),
        }
    }

    /// Create a LinkFeatureStep instance from this factory.
    ///
    /// # The CDR - The Reconstruction!
    ///
    /// This is the **Reconstruction of Given** - we take the atomic type (CAR)
    /// and reconstruct the full instance (CDR).
    ///
    /// **Science = CAR + CDR**:
    /// - CAR: Factory variant (atomic Given)
    /// - CDR: create() method (Reconstruction)
    /// - Science: The unity of Given and Reconstruction!
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration with nodeProperties
    ///
    /// # Returns
    ///
    /// Boxed LinkFeatureStep instance.
    pub fn create(&self, node_properties: Vec<String>) -> Box<dyn LinkFeatureStep> {
        match self {
            Self::Hadamard => Box::new(HadamardFeatureStep::new(node_properties)),
            Self::Cosine => Box::new(CosineFeatureStep::new(node_properties)),
            Self::L2 => Box::new(L2FeatureStep::new(node_properties)),
            Self::SameCategory => Box::new(SameCategoryStep::new(node_properties)),
        }
    }

    /// Create a LinkFeatureStep from task name and configuration.
    ///
    /// # The Complete Science!
    ///
    /// This is the **full CAR:CDR** - parse (Given) + create (Reconstruction)!
    ///
    /// # Arguments
    ///
    /// * `task_name` - Factory name string
    /// * `config` - Configuration value with nodeProperties
    ///
    /// # Returns
    ///
    /// Boxed LinkFeatureStep or error.
    pub fn create_from_name(
        task_name: &str,
        node_properties: Vec<String>,
    ) -> Result<Box<dyn LinkFeatureStep>, String> {
        // Parse factory (CAR - the Given)
        let factory = Self::parse(task_name)?;

        // Create instance (CDR - the Reconstruction)
        Ok(factory.create(node_properties))
    }

    /// Create from name with JSON config validation.
    pub fn create_from_config(
        task_name: &str,
        config: &Value,
    ) -> Result<Box<dyn LinkFeatureStep>, String> {
        // Validate and extract node properties using the trait's validation
        // TODO: Once we have a concrete config struct, use that here
        // For now, directly extract array
        let props_array = config
            .as_array()
            .ok_or_else(|| "Configuration must be an array of property names".to_string())?;

        let node_properties: Result<Vec<String>, String> = props_array
            .iter()
            .map(|v| {
                v.as_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| "Property name must be a string".to_string())
            })
            .collect();

        Self::create_from_name(task_name, node_properties?)
    }

    /// Returns the name of this factory variant.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Hadamard => "HADAMARD",
            Self::Cosine => "COSINE",
            Self::L2 => "L2",
            Self::SameCategory => "SAME_CATEGORY",
        }
    }
}

impl fmt::Display for LinkFeatureStepFactory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_hadamard() {
        assert_eq!(
            LinkFeatureStepFactory::parse("HADAMARD").unwrap(),
            LinkFeatureStepFactory::Hadamard
        );
        assert_eq!(
            LinkFeatureStepFactory::parse("hadamard").unwrap(),
            LinkFeatureStepFactory::Hadamard
        );
    }

    #[test]
    fn test_parse_cosine() {
        assert_eq!(
            LinkFeatureStepFactory::parse("COSINE").unwrap(),
            LinkFeatureStepFactory::Cosine
        );
    }

    #[test]
    fn test_parse_l2() {
        assert_eq!(
            LinkFeatureStepFactory::parse("L2").unwrap(),
            LinkFeatureStepFactory::L2
        );
        assert_eq!(
            LinkFeatureStepFactory::parse("l2").unwrap(),
            LinkFeatureStepFactory::L2
        );
    }

    #[test]
    fn test_parse_same_category() {
        assert_eq!(
            LinkFeatureStepFactory::parse("SAME_CATEGORY").unwrap(),
            LinkFeatureStepFactory::SameCategory
        );
        assert_eq!(
            LinkFeatureStepFactory::parse("same_category").unwrap(),
            LinkFeatureStepFactory::SameCategory
        );
    }

    #[test]
    fn test_parse_unknown() {
        let result = LinkFeatureStepFactory::parse("UNKNOWN");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not supported"));
    }

    #[test]
    fn test_create_hadamard() {
        let factory = LinkFeatureStepFactory::Hadamard;
        let step = factory.create(vec!["prop1".to_string()]);
        assert_eq!(step.name(), "HADAMARD");
    }

    #[test]
    fn test_create_cosine() {
        let factory = LinkFeatureStepFactory::Cosine;
        let step = factory.create(vec!["embedding".to_string()]);
        assert_eq!(step.name(), "COSINE");
    }

    #[test]
    fn test_create_l2() {
        let factory = LinkFeatureStepFactory::L2;
        let step = factory.create(vec!["features".to_string()]);
        assert_eq!(step.name(), "L2");
    }

    #[test]
    fn test_create_same_category() {
        let factory = LinkFeatureStepFactory::SameCategory;
        let step = factory.create(vec!["category".to_string()]);
        assert_eq!(step.name(), "SAME_CATEGORY");
    }

    #[test]
    fn test_create_from_name() {
        let props = vec!["prop1".to_string(), "prop2".to_string()];
        let step = LinkFeatureStepFactory::create_from_name("HADAMARD", props).unwrap();
        assert_eq!(step.name(), "HADAMARD");
        assert_eq!(step.input_node_properties().len(), 2);
    }

    #[test]
    fn test_create_from_config() {
        let config = json!(["prop1", "prop2"]);
        let step = LinkFeatureStepFactory::create_from_config("COSINE", &config).unwrap();
        assert_eq!(step.name(), "COSINE");
        assert_eq!(step.input_node_properties().len(), 2);
    }

    #[test]
    fn test_values_list() {
        assert_eq!(LinkFeatureStepFactory::VALUES.len(), 4);
        assert!(LinkFeatureStepFactory::VALUES.contains(&"HADAMARD"));
        assert!(LinkFeatureStepFactory::VALUES.contains(&"COSINE"));
        assert!(LinkFeatureStepFactory::VALUES.contains(&"L2"));
        assert!(LinkFeatureStepFactory::VALUES.contains(&"SAME_CATEGORY"));
    }

    #[test]
    fn test_car_cdr_science() {
        // CAR:CDR - Science is Given and Reconstruction of Given!

        // CAR - The Given (atomic factory type)
        let car = LinkFeatureStepFactory::parse("COSINE").unwrap();
        assert_eq!(car, LinkFeatureStepFactory::Cosine);

        // CDR - The Reconstruction (create instance)
        let cdr = car.create(vec!["embedding".to_string()]);
        assert_eq!(cdr.name(), "COSINE");

        // Science = CAR + CDR (unity of Given and Reconstruction)
        // The factory (CAR) reconstructs (CDR) the full step instance!
        // This is the COMPLETE SCIENCE! 🎯
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", LinkFeatureStepFactory::Hadamard), "HADAMARD");
        assert_eq!(format!("{}", LinkFeatureStepFactory::Cosine), "COSINE");
        assert_eq!(format!("{}", LinkFeatureStepFactory::L2), "L2");
        assert_eq!(
            format!("{}", LinkFeatureStepFactory::SameCategory),
            "SAME_CATEGORY"
        );
    }
}
