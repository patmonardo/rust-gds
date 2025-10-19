//! I/O configuration types for import and export operations

use super::base_types::{Config, WriteConfig};
use super::validation::{ConfigError, ConfigValidation};
use crate::projection::RelationshipType;

/// File exporter configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileExporterConfig {
    pub export_path: String,
    pub write_concurrency: usize,
    pub batch_size: usize,
    pub default_relationship_type: RelationshipType,
    pub compression_enabled: bool,
    pub compression_level: u32,
    pub include_metadata: bool,
}

impl Default for FileExporterConfig {
    fn default() -> Self {
        Self {
            export_path: String::from("/tmp/graphstore-export"),
            write_concurrency: num_cpus::get(),
            batch_size: 10000,
            default_relationship_type: RelationshipType::of("REL"),
            compression_enabled: true,
            compression_level: 6,
            include_metadata: true,
        }
    }
}

impl Config for FileExporterConfig {}

impl WriteConfig for FileExporterConfig {
    fn write_concurrency(&self) -> usize {
        self.write_concurrency
    }
}

impl FileExporterConfig {
    pub fn builder() -> FileExporterConfigBuilder {
        FileExporterConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_path(&self.export_path)?;
        ConfigValidation::validate_positive(self.write_concurrency as f64, "writeConcurrency")?;
        ConfigValidation::validate_range(
            self.write_concurrency as f64,
            1.0,
            100.0,
            "writeConcurrency",
        )?;
        ConfigValidation::validate_positive(self.batch_size as f64, "batchSize")?;
        ConfigValidation::validate_range(
            self.compression_level as f64,
            1.0,
            9.0,
            "compressionLevel",
        )?;
        Ok(())
    }
}

/// Builder for FileExporterConfig
#[derive(Debug, Default)]
pub struct FileExporterConfigBuilder {
    export_path: Option<String>,
    write_concurrency: Option<usize>,
    batch_size: Option<usize>,
    default_relationship_type: Option<RelationshipType>,
    compression_enabled: Option<bool>,
    compression_level: Option<u32>,
    include_metadata: Option<bool>,
}

impl FileExporterConfigBuilder {
    pub fn export_path(mut self, path: String) -> Self {
        self.export_path = Some(path);
        self
    }

    pub fn write_concurrency(mut self, concurrency: usize) -> Self {
        self.write_concurrency = Some(concurrency);
        self
    }

    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = Some(size);
        self
    }

    pub fn default_relationship_type(mut self, rel_type: RelationshipType) -> Self {
        self.default_relationship_type = Some(rel_type);
        self
    }

    pub fn compression_enabled(mut self, enabled: bool) -> Self {
        self.compression_enabled = Some(enabled);
        self
    }

    pub fn compression_level(mut self, level: u32) -> Self {
        self.compression_level = Some(level);
        self
    }

    pub fn include_metadata(mut self, include: bool) -> Self {
        self.include_metadata = Some(include);
        self
    }

    pub fn build(self) -> Result<FileExporterConfig, ConfigError> {
        let defaults = FileExporterConfig::default();

        let config = FileExporterConfig {
            export_path: self.export_path.unwrap_or(defaults.export_path),
            write_concurrency: self.write_concurrency.unwrap_or(defaults.write_concurrency),
            batch_size: self.batch_size.unwrap_or(defaults.batch_size),
            default_relationship_type: self
                .default_relationship_type
                .unwrap_or(defaults.default_relationship_type),
            compression_enabled: self
                .compression_enabled
                .unwrap_or(defaults.compression_enabled),
            compression_level: self.compression_level.unwrap_or(defaults.compression_level),
            include_metadata: self.include_metadata.unwrap_or(defaults.include_metadata),
        };

        config.validate()?;
        Ok(config)
    }
}

/// Database exporter configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DatabaseExporterConfig {
    pub database_name: String,
    pub write_concurrency: usize,
    pub batch_size: usize,
    pub default_relationship_type: RelationshipType,
    pub enable_debug_log: bool,
    pub database_format: String,
    pub high_io: bool,
    pub force: bool,
    pub transactional: bool,
    pub index_optimization: bool,
}

impl Default for DatabaseExporterConfig {
    fn default() -> Self {
        Self {
            database_name: String::from("graphstore_export"),
            write_concurrency: num_cpus::get(),
            batch_size: 10000,
            default_relationship_type: RelationshipType::of("REL"),
            enable_debug_log: false,
            database_format: String::from("standard"),
            high_io: false,
            force: false,
            transactional: true,
            index_optimization: true,
        }
    }
}

impl Config for DatabaseExporterConfig {}

impl WriteConfig for DatabaseExporterConfig {
    fn write_concurrency(&self) -> usize {
        self.write_concurrency
    }
}

