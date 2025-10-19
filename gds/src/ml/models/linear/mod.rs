//! Linear regression model package.
//!
//! 1:1 translation of `org.neo4j.gds.ml.models.linearregression` from Java GDS.
//! The module is organized exactly like the Java package:
//!
//! - `data`       → `LinearRegressionData`
//! - `regressor`  → `LinearRegressor`
//! - `objective`  → `LinearRegressionObjective`
//! - `trainer`    → `LinearRegressionTrainer`
//! - `config`     → `LinearRegressionTrainConfig`

pub mod config;
pub mod data;
pub mod objective;
pub mod regressor;
pub mod trainer;

pub use config::LinearRegressionTrainConfig;
pub use data::LinearRegressionData;
pub use objective::LinearRegressionObjective;
pub use regressor::LinearRegressor;
pub use trainer::LinearRegressionTrainer;
