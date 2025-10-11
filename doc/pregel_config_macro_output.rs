// This file demonstrates the *macro-generated* output we would expect from
// an attribute macro applied to the user-defined `PregelConfig` struct.
// It's kept under `doc/` for review and does not compile into the library.

// === Macro-generated code (illustrative) ===

// Builder type generated for PregelConfig
pub struct PregelConfigBuilder {
    concurrency: Option<usize>,
    max_iterations: Option<usize>,
    tolerance: Option<f64>,
    is_asynchronous: Option<bool>,
    partitioning: Option<crate::config::Partitioning>,
    track_sender: Option<bool>,
}

impl Default for PregelConfigBuilder {
    fn default() -> Self {
        Self {
            concurrency: None,
            max_iterations: None,
            tolerance: None,
            is_asynchronous: None,
            partitioning: None,
            track_sender: None,
        }
    }
}

impl PregelConfigBuilder {
    pub fn concurrency(mut self, v: usize) -> Self { self.concurrency = Some(v); self }
    pub fn max_iterations(mut self, v: usize) -> Self { self.max_iterations = Some(v); self }
    pub fn tolerance(mut self, v: f64) -> Self { self.tolerance = Some(v); self }
    pub fn is_asynchronous(mut self, v: bool) -> Self { self.is_asynchronous = Some(v); self }
    pub fn partitioning(mut self, v: crate::config::Partitioning) -> Self { self.partitioning = Some(v); self }
    pub fn track_sender(mut self, v: bool) -> Self { self.track_sender = Some(v); self }

    // build() performs validation and returns the concrete config struct
    pub fn build(self) -> Result<crate::config::PregelConfig, crate::config::validation::ConfigError> {
        // This mirrors the hand-written validation in the existing code
        let defaults = crate::config::PregelConfig::default();

        let cfg = crate::config::PregelConfig {
            base: crate::config::base_types::AlgoBaseConfig { concurrency: self.concurrency.unwrap_or(defaults.base.concurrency), node_labels: defaults.base.node_labels, relationship_types: defaults.base.relationship_types },
            max_iterations: self.max_iterations.unwrap_or(defaults.max_iterations),
            tolerance: self.tolerance.or(defaults.tolerance),
            is_asynchronous: self.is_asynchronous.unwrap_or(defaults.is_asynchronous),
            partitioning: self.partitioning.unwrap_or(defaults.partitioning),
            track_sender: self.track_sender.unwrap_or(defaults.track_sender),
        };

        cfg.validate()?;
        Ok(cfg)
    }
}

// Generated helper: metadata about the config (used by tooling / AI)
pub fn pregel_config_metadata() -> &'static str {
    r#"{
  "name": "PregelConfig",
  "fields": [
    {"name":"base","type":"AlgoBaseConfig","default":null},
    {"name":"max_iterations","type":"usize","default":20},
    {"name":"tolerance","type":"Option<f64>","default":null},
    {"name":"is_asynchronous","type":"bool","default":false},
    {"name":"partitioning","type":"Partitioning","default":"Range"},
    {"name":"track_sender","type":"bool","default":false}
  ]
}"
}

// Example of a generated trait impl bridging to the old trait-based API
// (the actual trait lives in `src/pregel/config.rs`).
// impl pregel::PregelConfig for crate::config::PregelConfig { ... }

// End of illustrative macro output
// Example: macro-expanded output for `#[config]` applied to `PregelConfig`
// This file is for illustration only (not compiled). It shows what a proc-macro
// like `#[config]` would generate for the existing `PregelConfig`.

use crate::config::validation::ConfigError;
use crate::config::validation::ConfigValidation;
use crate::projection::Partitioning;
use crate::types::ValueType;

// --- User-declared input (what they wrote) ---------------------------------
//
// #[config(kind = "pregel")]
// pub struct PregelConfig {
//     #[config(default = 20, validate = "positive")]
//     pub max_iterations: usize,
//
//     #[config(default = None)]
//     pub tolerance: Option<f64>,
//
//     #[config(default = false)]
//     pub is_asynchronous: bool,
//
//     #[config(default = Partitioning::Range)]
//     pub partitioning: Partitioning,
//
//     #[config(default = false)]
//     pub track_sender: bool,
// }
//
// The macro expands the following generated code:

// --- Generated: typed builder ------------------------------------------------
#[allow(dead_code)]
pub struct PregelConfigBuilder {
    max_iterations: Option<usize>,
    tolerance: Option<Option<f64>>,
    is_asynchronous: Option<bool>,
    partitioning: Option<Partitioning>,
    track_sender: Option<bool>,
}

impl Default for PregelConfigBuilder {
    fn default() -> Self {
        Self {
            max_iterations: None,
            tolerance: None,
            is_asynchronous: None,
            partitioning: None,
            track_sender: None,
        }
    }
}