impl DatabaseExporterConfig {
    pub fn builder() -> DatabaseExporterConfigBuilder {
        DatabaseExporterConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_database_name(&self.database_name)?;
        ConfigValidation::validate_positive(self.write_concurrency as f64, "writeConcurrency")?;
        ConfigValidation::validate_range(
            self.write_concurrency as f64,
            1.0,
            100.0,
            "writeConcurrency",
        )?;
        ConfigValidation::validate_positive(self.batch_size as f64, "batchSize")?;
        Ok(())
    }
}

/// Builder for DatabaseExporterConfig
#[derive(Debug, Default)]
pub struct DatabaseExporterConfigBuilder {
    database_name: Option<String>,
    write_concurrency: Option<usize>,
    batch_size: Option<usize>,
    default_relationship_type: Option<RelationshipType>,
    enable_debug_log: Option<bool>,
    database_format: Option<String>,
    high_io: Option<bool>,
    force: Option<bool>,
    transactional: Option<bool>,
    index_optimization: Option<bool>,
}

impl DatabaseExporterConfigBuilder {
    pub fn database_name(mut self, name: String) -> Self {
        self.database_name = Some(name);
        self
    }

    pub fn write_concurrency(mut self, concurrency: usize) -> Self {
        self.write_concurrency = Some(concurrency);
        self
    }

    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = Some(size);
        self
    }

    pub fn default_relationship_type(mut self, rel_type: RelationshipType) -> Self {
        self.default_relationship_type = Some(rel_type);
        self
    }

    pub fn enable_debug_log(mut self, enable: bool) -> Self {
        self.enable_debug_log = Some(enable);
        self
    }

    pub fn database_format(mut self, format: String) -> Self {
        self.database_format = Some(format);
        self
    }

    pub fn high_io(mut self, high: bool) -> Self {
        self.high_io = Some(high);
        self
    }

    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }

    pub fn transactional(mut self, transactional: bool) -> Self {
        self.transactional = Some(transactional);
        self
    }

    pub fn index_optimization(mut self, optimize: bool) -> Self {
        self.index_optimization = Some(optimize);
        self
    }

    pub fn build(self) -> Result<DatabaseExporterConfig, ConfigError> {
        let defaults = DatabaseExporterConfig::default();

        let config = DatabaseExporterConfig {
            database_name: self.database_name.unwrap_or(defaults.database_name),
            write_concurrency: self.write_concurrency.unwrap_or(defaults.write_concurrency),
            batch_size: self.batch_size.unwrap_or(defaults.batch_size),
            default_relationship_type: self
                .default_relationship_type
                .unwrap_or(defaults.default_relationship_type),
            enable_debug_log: self.enable_debug_log.unwrap_or(defaults.enable_debug_log),
            database_format: self.database_format.unwrap_or(defaults.database_format),
            high_io: self.high_io.unwrap_or(defaults.high_io),
            force: self.force.unwrap_or(defaults.force),
            transactional: self.transactional.unwrap_or(defaults.transactional),
            index_optimization: self
                .index_optimization
                .unwrap_or(defaults.index_optimization),
        };

        config.validate()?;
        Ok(config)
    }
}

/// File importer configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileImporterConfig {
    pub import_path: String,
    pub read_concurrency: usize,
    pub batch_size: usize,
    pub skip_invalid_lines: bool,
    pub delimiter: char,
    pub quotation_character: char,
    pub escape_character: char,
}

impl Default for FileImporterConfig {
    fn default() -> Self {
        Self {
            import_path: String::from("/tmp/graphstore-import"),
            read_concurrency: num_cpus::get(),
            batch_size: 10000,
            skip_invalid_lines: false,
            delimiter: ',',
            quotation_character: '"',
            escape_character: '\\',
        }
    }
}

impl Config for FileImporterConfig {}

impl FileImporterConfig {
    pub fn builder() -> FileImporterConfigBuilder {
        FileImporterConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_path(&self.import_path)?;
        ConfigValidation::validate_positive(self.read_concurrency as f64, "readConcurrency")?;
        ConfigValidation::validate_positive(self.batch_size as f64, "batchSize")?;
        Ok(())
    }
}

/// Builder for FileImporterConfig
#[derive(Debug, Default)]
pub struct FileImporterConfigBuilder {
    import_path: Option<String>,
    read_concurrency: Option<usize>,
    batch_size: Option<usize>,
    skip_invalid_lines: Option<bool>,
    delimiter: Option<char>,
    quotation_character: Option<char>,
    escape_character: Option<char>,
}

impl FileImporterConfigBuilder {
    pub fn import_path(mut self, path: String) -> Self {
        self.import_path = Some(path);
        self
    }

