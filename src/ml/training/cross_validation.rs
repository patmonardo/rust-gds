use crate::ml::{
    metrics::{
        Metric, MetricConsumer, ModelCandidateStats, ModelSpecificMetricsHandler, ModelStatsBuilder,
    },
    models::TrainerConfig,
    splitting::StratifiedKFoldSplitter,
};
use parking_lot::RwLock;
use std::{collections::BTreeSet, sync::Arc};

use super::statistics::TrainingStatistics;

/// ReadOnlyHugeLongArray - simple wrapper for array of node IDs
/// This is a simplified version matching Java's ReadOnlyHugeLongArray
pub type ReadOnlyHugeLongArray = Arc<Vec<i64>>;

/// Cross validation for model selection
pub struct CrossValidation<MODEL> {
    termination_flag: Arc<RwLock<bool>>,
    metrics: Vec<Box<dyn Metric>>,
    validation_folds: usize,
    random_seed: Option<u64>,
    model_trainer: ModelTrainer<MODEL>,
    model_evaluator: ModelEvaluator<MODEL>,
}

/// Model trainer function type
pub type ModelTrainer<MODEL> = Box<
    dyn Fn(ReadOnlyHugeLongArray, &dyn TrainerConfig, &ModelSpecificMetricsHandler, &str) -> MODEL
        + Send
        + Sync,
>;

/// Model evaluator function type
pub type ModelEvaluator<MODEL> =
    Box<dyn Fn(ReadOnlyHugeLongArray, &MODEL, &mut dyn MetricConsumer) + Send + Sync>;

impl<MODEL> CrossValidation<MODEL> {
    /// Creates a new CrossValidation instance
    pub fn new(
        termination_flag: Arc<RwLock<bool>>,
        metrics: Vec<Box<dyn Metric>>,
        validation_folds: usize,
        random_seed: Option<u64>,
        model_trainer: ModelTrainer<MODEL>,
        model_evaluator: ModelEvaluator<MODEL>,
    ) -> Self {
        Self {
            termination_flag,
            metrics,
            validation_folds,
            random_seed,
            model_trainer,
            model_evaluator,
        }
    }

    /// Selects the best model through cross validation
    /// 1:1 with selectModel() in Java
    pub fn select_model(
        &self,
        outer_train_set: ReadOnlyHugeLongArray,
        targets: impl Fn(u64) -> i64 + Send + Sync + 'static,
        distinct_internal_targets: BTreeSet<i64>,
        training_statistics: &mut TrainingStatistics,
        model_candidates: impl Iterator<Item = Box<dyn TrainerConfig>>,
    ) {
        log::debug!("Creating validation folds");

        let validation_splits = StratifiedKFoldSplitter::new(
            self.validation_folds,
            outer_train_set.clone(),
            move |node_id: i64| targets(node_id as u64),
            self.random_seed,
            distinct_internal_targets,
        )
        .splits();

        log::debug!("Selecting best model");

        for (trial, model_params) in model_candidates.enumerate() {
            log::debug!("Starting trial {}", trial + 1);

            if *self.termination_flag.read() {
                return;
            }

            log::info!(
                "Method: {:?}, Parameters: {:?}",
                model_params.method(),
                model_params.to_map()
            );

            let mut validation_stats_builder = ModelStatsBuilder::new(validation_splits.len());
            let mut train_stats_builder = ModelStatsBuilder::new(validation_splits.len());
            let metrics_handler = ModelSpecificMetricsHandler::new(&self.metrics, |_, _| {});

            for (fold, split) in validation_splits.iter().enumerate() {
                let train_set = split.train_set();
                let validation_set = split.test_set();

                log::debug!("Starting fold {} training", fold + 1);
                let trained_model = (self.model_trainer)(
                    train_set.clone(),
                    &*model_params,
                    &metrics_handler,
                    "DEBUG",
                );
                log::debug!("Finished fold {} training", fold + 1);

                (self.model_evaluator)(
                    validation_set,
                    &trained_model,
                    &mut validation_stats_builder as &mut dyn MetricConsumer,
                );
                (self.model_evaluator)(
                    train_set,
                    &trained_model,
                    &mut train_stats_builder as &mut dyn MetricConsumer,
                );
            }

            let candidate_stats = ModelCandidateStats::new(
                serde_json::to_value(model_params.to_map()).unwrap(),
                train_stats_builder.build(),
                validation_stats_builder.build(),
            );
            training_statistics.add_candidate_stats(candidate_stats);

            let validation_stats = training_statistics.validation_metrics_avg(trial);
            let train_stats = training_statistics.train_metrics_avg(trial);
            let main_metric = training_statistics.get_main_metric(trial);

            log::info!(
                "Main validation metric ({}): {:.4}",
                training_statistics.evaluation_metric(),
                main_metric
            );
            log::info!("Validation metrics: {:?}", validation_stats);
            log::info!("Training metrics: {:?}", train_stats);

            log::debug!("Completed trial {}", trial + 1);
        }

        let best_trial = training_statistics.best_trial_idx() + 1;
        let best_trial_score = training_statistics.best_trial_score();
        log::info!(
            "Best trial was Trial {} with main validation metric {:.4}",
            best_trial,
            best_trial_score
        );
    }
}
