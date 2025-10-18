use derive_more::From;
use serde::{Deserialize, Serialize};
use std::any::Any;

pub trait ConcreteParameter: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, From, Serialize, Deserialize)]
pub struct DoubleParameter(pub f64);

#[derive(Debug, Clone, From, Serialize, Deserialize)]
pub struct IntegerParameter(pub i32);

#[derive(Debug, Clone, From, Serialize, Deserialize)]
pub struct StringParameter(pub String);

#[derive(Debug, Clone, From, Serialize, Deserialize)]
pub struct ListParameter(pub Vec<i32>);

impl ConcreteParameter for DoubleParameter {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ConcreteParameter for IntegerParameter {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ConcreteParameter for StringParameter {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ConcreteParameter for ListParameter {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumericalRange<T> {
    pub min: T,
    pub max: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoubleRange {
    pub range: NumericalRange<f64>,
    pub log_scale: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegerRange {
    pub range: NumericalRange<i32>,
}

impl DoubleRange {
    pub fn new(min: f64, max: f64, log_scale: bool) -> Self {
        Self {
            range: NumericalRange { min, max },
            log_scale,
        }
    }
}

impl IntegerRange {
    pub fn new(min: i32, max: i32) -> Self {
        Self {
            range: NumericalRange { min, max },
        }
    }
}
