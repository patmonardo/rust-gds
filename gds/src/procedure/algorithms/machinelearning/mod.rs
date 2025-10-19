//! Machine learning algorithms - Faithful 1:1 translation from Java GDS
//!
//! This module contains faithful translations of Java GDS ML algorithms:
//! - `LinkScorer.java` → `link_scorer.rs`
//! - `LinkScorerFactory.java` → `link_scorer_factory.rs`
//! - `DoubleDistMultLinkScorer.java` → `double_dist_mult_link_scorer.rs`
//! - `DoubleEuclideanDistanceLinkScorer.java` → `double_euclidean_distance_link_scorer.rs`
//! - `FloatDistMultLinkScorer.java` → `float_dist_mult_link_scorer.rs`
//! - `FloatEuclideanDistanceLinkScorer.java` → `float_euclidean_distance_link_scorer.rs`
//! - `KGEPredictParameters.java` → `kge_predict_parameters.rs`
//! - `KGEPredictResult.java` → `kge_predict_result.rs`
//! - `KGEPredictConfigTransformer.java` → `kge_predict_config_transformer.rs`
//! - `TopKMapComputer.java` → `top_k_map_computer.rs`

pub mod link_scorer;
pub mod link_scorer_factory;
pub mod double_dist_mult_link_scorer;
pub mod double_euclidean_distance_link_scorer;
pub mod float_dist_mult_link_scorer;
pub mod float_euclidean_distance_link_scorer;
pub mod kge_predict_parameters;
pub mod kge_predict_result;
pub mod kge_predict_config_transformer;
pub mod top_k_map_computer;

// Re-export the translated types
pub use link_scorer::*;
pub use link_scorer_factory::*;
pub use double_dist_mult_link_scorer::*;
pub use double_euclidean_distance_link_scorer::*;
pub use float_dist_mult_link_scorer::*;
pub use float_euclidean_distance_link_scorer::*;
pub use kge_predict_parameters::*;
pub use kge_predict_result::*;
pub use kge_predict_config_transformer::*;
pub use top_k_map_computer::*;