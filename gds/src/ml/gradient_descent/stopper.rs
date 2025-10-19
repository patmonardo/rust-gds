use super::config::GradientDescentConfig;

pub trait TrainingStopper {
    fn register_loss(&mut self, loss: f64);
    fn terminated(&self) -> bool;
    fn converged(&self) -> bool;
}

/// Factory functions for TrainingStopper
pub mod factory {
    use super::*;

    /// Create a default stopper from config (matching Java's defaultStopper)
    pub fn default_stopper(config: &GradientDescentConfig) -> StreakStopper {
        StreakStopper::from_config(config)
    }
}

pub struct StreakStopper {
    min_epochs: usize,
    patience: usize,
    max_epochs: usize,
    tolerance: f64,
    ran_epochs: usize,
    best_loss: f64,
    unproductive_streak: usize,
}

impl StreakStopper {
    pub fn new(min_epochs: usize, patience: usize, max_epochs: usize, tolerance: f64) -> Self {
        Self {
            min_epochs,
            patience,
            max_epochs,
            tolerance,
            ran_epochs: 0,
            best_loss: f64::MAX,
            unproductive_streak: 0,
        }
    }

    pub fn from_config(config: &GradientDescentConfig) -> Self {
        Self::new(
            config.min_epochs(),
            config.patience(),
            config.max_epochs(),
            config.tolerance(),
        )
    }
}

impl TrainingStopper for StreakStopper {
    fn register_loss(&mut self, loss: f64) {
        if self.terminated() {
            panic!("Does not accept losses after convergence");
        }

        if self.ran_epochs >= self.min_epochs {
            if (loss - self.best_loss) >= -self.tolerance * self.best_loss.abs() {
                self.unproductive_streak += 1;
            } else {
                self.unproductive_streak = 0;
            }
        }

        self.ran_epochs += 1;
        self.best_loss = self.best_loss.min(loss);
    }

    fn terminated(&self) -> bool {
        self.ran_epochs >= self.max_epochs || self.unproductive_streak >= self.patience
    }

    fn converged(&self) -> bool {
        self.unproductive_streak >= self.patience
    }
}
