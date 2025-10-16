# Algorithm Macro Design - Eliminating Boilerplate

**Date**: October 16, 2025  
**Purpose**: Design declarative macros to eliminate 70%+ of algorithm implementation boilerplate  
**Status**: Design Phase

---

## The Problem: Repetitive Boilerplate

### Current State (Manual Implementation)

Every algorithm requires ~450-600 lines of repetitive code:

**1. Configuration Struct** (~80 lines)

```rust
pub struct PageRankConfig {
    pub damping_factor: f64,
    pub tolerance: f64,
    pub max_iterations: usize,
    pub scaler: ScalerFactory,
    pub source_nodes: Vec<NodeId>,
}

pub struct PageRankConfigBuilder {
    damping_factor: Option<f64>,
    tolerance: Option<f64>,
    // ... 50 more lines of builder code
}

impl PageRankConfigBuilder {
    pub fn damping_factor(mut self, value: f64) -> Result<Self, ConfigError> {
        if value <= 0.0 || value >= 1.0 {
            return Err(ConfigError::InvalidRange { /* ... */ });
        }
        self.damping_factor = Some(value);
        Ok(self)
    }
    // ... 40 more lines of validation
}
```

**2. AlgorithmSpec Implementation** (~150 lines)

```rust
impl AlgorithmSpec for PageRankAlgorithm {
    type Output = PageRankResult;

    fn name(&self) -> &str { "pagerank" }
    fn graph_name(&self) -> &str { &self.graph_name }
    fn projection_hint(&self) -> ProjectionHint { ProjectionHint::Dense }
    fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError> { /* ... */ }
    fn execute(&self, graph, config, context) -> Result<ComputationResult<Self::Output>, AlgorithmError> { /* ... */ }
    fn consume_result(&self, result, mode) -> Result<Self::Output, ConsumerError> { /* ... */ }
}
```

**3. Execution Mode Wrappers** (~200 lines total)

```rust
pub struct MutatePageRank { /* ... */ }
pub struct WritePageRank { /* ... */ }
pub struct StatsPageRank { /* ... */ }
pub struct StreamPageRank { /* ... */ }

impl MutatePageRank {
    pub fn run(&self) -> Result<MutateResult, Error> {
        // Boilerplate: call algorithm, write to graph_store
    }
}
// ... 150 more lines for write/stats/stream
```

**Total per algorithm**: ~450 lines of boilerplate, ~150 lines of actual logic

---

## The Solution: Declarative Macros

### Macro 1: `algorithm_config!` - Configuration Generation

**Invocation** (~15 lines):

```rust
algorithm_config! {
    /// PageRank configuration
    pub struct PageRankConfig {
        /// Damping factor for random walk
        #[default(0.85)]
        #[range(0.0..1.0)]
        pub damping_factor: f64,

        /// Convergence tolerance
        #[default(1e-7)]
        #[min(0.0)]
        pub tolerance: f64,

        /// Maximum iterations
        #[default(20)]
        #[min(1)]
        pub max_iterations: usize,

        /// Scaler for result normalization
        #[default(ScalerFactory::none())]
        pub scaler: ScalerFactory,

        /// Source nodes for PersonalizedPageRank
        #[default(vec![])]
        pub source_nodes: Vec<NodeId>,
    }
}
```

**Generates** (~80 lines):

- Struct definition with doc comments
- Builder struct with validation
- `Default` implementation
- `Serialize`/`Deserialize` for JSON
- Validation error messages

### Macro 2: `define_algorithm!` - Full Algorithm Registration

**Invocation** (~30 lines):

```rust
define_algorithm! {
    /// PageRank algorithm
    name: PageRank,
    category: Centrality,
    description: "Computes PageRank scores via power iteration",

    config: PageRankConfig,
    result: PageRankResult,
    algorithm: PageRankAlgorithm,

    projection_hint: Dense,

    modes: [mutate, write, stats, stream],

    execute: |algorithm, graph, config, context| {
        algorithm.compute(graph, config, context)
    },

    consume_result: |result, mode| {
        match mode {
            ExecutionMode::Mutate => result.to_mutate_result(),
            ExecutionMode::Write => result.to_write_result(),
            ExecutionMode::Stats => result.to_stats_result(),
            ExecutionMode::Stream => result.to_stream_result(),
        }
    },
}
```

