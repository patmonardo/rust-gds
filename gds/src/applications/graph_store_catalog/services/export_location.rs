/// Interface for export location configuration.
/// 
/// Mirrors Java ExportLocation interface.
/// Simple trait with 2 methods for handling export paths with error injection.
pub trait ExportLocation {
    /// Gets the export path, throwing an error if not configured.
    /// In Java, this calls getAcceptingError().
    fn get_accepting_error(&self) -> Result<std::path::PathBuf, String>;
    
    /// Gets the export path, returning None if not configured.
    /// In Java, this calls getAcceptingNull().
    fn get_accepting_null(&self) -> Option<std::path::PathBuf>;
}

/// Default implementation of ExportLocation.
/// 
/// This provides a simple implementation that can be extended as needed.
#[derive(Clone, Debug)]
pub struct DefaultExportLocation {
    path: Option<std::path::PathBuf>,
}

impl DefaultExportLocation {
    /// Creates a new DefaultExportLocation with an optional path.
    pub fn new(path: Option<std::path::PathBuf>) -> Self {
        Self { path }
    }
    
    /// Creates a new DefaultExportLocation with a required path.
    pub fn with_path(path: std::path::PathBuf) -> Self {
        Self { path: Some(path) }
    }
}

impl ExportLocation for DefaultExportLocation {
    fn get_accepting_error(&self) -> Result<std::path::PathBuf, String> {
        self.path.clone().ok_or_else(|| "Export location not configured".to_string())
    }
    
    fn get_accepting_null(&self) -> Option<std::path::PathBuf> {
        self.path.clone()
    }
}
