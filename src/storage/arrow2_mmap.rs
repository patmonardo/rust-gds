use std::{collections::HashMap, fs::File, io::Cursor, path::PathBuf, sync::Arc};

use arrow2::array::Array;
use arrow2::chunk::Chunk;
use arrow2::datatypes::Schema;
use arrow2::error::Result as Arrow2Result;
use arrow2::io::ipc::read::{read_file, read_file_metadata};
use memmap2::{Mmap, MmapOptions};

use crate::types::property::PropertySchema;

/// Memory-mapped Arrow IPC column. Keeps mmap alive so Arrow2 arrays can reference the mmap bytes.
#[derive(Debug, Clone)]
pub struct Arrow2MmapColumn {
    path: PathBuf,
    column_name: String,
    schema: PropertySchema,
    mmap: Option<Arc<Mmap>>,
    // optionally cache parsed chunk(s)
    cached_chunks: Option<Arc<Vec<Chunk<Arc<dyn Array>>>>>,
    // file-level schema (if needed)
    file_schema: Option<Arc<Schema>>,
}

impl Arrow2MmapColumn {
    pub fn new(
        path: impl Into<PathBuf>,
        column_name: impl Into<String>,
        schema: PropertySchema,
    ) -> Self {
        Self {
            path: path.into(),
            column_name: column_name.into(),
            mmap: None,
            cached_chunks: None,
            file_schema: None,
        }
    }

    fn ensure_mmap(&mut self) -> std::io::Result<Arc<Mmap>> {
        if let Some(m) = &self.mmap {
            return Ok(m.clone());
        }
        let file = File::open(&self.path)?;
        // Safety: read-only mapping
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let arc = Arc::new(mmap);
        self.mmap = Some(arc.clone());
        Ok(arc)
    }

    /// Parse the IPC file using arrow2 from the memory-mapped bytes.
    /// This aims to be zero-copy: arrow2 will create arrays that reference the mmap bytes.
    /// The returned Chunk(s) are cached and the mmap is kept alive inside self for lifetime safety.
    pub fn load_chunks(&mut self) -> Arrow2Result<Arc<Vec<Chunk<Arc<dyn Array>>>>> {
        if let Some(c) = &self.cached_chunks {
            return Ok(c.clone());
        }

        let mmap = self
            .ensure_mmap()
            .map_err(|e| arrow2::error::Error::from(e.to_string()))?;
        let bytes: &[u8] = &mmap[..];

        // Cursor over the mmap bytes (Read + Seek)
        let mut cursor = Cursor::new(bytes);

        // Read file metadata first (arrow2 provides helper functions)
        // Note: API names/params can change with arrow2 versions; adjust if your version differs.
        let metadata = read_file_metadata(&mut cursor)?;
        // Read the record batches as arrow2 chunks. read_file returns (schema, Vec<Chunk>).
        let (schema, chunks) = read_file(&mut cursor, &metadata, None)?;
        let arc_schema = Arc::new(schema);
        let arc_chunks = Arc::new(chunks);

        // Cache them and keep mmap alive in self
        self.file_schema = Some(arc_schema.clone());
        self.cached_chunks = Some(arc_chunks.clone());

        Ok(arc_chunks)
    }

    /// Convenience: try to find the named column in the first batch and return its Array (zero-copy).
    /// If the file contains multiple batches, this returns the concatenated array as a single Chunk element or the first chunk's column.
    pub fn load_named_column_array(&mut self) -> Arrow2Result<Option<Arc<dyn Array>>> {
        let chunks = self.load_chunks()?;
        if chunks.is_empty() {
            return Ok(None);
        }
        // Try first chunk to keep example simple. For multiple chunks you'd want to combine or expose all.
        let first = &chunks[0];
        // Find the column index by name from file_schema
        let schema = match &self.file_schema {
            Some(s) => s.clone(),
            None => return Ok(None),
        };

        let col_index = schema
            .fields
            .iter()
            .position(|f| f.name == self.column_name);
        let idx = match col_index {
            Some(i) => i,
            None => return Ok(None),
        };

        // first.arrays is a Vec<Arc<dyn Array>>
        let array = first.arrays().get(idx).cloned();
        Ok(array)
    }

    pub fn schema(&self) -> &PropertySchema {
        &self.schema
    }
}

/// Minimal property store that references Arrow2 mmap columns by key.
#[derive(Debug, Default)]
pub struct Arrow2MmapPropertyStore {
    columns: HashMap<String, Arrow2MmapColumn>,
}

impl Arrow2MmapPropertyStore {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
        }
    }

    pub fn register_column(
        &mut self,
        key: impl Into<String>,
        file_path: impl Into<PathBuf>,
        column_name: impl Into<String>,
        schema: PropertySchema,
    ) {
        let k = key.into();
        let col = Arrow2MmapColumn::new(file_path, column_name, schema);
        self.columns.insert(k, col);
    }

    /// Retrieve the zero-copy Arrow2 array for a registered key (first batch / first chunk).
    /// You can call load_chunks() for all batches instead.
    pub fn get_array_for_key(&mut self, key: &str) -> Arrow2Result<Option<Arc<dyn Array>>> {
        match self.columns.get_mut(key) {
            Some(col) => col.load_named_column_array(),
            None => Ok(None),
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.columns.contains_key(key)
    }
}
