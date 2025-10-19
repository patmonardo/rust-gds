use super::parameter::*;
use crate::ml::models::TrainerConfig;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;

/// HyperParameterOptimizer defines a strategy for searching through a hyperparameter space
pub trait HyperParameterOptimizer: Iterator<Item = TrainerConfig> {}

/// A basic implementation of random search through a hyperparameter space
pub struct RandomSearch {
    concrete_configs: Vec<TunableTrainerConfig>,
    tunable_configs: Vec<TunableTrainerConfig>,
    total_trials: usize,
    concrete_trials: usize,
    rng: ChaCha8Rng,
    finished_trials: usize,
}

impl RandomSearch {
    pub fn new(
        parameter_space: HashMap<String, Vec<TunableTrainerConfig>>,
        max_trials: usize,
        seed: Option<u64>,
    ) -> Self {
        let rng = ChaCha8Rng::seed_from_u64(seed.unwrap_or_else(random));

        let concrete = parameter_space
            .values()
            .flatten()
            .filter(|c| c.is_concrete())
            .cloned()
            .collect::<Vec<_>>();

        let tunable = parameter_space
            .values()
            .flatten()
            .filter(|c| !c.is_concrete())
            .cloned()
            .collect::<Vec<_>>();

        let concrete_count = concrete.len();
        let tunable_count = tunable.len();

        // Ensure we try each concrete config at least once
        let total = max_trials.max(concrete_count);

        Self {
            concrete_configs: concrete,
            tunable_configs: tunable,
            total_trials: total,
            concrete_trials: concrete_count,
            rng,
            finished_trials: 0,
        }
    }

    fn sample_integer(&mut self, range: &IntegerRange) -> i32 {
        self.rng.gen_range(range.range.min..=range.range.max)
    }

    fn sample_double(&mut self, range: &DoubleRange) -> f64 {
        let min = range.range.min;
        let max = range.range.max;

        if range.log_scale {
            let log_min = min.ln();
            let log_max = max.ln();
            let log_value = self.rng.gen_range(log_min..=log_max);
            log_value.exp()
        } else {
            self.rng.gen_range(min..=max)
        }
    }

    fn sample_config(&mut self, config: &TunableTrainerConfig) -> TrainerConfig {
        let mut params = config.concrete_parameters().clone();

        // Sample numerical ranges
        for (key, range) in config.double_ranges() {
            params.insert(
                key.to_string(),
                DoubleParameter(self.sample_double(range)).into(),
            );
        }

        for (key, range) in config.integer_ranges() {
            params.insert(
                key.to_string(),
                IntegerParameter(self.sample_integer(range)).into(),
            );
        }

        TrainerConfig::from_parameters(params)
    }
}

impl Iterator for RandomSearch {
    type Item = TrainerConfig;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished_trials >= self.total_trials {
            return None;
        }

        let config = if self.finished_trials < self.concrete_trials {
            // Use concrete configs first
            self.concrete_configs[self.finished_trials].to_trainer_config()
        } else {
            // Sample from tunable configs
            let idx = self.rng.gen_range(0..self.tunable_configs.len());
            self.sample_config(&self.tunable_configs[idx])
        };

        self.finished_trials += 1;
        Some(config)
    }
}
