// Arrow Projection Configuration
//
// Translation from: GraphProjectFromStoreConfig.java (199 lines)
// Design: Type-safe configuration with validation

use std::fmt;

/// Configuration for Arrow-native GraphStore projection.
///
/// This specifies what data to import and how to import it.
///
/// # Example
///
/// ```ignore
/// use gds::projection::factory::arrow::ArrowProjectionConfig;
///
/// let config = ArrowProjectionConfig::builder()
///     .node_table_name("nodes")
///     .edge_table_name("edges")
///     .concurrency(8)
///     .build()?;
/// ```
#[derive(Debug, Clone)]
pub struct ArrowProjectionConfig {
    /// Name of the node table (for multi-table sources)
    pub node_table_name: String,

    /// Name of the edge table (for multi-table sources)
    pub edge_table_name: String,

    /// Concurrency level for parallel import
    /// Default: number of CPUs
    pub concurrency: usize,

    /// Whether to validate schema before import
    /// Default: true
    pub validate_schema: bool,

    /// Whether to log progress during import
    /// Default: false
    pub log_progress: bool,

    /// Batch size for parallel processing
    /// Default: 10,000 rows per batch
    pub batch_size: usize,
}

impl ArrowProjectionConfig {
    /// Create a new configuration builder.
    pub fn builder() -> ArrowProjectionConfigBuilder {
        ArrowProjectionConfigBuilder::default()
    }