**Generates** (~350 lines):

- `AlgorithmSpec` implementation
- All 4 execution mode wrappers (Mutate/Write/Stats/Stream)
- Catalog registration
- Error handling boilerplate
- Progress tracking hooks

---

## Architecture: The Trilogy Unified

### Current Architecture (Separated)

**Layer 1: Configuration** (src/config/)

```rust
// Manual: ~80 lines per config
pub struct PageRankConfig { /* ... */ }
pub struct PageRankConfigBuilder { /* ... */ }
```

**Layer 2: Algorithm Implementation** (src/procedure/algo/)

```rust
// Manual: ~150 lines per algorithm
pub struct PageRankAlgorithm { /* ... */ }
impl PageRankAlgorithm {
    pub fn compute(&self, ...) -> Result<PageRankResult> { /* actual logic */ }
}
```

**Layer 3: AlgorithmSpec Integration** (src/projection/eval/procedure/)

```rust
// Manual: ~150 lines per algorithm
impl AlgorithmSpec for PageRankAlgorithm { /* ... */ }
```

**Layer 4: Execution Modes** (src/procedure/facade/)

```rust
// Manual: ~200 lines per algorithm
pub struct MutatePageRank { /* ... */ }
pub struct WritePageRank { /* ... */ }
pub struct StatsPageRank { /* ... */ }
pub struct StreamPageRank { /* ... */ }
```

**Total**: ~580 lines of boilerplate per algorithm

### New Architecture (Macro-Unified)

**Single Declaration** (~30 lines):

```rust
define_algorithm! {
    name: PageRank,
    config: PageRankConfig { /* field declarations */ },
    algorithm: PageRankAlgorithm,
    modes: [mutate, write, stats, stream],
    // ... minimal glue code
}
```

**Generates All Layers**:

- Config struct + builder + validation
- AlgorithmSpec implementation
- All 4 execution mode wrappers
- Catalog registration

**Result**: 30 lines declaration → 580 lines generated = **95% reduction in boilerplate**

---

## Detailed Macro Design

### Macro 1: `algorithm_config!`

**Purpose**: Generate configuration struct with validation

**Syntax**:

```rust
algorithm_config! {
    $(#[$meta:meta])*
    $vis:vis struct $name:ident {
        $(
            $(#[$field_meta:meta])*
            $field_vis:vis $field_name:ident: $field_type:ty
        ),* $(,)?
    }
}
```

**Attributes Supported**:

- `#[default(expr)]` - Default value for field
- `#[range(min..max)]` - Numeric range validation
- `#[min(value)]` - Minimum value (inclusive)
- `#[max(value)]` - Maximum value (inclusive)
- `#[choices(a, b, c)]` - Enum-like choices
- `#[optional]` - Field is `Option<T>`, no default required

**Generated Code**:

1. **Main Struct**:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRankConfig {
    pub damping_factor: f64,
    pub tolerance: f64,
    pub max_iterations: usize,
    pub scaler: ScalerFactory,
    pub source_nodes: Vec<NodeId>,
}
```

2. **Builder Struct**:

```rust
pub struct PageRankConfigBuilder {
    damping_factor: Option<f64>,
    tolerance: Option<f64>,
    max_iterations: Option<usize>,
    scaler: Option<ScalerFactory>,
    source_nodes: Option<Vec<NodeId>>,
}
```

3. **Builder Methods with Validation**:

```rust
impl PageRankConfigBuilder {
    pub fn damping_factor(mut self, value: f64) -> Result<Self, ConfigError> {
        // Generated from #[range(0.0..1.0)]
        if value <= 0.0 || value >= 1.0 {
            return Err(ConfigError::InvalidRange {
                field: "damping_factor",
                value: value.to_string(),
                min: "0.0".to_string(),
                max: "1.0".to_string(),
            });
        }
        self.damping_factor = Some(value);
        Ok(self)
    }

    // Similar for other fields...

