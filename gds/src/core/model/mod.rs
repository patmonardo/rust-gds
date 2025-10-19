mod catalog;
mod types;
mod user_catalog;
mod model;
mod model_metadata;

pub use catalog::{EmptyModelCatalog, ModelCatalog, ModelCatalogListener};
pub use types::{CustomInfo as ModelCatalogCustomInfo, ModelConfig, ModelData};
pub use user_catalog::UserCatalog;
pub use model::Model as MLModel;
pub use model_metadata::ModelMetaData;

// Constants
pub const MODEL_NAME_KEY: &str = "modelName";
pub const MODEL_TYPE_KEY: &str = "modelType";
pub const ALL_USERS: &str = "*";
pub const PUBLIC_SUFFIX: &str = "_public";