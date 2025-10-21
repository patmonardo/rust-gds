use crate::types::{DefaultValue, ValueType};
use serde::{Deserialize, Serialize};

/// Descriptor for a procedure exposed through the public surface.
///
/// The descriptor captures static, code-generated metadata that lets
/// the projection codegen system wire procedures into multiple frontends
/// (N-API, TypeScript facades, CLIs) without depending on runtime traits.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcedureDescriptor {
    /// Fully qualified name such as `gds.pageRank.stream`.
    pub qualified_name: String,
    /// Domain grouping used by catalogs and documentation.
    pub category: ProcedureCategory,
    /// Available execution modes (e.g. `stream`, `stats`, `mutate`).
    pub modes: Vec<ProcedureMode>,
    /// Description surfaced to users.
    pub description: Option<String>,
    /// Configuration envelope shape exposed to callers.
    pub config_format: ProcedureConfigFormat,
    /// Parameter definitions for the procedure facade.
    pub parameters: Vec<ProcedureParameterDescriptor>,
    /// How this procedure is surfaced to external APIs.
    pub facade: Option<ProcedureFacadeDescriptor>,
    /// If present, indicates deprecation and optionally carries guidance.
    pub deprecation_notice: Option<String>,
    /// The form of the response payload expected back from execution.
    pub return_format: ProcedureReturnFormat,
}

impl ProcedureDescriptor {
    /// Construct a new descriptor with a qualified name and category.
    pub fn new(qualified_name: impl Into<String>, category: ProcedureCategory) -> Self {
        Self {
            qualified_name: qualified_name.into(),
            category,
            modes: Vec::new(),
            description: None,
            config_format: ProcedureConfigFormat::ConfigObject,
            parameters: Vec::new(),
            facade: None,
            deprecation_notice: None,
            return_format: ProcedureReturnFormat::Stream,
        }
    }

    /// Attach execution modes.
    pub fn with_modes<I>(mut self, modes: I) -> Self
    where
        I: IntoIterator<Item = ProcedureMode>,
    {
        self.modes = modes.into_iter().collect();
        self
    }

    /// Provide a description for catalog surfaces.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Override the configuration envelope format.
    pub fn with_config_format(mut self, format: ProcedureConfigFormat) -> Self {
        self.config_format = format;
        self
    }

    /// Add a single facade descriptor (TypeScript/CLI exposure).
    pub fn with_facade(mut self, facade: ProcedureFacadeDescriptor) -> Self {
        self.facade = Some(facade);
        self
    }

    /// Mark procedure as deprecated with optional guidance text.
    pub fn deprecated(mut self, notice: impl Into<String>) -> Self {
        self.deprecation_notice = Some(notice.into());
        self
    }

    /// Set explicit return format.
    pub fn with_return_format(mut self, return_format: ProcedureReturnFormat) -> Self {
        self.return_format = return_format;
        self
    }

    /// Attach parameters replacing the existing list.
    pub fn with_parameters<I>(mut self, params: I) -> Self
    where
        I: IntoIterator<Item = ProcedureParameterDescriptor>,
    {
        self.parameters = params.into_iter().collect();
        self
    }

    /// Push a single parameter descriptor.
    pub fn push_parameter(mut self, param: ProcedureParameterDescriptor) -> Self {
        self.parameters.push(param);
        self
    }
}

/// Procedure category matches documentation groupings.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProcedureCategory {
    Centrality,
    CommunityDetection,
    Similarity,
    PathFinding,
    NodeEmbeddings,
    MachineLearning,
    Utility,
    GraphManagement,
    Pipeline,
}

/// Execution surface offered for a procedure.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProcedureMode {
    Stream,
    Stats,
    Estimation,
    Mutate,
    Write,
    Train,
    Predict,
}

/// Shape of the configuration payload accepted by the procedure.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum ProcedureConfigFormat {
    /// Classic `config { }` map as seen in Neo4j GDS 1.x/2.x.
    #[default]
    ConfigObject,
    /// Array-based positional params (rare, mostly alpha procedures).
    Positional,
    /// Builder-style strongly typed config struct (Rust-first APIs).
    Struct,
}

/// How the procedure result payload is shaped when surfaced to clients.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProcedureReturnFormat {
    Stream,
    Stats,
    Graph,
    Model,
}

/// Parameters emitted for TypeScript bindings and documentation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcedureParameterDescriptor {
    pub name: String,
    pub kind: ProcedureParameterKind,
    pub value_type: ValueType,
    pub required: bool,
    pub default_value: Option<DefaultValue>,
    pub description: Option<String>,
}

impl ProcedureParameterDescriptor {
    pub fn new(
        name: impl Into<String>,
        kind: ProcedureParameterKind,
        value_type: ValueType,
    ) -> Self {
        Self {
            name: name.into(),
            kind,
            value_type,
            required: true,
            default_value: None,
            description: None,
        }
    }

    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn with_default(mut self, default_value: DefaultValue) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Facade metadata for generated bindings.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcedureFacadeDescriptor {
    /// Rust module path (e.g. `crate::procedures::algo::pagerank`).
    pub rust_module: String,
    /// Symbol exported to the binding surface (e.g. `page_rank`).
    pub rust_symbol: String,
    /// Optional TypeScript/JS name if it differs from the Rust export.
    pub export_name: Option<String>,
}

impl ProcedureFacadeDescriptor {
    pub fn new(rust_module: impl Into<String>, rust_symbol: impl Into<String>) -> Self {
        Self {
            rust_module: rust_module.into(),
            rust_symbol: rust_symbol.into(),
            export_name: None,
        }
    }

    pub fn with_export_name(mut self, export_name: impl Into<String>) -> Self {
        self.export_name = Some(export_name.into());
        self
    }
}

/// Kind of parameter, used to render friendly documentation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ProcedureParameterKind {
    Graph,
    Config,
    Data,
    Metadata,
}
