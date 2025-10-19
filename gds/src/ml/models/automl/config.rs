use super::parameter::*;
use crate::ml::models::TrainerConfig;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const LOG_SCALE_PARAMETERS: &[&str] = &["penalty", "learning_rate", "tolerance"];
const EPSILON: f64 = 1e-8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunableTrainerConfig {
    concrete_parameters: HashMap<String, Box<dyn ConcreteParameter>>,
    double_ranges: HashMap<String, DoubleRange>,
    integer_ranges: HashMap<String, IntegerRange>,
}

impl TunableTrainerConfig {
    pub fn new(
        concrete_parameters: HashMap<String, Box<dyn ConcreteParameter>>,
        double_ranges: HashMap<String, DoubleRange>,
        integer_ranges: HashMap<String, IntegerRange>,
    ) -> Self {
        Self {
            concrete_parameters,
            double_ranges,
            integer_ranges,
        }
    }

    pub fn concrete_parameters(&self) -> &HashMap<String, Box<dyn ConcreteParameter>> {
        &self.concrete_parameters
    }

    pub fn double_ranges(&self) -> &HashMap<String, DoubleRange> {
        &self.double_ranges
    }

    pub fn integer_ranges(&self) -> &HashMap<String, IntegerRange> {
        &self.integer_ranges
    }

    pub fn is_concrete(&self) -> bool {
        self.double_ranges.is_empty() && self.integer_ranges.is_empty()
    }

    pub fn to_trainer_config(&self) -> TrainerConfig {
        assert!(
            self.is_concrete(),
            "Cannot convert non-concrete config to TrainerConfig"
        );
        TrainerConfig::from_parameters(self.concrete_parameters.clone())
    }

    pub fn parse(input: &HashMap<String, serde_json::Value>) -> Result<Self> {
        let mut concrete_params = HashMap::new();
        let mut double_ranges = HashMap::new();
        let mut integer_ranges = HashMap::new();

        for (key, value) in input {
            if let Some(range) = value.as_object().and_then(|obj| obj.get("range")) {
                let range_values = range
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("Range must be an array"))?;

                if range_values.len() != 2 {
                    anyhow::bail!("Range must have exactly 2 values");
                }

                // Parse ranges based on value type
                if range_values[0].is_f64() {
                    let min = range_values[0].as_f64().unwrap();
                    let max = range_values[1].as_f64().unwrap();
                    let log_scale = LOG_SCALE_PARAMETERS.contains(&key.as_str());
                    double_ranges.insert(key.clone(), DoubleRange::new(min, max, log_scale));
                } else if range_values[0].is_i64() {
                    let min = range_values[0].as_i64().unwrap() as i32;
                    let max = range_values[1].as_i64().unwrap() as i32;
                    integer_ranges.insert(key.clone(), IntegerRange::new(min, max));
                }
            } else {
                // Parse concrete parameters
                match value {
                    serde_json::Value::Number(n) => {
                        if n.is_i64() {
                            concrete_params.insert(
                                key.clone(),
                                Box::new(IntegerParameter(n.as_i64().unwrap() as i32)),
                            );
                        } else if n.is_f64() {
                            concrete_params.insert(
                                key.clone(),
                                Box::new(DoubleParameter(n.as_f64().unwrap())),
                            );
                        }
                    }
                    serde_json::Value::String(s) => {
                        concrete_params.insert(key.clone(), Box::new(StringParameter(s.clone())));
                    }
                    serde_json::Value::Array(arr) => {
                        if arr.iter().all(|v| v.is_i64()) {
                            concrete_params.insert(
                                key.clone(),
                                Box::new(ListParameter(
                                    arr.iter().map(|v| v.as_i64().unwrap() as i32).collect(),
                                )),
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Self::new(concrete_params, double_ranges, integer_ranges))
    }
}
