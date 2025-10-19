use anyhow::{anyhow, Result};
use parking_lot::RwLock;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::types::{CustomInfo, Model, ModelConfig, ModelData};

pub struct UserCatalog {
    models: RwLock<HashMap<String, CatalogEntry>>,
    model_types: RwLock<HashMap<String, String>>,
}

struct CatalogEntry {
    erased: Arc<dyn Any + Send + Sync>,
    storage_info: Arc<dyn ModelStorageInfo + Send + Sync>,
}

impl CatalogEntry {
    fn new(
        model: Arc<dyn Any + Send + Sync>,
        storage_info: Arc<dyn ModelStorageInfo + Send + Sync>,
    ) -> Self {
        Self {
            erased: model,
            storage_info,
        }
    }

    fn erased(&self) -> Arc<dyn Any + Send + Sync> {
        Arc::clone(&self.erased)
    }
}

impl Default for UserCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl UserCatalog {
    pub fn new() -> Self {
        Self {
            models: RwLock::new(HashMap::new()),
            model_types: RwLock::new(HashMap::new()),
        }
    }

    pub fn get<D, C, I>(&self, model_name: &str) -> Result<Arc<Model<D, C, I>>>
    where
        D: ModelData + 'static,
        C: ModelConfig + 'static,
        I: CustomInfo + 'static,
    {
        let models = self.models.read();
        let entry = models
            .get(model_name)
            .ok_or_else(|| anyhow!("Model '{}' not found", model_name))?;

        entry
            .erased()
            .downcast::<Model<D, C, I>>()
            .map_err(|_| anyhow!("Model '{}' has incompatible type", model_name))
    }

    pub fn get_untyped(&self, model_name: &str) -> Result<Arc<dyn Any + Send + Sync>> {
        let models = self.models.read();
        models
            .get(model_name)
            .map(|entry| entry.erased())
            .ok_or_else(|| anyhow!("Model '{}' not found", model_name))
    }

    pub fn set<D, C, I>(&self, model: Model<D, C, I>) -> Result<()>
    where
        D: ModelData + 'static,
        C: ModelConfig + 'static,
        I: CustomInfo + 'static,
    {
        let mut models = self.models.write();
        let mut types = self.model_types.write();

        let model_name = model.name().to_string();
        let model_type = model.algo_type().to_string();

        let arc_model = Arc::new(model);
        let erased: Arc<dyn Any + Send + Sync> = arc_model.clone();
        let storage_info: Arc<dyn ModelStorageInfo + Send + Sync> = arc_model.clone();

        models.insert(model_name.clone(), CatalogEntry::new(erased, storage_info));
        types.insert(model_name, model_type);

        Ok(())
    }

    pub fn list(&self) -> Vec<Arc<dyn Any + Send + Sync>> {
        let models = self.models.read();
        models.values().map(|entry| entry.erased()).collect()
    }

    pub fn list_names(&self) -> HashSet<String> {
        let models = self.models.read();
        models.keys().cloned().collect()
    }

    pub fn exists(&self, model_name: &str) -> bool {
        let models = self.models.read();
        models.contains_key(model_name)
    }

    pub fn get_type(&self, model_name: &str) -> Option<String> {
        let types = self.model_types.read();
        types.get(model_name).cloned()
    }

    pub fn drop(
        &self,
        model_name: &str,
        fail_on_missing: bool,
    ) -> Result<Arc<dyn Any + Send + Sync>> {
        let mut models = self.models.write();
        let mut types = self.model_types.write();

        if fail_on_missing && !models.contains_key(model_name) {
            return Err(anyhow!("Model '{}' not found", model_name));
        }

        let model = models.remove(model_name).map(|entry| entry.erased());
        types.remove(model_name);

        model.ok_or_else(|| anyhow!("Model '{}' not found", model_name))
    }

    pub fn remove_all_loaded(&self) {
        let mut models = self.models.write();
        let mut types = self.model_types.write();

        // Only retain models that have been persisted to disk
        models.retain(|_, entry| entry.storage_info.is_stored());

        // Clean up orphaned type entries
        types.retain(|name, _| models.contains_key(name));
    }

    pub fn verify_model_can_be_stored(&self, model_name: &str, model_type: &str) -> Result<()> {
        if let Some(existing_type) = self.get_type(model_name) {
            if existing_type != model_type {
                return Err(anyhow!(
                    "Model '{}' exists with different type. Found: {}, Expected: {}",
                    model_name,
                    existing_type,
                    model_type
                ));
            }
        }
        Ok(())
    }

    pub fn size(&self) -> usize {
        let models = self.models.read();
        models.len()
    }
}

/// Helper trait for checking model storage status
trait ModelStorageInfo: Send + Sync {
    fn is_stored(&self) -> bool;
}

impl<D, C, I> ModelStorageInfo for Model<D, C, I>
where
    D: ModelData,
    C: ModelConfig,
    I: CustomInfo,
{
    fn is_stored(&self) -> bool {
        Self::is_stored(self)
    }
}
