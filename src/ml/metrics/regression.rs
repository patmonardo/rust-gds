use crate::ml::metrics::{Metric, MetricComparator};

#[derive(Debug, Clone, Copy)]
pub enum RegressionMetric {
    MSE,
    MAE,
    RMSE,
    R2,
}

impl RegressionMetric {
    pub fn compute(&self, targets: &[f64], predictions: &[f64]) -> f64 {
        match self {
            Self::MSE => self.compute_mse(targets, predictions),
            Self::MAE => self.compute_mae(targets, predictions),
            Self::RMSE => self.compute_rmse(targets, predictions),
            Self::R2 => self.compute_r2(targets, predictions),
        }
    }

    fn compute_mse(&self, targets: &[f64], predictions: &[f64]) -> f64 {
        let n = targets.len() as f64;
        targets
            .iter()
            .zip(predictions)
            .map(|(t, p)| (t - p).powi(2))
            .sum::<f64>()
            / n
    }

    fn compute_mae(&self, targets: &[f64], predictions: &[f64]) -> f64 {
        let n = targets.len() as f64;
        targets
            .iter()
            .zip(predictions)
            .map(|(t, p)| (t - p).abs())
            .sum::<f64>()
            / n
    }

    fn compute_rmse(&self, targets: &[f64], predictions: &[f64]) -> f64 {
        self.compute_mse(targets, predictions).sqrt()
    }

    fn compute_r2(&self, targets: &[f64], predictions: &[f64]) -> f64 {
        let mean = targets.iter().sum::<f64>() / targets.len() as f64;
        let ss_tot = targets.iter().map(|t| (t - mean).powi(2)).sum::<f64>();
        let ss_res = targets
            .iter()
            .zip(predictions)
            .map(|(t, p)| (t - p).powi(2))
            .sum::<f64>();

        1.0 - ss_res / ss_tot
    }
}

impl Metric for RegressionMetric {
    fn name(&self) -> &str {
        match self {
            Self::MSE => "MSE",
            Self::MAE => "MAE",
            Self::RMSE => "RMSE",
            Self::R2 => "R2",
        }
    }

    fn comparator(&self) -> MetricComparator {
        match self {
            Self::R2 => MetricComparator::Natural,
            _ => MetricComparator::Inverse,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mse() {
        let targets = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let predictions = vec![1.1, 2.2, 2.8, 4.2, 4.9];
        let mse = RegressionMetric::MSE.compute(&targets, &predictions);
        assert!((mse - 0.05).abs() < 1e-10);
    }

    #[test]
    fn test_r2() {
        let targets = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let predictions = vec![1.1, 2.2, 2.8, 4.2, 4.9];
        let r2 = RegressionMetric::R2.compute(&targets, &predictions);
        assert!(r2 > 0.95);
    }
}