    pub fn read_concurrency(mut self, concurrency: usize) -> Self {
        self.read_concurrency = Some(concurrency);
        self
    }

    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = Some(size);
        self
    }

    pub fn skip_invalid_lines(mut self, skip: bool) -> Self {
        self.skip_invalid_lines = Some(skip);
        self
    }

    pub fn delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = Some(delimiter);
        self
    }

    pub fn quotation_character(mut self, character: char) -> Self {
        self.quotation_character = Some(character);
        self
    }

    pub fn escape_character(mut self, character: char) -> Self {
        self.escape_character = Some(character);
        self
    }

    pub fn build(self) -> Result<FileImporterConfig, ConfigError> {
        let defaults = FileImporterConfig::default();

        let config = FileImporterConfig {
            import_path: self.import_path.unwrap_or(defaults.import_path),
            read_concurrency: self.read_concurrency.unwrap_or(defaults.read_concurrency),
            batch_size: self.batch_size.unwrap_or(defaults.batch_size),
            skip_invalid_lines: self
                .skip_invalid_lines
                .unwrap_or(defaults.skip_invalid_lines),
            delimiter: self.delimiter.unwrap_or(defaults.delimiter),
            quotation_character: self
                .quotation_character
                .unwrap_or(defaults.quotation_character),
            escape_character: self.escape_character.unwrap_or(defaults.escape_character),
        };

        config.validate()?;
        Ok(config)
    }
}

/// Database importer configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DatabaseImporterConfig {
    pub database_name: String,
    pub read_concurrency: usize,
    pub batch_size: usize,
    pub node_query: Option<String>,
    pub relationship_query: Option<String>,
}

impl Default for DatabaseImporterConfig {
    fn default() -> Self {
        Self {
            database_name: String::from("source-graph"),
            read_concurrency: num_cpus::get(),
            batch_size: 10000,
            node_query: None,
            relationship_query: None,
        }
    }
}

impl Config for DatabaseImporterConfig {}

impl DatabaseImporterConfig {
    pub fn builder() -> DatabaseImporterConfigBuilder {
        DatabaseImporterConfigBuilder::default()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_database_name(&self.database_name)?;
        ConfigValidation::validate_positive(self.read_concurrency as f64, "readConcurrency")?;
        ConfigValidation::validate_positive(self.batch_size as f64, "batchSize")?;
        Ok(())
    }
}

/// Builder for DatabaseImporterConfig
#[derive(Debug, Default)]
pub struct DatabaseImporterConfigBuilder {
    database_name: Option<String>,
    read_concurrency: Option<usize>,
    batch_size: Option<usize>,
    node_query: Option<String>,
    relationship_query: Option<String>,
}

impl DatabaseImporterConfigBuilder {
    pub fn database_name(mut self, name: String) -> Self {
        self.database_name = Some(name);
        self
    }

    pub fn read_concurrency(mut self, concurrency: usize) -> Self {
        self.read_concurrency = Some(concurrency);
        self
    }

    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = Some(size);
        self
    }

    pub fn node_query(mut self, query: String) -> Self {
        self.node_query = Some(query);
        self
    }

    pub fn relationship_query(mut self, query: String) -> Self {
        self.relationship_query = Some(query);
        self
    }

    pub fn build(self) -> Result<DatabaseImporterConfig, ConfigError> {
        let defaults = DatabaseImporterConfig::default();

        let config = DatabaseImporterConfig {
            database_name: self.database_name.unwrap_or(defaults.database_name),
            read_concurrency: self.read_concurrency.unwrap_or(defaults.read_concurrency),
            batch_size: self.batch_size.unwrap_or(defaults.batch_size),
            node_query: self.node_query.or(defaults.node_query),
            relationship_query: self.relationship_query.or(defaults.relationship_query),
        };

        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_exporter_default() {
        let config = FileExporterConfig::default();
        assert_eq!(config.batch_size, 10000);
        assert!(config.compression_enabled);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_file_exporter_builder() {
        let config = FileExporterConfig::builder()
            .export_path(String::from("/custom/path"))
            .compression_level(9)
            .include_metadata(false)
            .build()
            .unwrap();

        assert_eq!(config.export_path, "/custom/path");
        assert_eq!(config.compression_level, 9);
        assert!(!config.include_metadata);
    }

    #[test]
    fn test_database_exporter_default() {
        let config = DatabaseExporterConfig::default();
        assert_eq!(config.database_name, "graphstore_export");
        assert!(config.transactional);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_file_importer_default() {
        let config = FileImporterConfig::default();
        assert_eq!(config.delimiter, ',');
        assert!(!config.skip_invalid_lines);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_database_importer_builder() {
        let config = DatabaseImporterConfig::builder()
            .database_name(String::from("mydb"))
            .node_query(String::from("MATCH (n) RETURN n"))
            .build()
            .unwrap();

        assert_eq!(config.database_name, "mydb");
        assert_eq!(config.node_query, Some(String::from("MATCH (n) RETURN n")));
    }

    #[test]
    fn test_invalid_database_name() {
        let result = DatabaseExporterConfig::builder()
            .database_name(String::from("123invalid"))
            .build();

        assert!(result.is_err());
    }
}
