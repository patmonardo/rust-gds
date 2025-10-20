//! I/O configuration types for import and export operations

use super::base_types::WriteConfig;
use super::validation::ConfigValidation;
use crate::projection::RelationshipType;
use crate::define_config;

define_config!(
    pub struct FileExporterConfig {
        validate = |cfg: &FileExporterConfig| {
            ConfigValidation::validate_path(&cfg.export_path)?;
            ConfigValidation::validate_positive(cfg.write_concurrency as f64, "writeConcurrency")?;
            ConfigValidation::validate_range(
                cfg.write_concurrency as f64,
                1.0,
                100.0,
                "writeConcurrency",
            )?;
            ConfigValidation::validate_positive(cfg.batch_size as f64, "batchSize")?;
            ConfigValidation::validate_range(
                cfg.compression_level as f64,
                1.0,
                9.0,
                "compressionLevel",
            )?;
            Ok(())
        },
        export_path: String = String::from("/tmp/graphstore-export"),
        write_concurrency: usize = num_cpus::get(),
        batch_size: usize = 10000,
        default_relationship_type: RelationshipType = RelationshipType::of("REL"),
        compression_enabled: bool = true,
        compression_level: u32 = 6,
        include_metadata: bool = true,
    }
);

impl WriteConfig for FileExporterConfig {
    fn write_concurrency(&self) -> usize {
        self.write_concurrency
    }
}

define_config!(
    pub struct DatabaseExporterConfig {
        validate = |cfg: &DatabaseExporterConfig| {
            ConfigValidation::validate_database_name(&cfg.database_name)?;
            ConfigValidation::validate_positive(cfg.write_concurrency as f64, "writeConcurrency")?;
            ConfigValidation::validate_range(
                cfg.write_concurrency as f64,
                1.0,
                100.0,
                "writeConcurrency",
            )?;
            ConfigValidation::validate_positive(cfg.batch_size as f64, "batchSize")?;
            Ok(())
        },
        database_name: String = String::from("graphstore_export"),
        write_concurrency: usize = num_cpus::get(),
        batch_size: usize = 10000,
        default_relationship_type: RelationshipType = RelationshipType::of("REL"),
        enable_debug_log: bool = false,
        database_format: String = String::from("standard"),
        high_io: bool = false,
        force: bool = false,
        transactional: bool = true,
        index_optimization: bool = true,
    }
);

impl WriteConfig for DatabaseExporterConfig {
    fn write_concurrency(&self) -> usize {
        self.write_concurrency
    }
}

define_config!(
    pub struct FileImporterConfig {
        validate = |cfg: &FileImporterConfig| {
            ConfigValidation::validate_path(&cfg.import_path)?;
            ConfigValidation::validate_positive(cfg.read_concurrency as f64, "readConcurrency")?;
            ConfigValidation::validate_positive(cfg.batch_size as f64, "batchSize")?;
            Ok(())
        },
        import_path: String = String::from("/tmp/graphstore-import"),
        read_concurrency: usize = num_cpus::get(),
        batch_size: usize = 10000,
        skip_invalid_lines: bool = false,
        delimiter: char = ',',
        quotation_character: char = '"',
        escape_character: char = '\\',
    }
);

define_config!(
    pub struct DatabaseImporterConfig {
        validate = |cfg: &DatabaseImporterConfig| {
            ConfigValidation::validate_database_name(&cfg.database_name)?;
            ConfigValidation::validate_positive(cfg.read_concurrency as f64, "readConcurrency")?;
            ConfigValidation::validate_positive(cfg.batch_size as f64, "batchSize")?;
            Ok(())
        },
        database_name: String = String::from("source-graph"),
        read_concurrency: usize = num_cpus::get(),
        batch_size: usize = 10000,
        node_query: Option<String> = None,
        relationship_query: Option<String> = None,
    }
);

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
            .node_query(Some(String::from("MATCH (n) RETURN n")))
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
