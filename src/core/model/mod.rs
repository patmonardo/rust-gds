mod catalog;
mod types;
mod user_catalog;

pub use catalog::{EmptyModelCatalog, ModelCatalog, ModelCatalogListener};
pub use types::{CustomInfo, Model, ModelConfig, ModelData};
pub use user_catalog::UserCatalog;

// Constants
pub const MODEL_NAME_KEY: &str = "modelName";
pub const MODEL_TYPE_KEY: &str = "modelType";
pub const ALL_USERS: &str = "*";
pub const PUBLIC_SUFFIX: &str = "_public";
