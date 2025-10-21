# Config System Cleanup Plan

## Executive Summary

The **Config System** is actually a **major success story** and the **perfect template** for broader codegen consolidation. The `define_config!` macro demonstrates exactly what we want: **declarative DSL → boilerplate generation**.

## Current Config System Success

### What Works Brilliantly

**Input** (10 lines of DSL):
```rust
define_config!(
    pub struct PageRankConfig {
        validate = |cfg: &PageRankConfig| {
            validate_positive(cfg.base.concurrency as f64, "concurrency")?;
            validate_positive(cfg.max_iterations as f64, "maxIterations")?;
            validate_range(cfg.damping_factor, 0.0, 1.0, "dampingFactor")?;
            validate_positive(cfg.tolerance, "tolerance")?;
            Ok(())
        },
        base: AlgoBaseConfig = AlgoBaseConfig::default(),
        max_iterations: usize = 20,
        tolerance: f64 = 0.0000001,
        damping_factor: f64 = 0.85,
        source_nodes: Option<Vec<String>> = None,
    }
);
```

**Output** (Generated automatically):
- ✅ `PageRankConfig` struct with fields
- ✅ `Default` implementation
- ✅ `PageRankConfigBuilder` with fluent methods
- ✅ `validate()` method with custom validation
- ✅ `Config` trait implementation
- ✅ Type-safe builder pattern

**Result**: ~10 lines of DSL → ~100 lines of boilerplate!

## Config System as Template

This is **exactly** the pattern we want for:

1. **Algorithm Macros** - Generate `AlgorithmSpec` implementations
2. **Application Macros** - Generate Application facades  
3. **TypeScript Codegen** - Generate NAPI bindings

## Phase 1: Config System Cleanup

### Current Issues to Fix

1. **Over-complex macro** - The `define_config!` macro is ~170 lines
2. **Unused abstractions** - `FormShape`, `Container`, `ValidationUtils` not used
3. **Inconsistent validation** - Mix of `ConfigValidation::` and direct validation
4. **Missing features** - No JSON parsing, no serde integration

### Cleanup Goals

**Simplify the macro**:
```rust
// Current (complex)
define_config!(
    pub struct PageRankConfig {
        validate = |cfg: &PageRankConfig| { /* complex validation */ },
        field: Type = default,
    }
);

// Target (simpler)
define_config! {
    name: PageRankConfig,
    fields: {
        damping_factor: f64 = 0.85 { range: 0.0..1.0 },
        tolerance: f64 = 1e-6 { validate: |v| v > 0.0 },
        max_iterations: usize = 20,
        source_nodes: Option<Vec<String>> = None,
    }
}
```

**Add missing features**:
- JSON parsing from `serde_json::Value`
- Serde integration (`Serialize`, `Deserialize`)
- Better error messages
- Documentation generation

### Implementation Plan

1. **Audit current usage** - Which configs actually use the macro?
2. **Simplify macro** - Reduce from 170 lines to ~80 lines
3. **Add JSON parsing** - Generate `from_json()` method
4. **Add serde support** - Generate `Serialize`/`Deserialize`
5. **Test with existing configs** - Ensure PageRank, Pregel still work
6. **Document the pattern** - Create usage guide

## Phase 2: Algorithm Macro Design

### Using Config Pattern for Algorithms

**Target DSL**:
```rust
define_algorithm! {
    name: "pagerank",
    category: Centrality,
    
    config: {
        damping_factor: f64 = 0.85 { range: 0.0..1.0 },
        tolerance: f64 = 1e-6 { validate: |v| v > 0.0 },
        max_iterations: usize = 100,
        source_nodes: Option<Vec<u64>> = None,
        weight_property: Option<String> = None,
    },
    
    result: PageRankResult {
        scores: Vec<f64>,
        iterations: usize,
        converged: bool,
        execution_time: Duration,
    },
    
    projection_hint: Dense,
    modes: [Stream, Stats],
    
    // Developer writes ONLY this
    execute: |graph_store, config, context| {
        // Actual algorithm logic
        // config.damping_factor is already parsed!
    }
}
```

**What it generates**:
1. **Config struct** - Using `define_config!` internally
2. **Result struct** - With `Debug`, `Clone`, `Serialize`
3. **AlgorithmSpec implementation** - All boilerplate methods
4. **JSON parsing** - `parse_config()` method
5. **Mode handling** - `consume_result()` method
6. **Tests** - Config parsing, validation, modes

