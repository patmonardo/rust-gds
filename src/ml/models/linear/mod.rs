//! Linear model implementations
//!
//! 1:1 translation of org.neo4j.gds.ml.models.linearregression and logisticregression packages.
//!
//! Contains:
//! - Linear regression (for continuous targets)
//! - Logistic regression (for binary/multiclass classification)
//! - Trainers (TODO: translate from Java - require gradient descent infrastructure)

mod linear_regression;
mod linear_regression_trainer;
mod logistic_regression;
mod logistic_regression_trainer;

pub use linear_regression::*;

pub use logistic_regression::*;

// TODO: Translate trainers from Java GDS:
// - LinearRegressionTrainer (requires gradient descent + Training infrastructure)
// - LogisticRegressionTrainer (requires gradient descent + Training infrastructure)
