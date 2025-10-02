use std::{collections::HashMap, fs::File, path::PathBuf, sync::Arc};

use memmap2::{Mmap, MmapOptions};
use polars::prelude::*;

use crate::types::property::PropertySchema;

/// A single column persisted as an Arrow IPC file and memory-mapped on demand.
/// `column_name` is the column inside the IPC file that holds the Series we want.
#[derive(Debug, Clone)]
pub struct MmapColumn {
    path: PathBuf,
    column_name: String,
    schema: PropertySchema,
    mmap: Option<Arc<Mmap>>,
}

impl MmapColumn {
    pub fn new(
        path: impl Into<PathBuf>,
        column_name: impl Into<String>,
        schema: PropertySchema,
    ) -> Self {
        Self {
            path: path.into(),
            column_name: column_name.into(),
            schema,
            mmap: None,
        }
    }

    /// Ensure the file is memory-mapped and return the Arc<Mmap>.
    fn ensure_mmap(&mut self) -> PolarsResult<Arc<Mmap>> {
        if let Some(m) = &self.mmap {
            return Ok(m.clone());
        }
        let file = File::open(&self.path)
            .map_err(|e| PolarsError::ComputeError(format!("open mmap file: {}", e).into()))?;
        // Safety: mapping a read-only file
        let mmap = unsafe {
            MmapOptions::new()
                .map(&file)
                .map_err(|e| PolarsError::ComputeError(format!("mmap failed: {}", e).into()))?
        };
        let arc = Arc::new(mmap);
        self.mmap = Some(arc.clone());
        Ok(arc)
    }

    /// Lazily read the IPC file from the mmap and extract the named column as a Series.
    /// This uses polars' IpcReader with a Cursor over the mmap bytes.
    /// Note: this parses the IPC stream into a DataFrame/Series. We keep the mmap alive so
    /// any zero-copy internals (if available) can reference the bytes.
    pub fn load_series(&mut self) -> PolarsResult<Series> {
        let mmap = self.ensure_mmap()?;
        let bytes = &mmap[..];
        let cursor = std::io::Cursor::new(bytes);
        // IpcReader requires a Read+Seek. Cursor<&[u8]> satisfies that.
        let mut reader = IpcReader::new(cursor);
        // If the IPC file contains a single table, we can finish to get the DataFrame.
        let df = reader.finish()?;
        df.column(&self.column_name)
            .map(|s| s.clone())
            .map_err(|e| {
                PolarsError::ComputeError(
                    format!("missing column {}: {}", &self.column_name, e).into(),
                )
            })
    }

    pub fn schema(&self) -> &PropertySchema {
        &self.schema
    }
}

/// A minimal property store that holds references to memory-mapped column files.
/// Keys are property keys. Each property is one column inside its IPC file.
#[derive(Debug, Default)]
pub struct MmapPropertyStore {
    columns: HashMap<String, MmapColumn>,
}

impl MmapPropertyStore {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
        }
    }

    /// Register a column that lives in an Arrow IPC file.
    /// `key` is the property key; `file_path` the IPC file; `column_name` the column inside that file.
    pub fn register_column(
        &mut self,
        key: impl Into<String>,
        file_path: impl Into<PathBuf>,
        column_name: impl Into<String>,
        schema: PropertySchema,
    ) {
        let k = key.into();
        let col = MmapColumn::new(file_path, column_name, schema);
        self.columns.insert(k, col);
    }

    /// Get a Series for a registered key. This will ensure the file is mapped and parsed.
    pub fn get_series(&mut self, key: &str) -> PolarsResult<Option<Series>> {
        match self.columns.get_mut(key) {
            Some(col) => col.load_series().map(Some),
            None => Ok(None),
        }
    }

    /// Quick check whether key exists.
    pub fn contains_key(&self, key: &str) -> bool {
        self.columns.contains_key(key)
    }
}