### Implementation Strategy

**Step 1**: Extract config generation to `define_config!`
**Step 2**: Add result struct generation
**Step 3**: Add AlgorithmSpec boilerplate generation
**Step 4**: Add JSON parsing integration
**Step 5**: Test with PageRank & Sum

## Phase 3: Application Macro Design

### Using Config Pattern for Applications

**Target DSL**:
```rust
define_application! {
    name: StreamNodeProperties,
    category: Stream,
    
    dependencies: {
        catalog: GraphStoreCatalogService,
        logger: UserLogRegistryFactory,
    },
    
    method: stream {
        params: {
            graph_name: &str,
            properties: Vec<String>,
            user: &User,
        },
        returns: Vec<GraphStreamNodePropertiesResult>,
        
        implementation: |self, graph_name, properties, user| {
            // Business logic
        }
    }
}
```

**What it generates**:
1. **Application struct** - With service dependencies
2. **Constructor** - Builder pattern for dependencies
3. **Methods** - With timing, logging, error handling
4. **TypeScript facade** - Future NAPI integration

## Phase 4: TypeScript Codegen

### Future Integration

Once Applications are generated, we can add TypeScript codegen:

```rust
// From algorithm descriptor
export class PageRankAlgorithm {
    async stream(graphName: string, config: PageRankConfig): Promise<PageRankResult> {
        return this.backend.call("pagerank.stream", { graphName, config });
    }
}

// From application descriptor  
export class StreamNodePropertiesApplication {
    async stream(
        graphName: string,
        properties: string[],
        user: User
    ): Promise<GraphStreamNodePropertiesResult[]> {
        return this.backend.call("stream_node_properties", { graphName, properties, user });
    }
}
```

## Migration Strategy

### Step 1: Config System Cleanup (Week 1)

1. **Audit** - Find all `define_config!` usage
2. **Simplify** - Reduce macro complexity
3. **Add features** - JSON parsing, serde support
4. **Test** - Ensure existing configs work
5. **Document** - Create usage guide

### Step 2: Algorithm Macro (Week 2)

1. **Design** - Create `define_algorithm!` DSL
2. **Implement** - Generate AlgorithmSpec boilerplate
3. **Test** - Replace PageRank & Sum spec.rs
4. **Validate** - Ensure executor still works

### Step 3: Application Macro (Week 3)

1. **Design** - Create `define_application!` DSL
2. **Implement** - Generate Application facades
3. **Test** - Replace existing applications
4. **Validate** - Ensure applications work

### Step 4: Documentation (Week 4)

1. **Consolidate** - Move unused abstractions to `doc/archive/`
2. **Document** - Create comprehensive guide
3. **Examples** - Show before/after comparisons
4. **Migration** - Guide for future algorithms

## Success Criteria

After cleanup:

1. ✅ **Config macro** simplified from 170 to ~80 lines
2. ✅ **Algorithm macro** generates AlgorithmSpec from ~40 lines of DSL
3. ✅ **Application macro** generates Application facades from ~30 lines of DSL
4. ✅ **PageRank & Sum** use new macro system
5. ✅ **All tests pass** - No regression
6. ✅ **Documentation** explains the pattern
7. ✅ **TypeScript codegen** ready for future integration

## The Big Picture

This cleanup creates a **unified codegen pattern**:

```
Developer writes:
├── define_config! { /* config schema */ }           ← Config System
├── define_algorithm! { /* algorithm spec */ }       ← Algorithm System  
└── define_application! { /* application spec */ }   ← Application System

Macros generate:
├── Config structs + builders + validation           ← Boilerplate
├── AlgorithmSpec implementations + tests            ← Boilerplate
├── Application facades + methods + error handling   ← Boilerplate
└── TypeScript facades (future)                      ← GDSL Integration
```

**Philosophy**: Move complexity from **runtime abstractions** to **compile-time code generation**.

## Next Steps

1. **Start with Config** - Clean up the successful pattern
2. **Extend to Algorithms** - Use config pattern as template
3. **Extend to Applications** - Use same pattern
4. **Future TypeScript** - Generate NAPI bindings

The Config System is the **perfect foundation** for this broader consolidation!