impl PregelConfigBuilder {
    pub fn max_iterations(mut self, v: usize) -> Self {
        self.max_iterations = Some(v);
        self
    }

    pub fn tolerance(mut self, v: Option<f64>) -> Self {
        self.tolerance = Some(v);
        self
    }

    pub fn is_asynchronous(mut self, v: bool) -> Self {
        self.is_asynchronous = Some(v);
        self
    }

    pub fn partitioning(mut self, v: Partitioning) -> Self {
        self.partitioning = Some(v);
        self
    }

    pub fn track_sender(mut self, v: bool) -> Self {
        self.track_sender = Some(v);
        self
    }

    /// Validate and build the final config struct
    pub fn build(self) -> Result<PregelConfig, ConfigError> {
        // apply defaults (same defaults as existing manual impl)
        let defaults = PregelConfig::default();

        let cfg = PregelConfig {
            base: crate::config::base_types::AlgoBaseConfig::default(), // macro could copy base defaults
            max_iterations: self.max_iterations.unwrap_or(defaults.max_iterations),
            tolerance: self.tolerance.unwrap_or(defaults.tolerance),
            is_asynchronous: self.is_asynchronous.unwrap_or(defaults.is_asynchronous),
            partitioning: self.partitioning.unwrap_or(defaults.partitioning),
            track_sender: self.track_sender.unwrap_or(defaults.track_sender),
        };

        // run generated validation hooks
        cfg.validate()?;
        Ok(cfg)
    }
}

// --- Generated: impls on the struct -----------------------------------------
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PregelConfig {
    pub base: crate::config::base_types::AlgoBaseConfig,
    pub max_iterations: usize,
    pub tolerance: Option<f64>,
    pub is_asynchronous: bool,
    pub partitioning: Partitioning,
    pub track_sender: bool,
}

impl Default for PregelConfig {
    fn default() -> Self {
        Self {
            base: crate::config::base_types::AlgoBaseConfig::default(),
            max_iterations: 20,
            tolerance: None,
            is_asynchronous: false,
            partitioning: Partitioning::Range,
            track_sender: false,
        }
    }
}

// Generated: config trait impls (bridge to existing traits used by runtime)
impl crate::config::base_types::Config for PregelConfig {}

impl crate::config::base_types::ConcurrencyConfig for PregelConfig {
    fn concurrency(&self) -> usize {
        self.base.concurrency
    }
}

impl crate::config::base_types::IterationsConfig for PregelConfig {
    fn max_iterations(&self) -> usize {
        self.max_iterations
    }
    fn tolerance(&self) -> Option<f64> {
        self.tolerance
    }
}

// Generated: validation function (calls into shared validators or inline)
impl PregelConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        ConfigValidation::validate_positive(self.base.concurrency as f64, "concurrency")?;
        ConfigValidation::validate_positive(self.max_iterations as f64, "maxIterations")?;
        if let Some(tol) = self.tolerance {
            ConfigValidation::validate_positive(tol, "tolerance")?;
        }
        Ok(())
    }

    // generated convenience: builder() constructor
    pub fn builder() -> PregelConfigBuilder {
        PregelConfigBuilder::default()
    }

    // Generated metadata function for AI/codegen consumers
    pub fn metadata() -> ConfigMetadata {
        ConfigMetadata {
            name: "PregelConfig".to_string(),
            fields: vec![
                FieldMetadata::new(
                    "max_iterations",
                    "usize",
                    Some("20".to_string()),
                    Some("positive".to_string()),
                ),
                FieldMetadata::new("tolerance", "Option<f64>", None, None),
                FieldMetadata::new("is_asynchronous", "bool", Some("false".to_string()), None),
                FieldMetadata::new(
                    "partitioning",
                    "Partitioning",
                    Some("Range".to_string()),
                    None,
                ),
                FieldMetadata::new("track_sender", "bool", Some("false".to_string()), None),
            ],
        }
    }
}

// --- Generated: helpful metadata types for tooling / AI ---------------------
#[allow(dead_code)]
pub struct ConfigMetadata {
    pub name: String,
    pub fields: Vec<FieldMetadata>,
}

#[allow(dead_code)]
pub struct FieldMetadata {
    pub name: &'static str,
    pub ty: &'static str,
    pub default: Option<String>,
    pub validation: Option<String>,
}

impl FieldMetadata {
    pub fn new(
        name: &'static str,
        ty: &'static str,
        default: Option<String>,
        validation: Option<String>,
    ) -> Self {
        Self {
            name,
            ty,
            default,
            validation,
        }
    }
}

// --- Generated: optional registration helper -------------------------------
#[allow(dead_code)]
pub fn register_pregel_config() -> bool {
    // The macro could emit a registry entry call so tooling discovers available configs.
    // For illustration, pretend there's a global ConfigRegistry API.
    // ConfigRegistry::register("PregelConfig", PregelConfig::metadata());
    true
}

// End of example macro expansion