    /// Validate configuration.
    ///
    /// Checks:
    /// - Table names are not empty
    /// - Concurrency is > 0
    /// - Batch size is > 0
    pub fn validate(&self) -> Result<(), ArrowProjectionError> {
        if self.node_table_name.is_empty() {
            return Err(ArrowProjectionError::InvalidConfig(
                "node_table_name cannot be empty".to_string(),
            ));
        }

        if self.edge_table_name.is_empty() {
            return Err(ArrowProjectionError::InvalidConfig(
                "edge_table_name cannot be empty".to_string(),
            ));
        }

        if self.concurrency == 0 {
            return Err(ArrowProjectionError::InvalidConfig(
                "concurrency must be > 0".to_string(),
            ));
        }

        if self.batch_size == 0 {
            return Err(ArrowProjectionError::InvalidConfig(
                "batch_size must be > 0".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for ArrowProjectionConfig {
    fn default() -> Self {
        Self {
            node_table_name: "nodes".to_string(),
            edge_table_name: "edges".to_string(),
            concurrency: num_cpus::get(),
            validate_schema: true,
            log_progress: false,
            batch_size: 10_000,
        }
    }
}

/// Builder for ArrowProjectionConfig.
#[derive(Debug, Default)]
pub struct ArrowProjectionConfigBuilder {
    node_table_name: Option<String>,
    edge_table_name: Option<String>,
    concurrency: Option<usize>,
    validate_schema: Option<bool>,
    log_progress: Option<bool>,
    batch_size: Option<usize>,
}

impl ArrowProjectionConfigBuilder {
    /// Set the node table name.
    pub fn node_table_name(mut self, name: impl Into<String>) -> Self {
        self.node_table_name = Some(name.into());
        self
    }

    /// Set the edge table name.
    pub fn edge_table_name(mut self, name: impl Into<String>) -> Self {
        self.edge_table_name = Some(name.into());
        self
    }

    /// Set the concurrency level.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    /// Set whether to validate schema.
    pub fn validate_schema(mut self, validate: bool) -> Self {
        self.validate_schema = Some(validate);
        self
    }

    /// Set whether to log progress.
    pub fn log_progress(mut self, log: bool) -> Self {
        self.log_progress = Some(log);
        self
    }

    /// Set the batch size.
    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = Some(size);
        self
    }

    /// Build the configuration.
    ///
    /// Uses defaults for unset fields, then validates.
    pub fn build(self) -> Result<ArrowProjectionConfig, ArrowProjectionError> {
        let config = ArrowProjectionConfig {
            node_table_name: self.node_table_name.unwrap_or_else(|| "nodes".to_string()),
            edge_table_name: self.edge_table_name.unwrap_or_else(|| "edges".to_string()),
            concurrency: self.concurrency.unwrap_or_else(num_cpus::get),
            validate_schema: self.validate_schema.unwrap_or(true),
            log_progress: self.log_progress.unwrap_or(false),
            batch_size: self.batch_size.unwrap_or(10_000),
        };

        config.validate()?;
        Ok(config)
    }
}

/// Error type for Arrow projection operations.
#[derive(Debug)]
pub enum ArrowProjectionError {
    /// Invalid configuration
    InvalidConfig(String),

    /// Schema validation failed
    SchemaValidation(String),

    /// Arrow error (from arrow-rs crate)
    Arrow(String),

    /// Import error
    Import(String),

    /// Generic error
    Other(String),
}

impl fmt::Display for ArrowProjectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            Self::SchemaValidation(msg) => write!(f, "Schema validation failed: {}", msg),
            Self::Arrow(msg) => write!(f, "Arrow error: {}", msg),
            Self::Import(msg) => write!(f, "Import error: {}", msg),
            Self::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ArrowProjectionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ArrowProjectionConfig::default();
        assert_eq!(config.node_table_name, "nodes");
        assert_eq!(config.edge_table_name, "edges");
        assert!(config.concurrency > 0);
        assert!(config.validate_schema);
        assert!(!config.log_progress);
        assert_eq!(config.batch_size, 10_000);
    }

    #[test]
    fn test_default_config_validates() {
        let config = ArrowProjectionConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_builder_with_defaults() {
        let config = ArrowProjectionConfig::builder().build().unwrap();

        assert_eq!(config.node_table_name, "nodes");
        assert_eq!(config.edge_table_name, "edges");
    }

    #[test]
    fn test_builder_with_custom_values() {
        let config = ArrowProjectionConfig::builder()
            .node_table_name("my_nodes")
            .edge_table_name("my_edges")
            .concurrency(4)
            .validate_schema(false)
            .log_progress(true)
            .batch_size(5000)
            .build()
            .unwrap();

        assert_eq!(config.node_table_name, "my_nodes");
        assert_eq!(config.edge_table_name, "my_edges");
        assert_eq!(config.concurrency, 4);
        assert!(!config.validate_schema);
        assert!(config.log_progress);
        assert_eq!(config.batch_size, 5000);
    }

    #[test]
    fn test_validation_empty_node_table() {
        let mut config = ArrowProjectionConfig::default();
        config.node_table_name = "".to_string();

        let result = config.validate();
        assert!(result.is_err());
        match result {
            Err(ArrowProjectionError::InvalidConfig(msg)) => {
                assert!(msg.contains("node_table_name"));
            }
            _ => panic!("Expected InvalidConfig error"),
        }
    }

    #[test]
    fn test_validation_empty_edge_table() {
        let mut config = ArrowProjectionConfig::default();
        config.edge_table_name = "".to_string();

        let result = config.validate();
        assert!(result.is_err());
        match result {
            Err(ArrowProjectionError::InvalidConfig(msg)) => {
                assert!(msg.contains("edge_table_name"));
            }
            _ => panic!("Expected InvalidConfig error"),
        }
    }

    #[test]
    fn test_validation_zero_concurrency() {
        let mut config = ArrowProjectionConfig::default();
        config.concurrency = 0;

        let result = config.validate();
        assert!(result.is_err());
        match result {
            Err(ArrowProjectionError::InvalidConfig(msg)) => {
                assert!(msg.contains("concurrency"));
            }
            _ => panic!("Expected InvalidConfig error"),
        }
    }

    #[test]
    fn test_validation_zero_batch_size() {
        let mut config = ArrowProjectionConfig::default();
        config.batch_size = 0;

        let result = config.validate();
        assert!(result.is_err());
        match result {
            Err(ArrowProjectionError::InvalidConfig(msg)) => {
                assert!(msg.contains("batch_size"));
            }
            _ => panic!("Expected InvalidConfig error"),
        }
    }

    #[test]
    fn test_error_display() {
        let err = ArrowProjectionError::InvalidConfig("test".to_string());
        assert!(format!("{}", err).contains("Invalid configuration"));

        let err = ArrowProjectionError::SchemaValidation("test".to_string());
        assert!(format!("{}", err).contains("Schema validation"));

        let err = ArrowProjectionError::Arrow("test".to_string());
        assert!(format!("{}", err).contains("Arrow error"));
    }
}
