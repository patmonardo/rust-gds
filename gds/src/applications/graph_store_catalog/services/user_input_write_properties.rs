/// Utility class for parsing user input write properties.
/// 
/// Mirrors Java UserInputWriteProperties class.
/// Handles parsing of property specifications for write operations.
pub struct UserInputWriteProperties;

impl UserInputWriteProperties {
    /// Creates a new UserInputWriteProperties (utility class).
    pub fn new() -> Self {
        Self
    }
    
    /// Parses user input into a list of property specifications.
    /// 
    /// In Java, this uses UserInputWriteProperties.parse(userInput, configurationKey).
    /// It handles both single strings and lists of strings, with optional renaming.
    pub fn parse(user_input: &str, configuration_key: &str) -> Result<Vec<PropertySpec>, String> {
        // Simple parsing implementation - in real implementation would handle lists and renaming
        let spec = PropertySpec::new(user_input.to_string(), None);
        Ok(vec![spec])
    }
    
    /// Parses user input from a list format.
    /// 
    /// In Java, this handles List<String> or List<Map<String, String>> inputs.
    pub fn parse_from_list(inputs: &[String]) -> Result<Vec<PropertySpec>, String> {
        let mut specs = Vec::new();
        for input in inputs {
            let spec = PropertySpec::new(input.clone(), None);
            specs.push(spec);
        }
        Ok(specs)
    }
}

impl Default for UserInputWriteProperties {
    fn default() -> Self {
        Self::new()
    }
}

/// Specification for a property to be written.
/// 
/// Mirrors Java UserInputWriteProperties.PropertySpec.
#[derive(Clone, Debug)]
pub struct PropertySpec {
    node_property_name: String,
    renamed_node_property: Option<String>,
}

impl PropertySpec {
    /// Creates a new PropertySpec.
    pub fn new(node_property_name: String, renamed_node_property: Option<String>) -> Self {
        Self {
            node_property_name,
            renamed_node_property,
        }
    }
    
    /// Returns the property name to write to the database.
    /// In Java, this calls writeProperty().
    pub fn write_property(&self) -> String {
        self.renamed_node_property.clone().unwrap_or_else(|| self.node_property_name.clone())
    }
    
    /// Returns the original node property name.
    /// In Java, this calls nodeProperty().
    pub fn node_property(&self) -> String {
        self.node_property_name.clone()
    }
    
    /// Returns the renamed property name if specified.
    pub fn renamed_property(&self) -> Option<String> {
        self.renamed_node_property.clone()
    }
}
