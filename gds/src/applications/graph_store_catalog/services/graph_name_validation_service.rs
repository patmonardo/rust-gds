/// Service for validating graph names.
/// 
/// Mirrors Java GraphNameValidationService class.
/// Handles string validation logic for graph names with various validation modes.
pub struct GraphNameValidationService;

impl GraphNameValidationService {
    /// Creates a new GraphNameValidationService.
    pub fn new() -> Self {
        Self
    }
    
    /// Validates a graph name string.
    /// In Java, this calls GraphName.parse() and handles validation.
    pub fn validate(&self, graph_name: &str) -> Result<String, String> {
        if graph_name.is_empty() {
            return Err("Graph name cannot be empty".to_string());
        }
        
        if graph_name.len() > 100 {
            return Err("Graph name too long".to_string());
        }
        
        // Basic validation - in real implementation would use GraphName::parse()
        Ok(graph_name.to_string())
    }
    
    /// Validates a graph name strictly (for creation operations).
    /// In Java, this calls GraphName.parseStrictly().
    pub fn validate_strictly(&self, graph_name: &str) -> Result<String, String> {
        self.validate(graph_name)
    }
    
    /// Validates a graph name that might be null.
    /// In Java, this returns Optional<GraphName>.
    pub fn validate_possible_null(&self, graph_name: Option<&str>) -> Option<String> {
        graph_name.and_then(|name| self.validate(name).ok())
    }
    
    /// Validates a single graph name or list of graph names.
    /// In Java, this handles both String and List<String> inputs.
    pub fn validate_single_or_list(&self, input: &str) -> Result<Vec<String>, String> {
        // Simple implementation - in real implementation would parse lists
        Ok(vec![self.validate(input)?])
    }
}

impl Default for GraphNameValidationService {
    fn default() -> Self {
        Self::new()
    }
}