    pub fn build(self) -> Result<PageRankConfig, ConfigError> {
        Ok(PageRankConfig {
            damping_factor: self.damping_factor.unwrap_or(0.85),
            tolerance: self.tolerance.unwrap_or(1e-7),
            max_iterations: self.max_iterations.unwrap_or(20),
            scaler: self.scaler.unwrap_or_else(|| ScalerFactory::none()),
            source_nodes: self.source_nodes.unwrap_or_default(),
        })
    }
}
```

4. **Default Implementation**:

```rust
impl Default for PageRankConfig {
    fn default() -> Self {
        Self::builder().build().expect("default config should be valid")
    }
}
```

5. **Builder Constructor**:

```rust
impl PageRankConfig {
    pub fn builder() -> PageRankConfigBuilder {
        PageRankConfigBuilder::default()
    }
}
```

### Macro 2: `define_algorithm!`

**Purpose**: Generate complete algorithm integration (AlgorithmSpec + execution modes)

**Syntax**:

```rust
define_algorithm! {
    $(#[$meta:meta])*
    name: $name:ident,
    category: $category:ident,
    description: $desc:literal,

    config: $config:ty,
    result: $result:ty,
    algorithm: $algorithm:ty,

    $(projection_hint: $hint:ident,)?

    modes: [$($mode:ident),*],

    execute: $execute:expr,
    consume_result: $consume:expr,

    $(
        // Optional hooks
        preprocess_config: $preprocess:expr,
        validation_config: $validation:expr,
        estimate_memory: $memory:expr,
    )?
}
```

**Generated Code**:

1. **AlgorithmSpec Implementation**:

```rust
impl AlgorithmSpec for PageRankAlgorithm {
    type Output = PageRankResult;

    fn name(&self) -> &str {
        "PageRank"
    }

    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    fn projection_hint(&self) -> ProjectionHint {
        ProjectionHint::Dense
    }

    fn parse_config(&self, input: &JsonValue) -> Result<JsonValue, ConfigError> {
        // Parse JSON into PageRankConfig using serde
        serde_json::from_value::<PageRankConfig>(input.clone())
            .map(|config| serde_json::to_value(config).unwrap())
            .map_err(|e| ConfigError::ParseError {
                algorithm: "PageRank",
                message: e.to_string(),
            })
    }

    fn execute<G: GraphStore>(
        &self,
        graph_store: &G,
        config: &JsonValue,
        context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
        let parsed_config: PageRankConfig = serde_json::from_value(config.clone())?;
        let execute_fn = $execute;
        execute_fn(self, graph_store, &parsed_config, context)
    }

    fn consume_result(
        &self,
        result: ComputationResult<Self::Output>,
        mode: &ExecutionMode,
    ) -> Result<Self::Output, ConsumerError> {
        let consume_fn = $consume;
        consume_fn(result, mode)
    }
}
```

2. **Mutate Mode Wrapper** (if `mutate` in modes):

```rust
pub struct MutatePageRank {
    algorithm: PageRankAlgorithm,
    mutate_property: String,
}

impl MutatePageRank {
    pub fn new(
        graph_name: String,
        config: PageRankConfig,
        mutate_property: String,
    ) -> Self {
        Self {
            algorithm: PageRankAlgorithm::new(graph_name, config),
            mutate_property,
        }
    }

    pub fn run<G: GraphStore>(
        &self,
        graph_store: &mut G,
        context: &ExecutionContext,
    ) -> Result<MutateResult<PageRankResult>, Error> {
        // Execute algorithm
        let result = self.algorithm.execute(
            graph_store,
            &serde_json::to_value(&self.algorithm.config)?,
            context,
        )?;

        // Write to graph_store
        graph_store.add_node_property(
            &self.mutate_property,
            result.output.scores.clone(),
        )?;

        Ok(MutateResult {
            algorithm_result: result.output,
            mutate_millis: result.compute_millis,
            property_name: self.mutate_property.clone(),
        })
    }
}
```

3. **Write Mode Wrapper** (if `write` in modes):

```rust
pub struct WritePageRank {
    algorithm: PageRankAlgorithm,
    write_property: String,
}

impl WritePageRank {
    pub fn run(
        &self,
        graph_store: &impl GraphStore,
        database: &mut impl DatabaseWriter,
        context: &ExecutionContext,
    ) -> Result<WriteResult<PageRankResult>, Error> {
        // Execute + write to database
        // ... generated code
    }
}
```

4. **Stats Mode Wrapper** (if `stats` in modes):

```rust
pub struct StatsPageRank {
    algorithm: PageRankAlgorithm,
}

impl StatsPageRank {
    pub fn run(
        &self,
        graph_store: &impl GraphStore,
        context: &ExecutionContext,
    ) -> Result<StatsResult<PageRankResult>, Error> {
        // Execute + return statistics only (no mutation/write)
        // ... generated code
    }
}
```

5. **Stream Mode Wrapper** (if `stream` in modes):

```rust
pub struct StreamPageRank {
    algorithm: PageRankAlgorithm,
}

impl StreamPageRank {
    pub fn run(
        &self,
        graph_store: &impl GraphStore,
        context: &ExecutionContext,
    ) -> Result<impl Iterator<Item = (NodeId, f64)>, Error> {
        // Execute + return iterator over results
        // ... generated code
    }
}
```

6. **Catalog Registration**:

```rust
inventory::submit! {
    AlgorithmCatalogEntry {
        name: "PageRank",
        category: AlgorithmCategory::Centrality,
        description: "Computes PageRank scores via power iteration",
        config_type: std::any::TypeId::of::<PageRankConfig>(),
        result_type: std::any::TypeId::of::<PageRankResult>(),
        constructor: Box::new(|graph_name, config| {
            Box::new(PageRankAlgorithm::new(graph_name, config))
        }),
    }
}
```

---

## Usage Examples

### Example 1: Simple Algorithm (DegreeCount)

**Declaration** (~25 lines):

```rust
use crate::algorithm_config;
use crate::define_algorithm;

algorithm_config! {
    pub struct DegreeCountConfig {
        #[default(false)]
        pub weighted: bool,

        #[default(RelationshipOrientation::Natural)]
        pub orientation: RelationshipOrientation,
    }
}

define_algorithm! {
    name: DegreeCount,
    category: Centrality,
    description: "Counts degree for each node",

    config: DegreeCountConfig,
    result: DegreeCountResult,
    algorithm: DegreeCountAlgorithm,

    projection_hint: Dense,
    modes: [mutate, write, stats, stream],

    execute: |algorithm, graph, config, _context| {
        algorithm.compute(graph, config)
    },

    consume_result: |result, mode| {
        result.output
    },
}
```

**Generated**: ~450 lines of implementation code

### Example 2: Complex Algorithm (PageRank)

**Declaration** (~40 lines):

```rust
algorithm_config! {
    pub struct PageRankConfig {
        #[default(0.85)]
        #[range(0.0..1.0)]
        pub damping_factor: f64,

        #[default(1e-7)]
        #[min(0.0)]
        pub tolerance: f64,

        #[default(20)]
        #[min(1)]
        pub max_iterations: usize,

        #[default(ScalerFactory::none())]
        pub scaler: ScalerFactory,

        #[default(vec![])]
        pub source_nodes: Vec<NodeId>,
    }
}

define_algorithm! {
    name: PageRank,
    category: Centrality,
    description: "Computes PageRank scores via power iteration",

    config: PageRankConfig,
    result: PageRankResult,
    algorithm: PageRankAlgorithm,

    projection_hint: Dense,
    modes: [mutate, write, stats, stream],

    execute: |algorithm, graph, config, context| {
        // Run PageRank with BSP
        algorithm.compute_bsp(graph, config, context)
    },

    consume_result: |result, mode| {
        match mode {
            ExecutionMode::Mutate => result.output,
            ExecutionMode::Write => result.output,
            ExecutionMode::Stats => result.output.to_stats(),
            ExecutionMode::Stream => result.output,
        }
    },
}
```

**Generated**: ~580 lines of implementation code

---

## Implementation Strategy

### Phase 1: Config Macro (~2-3 hours)

**File**: `src/procedure/codegen/config_macro.rs`

**Goals**:

- Parse attribute annotations (#[default], #[range], etc.)
- Generate struct definition
- Generate builder with validation
- Generate Default impl
- Add Serialize/Deserialize

**Test with**:

```rust
algorithm_config! {
    pub struct SimpleConfig {
        #[default(42)]
        #[min(0)]
        pub value: i32,
    }
}

let config = SimpleConfig::builder().value(100)?.build()?;
assert_eq!(config.value, 100);
```

### Phase 2: Algorithm Macro (~3-4 hours)

**File**: `src/procedure/codegen/algorithm_macro.rs`

**Goals**:

- Parse algorithm declaration syntax
- Generate AlgorithmSpec implementation
- Generate execution mode wrappers
- Generate catalog registration

**Test with**:

```rust
define_algorithm! {
    name: Trivial,
    config: TrivialConfig,
    result: i32,
    algorithm: TrivialAlgorithm,
    modes: [mutate],
    execute: |_, _, _, _| Ok(42),
    consume_result: |result, _| result.output,
}

let algo = TrivialAlgorithm::new("g".into(), TrivialConfig::default());
assert_eq!(algo.name(), "Trivial");
```

### Phase 3: Integration & Documentation (~2 hours)

**Files**:

- `src/procedure/codegen/mod.rs` - Module interface
- `doc/PROCEDURE_SUBSYSTEM_GUIDE.md` - Usage guide
- `examples/algorithm_macro_showcase.rs` - Examples

**Test with**: Real algorithm (DegreeCount or simpler)

---

## Benefits Summary

### Code Reduction

- **Config**: 80 lines → 15 lines (81% reduction)
- **AlgorithmSpec**: 150 lines → 0 lines (100% reduction, in macro)
- **Execution Modes**: 200 lines → 0 lines (100% reduction, in macro)
- **Total**: ~450 lines → ~30 lines (**93% reduction**)

### Consistency

- All algorithms follow same pattern
- No copy-paste errors
- Centralized validation logic
- Single source of truth

### Maintainability

- Change macro once, all algorithms updated
- Easy to add new execution modes
- Clear separation: declaration vs implementation
- Self-documenting via attributes

### Developer Experience

- Focus on algorithm logic, not plumbing
- Type-safe by construction
- Compile-time validation
- Excellent error messages from generated code

---

## Open Design Questions

### 1. Macro Complexity vs Flexibility

**Trade-off**: More complex macro = more features, but harder to debug

**Options**:

- **A**: Simple macro, limited features (current design)
- **B**: Complex macro, more automation (hooks, validation, etc.)

**Recommendation**: Start with A, add B features incrementally

### 2. Error Messages

**Challenge**: Macro-generated code can have obscure errors

**Strategies**:

- Use `compile_error!` for invalid attribute combinations
- Generate code with clear variable names
- Add `#[doc(hidden)]` to internal generated items
- Provide examples of common patterns

### 3. Testing Strategy

**Options**:

- **A**: Macro expansion tests (use `macrotest` crate)
- **B**: Integration tests with generated code
- **C**: Both

**Recommendation**: C - test both macro expansion and runtime behavior

---

## Next Steps

1. ✅ Review this design document
2. ⬜ Implement `algorithm_config!` macro (Phase 1)
3. ⬜ Test config macro with simple examples
4. ⬜ Implement `define_algorithm!` macro (Phase 2)
5. ⬜ Test algorithm macro with trivial algorithm
6. ⬜ Document usage patterns
7. ⬜ Apply to real algorithm (DegreeCount or similar)

**Timeline**: 7-9 hours total (full day of work)

---

## Appendix: Macro Expansion Example

### Input:

```rust
algorithm_config! {
    pub struct DegreeConfig {
        #[default(false)]
        pub weighted: bool,
    }
}
```

### Expanded Output:

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DegreeConfig {
    pub weighted: bool,
}

#[derive(Default)]
pub struct DegreeConfigBuilder {
    weighted: Option<bool>,
}

impl DegreeConfigBuilder {
    pub fn weighted(mut self, value: bool) -> Self {
        self.weighted = Some(value);
        self
    }

    pub fn build(self) -> Result<DegreeConfig, crate::errors::ConfigError> {
        Ok(DegreeConfig {
            weighted: self.weighted.unwrap_or(false),
        })
    }
}

impl Default for DegreeConfig {
    fn default() -> Self {
        Self::builder().build().expect("default config should be valid")
    }
}

impl DegreeConfig {
    pub fn builder() -> DegreeConfigBuilder {
        DegreeConfigBuilder::default()
    }
}
```

**Result**: 15 lines input → 32 lines output (still huge savings when considering full algorithm stack)

---

**Ready to implement! 🚀**
