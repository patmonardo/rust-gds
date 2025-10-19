use super::Updater;
use crate::ml::core::tensor::Tensor;
use parking_lot::RwLock;
use std::sync::Arc;

const CLIP_MAX: f64 = 5.0;
const CLIP_MIN: f64 = -5.0;

/// Adam optimizer implementation based on https://arxiv.org/pdf/1412.6980.pdf
/// 
/// Division, squaring and square-rooting is done element-wise.
pub struct AdamOptimizer {
    learning_rate: f64,
    beta_1: f64,
    beta_2: f64,
    epsilon: f64,
    weights: Vec<Arc<RwLock<Box<dyn Tensor>>>>,
    momentum_terms: Vec<Box<dyn Tensor>>,
    velocity_terms: Vec<Box<dyn Tensor>>,
    iteration: usize,
}

impl AdamOptimizer {
    /// Calculate memory size in bytes for given dimensions
    pub fn size_in_bytes(rows: usize, cols: usize) -> usize {
        let term_size = rows * cols * std::mem::size_of::<f64>();
        std::mem::size_of::<Self>() + 
            2 * term_size + // fields
            2 * term_size   // working memory: mCap, vCap
    }

    /// Create a new Adam optimizer
    pub fn new(weights: Vec<Arc<RwLock<Box<dyn Tensor>>>>, learning_rate: f64) -> Self {
        let momentum_terms: Vec<Box<dyn Tensor>> = weights
            .iter()
            .map(|w| w.read().create_with_same_dimensions())
            .collect();
        
        let velocity_terms: Vec<Box<dyn Tensor>> = weights
            .iter()
            .map(|w| w.read().create_with_same_dimensions())
            .collect();

        Self {
            learning_rate,
            beta_1: 0.9,
            beta_2: 0.999,
            epsilon: 1e-8,
            weights,
            momentum_terms,
            velocity_terms,
            iteration: 0,
        }
    }

    /// Clip gradient values to avoid exploding gradients
    fn clip(value: f64) -> f64 {
        if value > CLIP_MAX {
            CLIP_MAX
        } else {
            value.max(CLIP_MIN)
        }
    }
}

impl Updater for AdamOptimizer {
    fn update(&mut self, context_local_weight_gradients: &[Box<dyn Tensor>]) {
        self.iteration += 1;

        for i in 0..self.weights.len() {
            let mut weight = self.weights[i].write();
            let gradient = context_local_weight_gradients[i].as_ref();
            let momentum_term = self.momentum_terms[i].as_mut();
            let velocity_term = self.velocity_terms[i].as_mut();

            // Clip gradient to avoid exploding gradients
            let mut clipped_gradient = gradient.map(Self::clip);

            // In-Place update momentum term
            // m_t = beta_1 * m_t + (1 - beta_1) * g_t
            momentum_term.scalar_multiply_mutate(self.beta_1);
            let scaled_gradient = clipped_gradient.as_ref().scalar_multiply(1.0 - self.beta_1);
            momentum_term.add_inplace(scaled_gradient.as_ref());

            // In-Place updates the velocity terms
            // v_t = beta_2 * v_t + (1 - beta_2) * (g_t^2)
            clipped_gradient.map_inplace(|v| v * v);
            velocity_term.scalar_multiply_mutate(self.beta_2);
            let scaled_squared = clipped_gradient.as_ref().scalar_multiply(1.0 - self.beta_2);
            velocity_term.add_inplace(scaled_squared.as_ref());

            // m_cap = m_t / (1 - beta_1^t)  // calculates the bias-corrected estimates
            let m_cap = momentum_term.scalar_multiply(
                1.0 / (1.0 - self.beta_1.powi(self.iteration as i32))
            );

            // v_cap = v_t / (1 - beta_2^t)  // calculates the bias-corrected estimates
            let v_cap = velocity_term.scalar_multiply(
                1.0 / (1.0 - self.beta_2.powi(self.iteration as i32))
            );

            // theta_0 = theta_0 - (alpha * m_cap) / (sqrt(v_cap) + epsilon)  // updates the parameters
            let mut update = m_cap.scalar_multiply(-self.learning_rate);
            let mut v_cap_sqrt = v_cap.map(f64::sqrt);
            let mut epsilon_tensor = v_cap_sqrt.ones_like();
            epsilon_tensor.scalar_multiply_mutate(self.epsilon);
            v_cap_sqrt.add_inplace(epsilon_tensor.as_ref());
            let v_cap_inv = v_cap_sqrt.map(|v| 1.0 / v);
            update = update.elementwise_product(v_cap_inv.as_ref());
            
            weight.add_inplace(update.as_ref());
        }
    }
}
