use super::types::Model;
use anyhow::Result;
use std::any::Any;
use std::path::Path;
use std::sync::Arc;

/// Listener for model catalog events
pub trait ModelCatalogListener: Send + Sync {
    fn on_insert(&self, model: &dyn Any);
    fn on_store(&self, model: &dyn Any);
    fn on_load(&self);
}

/// Core interface for model catalog operations
pub trait ModelCatalog: Send + Sync {
    /// Register a new listener
    fn register_listener(&self, listener: Box<dyn ModelCatalogListener>);

    /// Remove a listener
    fn unregister_listener(&self, listener: &dyn ModelCatalogListener);

    /// Store a new model in the catalog
    fn set<D, C, I>(&self, model: Model<D, C, I>) -> Result<()>
    where
        D: super::types::ModelData + 'static,
        C: super::types::ModelConfig + 'static,
        I: super::types::CustomInfo + 'static;

    /// Get a model with type checking
    fn get<D, C, I>(&self, username: &str, model_name: &str) -> Result<Arc<Model<D, C, I>>>
    where
        D: super::types::ModelData + 'static,
        C: super::types::ModelConfig + 'static,
        I: super::types::CustomInfo + 'static;

    /// Get a model without type checking
    fn get_untyped(&self, username: &str, model_name: &str) -> Result<Arc<dyn Any + Send + Sync>>;

    /// Check if a model exists
    fn exists(&self, username: &str, model_name: &str) -> bool;

    /// Drop a model from the catalog
    fn drop(&self, username: &str, model_name: &str) -> Result<Arc<dyn Any + Send + Sync>>;

    /// List all models for a user
    fn list(&self, username: &str) -> Vec<Arc<dyn Any + Send + Sync>>;

    /// Publish a model making it available to all users
    fn publish(&self, username: &str, model_name: &str) -> Result<Arc<dyn Any + Send + Sync>>;

    /// Store model to disk
    fn store(
        &self,
        username: &str,
        model_name: &str,
        model_dir: &Path,
    ) -> Result<Arc<dyn Any + Send + Sync>>;

    /// Get total number of models
    fn model_count(&self) -> usize;

    /// Check if catalog is empty
    fn is_empty(&self) -> bool {
        self.model_count() == 0
    }

    /// Remove all loaded models
    fn remove_all_loaded(&self);

    /// Verify a model can be stored
    fn verify_model_can_be_stored(
        &self,
        username: &str,
        model_name: &str,
        model_type: &str,
    ) -> Result<()>;
}

/// Empty implementation of ModelCatalog
pub struct EmptyModelCatalog;

impl ModelCatalog for EmptyModelCatalog {
    fn register_listener(&self, _listener: Box<dyn ModelCatalogListener>) {}
    fn unregister_listener(&self, _listener: &dyn ModelCatalogListener) {}
    fn set<D, C, I>(&self, _model: Model<D, C, I>) -> Result<()>
    where
        D: super::types::ModelData + 'static,
        C: super::types::ModelConfig + 'static,
        I: super::types::CustomInfo + 'static,
    {
        Ok(())
    }

    fn get<D, C, I>(&self, _username: &str, _model_name: &str) -> Result<Arc<Model<D, C, I>>>
    where
        D: super::types::ModelData + 'static,
        C: super::types::ModelConfig + 'static,
        I: super::types::CustomInfo + 'static,
    {
        anyhow::bail!("Empty catalog")
    }
    fn get_untyped(
        &self,
        _username: &str,
        _model_name: &str,
    ) -> Result<Arc<dyn Any + Send + Sync>> {
        anyhow::bail!("Empty catalog")
    }
    fn exists(&self, _username: &str, _model_name: &str) -> bool {
        false
    }
    fn drop(&self, _username: &str, _model_name: &str) -> Result<Arc<dyn Any + Send + Sync>> {
        anyhow::bail!("Empty catalog")
    }
    fn list(&self, _username: &str) -> Vec<Arc<dyn Any + Send + Sync>> {
        Vec::new()
    }
    fn publish(&self, _username: &str, _model_name: &str) -> Result<Arc<dyn Any + Send + Sync>> {
        anyhow::bail!("Empty catalog")
    }
    fn store(
        &self,
        _username: &str,
        _model_name: &str,
        _model_dir: &Path,
    ) -> Result<Arc<dyn Any + Send + Sync>> {
        anyhow::bail!("Empty catalog")
    }
    fn model_count(&self) -> usize {
        0
    }
    fn remove_all_loaded(&self) {}
    fn verify_model_can_be_stored(
        &self,
        _username: &str,
        _model_name: &str,
        _model_type: &str,
    ) -> Result<()> {
        Ok(())
    }
}
